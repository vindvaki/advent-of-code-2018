extern crate regex;

use std::collections::{HashMap, HashSet};
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let mut samples = Vec::new();
    for sample_str in data.split("\n\n") {
        if let Ok(sample) = parse_sample(sample_str) {
            samples.push(sample);
        } else {
            break;
        }
    }
    let sample_lines = samples.len() * 4 + 2;
    let mut test_program = Vec::new();
    for line in data.split("\n").skip(sample_lines) {
        let opargs = line
            .split(" ")
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i64>>();
        test_program.push((opargs[0] as usize, (opargs[1], opargs[2], opargs[3])));
    }
    println!("part_1: {}", part_1(&samples));
    println!("part_2: {}", part_2(&samples, &test_program));
}

fn part_1(samples: &Vec<Sample>) -> usize {
    samples
        .iter()
        .map(|sample| {
            // operations agreeing with sample
            OPS.iter()
                .filter(|(_name, op)| op(sample.args, sample.before) == sample.after)
                .count()
        }).filter(|&c| c >= 3)
        .count()
}

fn part_2(samples: &Vec<Sample>, program: &Vec<Instruction>) -> i64 {
    let mut candidates = HashMap::new();
    for sample in samples.iter() {
        for &op in OPS.iter() {
            if op.1(sample.args, sample.before) == sample.after {
                candidates
                    .entry(op)
                    .or_insert(HashSet::new())
                    .insert(sample.opcode);
            }
        }
    }
    let mut resolved = HashMap::new();
    while !candidates.is_empty() {
        let (op, opcode) = candidates
            .iter()
            .find(|(_, opcodes)| opcodes.len() == 1)
            .map(|(op, cs)| (op.clone(), *cs.iter().next().unwrap()))
            .expect("All candidates are ambiguous");
        resolved.insert(opcode, op);
        candidates.remove(&op);
        for opcodes in candidates.values_mut() {
            opcodes.remove(&opcode);
        }
    }

    let mut registers = [0, 0, 0, 0];
    for &(opcode, args) in program.iter() {
        let op = resolved.get(&opcode).expect("Invalid opcode");
        registers = op.1(args, registers);
    }
    registers[0]
}

type Args = (i64, i64, i64);
type Registers = [i64; 4];
type Instruction = (usize, Args);
type Operation = fn(Args, Registers) -> Registers;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Sample {
    before: Registers,
    opcode: usize,
    args: Args,
    after: Registers,
}

fn parse_sample(data: &str) -> Result<Sample, &'static str> {
    let re = regex::Regex::new(
        r"(?m)Before: \[(?P<before>.*)\]
(?P<opargs>.*)
After:  \[(?P<after>.*)\]",
    ).unwrap();
    let caps = re.captures(data).ok_or("No captures")?;
    let before_str = caps.name("before").ok_or("could not find before")?.as_str();
    let before_vec: Vec<i64> = before_str.split(", ").map(|s| s.parse().unwrap()).collect();
    let opargs_str = caps.name("opargs").ok_or("could not find opargs")?.as_str();
    let opargs_vec: Vec<i64> = opargs_str.split(" ").map(|s| s.parse().unwrap()).collect();
    let after_str = caps.name("after").ok_or("could not find before")?.as_str();
    let after_vec: Vec<i64> = after_str.split(", ").map(|s| s.parse().unwrap()).collect();
    Ok(Sample {
        before: [before_vec[0], before_vec[1], before_vec[2], before_vec[3]],
        opcode: opargs_vec[0] as usize,
        args: (opargs_vec[1], opargs_vec[2], opargs_vec[3]),
        after: [after_vec[0], after_vec[1], after_vec[2], after_vec[3]],
    })
}

fn addi((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c as usize] = r[a as usize] + b;
    out
}

fn addr((a, b, c): Args, r: Registers) -> Registers {
    addi((a, r[b as usize], c), r)
}

fn muli((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c as usize] = r[a as usize] * b;
    out
}

fn mulr((a, b, c): Args, r: Registers) -> Registers {
    muli((a, r[b as usize], c), r)
}

fn bani((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c as usize] = r[a as usize] & b;
    out
}

fn banr((a, b, c): Args, r: Registers) -> Registers {
    bani((a, r[b as usize], c), r)
}

fn bori((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c as usize] = r[a as usize] | b;
    out
}

fn borr((a, b, c): Args, r: Registers) -> Registers {
    bori((a, r[b as usize], c), r)
}

fn seti((a, _, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c as usize] = a;
    out
}

fn setr((a, b, c): Args, r: Registers) -> Registers {
    seti((r[a as usize], b, c), r)
}

fn gtir((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c as usize] = if a > r[b as usize] { 1 } else { 0 };
    out
}

fn gtri((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c as usize] = if r[a as usize] > b { 1 } else { 0 };
    out
}

fn gtrr((a, b, c): Args, r: Registers) -> Registers {
    gtri((a, r[b as usize], c), r)
}

fn eqir((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c as usize] = if a == r[b as usize] { 1 } else { 0 };
    out
}

fn eqri((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c as usize] = if r[a as usize] == b { 1 } else { 0 };
    out
}

fn eqrr((a, b, c): Args, r: Registers) -> Registers {
    eqri((a, r[b as usize], c), r)
}

// include name for debugging purposes
const OPS: [(&'static str, Operation); 16] = [
    ("addi", addi), //
    ("addr", addr), //
    ("muli", muli), //
    ("mulr", mulr), //
    ("bani", bani), //
    ("banr", banr), //
    ("bori", bori), //
    ("borr", borr), //
    ("seti", seti), //
    ("setr", setr), //
    ("gtir", gtir), //
    ("gtri", gtri), //
    ("gtrr", gtrr), //
    ("eqir", eqir), //
    ("eqri", eqri), //
    ("eqrr", eqrr), //
];

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_sample() {
        use parse_sample;
        use Sample;
        let sample_str = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
        let sample = parse_sample(&sample_str).unwrap();
        assert_eq!(
            sample,
            Sample {
                before: [3, 2, 1, 1],
                opcode: 9,
                args: (2, 1, 2),
                after: [3, 2, 2, 1],
            }
        );
    }

    #[test]
    fn test_ops() {
        use parse_sample;
        use OPS;
        let sample_str = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
        let sample = parse_sample(&sample_str).unwrap();
        let matches: Vec<_> = OPS
            .iter()
            .filter(|(name, op)| op(sample.args, sample.before) == sample.after)
            .collect();
        println!("{:?}", matches);
        assert_eq!(matches.len(), 3);
    }
}
