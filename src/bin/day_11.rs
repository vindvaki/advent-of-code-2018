extern crate aoc2018;

use aoc2018::Mat;
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let serial_number: usize = data.parse().unwrap();
    let grid = init_fuel_grid(serial_number);
    println!("part_1: {:?}", part_1(&grid));
    println!("part_2: {:?}", part_2(&grid));
}

fn init_fuel_grid(serial_number: usize) -> Mat<i64> {
    let mut grid = Mat::new(300, 300, 0);
    for y in 0..300 {
        for x in 0..300 {
            let rack_id = x as i64 + 10;
            let mut power_level = rack_id * (y as i64);
            power_level += serial_number as i64;
            power_level *= rack_id;
            power_level = (power_level / 100) % 10;
            power_level -= 5;
            grid.set(y, x, power_level);
        }
    }
    grid
}

pub fn part_1(grid: &Mat<i64>) -> (i64, (usize, usize)) {
    let mut max_corner = (0, 0);
    let mut max_power = 0;
    for y0 in 0..grid.rows() - 3 {
        for x0 in 0..grid.cols() - 3 {
            let mut power = 0;
            for y in y0..y0 + 3 {
                for x in x0..x0 + 3 {
                    power += grid.get(y, x);
                }
            }
            if power > max_power {
                max_power = power;
                max_corner = (x0, y0);
            }
        }
    }
    (max_power, max_corner)
}

pub fn part_2(grid: &Mat<i64>) -> (i64, (usize, usize), usize) {
    let mut max_corner = (0, 0);
    let mut max_power = 0;
    let mut max_dim = 0;
    let mut cache = Mat::new(grid.rows(), grid.cols(), 0);
    for dim in 1..(grid.rows().min(grid.cols())) {
        for y0 in 0..grid.rows() - dim {
            for x0 in 0..grid.cols() - dim {
                let mut power = *cache.get(y0, x0);
                for x in x0..x0 + dim {
                    power += grid.get(y0 + dim - 1, x);
                }
                for y in y0..y0 + dim {
                    power += grid.get(y, x0 + dim - 1);
                }
                power -= grid.get(y0 + dim - 1, x0 + dim - 1);
                if power > max_power {
                    max_power = power;
                    max_corner = (x0, y0);
                    max_dim = dim;
                }
                cache.set(y0, x0, power);
            }
        }
    }
    (max_power, max_corner, max_dim)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use init_fuel_grid;
        use part_1;

        let grid_18 = init_fuel_grid(18);
        assert_eq!(part_1(&grid_18), (29, (33, 45)));

        let grid_42 = init_fuel_grid(42);
        assert_eq!(part_1(&grid_42), (30, (21, 61)));
    }

    #[test]
    fn test_part_2() {
        use init_fuel_grid;
        use part_2;

        let grid_18 = init_fuel_grid(18);
        assert_eq!(part_2(&grid_18), (113, (90, 269), 16));

        let grid_42 = init_fuel_grid(42);
        assert_eq!(part_2(&grid_42), (119, (232, 251), 12));
    }

}
