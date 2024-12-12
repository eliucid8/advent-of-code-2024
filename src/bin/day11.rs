use std::{collections::HashMap, hash::Hash};

use aoc24::*;

fn main() {
    let binding = read_input(11);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let stones_raw = parse_row_major::<usize>(input, " ");
    let mut stones: HashMap<usize, usize> = HashMap::new();

    for stone in &stones_raw[0] {
        stones.insert(*stone, 1);
    }

    for _ in 0..25 {
        stones = convert_stone_counter(&stones);
    }

    stones
        .iter()
        .fold(0, |acc, stone| acc + *stone.1)
        .to_string()
    //"".to_string()
}

fn part2(input: &str) -> String {
    let stones_raw = parse_row_major::<usize>(input, " ");
    let mut stones: HashMap<usize, usize> = HashMap::new();

    for stone in &stones_raw[0] {
        stones.insert(*stone, 1);
    }

    for _ in 0..75 {
        stones = convert_stone_counter(&stones);
    }

    stones
        .iter()
        .fold(0, |acc, stone| acc + *stone.1)
        .to_string()
    //"".to_string()
}

fn display_stones(stones: &HashMap<usize, usize>) -> String {
    let mut ret = String::new();
    for stone in stones {
        ret.push_str(format!("{}: {}", *stone.0, *stone.1).as_str());
    }
    ret
}

fn convert_stone_counter(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones: HashMap<usize, usize> = HashMap::new();
    for stone in stones {
        if *stone.0 == 0 {
            increment_counter(&mut new_stones, &1, *stone.1);
        } else if num_digits(*stone.0) % 2 == 0 {
            let new_nums = split_number(*stone.0);
            increment_counter(&mut new_stones, &new_nums.0, *stone.1);
            increment_counter(&mut new_stones, &new_nums.1, *stone.1);
        } else {
            increment_counter(&mut new_stones, &(*stone.0 * 2024), *stone.1);
        }
    }
    new_stones
}

//fn convert_stone(x: usize) -> usize {
//    if x == 0 {
//        return 1;
//    }
//    if num_digits(x) % 2 == 0 {
//        return split_number(x).0;
//    }
//    return x * 2024;
//}

fn split_number(x: usize) -> (usize, usize) {
    let num = num_digits(x);
    let mut power = 1;
    for _ in 0..num / 2 {
        power *= 10;
    }

    return (x / power, x % power);
}

fn num_digits(x: usize) -> usize {
    if x == 0 {
        return 1;
    }

    let mut temp = x;
    let mut digits = 0;
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }
    return digits;
}

fn increment_counter<T: Eq + Hash + Clone>(
    counter: &mut HashMap<T, usize>,
    key: &T,
    amount: usize,
) -> usize {
    if !counter.contains_key(&key) {
        counter.insert(key.clone(), amount);
        amount
    } else {
        let prev_value = counter.get(&key).unwrap().clone();
        counter.insert(key.clone(), prev_value + amount);
        prev_value + amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "55312");
    }

    #[test]
    fn test_part1_visual() {
        assert_eq!(part1_visual(TEST_INPUT), "22");
    }

    fn part1_visual(input: &str) -> String {
        let stones_raw = parse_row_major::<usize>(input, " ");
        println!("{:#?}", stones_raw);
        let mut stones: HashMap<usize, usize> = HashMap::new();

        for stone in &stones_raw[0] {
            stones.insert(*stone, 1);
        }
        println!("{}", display_stones(&stones));

        for _ in 0..6 {
            stones = convert_stone_counter(&stones);
            println!("{}", display_stones(&stones))
        }

        stones
            .iter()
            .fold(0, |acc, stone| acc + *stone.1)
            .to_string()
        //"".to_string()
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "0");
    }
}
