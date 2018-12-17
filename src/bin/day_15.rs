extern crate aoc2018;

use aoc2018::Mat;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io::Read;

pub fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let state = parse_state(&data).unwrap();
    println!("part_1: {:?}", part_1(&state));
    println!("part_2: {:?}", part_2(&state));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Elf(usize),
    Goblin(usize),
    Open,
    Wall,
}

impl Cell {
    fn is_enemy(&self, other: Cell) -> bool {
        use Cell::*;
        match self {
            Elf(_) => match other {
                Goblin(_) => true,
                _ => false,
            },
            Goblin(_) => match other {
                Elf(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn is_world(&self) -> bool {
        use Cell::*;
        match self {
            Open | Wall => true,
            _ => false,
        }
    }

    fn is_unit(&self) -> bool {
        !self.is_world()
    }

    fn is_elf(&self) -> bool {
        match self {
            Cell::Elf(_) => true,
            _ => false,
        }
    }

    fn is_goblin(&self) -> bool {
        match self {
            Cell::Goblin(_) => true,
            _ => false,
        }
    }

    fn hitpoints(&self) -> usize {
        use Cell::*;
        match *self {
            Open | Wall => usize::max_value(),
            Elf(hp) | Goblin(hp) => hp,
        }
    }

    fn attacked(&self, damage: usize) -> Cell {
        use Cell::*;
        match *self {
            Elf(hp) => {
                if hp <= damage {
                    Open
                } else {
                    Elf(hp - damage)
                }
            }
            Goblin(hp) => {
                if hp <= damage {
                    Open
                } else {
                    Goblin(hp - damage)
                }
            }
            _ => *self,
        }
    }
}

fn parse_cell(c: char) -> Result<Cell, &'static str> {
    use Cell::*;
    match c {
        '#' => Ok(Wall),
        '.' => Ok(Open),
        'E' => Ok(Elf(200)),
        'G' => Ok(Goblin(200)),
        _ => Err("Invalid cell"),
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Cell::*;
        let c = match self {
            Wall => '#',
            Open => '.',
            Elf(_) => 'E',
            Goblin(_) => 'G',
        };
        write!(f, "{}", c)?;
        Ok(())
    }
}

fn parse_state(data: &str) -> Result<State, &'static str> {
    let rows = data.lines().count();
    let cols = data.lines().next().unwrap_or("").chars().count();
    let mut map = Mat::new(rows, cols, Cell::Open);
    let mut units = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let cell = parse_cell(c)?;
            if cell.is_unit() {
                units.insert((row, col), cell);
            }
            map.set(row, col, cell);
        }
    }
    Ok(State {
        map: map,
        units: units,
        round: 0,
        elf_attack_power: 3,
    })
}

#[derive(Debug, Clone)]
struct State {
    map: Mat<Cell>,
    units: HashMap<(usize, usize), Cell>,
    round: usize,
    elf_attack_power: usize,
}

impl State {
    fn make_turn(&mut self) -> bool {
        let mut elf_count = self.units.iter().filter(|(_, c)| c.is_elf()).count();
        let mut goblin_count = self.units.iter().filter(|(_, c)| c.is_goblin()).count();
        let mut queue: Vec<_> = self.units.keys().map(|&k| k).collect();
        queue.sort_unstable_by(|a, b| a.cmp(b).reverse());
        while !queue.is_empty() {
            let pos = queue.pop().unwrap();

            let cell = match self.units.remove(&pos) {
                None => continue,
                Some(c) => c,
            };

            // move
            let next_pos = self.find_move(pos).unwrap_or(pos);
            self.units.insert(next_pos, cell);
            self.map.set(pos.0, pos.1, Cell::Open);
            self.map.set(next_pos.0, next_pos.1, cell);

            // attack
            if let Some(target) = self.find_target(next_pos) {
                if let Some(&target_cell) = self.units.get(&target) {
                    let next_target_cell = target_cell.attacked(self.attack_power(cell));
                    self.map.set(target.0, target.1, next_target_cell);
                    if next_target_cell.is_world() {
                        self.units.remove(&target);
                        if target_cell.is_elf() {
                            elf_count -= 1;
                        }
                        if target_cell.is_goblin() {
                            goblin_count -= 1;
                        }
                        if goblin_count == 0 || elf_count == 0 {
                            break;
                        }
                    } else {
                        self.units.insert(target, next_target_cell);
                    }
                }
            }
        }
        if !queue.iter().any(|p| self.units.contains_key(p)) {
            self.round += 1;
        }
        goblin_count > 0 && elf_count > 0
    }

    fn find_target(&self, p: (usize, usize)) -> Option<(usize, usize)> {
        let unit = *self.map.get(p.0, p.1);
        self.neighbors(p)
            .iter()
            .map(|n| (n, self.map.get(n.0, n.1)))
            .filter(|(_, other)| other.is_enemy(unit))
            .min_by_key(|(n, other)| (other.hitpoints(), n.0, n.1))
            .map(|(n, _)| *n)
    }

    fn find_move(&self, source: (usize, usize)) -> Option<(usize, usize)> {
        let s = *self.map.get(source.0, source.1);
        let mut targets = HashSet::new();
        for (p, t) in self.units.iter() {
            if t.is_enemy(s) {
                for n in self.neighbors(*p) {
                    targets.insert(n);
                }
            }
        }
        let mut seen = HashSet::new();
        seen.insert(source);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, source, None));
        let mut maybe_min_target = None;
        while !queue.is_empty() {
            let (d, p, q) = queue.pop_front().unwrap();
            if let Some((d0, _, _)) = maybe_min_target {
                if d0 < d {
                    break;
                }
            }
            if targets.contains(&p) {
                maybe_min_target = maybe_min_target
                    .or(Some((d, p, q)))
                    .map(|x| x.min((d, p, q)));
                // no point in looking at the neighbors
                continue;
            }
            for &n in self.neighbors(p).iter() {
                if seen.contains(&n) {
                    continue;
                }
                if *self.map.get(n.0, n.1) == Cell::Open {
                    seen.insert(n);
                    queue.push_back((d + 1, n, if d == 1 { Some(p) } else { q }));
                }
            }
        }
        maybe_min_target.map(|(_, p, q)| q.unwrap_or(p))
    }

    /// returns the lexicographically sorted neighbors
    fn neighbors(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        vec![
            (row - 1, col),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col),
        ]
    }

    fn resolve_battle(&mut self) {
        while self.make_turn() {}
    }

    fn outcome(&self) -> usize {
        self.round * self.hitpoints()
    }

    fn hitpoints(&self) -> usize {
        self.units.iter().map(|(_, u)| u.hitpoints()).sum::<usize>()
    }

    fn attack_power(&self, cell: Cell) -> usize {
        match cell {
            Cell::Elf(_) => self.elf_attack_power,
            Cell::Goblin(_) => 3,
            _ => 0,
        }
    }
}

fn part_1(state_0: &State) -> (usize, usize, usize) {
    let mut state = state_0.clone();
    state.resolve_battle();
    let round = state.round;
    let hp = state.hitpoints();
    (round, hp, round * hp)
}

fn part_2(state_0: &State) -> usize {
    // state.resolve_battle():
    let elves_at_start = state_0.units.values().filter(|u| u.is_elf()).count();
    for elf_attack_power in 0.. {
        let mut state = state_0.clone();
        state.elf_attack_power = elf_attack_power;
        state.resolve_battle();
        let elves_at_end = state.units.values().filter(|u| u.is_elf()).count();
        if elves_at_end == elves_at_start {
            return state.outcome();
        }
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_move() {
        use parse_state;
        let mut data = r"#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let mut state = parse_state(&data).unwrap();
        assert_eq!(Some((1, 2)), state.find_move((1, 1)));

        data = r"#######
#.E...#
#.....#
#...G.#
#######";
        state = parse_state(&data).unwrap();
        assert_eq!(Some((1, 3)), state.find_move((1, 2)));
    }

    #[test]
    fn test_make_turn() {
        use parse_state;
        let mut state = parse_state(
            &r"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########",
        ).unwrap();
        state.make_turn();
        println!("{}", state.map);
        assert_eq!(
            state.map.to_string().trim(),
            r"#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########"
        );
    }

    #[test]
    fn test_part_1() {
        use parse_state;
        use part_1;
        let mut data_map = Vec::new();
        data_map.push((
            r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
            (47, 590, 27730),
        ));
        data_map.push((
            r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
            (37, 982, 36334),
        ));

        data_map.push((
            r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
            (46, 859, 39514),
        ));

        data_map.push((
            r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
            (35, 793, 27755),
        ));

        data_map.push((
            r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
            (54, 536, 28944),
        ));

        data_map.push((
            r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
            (20, 937, 18740),
        ));

        for &(data, expected) in data_map.iter() {
            let state = parse_state(&data).unwrap();
            println!("{}", state.map);
            println!("{:?}", state.units);
            assert_eq!(expected, part_1(&state));
        }
    }
}
