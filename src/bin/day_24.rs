extern crate regex;

use regex::Regex;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::fmt;
use std::io::Read;
use std::iter::FromIterator;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let state: State = data.parse().unwrap();
    println!("part_1: {}", part_1(&state));
    println!("part_2: {}", part_2(&state).1);
}

fn part_1(state_0: &State) -> usize {
    let mut state = state_0.clone();
    let _winner = state.resolve();
    state.groups.iter().map(|g| g.unit_count).sum()
}

fn part_2(state_0: &State) -> (usize, usize) {
    let mut state = state_0.clone();
    let mut boost_min = 0;
    let mut boost_max = 1;
    state.boost_immune_system(boost_max);
    while state.resolve() != UnitType::ImmuneSystem {
        boost_min = boost_max;
        boost_max *= 2;
        state = state_0.clone();
        state.boost_immune_system(boost_max);
    }
    while boost_min < boost_max {
        let boost_mid = (boost_min + boost_max) / 2;
        state = state_0.clone();
        state.boost_immune_system(boost_mid);
        if state.resolve() == UnitType::ImmuneSystem {
            boost_max = boost_mid;
        } else {
            boost_min = boost_mid + 1;
        }
    }
    // need to play it out once more because final boost might not have been boost_min
    state = state_0.clone();
    state.boost_immune_system(boost_min);
    state.resolve();
    (boost_min, state.groups.iter().map(|g| g.unit_count).sum())
}

type AttackType = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum UnitType {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct UnitGroup {
    id: usize,
    unit_type: UnitType,
    unit_count: usize,
    /// per unit hp
    hit_points: usize,
    /// per unit dmg
    attack_damage: usize,
    attack_type: AttackType,
    initiative: usize,
    weaknesses: HashSet<AttackType>,
    immunities: HashSet<AttackType>,
}

impl UnitGroup {
    fn effective_power(&self) -> usize {
        self.unit_count * self.attack_damage
    }

    fn attack_potential(&self, other: &UnitGroup) -> usize {
        if other.immunities.contains(&self.attack_type) {
            return 0;
        }
        let multiplier = if other.weaknesses.contains(&self.attack_type) {
            2
        } else {
            1
        };
        multiplier * self.effective_power()
    }

    fn is_enemy(&self, other: &UnitGroup) -> bool {
        self.unit_type != other.unit_type
    }

    fn take_damage(&mut self, damage: usize) -> usize {
        let casualties = (damage / self.hit_points).min(self.unit_count);
        self.unit_count -= casualties;
        casualties
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    groups: Vec<UnitGroup>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
struct Casualties {
    immune_system: usize,
    infection: usize,
}

impl State {
    fn boost_immune_system(&mut self, boost: usize) {
        for g in self.groups.iter_mut() {
            if g.unit_type == UnitType::ImmuneSystem {
                g.attack_damage += boost;
            }
        }
    }

    fn fight(&mut self) -> Casualties {
        let n = self.groups.len();

        let mut by_power: Vec<_> = (0..n).collect();
        by_power.sort_by_key(|&i| {
            let g = &self.groups[i];
            Reverse((g.effective_power(), g.initiative))
        });

        let mut by_initiative: Vec<_> = (0..n).collect();
        by_initiative.sort_by_key(|&i| {
            let g = &self.groups[i];
            Reverse(g.initiative)
        });

        // target selection
        let mut target_pool = HashSet::<usize>::from_iter(0..n);
        let mut targets: Vec<_> = std::iter::repeat(None).take(n).collect();
        for &i in by_power.iter() {
            let attacker = &self.groups[i];
            if let Some(&j) = target_pool
                .iter()
                .filter(|&&j| attacker.is_enemy(&self.groups[j]))
                .max_by_key(|&&j| {
                    (
                        attacker.attack_potential(&self.groups[j]),
                        self.groups[j].effective_power(),
                        self.groups[j].initiative,
                    )
                })
            {
                if attacker.attack_potential(&self.groups[j]) > 0 {
                    target_pool.remove(&j);
                    targets[i] = Some(j);
                }
            }
        }
        // targets[i] is the target selected by self.groups[i]

        let mut casualties = Casualties::default();
        // attacking
        for &i in by_initiative.iter() {
            if let Some(j) = targets[i] {
                let damage = self.groups[i].attack_potential(&self.groups[j]);
                let attack_casualties = self.groups[j].take_damage(damage);
                match self.groups[j].unit_type {
                    UnitType::Infection => casualties.infection += attack_casualties,
                    UnitType::ImmuneSystem => casualties.immune_system += attack_casualties,
                };
                /*
                println!(
                    "{} group {} does {} damage to {} group {}, killing {}",
                    self.groups[i].unit_type,
                    self.groups[i].id,
                    damage,
                    self.groups[j].unit_type,
                    self.groups[j].id,
                    attack_casualties
                );
                */
            }
        }

        // garbage collection
        self.groups = self
            .groups
            .iter()
            .filter(|g| g.unit_count > 0)
            .cloned()
            .collect();

        casualties
    }

    fn winner(&self) -> Option<UnitType> {
        if self
            .groups
            .iter()
            .all(|g| g.unit_count == 0 || g.unit_type == UnitType::ImmuneSystem)
        {
            Some(UnitType::ImmuneSystem)
        } else if self
            .groups
            .iter()
            .all(|g| g.unit_count == 0 || g.unit_type == UnitType::Infection)
        {
            Some(UnitType::Infection)
        } else {
            None
        }
    }

    fn resolve(&mut self) -> UnitType {
        loop {
            let casualties = self.fight();
            if let Some(winner) = self.winner() {
                return winner;
            }
            // stalemate
            if casualties == Casualties::default() {
                return UnitType::Infection;
            }
        }
    }
}

impl std::str::FromStr for State {
    type Err = &'static str;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let outer_re =
            Regex::new("(?P<unit_type>Immune System|Infection):\n(?P<groups>(.\n?)+)").unwrap();
        let inner_re = Regex::new(r"(?P<unit_count>\d+) units each with (?P<hp>\d+) hit points (?P<specialties>\(.*\) )?with an attack that does (?P<dmg>\d+) (?P<attack_type>\w+) damage at initiative (?P<initiative>\d+)").unwrap();
        let immunities_re = Regex::new(r"immune to ([^);]+)").unwrap();
        let weaknesses_re = Regex::new(r"weak to ([^);]+)").unwrap();
        let mut state = State { groups: Vec::new() };
        for outer_caps in outer_re.captures_iter(data) {
            let unit_type_str = outer_caps.name("unit_type").ok_or("no unit_type")?.as_str();
            let groups_str = outer_caps.name("groups").ok_or("no groups")?.as_str();
            let unit_type = match unit_type_str {
                "Immune System" => UnitType::ImmuneSystem,
                "Infection" => UnitType::Infection,
                _ => return Err("invalid unit type"),
            };
            for (i, inner_caps) in inner_re.captures_iter(groups_str).enumerate() {
                let unit_count: usize = inner_caps
                    .name("unit_count")
                    .ok_or("no unit_count")?
                    .as_str()
                    .parse()
                    .map_err(|_| "invalid unit_count")?;
                let hit_points: usize = inner_caps
                    .name("hp")
                    .ok_or("no hp")?
                    .as_str()
                    .parse()
                    .map_err(|_| "invalid hp")?;
                let attack_type = inner_caps
                    .name("attack_type")
                    .ok_or("no attack type")?
                    .as_str();
                let attack_damage: usize = inner_caps
                    .name("dmg")
                    .ok_or("no dmg")?
                    .as_str()
                    .parse()
                    .map_err(|_| "invalid dmg")?;

                let mut immunities = HashSet::new();
                let mut weaknesses = HashSet::new();

                if let Some(specialties_str) = inner_caps.name("specialties").map(|m| m.as_str()) {
                    if let Some(immunities_caps) = immunities_re.captures(specialties_str) {
                        immunities = HashSet::from_iter(
                            immunities_caps
                                .get(1)
                                .unwrap()
                                .as_str()
                                .split(", ")
                                .map(|s| s.to_owned()),
                        );
                    }
                    if let Some(weaknesses_caps) = weaknesses_re.captures(specialties_str) {
                        weaknesses = HashSet::from_iter(
                            weaknesses_caps
                                .get(1)
                                .unwrap()
                                .as_str()
                                .split(", ")
                                .map(|s| s.to_owned()),
                        );
                    }
                }
                let initiative: usize = inner_caps
                    .name("initiative")
                    .ok_or("no initiative")?
                    .as_str()
                    .parse()
                    .map_err(|_| "invalid initiative")?;
                let group = UnitGroup {
                    id: i + 1,
                    unit_type: unit_type,
                    unit_count: unit_count,
                    hit_points: hit_points,
                    attack_damage: attack_damage,
                    attack_type: attack_type.to_string(),
                    initiative: initiative,
                    immunities: immunities,
                    weaknesses: weaknesses,
                };
                state.groups.push(group);
            }
        }
        Ok(state)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Immune System:")?;
        for g in self
            .groups
            .iter()
            .filter(|g| g.unit_type == UnitType::ImmuneSystem)
        {
            writeln!(f, "Group {}: {} Units", g.id, g.unit_count)?;
        }
        writeln!(f, "Infection:")?;
        for g in self
            .groups
            .iter()
            .filter(|g| g.unit_type == UnitType::Infection)
        {
            writeln!(f, "Group {}: {} Units", g.id, g.unit_count)?;
        }
        Ok(())
    }
}

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnitType::ImmuneSystem => write!(f, "Immune System"),
            UnitType::Infection => write!(f, "Infection"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        use State;
        let input = r"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let state: State = input.parse().unwrap();
        assert_eq!(5216, part_1(&state));
    }

    #[test]
    fn test_part_2() {
        use part_2;
        use State;
        let input = r"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let state: State = input.parse().unwrap();
        assert_eq!((1570, 51), part_2(&state));
    }
}
