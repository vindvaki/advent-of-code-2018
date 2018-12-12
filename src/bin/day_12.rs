#[macro_use]
extern crate error_chain;

use std::collections::HashMap;
use std::io::Read;

pub mod errors {
    error_chain!{}
}

use errors::*;

pub fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let (initial_state, rule_map) = parse_input(&data).unwrap();
    println!("part_1: {}", part_1(initial_state, &rule_map));
    println!("part_2: {}", part_2(initial_state, &rule_map));
}

fn parse_input<'a>(data: &'a str) -> Result<(&'a str, HashMap<&'a str, &'a str>)> {
    let mut lines = data.lines();
    let initial_state = parse_initial_state(lines.next().chain_err(|| "no initial state given")?)?;
    lines.next(); // discard
    let mut rule_map = HashMap::new();
    for line in lines {
        let (from, to) = parse_rule(line)?;
        rule_map.insert(from, to);
    }
    Ok((initial_state, rule_map))
}

fn parse_initial_state<'a>(data: &'a str) -> Result<&'a str> {
    let mut iter = data.split(": ");
    let _first = iter.next().chain_err(|| "unable to parse initial state")?;
    let result = iter.next().chain_err(|| "unable to parse initial state")?;
    Ok(result)
}

fn parse_rule<'a>(data: &'a str) -> Result<(&'a str, &'a str)> {
    let mut iter = data.split(" => ");
    let first = iter
        .next()
        .chain_err(|| "unable to parse first part of rule")?;
    let second = iter
        .next()
        .chain_err(|| "unable to parse second part of rule")?;
    if iter.next() != None {
        bail!{ "rule contains unexpected data" };
    }
    Ok((first, second))
}

fn apply_rules(state: &str, rule_map: &HashMap<&str, &str>) -> String {
    let mut result = String::new();
    // note: Rule length is always 5
    let padded_state = ["....", state, "...."].concat();
    for i in 2..padded_state.len() - 3 {
        let key = &padded_state[i - 2..i + 3];
        // 0,    1, 2, 3, 4, 5
        // i-2      i        i+3
        if let Some(new_middle) = rule_map.get(key) {
            result.push_str(new_middle);
        } else {
            result.push('.');
        }
    }
    result
}

fn part_1(initial_state: &str, rule_map: &HashMap<&str, &str>) -> usize {
    let mut state = initial_state.to_owned();
    let num_rounds = 20;
    let offset = 2 * num_rounds; // left pads by 2 chars every round
    for _ in 0..num_rounds {
        state = apply_rules(state.as_str(), rule_map);
    }
    state
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i - offset)
        .sum()
}

fn part_2(initial_state: &str, rule_map: &HashMap<&str, &str>) -> usize {
    let mut state = initial_state.to_owned();
    let mut prev_trimmed_state = state.trim_matches('.').to_owned();
    let mut sum;;
    let mut prev_sum = 0;
    let mut round = 0;
    loop {
        round += 1;
        state = apply_rules(state.as_str(), rule_map);
        sum = state
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i - round * 2)
            .sum();
        let trimmed_state = state.trim_matches('.').to_owned();
        if prev_trimmed_state == trimmed_state {
            // found cycle
            break;
        }
        prev_trimmed_state = trimmed_state;
        prev_sum = sum;
    }
    let diff = sum - prev_sum;
    let target = 50_000_000_000 - round;
    sum + target * diff
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use apply_rules;
        use parse_input;
        use part_1;
        let data = r"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
        let (initial_state, rule_map) = parse_input(&data).unwrap();
        let next_state = apply_rules(initial_state, &rule_map);
        println!("{}", initial_state);
        println!("{}", next_state);
        // assert_eq!("#...#....#.....#..#..#..#", next_state);
        assert_eq!(325, part_1(initial_state, &rule_map));
    }
}
