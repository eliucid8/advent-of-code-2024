use std::collections::{HashMap, HashSet};

use aoc24::*;

fn main() {
    let binding = read_input(19);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let (patterns, designs) = parse_inputs(input);
    let mut possible = HashSet::new();
    let mut impossible = HashSet::new();
    for pattern in patterns {
        possible.insert(pattern);
    }

    let mut total_possible = 0;
    for design in designs {
        if check_possible(design.clone(), &mut possible, &mut impossible) {
            //println!("{} possible", design);
            total_possible += 1;
        } else {
            //println!("{} impossible", design);
        }
    }

    total_possible.to_string()
}

fn part2(input: &str) -> String {
    let (patterns, designs) = parse_inputs(input);
    let mut num_combos = initialize_possible(&patterns);
    let original = num_combos.clone();
    //println!("{:?}", num_combos);

    let mut total_possible = 0;
    for design in designs {
        let count_possible = count_possible(design.clone(), &mut num_combos, &original);
        total_possible += count_possible;
        //println!("{}: {}", design, count_possible);
        //println!("{:?}", num_combos);
    }

    total_possible.to_string()
}

fn initialize_possible(patterns: &Vec<String>) -> HashMap<String, usize> {
    let mut num_combos = HashMap::new();

    let max_len = patterns
        .iter()
        .max_by(|a, b| (&a.len()).cmp(&b.len()))
        .unwrap()
        .len();
    let mut by_len = Vec::new();
    for _ in 0..max_len {
        let row: Vec<String> = Vec::new();
        by_len.push(row);
    }
    for pattern in patterns {
        by_len[pattern.len() - 1].push(pattern.clone());
    }

    //initialize hashmap to be all zeroes
    num_combos.insert("".to_string(), 0);
    num_combos.insert("w".to_string(), 0);
    num_combos.insert("u".to_string(), 0);
    num_combos.insert("b".to_string(), 0);
    num_combos.insert("r".to_string(), 0);
    num_combos.insert("g".to_string(), 0);

    for len in by_len {
        for pattern in len {
            let original = num_combos.clone();
            let sub_patterns = count_possible(pattern.clone(), &mut num_combos.clone(), &original);
            //println!("{}: sub {}", pattern, sub_patterns);
            num_combos.insert(pattern.clone(), sub_patterns + 1);
        }
        //println!("{:?}", num_combos);
    }

    num_combos
}

fn count_possible(
    design: String,
    num_combos: &mut HashMap<String, usize>,
    original: &HashMap<String, usize>,
) -> usize {
    // this base case ideally should be moved out of the method
    if num_combos.contains_key(&design) {
        return num_combos[&design];
    }

    let mut ways = 0;
    for i in (1..design.len()).rev() {
        let left_sub = design[0..i].to_string();
        if original.contains_key(&left_sub) && original[&left_sub] > 0 {
            let right_sub = design[i..].to_string();
            let right_ways;
            if num_combos.contains_key(&right_sub) {
                right_ways = num_combos[&right_sub];
            } else {
                right_ways = count_possible(right_sub, num_combos, original);
            }
            ways += right_ways;
            //if right_ways > 0 {
            //    break;
            //}
        }
    }
    num_combos.insert(design, ways);
    return ways;
}

fn check_possible(
    design: String,
    possible: &mut HashSet<String>,
    impossible: &mut HashSet<String>,
) -> bool {
    //println!("design: {}", design);
    if possible.contains(&design) {
        return true;
    }
    if impossible.contains(&design) {
        return false;
    }

    for i in 1..design.len() {
        let substr = design[0..i].to_string();
        //println!("\tcheck {}", substr);
        if possible.contains(&substr) {
            //println!("\thit {}, remaining {}", substr, design[i..].to_string());
            let back_result = check_possible(design[i..].to_string(), possible, impossible);
            if back_result {
                possible.insert(design.clone());
                return true;
            }
        }
    }
    impossible.insert(design.clone());
    false
}

fn parse_inputs(input: &str) -> (Vec<String>, Vec<String>) {
    let sections = split_sections(input);
    let patterns = parse_row_major::<String>(sections[0], ", ")[0].clone();
    let designs: Vec<String> = sections[1].lines().map(|s| s.to_string()).collect();
    return (patterns, designs);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    static TEST_1: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr";

    static TEST_2: &str = "r, wr, b, g, bwu, rb, gb, br

bwurrg
brgr";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "6");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "16");
    }

    #[test]
    fn test_small() {
        assert_eq!(part2(TEST_1), "2");
    }

    #[test]
    fn test_carryover() {
        assert_eq!(part2(TEST_2), "3");
    }
}
