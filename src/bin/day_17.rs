extern crate aoc2018;
extern crate regex;

use aoc2018::Mat;
use regex::Regex;
use std::fmt;
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let (ground, origin) = parse_ground(&data).unwrap();
    println!("part_1: {}", part_1(&ground, origin));
    println!("part_1: {}", part_2(&ground, origin));
}

fn part_1(ground_0: &Ground, origin: (usize, usize)) -> usize {
    let mut result = 0;
    let mut ground = ground_0.clone();
    drip(&mut ground, 0, 500 - origin.1, DripDirection::Down);
    for row in 0..ground.rows() {
        for col in 0..ground.cols() {
            match *ground.get(row, col) {
                Square::Water | Square::Flow => result += 1,
                _ => (),
            };
        }
    }
    result
}

fn part_2(ground_0: &Ground, origin: (usize, usize)) -> usize {
    let mut result = 0;
    let mut ground = ground_0.clone();
    drip(&mut ground, 0, 500 - origin.1, DripDirection::Down);
    for row in 0..ground.rows() {
        for col in 0..ground.cols() {
            match *ground.get(row, col) {
                Square::Water => result += 1,
                _ => (),
            };
        }
    }
    result
}

fn drip(ground: &mut Ground, row: usize, col: usize, dir: DripDirection) -> bool {
    use DripDirection::*;
    use Square::*;
    if row == ground.rows() || col == ground.cols() {
        return true;
    }
    match *ground.get(row, col) {
        Water | Clay => return false,
        Sand | Flow => (),
    };
    if col == 0 {
        return true;
    }
    ground.set(row, col, Flow);
    if drip(ground, row + 1, col, Down) {
        return true;
    }
    let left = dir != Right && drip(ground, row, col - 1, Left);
    let right = dir != Left && drip(ground, row, col + 1, Right);
    if !(left || right) {
        ground.set(row, col, Water);
        return false;
    }
    if !left && right {
        let mut icol = col - 1;
        while *ground.get(row, icol) == Water {
            ground.set(row, icol, Flow);
            if icol == 0 {
                break;
            }
            icol -= 1;
        }
    }
    if left && !right {
        let mut icol = col + 1;
        while icol < ground.cols() && *ground.get(row, icol) == Water {
            ground.set(row, icol, Flow);
            icol += 1;
        }
    }
    true
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum DripDirection {
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Square {
    Sand,
    Clay,
    Flow,
    Water,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Square::*;
        let c = match self {
            Sand => '.',
            Clay => '#',
            Flow => '|',
            Water => '~',
        };
        write!(f, "{}", c)?;
        Ok(())
    }
}

type Ground = Mat<Square>;

fn parse_ground(data: &str) -> Result<(Ground, (usize, usize)), &'static str> {
    let x_re = Regex::new(r"x=(\d+)(\.\.(\d+))?").unwrap();
    let y_re = Regex::new(r"y=(\d+)(\.\.(\d+))?").unwrap();
    let parse_range = |re: &Regex, data: &str| -> Result<std::ops::Range<usize>, &'static str> {
        let caps = re.captures(data).ok_or("No matches")?;
        let start_str = caps.get(1).ok_or("Range start not found")?.as_str();
        let start = start_str
            .parse::<usize>()
            .map_err(|_| "Unable to parse start")?;
        let end = caps
            .get(3)
            .map(|s| {
                s.as_str()
                    .parse::<usize>()
                    .map_err(|_| "Unable to parse end")
            }).unwrap_or(Ok(start))?;
        Ok(start..end + 1)
    };

    let mut clay_ranges = Vec::new();
    let mut x_min = usize::max_value();
    let mut x_max = usize::min_value();
    let mut y_min = usize::max_value();
    let mut y_max = usize::min_value();
    for line in data.lines() {
        let x_range = parse_range(&x_re, line)?;
        let y_range = parse_range(&y_re, line)?;
        x_min = x_min.min(x_range.start);
        x_max = x_max.max(x_range.end);
        y_min = y_min.min(y_range.start);
        y_max = y_max.max(y_range.end);
        clay_ranges.push((x_range, y_range));
    }
    x_min -= 2;
    x_max += 1;
    let rows = y_max - y_min;
    let cols = x_max - x_min;
    let mut ground = Ground::new(rows, cols, Square::Sand);
    for (x_range, y_range) in clay_ranges.iter() {
        for y in y_range.clone() {
            for x in x_range.clone() {
                ground.set(y - y_min, x - x_min, Square::Clay);
            }
        }
    }
    Ok((ground, (y_min, x_min)))
}

#[cfg(test)]
mod tests {
    const DATA: &'static str = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn test_part_1() {
        use parse_ground;
        use part_1;

        let (ground, origin) = parse_ground(&DATA).unwrap();
        println!("{}", ground);
        assert_eq!(57, part_1(&ground, origin));
    }

    #[test]
    fn test_part_2() {
        use parse_ground;
        use part_2;

        let (ground, origin) = parse_ground(&DATA).unwrap();
        println!("{}", ground);
        assert_eq!(29, part_2(&ground, origin));
    }
}
