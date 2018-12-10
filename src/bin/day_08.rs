#[macro_use]
extern crate error_chain;

use std::io::Read;

mod errors {
    error_chain!{}
}

use errors::*;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let tree = parse_input(&data).unwrap();
    println!("part_1: {}", part_1(&tree));
    println!("part_2: {}", part_2(&tree));
}

#[derive(Debug)]
struct LicenseTree {
    // node -> children
    structure: Vec<Vec<usize>>,
    // node -> metadata
    metadata: Vec<Vec<usize>>,
}

impl LicenseTree {
    pub fn new() -> LicenseTree {
        LicenseTree {
            structure: Vec::new(),
            metadata: Vec::new(),
        }
    }

    pub fn value(&self, node: usize) -> usize {
        let children = &self.structure[node];
        let metadata = &self.metadata[node];
        if children.is_empty() {
            metadata.iter().sum::<usize>()
        } else {
            metadata
                .iter()
                .map(|&i| {
                    if let Some(&child_node_id) = children.get(i - 1) {
                        self.value(child_node_id)
                    } else {
                        0
                    }
                }).sum()
        }
    }
}

fn parse_input(data: &str) -> Result<LicenseTree> {
    let mut tree = LicenseTree::new();
    let mut iter = data.split(" ");
    let mut stack = Vec::new();

    // pseudo-node as root to start parsing
    stack.push((0, 1, 0));
    tree.structure.push(Vec::new());
    tree.metadata.push(Vec::new());

    let mut node_count = 1;

    while !stack.is_empty() {
        let (node_id, child_count, metadata_count) = stack.pop().unwrap();
        if child_count > 0 {
            // put self back on stack
            stack.push((node_id, child_count - 1, metadata_count));
            // parse next child
            let child_node_id = node_count;
            let child_child_count: usize = iter
                .next()
                .chain_err(|| "expected child count")?
                .parse()
                .chain_err(|| "unable to parse child count")?;
            let child_metadata_count: usize = iter
                .next()
                .chain_err(|| "expected metadata count")?
                .parse()
                .chain_err(|| "unable to parse metadata count")?;
            // put child on stack
            stack.push((child_node_id, child_child_count, child_metadata_count));
            // update structure
            tree.structure[node_id].push(child_node_id);
            tree.structure.push(Vec::new());
            tree.metadata.push(Vec::new());
            node_count += 1;
        } else {
            // all children parsed

            // parse metadata
            for _ in 0..metadata_count {
                let metadata_entry: usize = iter
                    .next()
                    .chain_err(|| "expected metadata entry")?
                    .parse()
                    .chain_err(|| "unable to parse metadata entry")?;
                tree.metadata[node_id].push(metadata_entry);
            }
        }
    }

    return Ok(tree);
}

fn part_1(tree: &LicenseTree) -> usize {
    tree.metadata.iter().map(|v| v.iter().sum::<usize>()).sum()
}

fn part_2(tree: &LicenseTree) -> usize {
    tree.value(1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use parse_input;
        use part_1;
        let data = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let tree = parse_input(&data).unwrap();
        println!("{:?}", tree);
        assert_eq!(138, part_1(&tree));
    }

    #[test]
    fn test_part_2() {
        use parse_input;
        use part_2;
        let data = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let tree = parse_input(&data).unwrap();
        println!("{:?}", tree);
        assert_eq!(66, part_2(&tree));
    }

}
