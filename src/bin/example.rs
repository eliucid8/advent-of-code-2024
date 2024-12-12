use aoc24::*;

fn main() {
    let binding = read_input(0);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    "".to_string()
}

fn part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "0");
    }
}
