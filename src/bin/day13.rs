use core::fmt;

use aoc24::*;
use regex::Regex;

fn main() {
    let binding = read_input(13);
    let input = binding.as_str();

    let uh_oh_1 = uh_oh_count(&create_matrices(input));
    let uh_oh_2 = uh_oh_count(&create_matrices_2(input));
    println!("uh oh counts, part 1: {}, {}", uh_oh_1.0, uh_oh_1.1);
    println!("Part 1: {}", part1(&input));
    println!("uh oh counts, part 2: {}, {}", uh_oh_2.0, uh_oh_2.1);
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let matrices = create_matrices(input);
    let sum = matrices.iter().fold(0, |acc, mat| acc + mat.token_cost());
    sum.to_string()
}

fn part2(input: &str) -> String {
    let matrices = create_matrices_2(input);
    let sum = matrices.iter().fold(0, |acc, mat| acc + mat.token_cost());
    sum.to_string()
}

fn create_matrices(input: &str) -> Vec<AugMat> {
    let sections = split_sections(input);
    sections.iter().map(|s| AugMat::parse_section(s)).collect()
}

fn create_matrices_2(input: &str) -> Vec<AugMat> {
    let sections = split_sections(input);
    sections
        .iter()
        .map(|s| AugMat::parse_section_2(s))
        .collect()
}

/**
 * Check if we're gonna have to do ugly integer programming
 */
fn uh_oh_count(matrices: &Vec<AugMat>) -> (i64, i64) {
    let od_count = matrices.iter().fold(0, |acc, mat| {
        if mat.check_overdetermined().0 {
            acc + 1
        } else {
            acc
        }
    });

    let lp_count = matrices.iter().fold(0, |acc, mat| {
        if mat.check_overdetermined().1 {
            acc + 1
        } else {
            acc
        }
    });

    (od_count, lp_count)
}

struct AugMat {
    v1: (i64, i64),
    v2: (i64, i64),
    target: (i64, i64),
}

impl AugMat {
    fn new(v1: (i64, i64), v2: (i64, i64), target: (i64, i64)) -> Self {
        AugMat { v1, v2, target }
    }

    fn parse_section(section: &str) -> Self {
        let vecs: Vec<(i64, i64)> = section.lines().map(|l| get_xy_pair(l)).collect();
        AugMat::new(vecs[0], vecs[1], vecs[2])
    }

    fn parse_section_2(section: &str) -> Self {
        let vecs: Vec<(i64, i64)> = section.lines().map(|l| get_xy_pair(l)).collect();
        AugMat::new(
            vecs[0],
            vecs[1],
            (vecs[2].0 + 10000000000000, vecs[2].1 + 10000000000000),
        )
    }

    /**
     * Check if we need to do integer programming
     */
    fn check_overdetermined(&self) -> (bool, bool) {
        return (
            self.det() == 0,
            self.det() == 0 && self.dx() == 0 && self.dy() == 0,
        );
    }

    fn det(&self) -> i64 {
        self.v1.0 * self.v2.1 - self.v2.0 * self.v1.1
    }

    fn dx(&self) -> i64 {
        self.target.0 * self.v2.1 - self.v2.0 * self.target.1
    }

    fn dy(&self) -> i64 {
        self.v1.0 * self.target.1 - self.target.0 * self.v1.1
    }

    fn cramer_int(&self) -> Option<(i64, i64)> {
        if self.det() == 0 {
            return None;
        }
        // checking for divisibility (integer number of steps)
        // and positive number of steps
        if self.dx() % self.det() == 0
            && (self.dx() / self.det()) >= 0
            && self.dy() % self.det() == 0
            && (self.dy() / self.det()) >= 0
        {
            let x = self.dx() / self.det();
            let y = self.dy() / self.det();

            Some((x, y))
        } else {
            None
        }
    }

    fn token_cost(&self) -> i64 {
        let int_soln = self.cramer_int();
        if int_soln.is_some() {
            return int_soln.unwrap().0 * 3 + int_soln.unwrap().1;
        } else {
            return 0;
        }
    }
}

impl fmt::Display for AugMat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} {} | {}]\n[{} {} | {}]",
            self.v1.0, self.v2.0, self.target.0, self.v1.1, self.v2.1, self.target.1,
        )
    }
}

fn get_xy_pair(line: &str) -> (i64, i64) {
    let re = Regex::new(r".*X.(?<a>\d+).*Y.(?<b>\d+)").unwrap();
    let ret = re
        .captures_iter(line)
        .map(|caps| {
            let a = caps.name("a").unwrap().as_str().parse::<i64>().unwrap();
            let b = caps.name("b").unwrap().as_str().parse::<i64>().unwrap();
            (a, b)
        })
        .nth(0)
        .unwrap();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "0");
    }

    #[test]
    fn test_parse() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        let am = AugMat::parse_section(input);
        assert_eq!(format!("{}", am), "[94 22 | 8400]\n[34 67 | 5400]")
    }

    #[test]
    fn test_overdet() {
        let am1 = AugMat::new((1, 2), (2, 4), (4, 8));
        assert_eq!(am1.check_overdetermined(), (true, true));
        let am2 = AugMat::new((1, 2), (3, 4), (5, 6));
        assert_eq!(am2.check_overdetermined(), (false, false));
    }

    #[test]
    fn test_cramer() {
        let am1 = AugMat::new((1, 2), (3, 4), (23, 34));
        assert_eq!(am1.cramer_int(), Some((5, 6)));
        let am2 = AugMat::new((1, 2), (3, 4), (5, 6));
        assert_eq!(am2.cramer_int(), None);
    }
}
