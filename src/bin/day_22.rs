extern crate regex;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let re = regex::Regex::new(
        r"^depth: (?P<depth>\d+)\s+target: (?P<target_x>[\d]+),(?P<target_y>[\d]+)\s*$",
    )
    .unwrap();
    let caps = re.captures(&data).expect("unable to parse input");
    let depth: usize = caps
        .name("depth")
        .expect("unable to find depth")
        .as_str()
        .parse()
        .expect("unable to parse depth");
    let target_x: usize = caps
        .name("target_x")
        .expect("unable to find target_x")
        .as_str()
        .parse()
        .expect("unable to parse target_x");
    let target_y: usize = caps
        .name("target_y")
        .expect("unable to find target_y")
        .as_str()
        .parse()
        .expect("unable to parse target_y");
    let target = (target_y, target_x);
    let mut cs = CaveSystem::new(depth, target);
    println!("part_1: {}", part_1(&mut cs));
    println!("part_2: {}", part_2(&mut cs));
}

fn part_1(cs: &mut CaveSystem) -> usize {
    let mut risk_sum = 0;
    for row in 0..=cs.target.0 {
        for col in 0..=cs.target.1 {
            risk_sum += cs.terrain_at(row, col).risk();
        }
    }
    risk_sum
}

fn part_2(cs: &mut CaveSystem) -> usize {
    cs.shortest_distance()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

impl Terrain {
    fn risk(&self) -> usize {
        use Terrain::*;
        match self {
            Rocky => 0,
            Wet => 1,
            Narrow => 2,
        }
    }
    fn tools(&self) -> [Tool; 2] {
        use Terrain::*;
        use Tool::*;
        match self {
            Rocky => [ClimbingGear, Torch],
            Wet => [ClimbingGear, Neither],
            Narrow => [Torch, Neither],
        }
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Terrain::*;
        let c = match self {
            Rocky => '.',
            Wet => '=',
            Narrow => '|',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Tool {
    ClimbingGear,
    Torch,
    Neither,
}

impl Tool {
    fn can_enter(&self, terrain: Terrain) -> bool {
        terrain.tools().contains(&self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CaveSystem {
    depth: usize,
    target: (usize, usize),
    geological_index_map: HashMap<(usize, usize), usize>,
    terrain_map: HashMap<(usize, usize), Terrain>,
}

impl CaveSystem {
    fn new(depth: usize, target: (usize, usize)) -> CaveSystem {
        CaveSystem {
            depth: depth,
            target: target,
            geological_index_map: HashMap::new(),
            terrain_map: HashMap::new(),
        }
    }

    fn erosion_at(&mut self, row: usize, col: usize) -> usize {
        (self.geological_index_at(row, col) + self.depth) % 20183
    }

    fn geological_index_at(&mut self, row: usize, col: usize) -> usize {
        if self.geological_index_map.contains_key(&(row, col)) {
            return *self.geological_index_map.get(&(row, col)).unwrap();
        }
        let result = if (row, col) == (0, 0) || (row, col) == self.target {
            0
        } else {
            match (row, col) {
                (0, _) => 16807 * col,
                (_, 0) => 48271 * row,
                _ => {
                    let a = self.erosion_at(row - 1, col);
                    let b = self.erosion_at(row, col - 1);
                    a * b
                }
            }
        };
        self.geological_index_map.insert((row, col), result);
        result
    }

    fn terrain_at(&mut self, row: usize, col: usize) -> Terrain {
        use Terrain::*;
        match self.erosion_at(row, col) % 3 {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => unreachable!(),
        }
    }

    fn shortest_distance(&mut self) -> usize {
        use Tool::*;
        // binary heaps are max heaps, so we need Reverse values
        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), 0, 0, Torch));
        distances.insert((0, 0, Torch), 0);
        while !queue.is_empty() {
            let (Reverse(d), row, col, tool) = queue.pop().unwrap();
            if ((row, col), tool) == (self.target, Torch) {
                return d;
            }
            let terrain = self.terrain_at(row, col);
            let mut neighbors = Vec::new();
            for nrow in row.max(1) - 1..=row + 1 {
                if nrow == row {
                    continue;
                }
                neighbors.push((nrow, col));
            }
            for ncol in col.max(1) - 1..=col + 1 {
                if ncol == col {
                    continue;
                }
                neighbors.push((row, ncol));
            }
            for &(nrow, ncol) in neighbors.iter() {
                let nterrain = self.terrain_at(nrow, ncol);
                for &ntool in terrain.tools().iter() {
                    let neighbor = (nrow, ncol, ntool);
                    if !ntool.can_enter(nterrain) {
                        continue;
                    }
                    let new_distance = if ntool == tool { d + 1 } else { d + 8 };
                    let old_distance = *distances.get(&neighbor).unwrap_or(&(new_distance + 1));
                    if new_distance < old_distance {
                        distances.insert(neighbor, new_distance);
                        queue.push((Reverse(new_distance), nrow, ncol, ntool));
                    }
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        use CaveSystem;
        let mut cs = CaveSystem::new(510, (10, 10));
        assert_eq!(114, part_1(&mut cs));
        assert_eq!(cs.erosion_at(0, 1), 17317);
        assert_eq!(cs.erosion_at(1, 0), 8415);
        assert_eq!(cs.erosion_at(1, 1), 1805);
    }

    #[test]
    fn test_part_2() {
        use part_2;
        use CaveSystem;
        let mut cs = CaveSystem::new(510, (10, 10));
        assert_eq!(45, part_2(&mut cs));
    }
}
