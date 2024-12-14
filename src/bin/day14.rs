use core::f64;

use aoc24::*;
use regex::Regex;

fn main() {
    let binding = read_input(14);
    let input = binding.as_str();

    //test_map("p=2,4 v=2,-3");

    println!("Part 1: {}", part1(&input));
    println!("Part 2:");
    part2(&input);
}

fn test_map(input: &str) {
    static TEST_BOUNDS: (i32, i32) = (7, 11);
    let mut robots = get_robots(input);

    let grid = get_robot_grid(&robots, TEST_BOUNDS);
    println!("step {}", 0);
    println!("{}\n", grid_string(grid));

    for i in 0..6 {
        for robot in &mut robots {
            robot.simulate(1, TEST_BOUNDS);
        }
        let grid = get_robot_grid(&robots, TEST_BOUNDS);
        println!("step {}", i + 1);
        println!("{}\n", grid_string(grid));
    }
}

fn part1(input: &str) -> String {
    static BOUNDS: (i32, i32) = (103, 101);
    part1_bounds(input, BOUNDS)
}

fn part2(input: &str) {
    static BOUNDS: (i32, i32) = (103, 101);
    let mut robots = get_robots(input);
    let mut steps = 0;
    let grid = get_robot_grid(&robots, BOUNDS);
    let mut min_variance = grid_variance(&grid);
    while steps < 10000 {
        let grid = get_robot_grid(&robots, BOUNDS);

        let variance = grid_variance(&grid);
        if variance < min_variance {
            min_variance = variance;
            println!("step {}: variance: {}", steps, min_variance);
        }

        for robot in &mut robots {
            robot.simulate(1, BOUNDS);
        }
        steps += 1;
    }
}

fn part1_bounds(input: &str, bounds: (i32, i32)) -> String {
    let (quadrant_counts, _) = part1_robot_sim(input, bounds);

    println!("{:#?}", quadrant_counts);
    let danger = quadrant_counts.iter().fold(1, |acc, count| acc * count);
    danger.to_string()
}

fn part1_map(input: &str, bounds: (i32, i32)) -> String {
    let (_, robots) = part1_robot_sim(input, bounds);

    let grid = get_robot_grid(&robots, bounds);
    grid_string(grid)
}

fn part1_robot_sim(input: &str, bounds: (i32, i32)) -> ([i32; 4], Vec<Robot>) {
    let mut robots = get_robots(input);
    // let _ = robots.iter_mut().map(|r| r.simulate(100, bounds));
    let mut quadrant_counts = [0, 0, 0, 0];
    for robot in &mut robots {
        robot.simulate(100, bounds);
        //println!("{}, {}", robot.position.0, robot.position.1);
        if robot.quadrant(bounds).is_some() {
            quadrant_counts[robot.quadrant(bounds).unwrap()] += 1;
        }
    }
    (quadrant_counts, robots)
}

fn grid_variance(grid: &Vec<Vec<usize>>) -> f64 {
    let mean = grid_mean(grid);
    let mut dist: f64 = 0.0;
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] > 0 {
                dist += pyth_normsq(mean, (i as f64, j as f64));
                count += 1;
            }
        }
    }
    return dist / (count as f64);
}

fn pyth_normsq(a: (f64, f64), b: (f64, f64)) -> f64 {
    return (a.0 - b.0).powi(2) + (a.1 - b.1).powi(2);
}

fn grid_mean(grid: &Vec<Vec<usize>>) -> (f64, f64) {
    let mut sumy = 0;
    let mut sumx = 0;
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] > 0 {
                sumx += i;
                sumy += j;
                total += 1;
            }
        }
    }
    (sumx as f64 / total as f64, sumy as f64 / total as f64)
}

fn grid_string(grid: Vec<Vec<usize>>) -> String {
    let mut ret = String::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] > 0 {
                ret.push_str(&grid[i][j].to_string());
            } else {
                ret.push('.');
            }
        }
        ret.push('\n');
    }
    ret
}

fn get_robot_grid(robots: &Vec<Robot>, bounds: (i32, i32)) -> Vec<Vec<usize>> {
    let mut grid = vec![vec![0; bounds.1 as usize]; bounds.0 as usize];

    for robot in robots {
        grid[robot.position.0 as usize][robot.position.1 as usize] += 1;
    }

    grid
}

fn get_robots(input: &str) -> Vec<Robot> {
    let robots: Vec<Robot> = input
        .lines()
        .skip_while(|l| l.is_empty())
        .map(|l| Robot::from_line(l))
        .collect();
    robots
}

fn get_xy_pair(line: &str) -> ((i32, i32), (i32, i32)) {
    let re = Regex::new(r".*=(?P<py>-?\d+),(?P<px>-?\d+).*=(?P<vy>-?\d+),(?P<vx>-?\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let px = caps.name("px").unwrap().as_str().parse::<i32>().unwrap();
    let py = caps.name("py").unwrap().as_str().parse::<i32>().unwrap();
    let vx = caps.name("vx").unwrap().as_str().parse::<i32>().unwrap();
    let vy = caps.name("vy").unwrap().as_str().parse::<i32>().unwrap();
    ((px, py), (vx, vy))
}

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn from_line(line: &str) -> Self {
        let pv = get_xy_pair(line);
        Robot {
            position: pv.0,
            velocity: pv.1,
        }
    }

    // returns the position after `steps` steps
    fn simulate(&mut self, steps: i32, bounds: (i32, i32)) -> (i32, i32) {
        let delta = (steps * self.velocity.0, steps * self.velocity.1);
        let nowrap = (self.position.0 + delta.0, self.position.1 + delta.1);
        self.position = (nowrap.0.rem_euclid(bounds.0), nowrap.1.rem_euclid(bounds.1));
        self.position
    }

    fn quadrant(&self, bounds: (i32, i32)) -> Option<usize> {
        let lims = (bounds.0 / 2, bounds.1 / 2);
        if self.position.0 < lims.0 && self.position.1 < lims.1 {
            return Some(0);
        } else if self.position.0 > lims.0 && self.position.1 < lims.1 {
            return Some(1);
        } else if self.position.0 > lims.0 && self.position.1 > lims.1 {
            return Some(2);
        } else if self.position.0 < lims.0 && self.position.1 > lims.1 {
            return Some(3);
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        assert_eq!(part1_bounds(TEST_INPUT, (7, 11)), "12");
    }

    #[test]
    fn test_part1_grid() {
        assert_eq!(
            part1_map(TEST_INPUT, (7, 11)),
            "......2..1.
...........
1..........
.11........
.....1.....
...12......
.1....1....\n"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "0");
    }
}
