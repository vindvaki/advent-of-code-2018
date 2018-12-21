#[macro_use]
extern crate error_chain;

extern crate aoc2018;

use aoc2018::Mat;
use std::fmt;
use std::io::Read;

mod errors {
    error_chain!{}
}

use errors::*;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let collection: Collection = data.parse().unwrap();
    println!("part_1: {}", part_1(&collection));
    println!("part_2: {}", part_2(&collection));
}

fn part_1(collection_0: &Collection) -> usize {
    let mut collection = collection_0.clone();
    for _minute in 1..=10 {
        collection = next_minute(&collection);
    }
    value(&collection)
}

fn part_2(collection_0: &Collection) -> usize {
    let mut collection = collection_0.clone();
    let mut minute_seen = std::collections::HashMap::new();
    let mut seen_at = Vec::new();
    let mut minute = 0;
    let mut cycle_start = 0;
    let limit = 1_000_000_000;
    seen_at.push(collection.clone());
    while minute <= limit {
        minute += 1;
        collection = next_minute(&collection);
        seen_at.push(collection.clone());
        if let Some(&seen_minute) = minute_seen.get(&collection) {
            cycle_start = seen_minute;
            break;
        }
        minute_seen.insert(collection.clone(), minute);
    }
    let cycle_length = minute - cycle_start;
    let last_minute_period = (limit - minute) % cycle_length;
    value(&seen_at[cycle_start + last_minute_period])
}

fn value(collection: &Collection) -> usize {
    use Acre::*;
    let mut lumber = 0;
    let mut tree = 0;
    for row in 0..collection.rows() {
        for col in 0..collection.cols() {
            match *collection.get(row, col) {
                Lumber => lumber += 1,
                Tree => tree += 1,
                _ => (),
            };
        }
    }
    lumber * tree
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Acre {
    Open,
    Tree,
    Lumber,
}

impl fmt::Display for Acre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Acre::*;
        let c = match self {
            Open => '.',
            Tree => '|',
            Lumber => '#',
        };
        write!(f, "{}", c)
    }
}

impl Default for Acre {
    fn default() -> Acre {
        Acre::Open
    }
}

impl std::str::FromStr for Acre {
    type Err = Error;

    fn from_str(data: &str) -> Result<Self> {
        use Acre::*;
        match data {
            "." => Ok(Open),
            "|" => Ok(Tree),
            "#" => Ok(Lumber),
            _ => bail!("Invalid acre {}", data.to_owned()),
        }
    }
}

type Collection = Mat<Acre>;

fn next_minute(collection: &Collection) -> Collection {
    use Acre::*;
    let mut next_collection = collection.clone();
    for row0 in 0..collection.rows() {
        for col0 in 0..collection.cols() {
            let mut adjacent = std::collections::HashMap::new();
            for row in row0.max(1) - 1..(row0 + 2).min(collection.rows()) {
                for col in col0.max(1) - 1..(col0 + 2).min(collection.cols()) {
                    if row == row0 && col == col0 {
                        continue;
                    }
                    *adjacent.entry(collection.get(row, col)).or_insert(0) += 1;
                }
            }
            let acre = *collection.get(row0, col0);
            let next_acre = match acre {
                Open => {
                    if *adjacent.get(&Tree).unwrap_or(&0) >= 3 {
                        Tree
                    } else {
                        Open
                    }
                }
                Tree => {
                    if *adjacent.get(&Lumber).unwrap_or(&0) >= 3 {
                        Lumber
                    } else {
                        Tree
                    }
                }
                Lumber => {
                    if *adjacent.get(&Lumber).unwrap_or(&0) > 0
                        && *adjacent.get(&Tree).unwrap_or(&0) > 0
                    {
                        Lumber
                    } else {
                        Open
                    }
                }
            };
            next_collection.set(row0, col0, next_acre);
        }
    }
    return next_collection;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        use Collection;

        let data = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let collection: Collection = data.parse().unwrap();
        assert_eq!(1147, part_1(&collection));
    }

    #[test]
    fn test_next_minute() {
        use next_minute;
        use Collection;
        let data = vec![
            r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.",
            r".......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|.",
            r".......#..
......|#..
.|.|||....
..##|||..#
..###|||#|
...#|||||.
|||||||||.
||||||||||
||||||||||
.|||||||||",
            r".......#..
....|||#..
.|.||||...
..###|||.#
...##|||#|
.||##|||||
||||||||||
||||||||||
||||||||||
||||||||||",
            r".....|.#..
...||||#..
.|.#||||..
..###||||#
...###||#|
|||##|||||
||||||||||
||||||||||
||||||||||
||||||||||",
            r"....|||#..
...||||#..
.|.##||||.
..####|||#
.|.###||#|
|||###||||
||||||||||
||||||||||
||||||||||
||||||||||",
            r"...||||#..
...||||#..
.|.###|||.
..#.##|||#
|||#.##|#|
|||###||||
||||#|||||
||||||||||
||||||||||
||||||||||",
            r"...||||#..
..||#|##..
.|.####||.
||#..##||#
||##.##|#|
|||####|||
|||###||||
||||||||||
||||||||||
||||||||||",
            r"..||||##..
..|#####..
|||#####|.
||#...##|#
||##..###|
||##.###||
|||####|||
||||#|||||
||||||||||
||||||||||",
            r"..||###...
.||#####..
||##...##.
||#....###
|##....##|
||##..###|
||######||
|||###||||
||||||||||
||||||||||",
            r".||##.....
||###.....
||##......
|##.....##
|##.....##
|##....##|
||##.####|
||#####|||
||||#|||||
||||||||||",
        ];
        let mut collection: Collection = data[0].parse().unwrap();
        for i in 1..=10 {
            collection = next_minute(&collection);
            if collection.to_string().trim() != data[i] {
                panic!(
                    "Minute {}. Collections should be equal. Expected\n{}\nbut got\n{}",
                    i, data[i], collection,
                );
            }
        }
    }
}
