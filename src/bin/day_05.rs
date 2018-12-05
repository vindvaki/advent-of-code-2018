extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    println!("part_1: {}", part_1(&data));
    println!("part_2: {}", part_2(&data));
}

fn part_1(data: &str) -> usize {
    let mut stack = Vec::new();
    for c in data.chars() {
        stack.push(c);
        while stack.len() >= 2 && is_reactive(stack[stack.len() - 1], stack[stack.len() - 2]) {
            stack.pop();
            stack.pop();
        }
    }
    return stack.len();
}

fn part_2(data: &str) -> usize {
    let mut unique_chars = HashSet::new();
    for c in data.chars() {
        unique_chars.insert(c.to_ascii_lowercase());
    }
    let mut solution = part_1(data);
    for c in unique_chars.iter() {
        let pattern = format!("[{}{}]", c, c.to_ascii_uppercase());
        let re = Regex::new(pattern.as_str()).unwrap();
        let replaced = re.replace_all(data, "");
        solution = solution.min(part_1(&replaced));
    }
    return solution;
}

fn is_reactive(a: char, b: char) -> bool {
    (a.is_ascii_lowercase() != b.is_ascii_lowercase()) && (a.eq_ignore_ascii_case(&b))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        let data = "dabAcCaCBAcCcaDA";
        assert_eq!(part_1(&data), 10);
    }

    #[test]
    fn test_part_2() {
        use part_2;
        let data = "dabAcCaCBAcCcaDA";
        assert_eq!(part_2(&data), 4);
    }
}
