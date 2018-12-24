extern crate aoc2018;

use aoc2018::elfcode;
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let machine: elfcode::Machine = data.parse().unwrap();
    print!("{}", machine.deparse());
}