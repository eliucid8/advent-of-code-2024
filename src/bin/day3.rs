use regex::Regex;

use aoc24::*;

fn main() {
    let binding = read_input(3);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input))
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let muls: Vec<(i32, i32)> = re
        .captures_iter(input)
        .map(|caps| {
            let a = caps.name("a").unwrap().as_str().parse::<i32>().unwrap();
            let b = caps.name("b").unwrap().as_str().parse::<i32>().unwrap();
            (a, b)
        })
        .collect();
    let acc = muls.iter().fold(0, |acc, tup| acc + (tup.0 * tup.1));
    return format!("{:#?}", acc);
}

fn part2(input: &str) -> String {
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let re_nums = Regex::new(r"\d+").unwrap();
    let commands: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    let mut acc = 0;
    let mut do_mul = true;
    for command in commands {
        if command.eq("do()") {
            do_mul = true;
        } else if command.eq("don't()") {
            do_mul = false;
        } else if do_mul {
            let nums: Vec<i32> = re_nums
                .find_iter(command)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect();
            acc += nums[0] * nums[1];
        }
    }

    return format!("{:#?}", acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part1(test_input), "161");
    }

    #[test]
    fn tst_part2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(test_input), "48");
    }
}
