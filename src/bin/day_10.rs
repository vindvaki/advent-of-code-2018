extern crate aoc2018;
#[macro_use]
extern crate error_chain;
extern crate regex;

use aoc2018::Mat;
use std::io::Read;

mod errors {
    error_chain!{}
}

use errors::*;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let points = parse_points(&data).unwrap();
    println!("part_1:\n{}", part_1(&points).unwrap());
    println!("part_2:\n{}", part_2(&points).unwrap());
}

#[derive(Debug, Clone, Copy)]
struct Point {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn parse_points(data: &str) -> Result<Vec<Point>> {
    let mut result = Vec::new();
    for line in data.lines() {
        result.push(parse_point(line)?);
    }
    Ok(result)
}

fn parse_point(data: &str) -> Result<Point> {
    let re = regex::Regex::new(r"^position=<(?P<pos>.*)> velocity=<(?P<vel>.*)>$").unwrap();
    let caps = re.captures(data).chain_err(|| "no match")?;
    let pos_str = caps.name("pos").chain_err(|| "no pos found")?.as_str();
    let vel_str = caps.name("vel").chain_err(|| "no vel found")?.as_str();
    Ok(Point {
        pos: parse_pair(pos_str)?,
        vel: parse_pair(vel_str)?,
    })
}

#[derive(Debug, Clone, Copy)]
struct BoundingBox {
    min: (i64, i64),
    max: (i64, i64),
}

impl BoundingBox {
    fn new() -> BoundingBox {
        BoundingBox {
            min: (i64::max_value(), i64::max_value()),
            max: (i64::min_value(), i64::min_value()),
        }
    }

    fn width(&self) -> usize {
        1 + (self.max.0 - self.min.0).abs() as usize
    }

    fn height(&self) -> usize {
        1 + (self.max.1 - self.min.1).abs() as usize
    }
}

fn bounding_box(points: &Vec<Point>) -> BoundingBox {
    points.iter().fold(BoundingBox::new(), |bb, p| BoundingBox {
        min: (bb.min.0.min(p.pos.0), bb.min.1.min(p.pos.1)),
        max: (bb.max.0.max(p.pos.0), bb.max.1.max(p.pos.1)),
    })
}

fn advance(points: &mut Vec<Point>) {
    for p in points.iter_mut() {
        p.pos.0 += p.vel.0;
        p.pos.1 += p.vel.1;
    }
}

fn part_1(points: &Vec<Point>) -> Option<Mat<char>> {
    find_message(points).map(|(m, _)| m)
}

fn part_2(points: &Vec<Point>) -> Option<usize> {
    find_message(points).map(|(_, t)| t)
}

fn find_message(initial_points: &Vec<Point>) -> Option<(Mat<char>, usize)> {
    let mut points = initial_points.clone();
    let mut bb = bounding_box(&points);
    let mut time = 0;
    // keep moving until height of bounding box is reasonably small
    while bb.height() > points.len() {
        advance(&mut points);
        bb = bounding_box(&points);
        time += 1;
    }
    // keep looking until the height exceeds max height
    while bb.height() <= points.len() {
        let d = draw(&points, &bb);
        advance(&mut points);
        bb = bounding_box(&points);
        // assumption: At least 1 character extends from top to bottom, and this is very unlikely
        for col in 0..d.cols() {
            if (0..d.rows()).all(|row| *d.get(row, col) == '#') {
                return Some((d, time));
            }
        }
        time += 1;
    }
    None
}

fn draw(points: &Vec<Point>, bb: &BoundingBox) -> Mat<char> {
    let mut mat = Mat::new(bb.height(), bb.width(), '.');
    for p in points.iter() {
        let row = (p.pos.1 - bb.min.1) as usize;
        let col = (p.pos.0 - bb.min.0) as usize;
        mat.set(row, col, '#');
    }
    mat
}

fn parse_pair(data: &str) -> Result<(i64, i64)> {
    let mut iter = data.split(",").map(|s| s.trim());
    let first_str = iter.next().chain_err(|| "expected a coordinate")?;
    let first: i64 = first_str
        .parse()
        .chain_err(|| format!("unable to parse integer {}", first_str))?;
    let second_str = iter.next().chain_err(|| "expected a coordinate")?;
    let second: i64 = second_str
        .parse()
        .chain_err(|| format!("unable to parse integer {}", second_str))?;
    Ok((first, second))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        use parse_points;
        let data = r"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        let points = parse_points(&data).unwrap();
        let msg = part_1(&points).unwrap();
    }

    #[test]
    fn test_part_2() {
        use part_2;
        use parse_points;
        let data = r"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        let points = parse_points(&data).unwrap();
        assert_eq!(part_2(&points).unwrap(), 3);
    }

}
