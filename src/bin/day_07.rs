#[macro_use]
extern crate error_chain;
extern crate regex;

use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

mod errors {
    error_chain!{}
}

use errors::*;

pub fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let steps = parse_steps(&data).unwrap();
    println!("part_1: {}", part_1(&steps));
    println!("part_1: {}", part_2(&steps, 5, 60));
}

fn parse_line(line: &str) -> Result<(char, char)> {
    let re = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();
    let caps = re.captures(line).chain_err(|| "Invalid line format")?;
    let first = caps
        .get(1)
        .chain_err(|| "First capture not present")?
        .as_str();
    let second = caps
        .get(2)
        .chain_err(|| "Second capture not present")?
        .as_str();
    let a = first.chars().next().chain_err(|| "Expected char")?;
    let b = second.chars().next().chain_err(|| "Expected char")?;
    Ok((a, b))
}

/// Returns a map from a step to its dependencies
fn parse_steps(data: &str) -> Result<Vec<(char, char)>> {
    let mut res = Vec::new();
    for line in data.lines() {
        res.push(parse_line(line)?);
    }
    return Ok(res);
}

struct DependencyData {
    time: usize,
    free: BTreeSet<char>,
    remaining: HashSet<char>,
    dependent_to_dependencies: HashMap<char, HashSet<char>>,
    dependency_to_dependents: HashMap<char, HashSet<char>>,
    worker_count: usize,
    busy_count: usize,
    base_cost: usize,
    in_progress: BTreeMap<usize, BTreeSet<char>>,
}

impl DependencyData {
    fn new(steps: &Vec<(char, char)>, worker_count: usize, base_cost: usize) -> DependencyData {
        let mut free = BTreeSet::new();
        let mut dependent_to_dependencies = HashMap::new();
        let mut dependency_to_dependents = HashMap::new();
        let mut remaining = HashSet::new();

        for &(dependency, dependent) in steps.iter() {
            remaining.insert(dependency);
            remaining.insert(dependent);
            free.insert(dependency);

            dependent_to_dependencies
                .entry(dependent)
                .or_insert(HashSet::new())
                .insert(dependency);

            dependency_to_dependents
                .entry(dependency)
                .or_insert(HashSet::new())
                .insert(dependent);
        }
        for step in dependent_to_dependencies.keys() {
            free.remove(step);
        }

        DependencyData {
            time: 0,
            free: free,
            remaining: remaining,
            dependency_to_dependents: dependency_to_dependents,
            dependent_to_dependencies: dependent_to_dependencies,
            worker_count: worker_count,
            busy_count: 0,
            in_progress: BTreeMap::new(),
            base_cost: base_cost,
        }
    }

    fn start_work(&mut self, step: char, cost: usize) -> bool {
        if !self.free.contains(&step) {
            return false;
        }
        if self.in_progress.len() == self.worker_count {
            return false;
        }
        self.free.remove(&step);

        let finishes_at = self.time + cost;
        self.in_progress
            .entry(finishes_at)
            .or_insert(BTreeSet::new())
            .insert(step);
        self.busy_count += 1;
        true
    }

    fn advance_time(&mut self) -> Vec<char> {
        // awkward borrow checker workarounds

        // find next time
        let next_time = if let Some(&time) = self.in_progress.keys().next() {
            time
        } else {
            self.time
        };
        self.time = next_time;

        // mark the jobs finished that should finish and release workers
        let mut finishing = BTreeSet::new();
        if self.in_progress.contains_key(&self.time) {
            finishing = self.in_progress.get(&self.time).unwrap().clone();
        }
        self.busy_count -= finishing.len();
        for step in finishing {
            self.mark_done(step);
        }
        self.in_progress.remove(&self.time);

        // for every free worker, pick up work if there is any
        let steps: Vec<char> = self
            .free
            .iter()
            .take(self.worker_count - self.busy_count)
            .map(|&c| c)
            .collect();
        let base_cost = self.base_cost;
        for &step in steps.iter() {
            self.start_work(step, (step as usize) - ('A' as usize) + 1 + base_cost);
        }

        steps
    }

    fn mark_done(&mut self, step: char) -> bool {
        self.free.remove(&step);
        self.remaining.remove(&step);
        // step has been completed
        for &dependent in self
            .dependency_to_dependents
            .get(&step)
            .unwrap_or(&HashSet::new())
            .iter()
        {
            let mut dependencies = self.dependent_to_dependencies.get_mut(&dependent).unwrap();
            // so it is no longer a dependency
            dependencies.remove(&step);
            if dependencies.is_empty() {
                // and this dependent might just have been freed
                self.free.insert(dependent);
            }
        }
        // and it no longer has any dependents
        self.dependency_to_dependents.remove(&step);

        true
    }

    fn has_work(&self) -> bool {
        !self.remaining.is_empty()
    }
}

fn part_1(steps: &Vec<(char, char)>) -> String {
    let mut result = String::new();
    let mut deps = DependencyData::new(&steps, 1, 0);
    while deps.has_work() {
        for step in deps.advance_time() {
            result.push(step);
        }
    }
    return result;
}

fn part_2(steps: &Vec<(char, char)>, worker_count: usize, base_cost: usize) -> usize {
    let mut deps = DependencyData::new(&steps, worker_count, base_cost);
    while deps.has_work() {
        deps.advance_time();
    }
    return deps.time;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use parse_steps;
        use part_1;
        let input = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let steps = parse_steps(&input).unwrap();
        let expected = "CABDFE";
        assert_eq!(expected, part_1(&steps));
    }

    #[test]
    fn test_part_2() {
        use parse_steps;
        use part_2;
        let input = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let steps = parse_steps(&input).unwrap();
        let expected = 15;
        assert_eq!(expected, part_2(&steps, 2, 0));
    }
}
