use std::collections::BTreeSet;
use std::io::Read;
use std::iter::FromIterator;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let points = parse_points(&data).unwrap();
    println!("part_1: {}", part_1(&points));
}

fn part_1(points: &Vec<Point>) -> usize {
    let graph = PointGraph::new(3, points);
    graph.constellation_count()
}

fn parse_points(data: &str) -> Result<Vec<Point>, std::num::ParseIntError> {
    return data.lines().map(|line| {
        let values: Vec<_> = line.split(",").map(|w| w.parse::<isize>()).collect::<Result<Vec<_>,_>>()?;
        Ok([values[0], values[1], values[2], values[3]])
    }).collect()
}

type Point = [isize; 4];

fn manhattan(p: &Point, q: &Point) -> usize {
    p.iter().zip(q.iter()).map(|(px, qx)| (px - qx).abs() as usize).sum()
}

#[derive(Debug, Clone)]
struct PointGraph {
    adjacency_threshold: usize,
    points: Vec<Point>,
    neighbors: Vec<Vec<usize>>,
}

impl PointGraph {
    fn new(adjacency_threshold: usize, points: &Vec<Point>) -> PointGraph {
        let mut neighbors: Vec<_> = vec![Vec::new(); points.len()];
        for (i, p) in points.iter().enumerate() {
            for (j, q) in points.iter().enumerate() {
                if i != j && manhattan(p, q) <= adjacency_threshold {
                    neighbors[i].push(j);
                    neighbors[j].push(i);
                }
            }
        }
        PointGraph {
            adjacency_threshold: adjacency_threshold,
            points: points.clone(),
            neighbors: neighbors,
        }
    }

    fn constellation_count(&self) -> usize {
        let mut pool = BTreeSet::from_iter(0..self.points.len());
        let mut count = 0;
        while !pool.is_empty() {
            count += 1;
            let root = *pool.iter().next().unwrap();
            pool.remove(&root);
            let mut stack = vec![root];
            while !stack.is_empty() {
                let node = stack.pop().unwrap();
                for &neighbor in self.neighbors[node].iter() {
                    if pool.contains(&neighbor) {
                        pool.remove(&neighbor);
                        stack.push(neighbor);
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    const TEST_CASES: [(&'static str, usize); 4] = [
(r"0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0", 2),
(r"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0", 4),
(r"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2", 3),
(r"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2", 8),
    ];

    #[test]
    fn test_part_1() {
        use parse_points;
        use part_1;
        for (data, expected_result) in TEST_CASES.iter() {
            let points = parse_points(data).unwrap();
            assert_eq!(*expected_result, part_1(&points));
        }
    }
}