extern crate aoc2018;

use aoc2018::Mat;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    println!("part_1: {}", part_1(&data));
    println!("part_2: {}", part_2(&data));
}

fn part_1(pattern: &str) -> usize {
    let paths = shortest_paths(pattern);
    *paths.values().max().unwrap_or(&0)
}

fn part_2(pattern: &str) -> usize {
    let paths = shortest_paths(pattern);
    paths.values().filter(|&&d| d >= 1000).count()
}

fn shortest_paths(pattern: &str) -> HashMap<(usize, usize), usize> {
    use Square::*;
    let (map, origin) = build_map(&pattern);
    let mut queue = VecDeque::new();
    let mut shortest_paths = HashMap::new();
    queue.push_back((0, origin));
    while !queue.is_empty() {
        let (d, (row, col)) = queue.pop_front().unwrap();
        for i in row.max(1) - 1..(row + 2).min(map.rows()) {
            for j in col.max(1) - 1..(col + 2).min(map.cols()) {
                if i == row && j == col {
                    continue;
                }
                match *map.get(i, j) {
                    HDoor | VDoor => {
                        let vdiff = (i as isize) - (row as isize);
                        let hdiff = (j as isize) - (col as isize);
                        let nrow = row as isize + 2 * vdiff;
                        let ncol = col as isize + 2 * hdiff;
                        if (nrow < 0 || ncol < 0)
                            || nrow as usize >= map.rows()
                            || ncol as usize >= map.cols()
                        {
                            continue;
                        }
                        let neighbor = (nrow as usize, ncol as usize);
                        if !shortest_paths.contains_key(&neighbor) {
                            queue.push_back((d + 1, neighbor));
                            shortest_paths.insert(neighbor, d + 1);
                        }
                    }
                    _ => (),
                };
            }
        }
    }
    shortest_paths
}

#[derive(Debug, Clone, Copy)]
enum Square {
    Wall,
    HDoor,
    VDoor,
    Room,
    Cursor,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Square::*;
        let c = match self {
            Wall => '#',
            HDoor => '-',
            VDoor => '|',
            Room => '.',
            Cursor => 'X',
        };
        write!(f, "{}", c)
    }
}

fn build_map(pattern: &str) -> (Mat<Square>, (usize, usize)) {
    use Square::*;
    let mut map = HashMap::new();
    map.insert((0, 0), Room);
    let mut row_min = 0;
    let mut row_max = 0;
    let mut col_min = 0;
    let mut col_max = 0;
    let mut row: isize = 0;
    let mut col: isize = 0;
    let mut stack = vec![(0, 0)];
    for c in pattern.trim().chars() {
        match c {
            '(' => {
                stack.push((row, col));
            }
            ')' => {
                let next = stack.pop().unwrap();
                row = next.0;
                col = next.1;
            }
            '|' => {
                let next = *stack.last().unwrap();
                row = next.0;
                col = next.1;
            }
            'W' => {
                map.insert((row, col - 1), VDoor);
                map.insert((row, col - 2), Room);
                col -= 2;
            }
            'E' => {
                map.insert((row, col + 1), VDoor);
                map.insert((row, col + 2), Room);
                col += 2;
            }
            'N' => {
                map.insert((row - 1, col), HDoor);
                map.insert((row - 2, col), Room);
                row -= 2;
            }
            'S' => {
                map.insert((row + 1, col), HDoor);
                map.insert((row + 2, col), Room);
                row += 2;
            }
            '^' | '$' => (),
            _ => panic!("unexpected path character '{}'", c),
        };
        row_min = row_min.min(row);
        row_max = row_max.max(row);
        col_min = col_min.min(col);
        col_max = col_max.max(col);
    }
    let rows = (row_max - row_min + 3) as usize;
    let cols = (col_max - col_min + 3) as usize;
    row_min -= 1;
    row_max += 1;
    col_min -= 1;
    col_max += 1;
    let origin = ((-row_min) as usize, (-col_min) as usize);

    let mut mat = Mat::new(rows, cols, Wall);
    for row in row_min..=row_max {
        let i = (row - row_min) as usize;
        for col in col_min..=col_max {
            let j = (col - col_min) as usize;
            let v = *map.get(&(row, col)).unwrap_or(&Wall);
            mat.set(i, j, v);
        }
    }
    mat.set(origin.0, origin.1, Cursor);
    (mat, origin)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_build_map() {
        use build_map;
        let (map, _origin) =
            build_map("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
        assert_eq!(
            r"###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############
",
            map.to_string()
        );
    }

    #[test]
    fn test_part_1() {
        use part_1;

        assert_eq!(
            31,
            part_1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$")
        );
    }
}
