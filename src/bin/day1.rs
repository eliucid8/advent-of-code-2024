use std::collections::HashMap;

use aoc24::*;

fn main() {
    let binding = read_input(1);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let lines = input.split("\n");
    let mut locations1: Vec<i32> = Vec::with_capacity(1000);
    let mut locations2: Vec<i32> = Vec::with_capacity(1000);

    // would be nice to have a function that automatically takes this string input and formats it into arrays.
    for line in lines {
        let nums: Vec<&str> = line.split("   ").collect();
        locations1.push(str::parse::<i32>(nums[0]).unwrap());
        locations2.push(str::parse::<i32>(nums[1]).unwrap());
    }
    
    locations1.sort();
    locations2.sort();
    
    let mut sum:i32 = 0;
    for (i, item) in locations1.iter().enumerate() {
        sum += (locations2[i] - item).abs();
    }
    return format!("{}", sum);
}

fn part2(input: &str) -> String {
    let lines = input.split("\n");
    let mut locations1: Vec<i32> = Vec::with_capacity(1000);
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let nums:Vec<i32> = line.split("   ").map(|x| str::parse::<i32>(x).unwrap()).collect();
        locations1.push(nums[0]);
        if !counter.contains_key(&nums[1]) {
            counter.insert(nums[1], 0);
        }
        let cur_value = counter[&nums[1]];
        counter.insert(nums[1], cur_value + 1);
    }
    
    let mut sum:i32 = 0;
    for item in locations1.iter() {
        if counter.contains_key(item) {
            sum += item * counter.get(item).unwrap();
        }
    }
    return format!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part1(test_input), "11");
    }

    #[test]
    fn test_part2() {
        let test_input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part2(test_input), "31");
    }
}