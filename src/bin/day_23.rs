extern crate nalgebra as na;
extern crate regex;

use na::Vector3;
use std::collections::BTreeSet;
use std::io::Read;
use std::iter::FromIterator;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let nanobots = parse_nanobots(&data).unwrap();
    println!("part_1: {}", part_1(&nanobots).unwrap());
    println!("part_2: {:?}", part_2(&nanobots));
}

fn parse_nanobots(data: &str) -> Result<Vec<Nanobot>, &'static str> {
    let mut out = Vec::new();
    for line in data.lines() {
        out.push(line.parse()?);
    }
    Ok(out)
}

fn part_1(bots: &Vec<Nanobot>) -> Option<usize> {
    let strongest_bot = bots.iter().max_by_key(|bot| bot.rad)?;
    Some(bots.iter().filter(|bot| strongest_bot.reaches(bot)).count())
}

fn part_2(bots: &Vec<Nanobot>) -> Option<usize> {
    let x = Vector3::new(1 as isize, 0, 0);
    let y = Vector3::new(0, 1 as isize, 0);
    let z = Vector3::new(0, 0, 1 as isize);

    // each facet in clockwise order such that normal faces towards origin
    // the normals are not unit vectors, but it doesn't matter because all
    // have the same normals, so the halfplane offsets will have the same scale
    let upper_facets = [(x, y, z), (z, y, -x), (-x, -y, z), (z, -y, x)];

    let mut normals = Vec::new();
    let mut offsets = Vec::new();
    for (a, b, c) in upper_facets.iter() {
        let normal = (c - a).cross(&(c - b));
        let mut normal_offsets = Vec::new();
        for bot in bots.iter() {
            // use arbitrary corners to calculate offsets
            let offset_start = normal.dot(&(bot.pos - bot.rad * c));
            let offset_end = normal.dot(&(bot.pos + bot.rad * c));

            // bot.pos should fall in both halfplanes
            assert!(normal.dot(&bot.pos) > offset_start);
            assert!(normal.dot(&bot.pos) < offset_end);
            assert!(offset_start < offset_end);

            // normal_offsets[bot_index] = ...
            normal_offsets.push((offset_start, offset_end));
        }
        // offsets[normal_index][bot_index]
        offsets.push(normal_offsets);
        normals.push(normal);
    }

    let (_max_combination, max_offsets) =
        recursive_scanline(&offsets, 0, &BTreeSet::from_iter(0..bots.len()));

    // we now need to solve:
    //
    // minimize
    //     |x| + |y| + |z|
    // subject to
    //      x + y + z <= a
    //      x - y + z <= b
    //     -x + y + z <= c
    //     -x - y + z <= d
    //      x + y - z <= e
    //      x - y - z <= f
    //     -x + y - z <= g
    //     -x - y - z <= h
    //
    // This could be transformed into a set of 8 linear programs, but we don't need to.
    // It suffices to check two things:
    //
    //   (1) Is the origin contained? Then the solution is 0.
    //   (2) Else, all boundaries are significant, and the solution
    //       is the smallest absolute offset.
    max_offsets
        .iter()
        .map(|(a, b)| a.abs().max(b.abs()) as usize)
        .max()
}

fn recursive_scanline(
    offsets: &Vec<Vec<(isize, isize)>>,
    normal_index: usize,
    parent_combination: &BTreeSet<usize>,
) -> (BTreeSet<usize>, Vec<(isize, isize)>) {
    if normal_index >= offsets.len() {
        return (parent_combination.clone(), Vec::new());
    }
    let mut events = Vec::new();
    for &bot_index in parent_combination.iter() {
        let (offset_start, offset_end) = offsets[normal_index][bot_index];
        events.push((offset_start, false, bot_index));
        events.push((offset_end, true, bot_index));
    }
    events.sort();

    let mut combination = BTreeSet::new();
    let mut offset_start = isize::min_value();
    let mut offset_end;
    let mut max_combination = BTreeSet::new();
    let mut max_offsets = Vec::new();
    for (offset, exited, bot_index) in events.iter() {
        if *exited {
            offset_end = *offset;
            if combination.len() > max_combination.len() {
                let (sub_max_combination, sub_offsets) =
                    recursive_scanline(offsets, normal_index + 1, &combination);
                if sub_max_combination.len() > max_combination.len() {
                    max_combination = sub_max_combination;
                    max_offsets = sub_offsets;
                    max_offsets.push((offset_start, offset_end));
                }
            }
            combination.remove(bot_index);
        } else {
            offset_start = *offset;
            combination.insert(*bot_index);
        }
    }

    return (max_combination, max_offsets);
}

#[derive(Debug, Clone)]
struct Nanobot {
    pos: Vector3<isize>,
    rad: isize,
}

impl Nanobot {
    fn manhattan(&self, other: &Nanobot) -> isize {
        (self.pos - other.pos).abs().iter().sum()
    }

    fn reaches(&self, other: &Nanobot) -> bool {
        self.manhattan(other) <= self.rad
    }
}

impl std::str::FromStr for Nanobot {
    type Err = &'static str;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();
        let caps = re.captures(data).ok_or("invalid nanobot input")?;
        let x = caps
            .get(1)
            .ok_or("missing x")?
            .as_str()
            .parse()
            .map_err(|_| "unable to parse x")?;
        let y = caps
            .get(2)
            .ok_or("missing y")?
            .as_str()
            .parse()
            .map_err(|_| "unable to parse y")?;
        let z = caps
            .get(3)
            .ok_or("missing z")?
            .as_str()
            .parse()
            .map_err(|_| "unable to parse z")?;
        let rad = caps
            .get(4)
            .ok_or("missing rad")?
            .as_str()
            .parse()
            .map_err(|_| "unable to parse rad")?;
        Ok(Nanobot {
            pos: Vector3::new(x, y, z),
            rad: rad,
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part_1() {
        use parse_nanobots;
        use part_1;
        let input = r"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        let bots = parse_nanobots(input).unwrap();
        assert_eq!(Some(7), part_1(&bots));
    }

    #[test]
    fn test_part_2() {
        use parse_nanobots;
        use part_2;
        let input = r"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        let bots = parse_nanobots(input).unwrap();
        assert_eq!(Some(36), part_2(&bots));
    }
}
