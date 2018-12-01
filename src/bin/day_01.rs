use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let data: Vec<i64> = stdin.lock().lines().map(|maybe_line| {
        maybe_line
            .expect("Unable to read line from stdin")
            .parse()
            .expect("Unable to parse integer")
    }).collect();

    println!("part_1: {}", part_1(&data));
    println!("part_2: {}", part_2(&data));
}

fn part_1(data: &Vec<i64>) -> i64 {
    data.iter().sum()
}

fn part_2(data: &Vec<i64>) -> i64 {
    let mut i = 0;
    let mut sum = 0;
    let mut seen = HashSet::new();
    loop {
        if seen.contains(&sum) {
            return sum;
        }
        seen.insert(sum);
        sum += data[i];
        i += 1;
        i %= data.len();
    }
}