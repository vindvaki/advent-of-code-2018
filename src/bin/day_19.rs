use std::io::Read;

extern crate aoc2018;
use aoc2018::elfcode;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let state: elfcode::Machine = data.parse().unwrap();
    println!("part_1: {}", part_1(&state));
    // NOTE: this initial number is specific to my input
    println!("part_2: {}", part_2(10551287));
}

fn part_1(state_0: &elfcode::Machine) -> usize {
    let mut state = state_0.clone();
    while state.next() {}
    state.registers[0]
}

fn part_2(n: usize) -> usize {
    let mut sum = 0;
    let mut q = 1;
    while q * q <= n {
        if n % q == 0 {
            sum += q;
            if q * q != n {
                sum += n / q;
            }
        }
        q += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    const DATA: &'static str = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn test_part_1() {
        use elfcode::Machine;
        use part_1;
        let state: elfcode::Machine = DATA.parse().unwrap();
        assert_eq!(7, part_1(&state));
    }
}
