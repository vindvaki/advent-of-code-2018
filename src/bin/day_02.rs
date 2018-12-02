extern crate aoc2018;

use std::io;
use std::io::BufRead;
use aoc2018::count_by_value;

fn main() {
    let stdin = io::stdin();
    let data: Vec<String> = stdin
        .lock()
        .lines()
        .map(|maybe_line| maybe_line.expect("Unable to read line"))
        .collect();

    println!("part_1: {}", part_1(&data));
    println!("part_1: {}", part_2(&data));
}

fn part_1(data: &Vec<String>) -> u64 {
    let mut has_2: u64 = 0;
    let mut has_3: u64 = 0;
    for id in data {
        let counts = count_by_value(id.chars());
        if counts.values().any(|&count| count == 2) {
            has_2 += 1;
        }
        if counts.values().any(|&count| count == 3) {
            has_3 += 1;
        }
    }
    return has_2 * has_3;
}

fn part_2(data: &Vec<String>) -> String {
    // this could be done in O(n) or O(n*log(n)) by being clever,
    // but the number of strings is so small that we don't need to
    // care
    let m = data.len();
    for i in 0..m - 1 {
        for j in i + 1..m - 1 {
            let equal = data[i]
                .chars()
                .zip(data[j].chars())
                .filter(|(u, v)| u == v)
                .map(|(u, _)| u)
                .collect::<String>();
            if equal.len() == data[i].len() - 1 {
                return equal;
            }
        }
    }
    return String::new();
}
