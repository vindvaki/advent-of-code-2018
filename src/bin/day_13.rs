extern crate aoc2018;

use aoc2018::Mat;
use std::collections::BTreeMap;
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let track = parse_track(&data);
    let (y1, x1) = part_1(&track);
    println!("part_1: {},{}", x1, y1);
    let (y2, x2) = part_2(&track);
    println!("part_2: {},{}", x2, y2);
}

fn part_1(track0: &TrackState) -> (usize, usize) {
    let mut track = track0.clone();
    loop {
        match track.tick() {
            None => (),
            Some(collision) => return collision,
        };
    }
}

fn part_2(track0: &TrackState) -> (usize, usize) {
    let mut track = track0.clone();
    while track.carts.len() != 1 {
        track.tick();
    }
    *track.carts.keys().next().unwrap()
}

#[derive(Debug, Clone)]
struct TrackState {
    grid: Mat<char>,
    carts: BTreeMap<(usize, usize), (char, usize)>,
}

impl std::fmt::Display for TrackState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.grid.rows() {
            for col in 0..self.grid.cols() {
                if let Some(&(direction, _)) = self.carts.get(&(row, col)) {
                    write!(f, "{}", direction)?;
                } else {
                    write!(f, "{}", self.grid.get(row, col))?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl TrackState {
    fn tick(&mut self) -> Option<(usize, usize)> {
        let mut next_carts = BTreeMap::new();
        let mut collision = None;
        while !self.carts.is_empty() {
            let (&(row, col), &(direction, parity)) = self.carts.iter().next().unwrap();
            self.carts.remove(&(row, col));

            let (next_row, next_col) = match direction {
                '>' => (row, col + 1),
                '<' => (row, col - 1),
                'v' => (row + 1, col),
                '^' => (row - 1, col),
                _ => panic!("invalid direction"),
            };
            let next_track = *self.grid.get(next_row, next_col);
            let next_direction = match next_track {
                '\\' => match direction {
                    // upper right
                    '^' => '<',
                    '>' => 'v',
                    // lower left
                    '<' => '^',
                    'v' => '>',
                    _ => panic!("invalid grid"),
                },
                '/' => match direction {
                    // upper left
                    '^' => '>',
                    '<' => 'v',
                    // lower right
                    'v' => '<',
                    '>' => '^',
                    _ => panic!("invalid grid"),
                },
                '+' => match parity {
                    0 => turn_left(direction),
                    1 => direction,
                    2 => turn_right(direction),
                    _ => panic!("impossible"),
                },
                '|' | '-' => direction,
                _ => panic!("off the rails!"),
            };
            let next_parity = if next_track == '+' {
                (parity + 1) % 3
            } else {
                parity
            };
            let next_pos = (next_row, next_col);

            if next_carts.contains_key(&next_pos) || self.carts.contains_key(&next_pos) {
                next_carts.remove(&next_pos);
                self.carts.remove(&next_pos);
                if collision == None {
                    collision = Some(next_pos);
                }
            } else {
                next_carts.insert(next_pos, (next_direction, next_parity));
            };
        }
        self.carts = next_carts;
        collision
    }
}

fn turn_left(c: char) -> char {
    match c {
        'v' => '>',
        '>' => '^',
        '^' => '<',
        '<' => 'v',
        _ => panic!("invalid direction"),
    }
}

fn turn_right(c: char) -> char {
    match c {
        'v' => '<',
        '<' => '^',
        '^' => '>',
        '>' => 'v',
        _ => panic!("invalid direction"),
    }
}

fn parse_track(data: &str) -> TrackState {
    let rows = data.lines().count();
    let cols = data.lines().next().unwrap_or(&"").chars().count();
    let mut grid = Mat::new(rows, cols, char::default());
    let mut carts = BTreeMap::new();
    // parse raw data
    for (row, line) in data.lines().enumerate() {
        for (col, val) in line.chars().enumerate() {
            match val {
                '^' | 'v' | '>' | '<' => {
                    carts.insert((row, col), (val, 0));
                }
                _ => (),
            };
            grid.set(row, col, val);
        }
    }
    // lift carts off the grid
    for &(row, col) in carts.keys() {
        // intersection
        if (row > 0 && row + 1 < grid.rows() && col > 0 && col + 1 < grid.cols())
            && "^v|/\\+".contains(*grid.get(row - 1, col))
            && "^v|/\\+".contains(*grid.get(row + 1, col))
            && "<>-/\\+".contains(*grid.get(row, col - 1))
            && "<>-/\\+".contains(*grid.get(row, col + 1))
        {
            grid.set(row, col, '+');
            continue;
        }
        // plain horizontal
        if (col > 0 && col + 1 < grid.cols())
            && "<>-/\\+".contains(*grid.get(row, col - 1))
            && "<>-/\\+".contains(*grid.get(row, col + 1))
        {
            grid.set(row, col, '-');
            continue;
        }
        // plain vertical
        if (row > 0 && row + 1 < grid.rows())
            && "^v|/\\+".contains(*grid.get(row - 1, col))
            && "^v|/\\+".contains(*grid.get(row + 1, col))
        {
            grid.set(row, col, '|');
            continue;
        }
        // upper left
        if (row > 0 && col + 1 < grid.cols())
            && "^v|\\+".contains(*grid.get(row - 1, col))
            && "-><\\+".contains(*grid.get(row, col + 1))
        {
            grid.set(row, col, '/');
            continue;
        }
        // upper right
        if (row > 0 && col > 0)
            && "^v|/+".contains(*grid.get(row - 1, col))
            && "-></+".contains(*grid.get(row, col - 1))
        {
            grid.set(row, col, '\\');
            continue;
        }
        // lower left
        if (row > 0 && col + 1 < grid.cols())
            && "^v|/+".contains(*grid.get(row - 1, col))
            && "-></+".contains(*grid.get(row, col + 1))
        {
            grid.set(row, col, '\\');
            continue;
        }
        // lower right
        if (row + 1 < grid.rows() && col + 1 < grid.cols())
            && "^v|\\+".contains(*grid.get(row + 1, col))
            && "-><\\+".contains(*grid.get(row, col + 1))
        {
            grid.set(row, col, '/');
            continue;
        }
    }
    TrackState {
        grid: grid,
        carts: carts,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use parse_track;
        use part_1;
        let simple_input = r"|
v
|
|
|
^
|";
        let simple_track = parse_track(&simple_input);
        assert_eq!((3, 0), part_1(&simple_track));
        let input = [
            r"/->-\        ",
            r"|   |  /----\",
            r"| /-+--+-\  |",
            r"| | |  | v  |",
            r"\-+-/  \-+--/",
            r"  \------/   ",
        ]
            .join("\n");
        let track = parse_track(&input);
        println!("{}", track.grid);
        assert_eq!((3, 7), part_1(&track));

        assert_eq!(
            (0, 5),
            part_1(&parse_track(
                r"/--->>---\
^        |
\--------/"
            ))
        );
        assert_eq!(
            (2, 0),
            part_1(&parse_track(
                r"/<------\
v       |
v       |
\-------/"
            ))
        );
    }

    #[test]
    fn test_part_2() {
        use parse_track;
        use part_2;
        let input = [
            r"/>-<\  ", r"|   |  ", r"| /<+-\", r"| | | v", r"\>+</ |", r"  |   ^", r"  \<->/",
        ]
            .join("\n");
        let track = parse_track(&input);
        println!("{}", track.grid);
        assert_eq!((4, 6), part_2(&track));
    }

}
