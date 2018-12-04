extern crate regex;

use regex::Regex;
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let grid = parse_grid(&data).unwrap();
    println!("part_1: {}", part_1(&grid));
    println!("part_2: {}", part_2(&grid));
}

struct Claim {
    id: i64,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

struct ClaimGrid {
    claims: Vec<Claim>,
    rows: usize,
    cols: usize,
    data: Vec<Vec<i64>>,
}

fn parse_claim(line: &str) -> Option<Claim> {
    let re = Regex::new(r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)$").unwrap();
    let captures = re.captures(line)?;
    Some(Claim {
        id: captures.name("id")?.as_str().parse().ok()?,
        x: captures.name("x")?.as_str().parse().ok()?,
        y: captures.name("y")?.as_str().parse().ok()?,
        w: captures.name("w")?.as_str().parse().ok()?,
        h: captures.name("h")?.as_str().parse().ok()?,
    })
}

fn parse_grid(data: &str) -> Option<ClaimGrid> {
    let mut grid = ClaimGrid {
        claims: Vec::new(),
        rows: 0,
        cols: 0,
        data: Vec::new(),
    };

    // parse lines
    for line in data.lines() {
        let claim = parse_claim(line)?;
        grid.rows = grid.rows.max(claim.y + claim.h + 1);
        grid.cols = grid.rows.max(claim.x + claim.w + 1);
        grid.claims.push(claim);
    }

    // allocate grid
    for i in 0..grid.rows - 1 {
        grid.data.push(Vec::new());
        for _ in 0..grid.cols - 1 {
            grid.data[i].push(0);
        }
    }

    // mark overlaps
    for claim in grid.claims.iter() {
        for i in claim.x..claim.x + claim.w {
            for j in claim.y..claim.y+claim.h {
                grid.data[i][j] = match grid.data[i][j] {
                    0  => claim.id,
                    _  => -1,
                };
            }
        }
    }
    return Some(grid);
}

fn part_1(grid: &ClaimGrid) -> usize {
    let mut count = 0;
    for row in grid.data.iter() {
        for &cell in row.iter() {
            if cell == -1 {
                count += 1;
            }
        }
    }
    return count;
}

fn part_2(grid: &ClaimGrid) -> i64 {
    for claim in grid.claims.iter() {
        let mut intact = true;
        for i in claim.x..claim.x + claim.w {
            for j in claim.y..claim.y + claim.h {
                if grid.data[i][j] != claim.id {
                    intact = false;
                    break;
                }
            }
            if !intact {
                break
            }
        }
        if intact {
            return claim.id;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use parse_grid;
        use part_1;
        let data = r"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
        let grid = parse_grid(data).unwrap();
        assert_eq!(4, part_1(&grid));
    }

    #[test]
    fn test_part_2() {
        use parse_grid;
        use part_2;
        let data = r"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
        let grid = parse_grid(data).unwrap();
        assert_eq!(3, part_2(&grid));
    }
}
