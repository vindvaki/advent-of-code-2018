extern crate aoc2018;

#[macro_use]
extern crate error_chain;

use aoc2018::count_by_value;
use aoc2018::Mat;
use std::collections::HashMap;
use std::io::Read;

mod errors {
    error_chain!{}
}

use errors::*;

pub fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let coordinates = parse_coordinates(&data).unwrap();
    println!("part_1: {}", part_1(&coordinates));
    println!("part_2: {}", part_2(&coordinates, 10_000));
}

fn parse_coordinates(data: &str) -> Result<Vec<(usize, usize)>> {
    let mut result = Vec::new();
    for line in data.lines() {
        let mut iter = line.split(", ");
        let x: usize = iter
            .next()
            .chain_err(|| "No x coordinate")?
            .parse()
            .chain_err(|| "Unable to parse x coordinate")?;
        let y: usize = iter
            .next()
            .chain_err(|| "No y coordinate")?
            .parse()
            .chain_err(|| "Unable to parse y coordinate")?;
        result.push((x, y));
    }
    return Ok(result);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum NearestNeighborFlag {
    Closest(usize, usize),
    Neutral(usize),
    NotSeen,
}

fn extract_dimensions(coordinates: &Vec<(usize, usize)>) -> ((usize, usize), (usize, usize)) {
    let x_min = coordinates.iter().map(|c| c.0).min().unwrap();
    let y_min = coordinates.iter().map(|c| c.1).min().unwrap();
    let x_max = coordinates.iter().map(|c| c.0).max().unwrap();
    let y_max = coordinates.iter().map(|c| c.1).max().unwrap();
    let rows = y_max - y_min + 1;
    let cols = x_max - x_min + 1;
    ((x_min, y_min), (rows, cols))
}

fn part_1(coordinates: &Vec<(usize, usize)>) -> usize {
    use NearestNeighborFlag::*;

    let ((x_min, y_min), (rows, cols)) = extract_dimensions(coordinates);

    let mut state = Mat::new(rows, cols, NotSeen);
    for &(x0, y0) in coordinates {
        let x = x0 - x_min;
        let y = y0 - y_min;

        for u in 0..cols {
            for v in 0..rows {
                let d_xy = manhattan((u, v), (x, y));

                match state.get(v, u) {
                    &NotSeen => {
                        state.set(v, u, Closest(x, y));
                    }
                    &Neutral(d_uv) => {
                        if d_xy < d_uv {
                            state.set(v, u, Closest(x, y));
                        }
                    }
                    &Closest(a, b) => {
                        let d_uv = manhattan((a, b), (u, v));
                        if d_xy < d_uv {
                            state.set(v, u, Closest(x, y));
                        } else if d_xy == d_uv {
                            state.set(v, u, Neutral(d_uv));
                        }
                    }
                };
            }
        }
    }

    let mut is_finite = HashMap::new();
    for y in 0..rows {
        for x in 0..cols {
            // wasteful, but more concise
            if x == 0 || x == cols - 1 || y == 0 || y == rows - 1 {
                if let Closest(a, b) = state.get(y, x) {
                    is_finite.insert((a, b), false);
                }
            }
        }
    }
    let finite_closest = state.iter().filter(|flag| match flag {
        Closest(a, b) => *is_finite.get(&(a, b)).unwrap_or(&true),
        _ => false,
    });
    let area = count_by_value(finite_closest)
        .values()
        .max()
        .unwrap()
        .clone() as usize;
    return area;
}

fn part_2(coordinates: &Vec<(usize, usize)>, limit: usize) -> usize {
    let ((x_min, y_min), (rows, cols)) = extract_dimensions(coordinates);
    let mut filtered_coordinates = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let sum: usize = coordinates.iter().map(|(x0, y0)| {
                let x = x0 - x_min;
                let y = y0 - y_min;
                manhattan((x, y), (row, col))
            }).sum();
            if sum < limit {
                filtered_coordinates.push((row, col));
            }
        }
    }
    return filtered_coordinates.len();
}

fn manhattan((x, y): (usize, usize), (u, v): (usize, usize)) -> usize {
    (u.max(x) - u.min(x)) + (v.max(y) - v.min(y))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use parse_coordinates;
        use part_1;
        let data = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let coordinates = parse_coordinates(&data).unwrap();
        assert_eq!(part_1(&coordinates), 17);
        // panic!("I just want to print");
    }

    #[test]
    fn test_part_2() {
        use parse_coordinates;
        use part_2;
        let data = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let coordinates = parse_coordinates(&data).unwrap();
        assert_eq!(part_2(&coordinates, 32), 16);
        // panic!("I just want to print");
    }
}
