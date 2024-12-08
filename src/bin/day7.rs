use aoc24::*;

fn main() {
    let binding = read_input(7);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let targets = parse_row_major::<usize>(input, ": ");
    let mut numbers = parse_row_major::<usize>(input, " ");

    let targets = unbox_first_element(&targets);
    remove_first_element(&mut numbers);

    let mut sum: u64 = 0;
    for i in 0..targets.len() {
        if check_compliant(targets[i], &numbers[i], 0, 0) {
            sum += targets[i] as u64;
        }
    }

    return sum.to_string();
}

fn part2(input: &str) -> String {
    let targets = parse_row_major::<usize>(input, ": ");
    let mut numbers = parse_row_major::<usize>(input, " ");

    let targets = unbox_first_element(&targets);
    remove_first_element(&mut numbers);

    let mut sum: u64 = 0;
    for i in 0..targets.len() {
        if check_compliant_concat(targets[i], &numbers[i], 0, 0) {
            sum += targets[i] as u64;
        }
    }

    return sum.to_string();
}

fn check_compliant_concat(
    target: usize,
    numbers: &Vec<usize>,
    index: usize,
    cur_value: usize,
) -> bool {
    if index == numbers.len() - 1 {
        return (cur_value + numbers[index] == target)
            || (cur_value * numbers[index] == target)
            || (num_concat(cur_value, numbers[index]) == target);
    }

    let mut result = false;
    if num_concat(cur_value, numbers[index]) <= target {
        result = result
            || check_compliant_concat(
                target,
                numbers,
                index + 1,
                num_concat(cur_value, numbers[index]),
            );
    }
    if cur_value * numbers[index] <= target {
        result = result
            || check_compliant_concat(target, numbers, index + 1, cur_value * numbers[index]);
    }
    if cur_value + numbers[index] <= target {
        result = result
            || check_compliant_concat(target, numbers, index + 1, cur_value + numbers[index]);
    }

    return result;
}

fn check_compliant(target: usize, numbers: &Vec<usize>, index: usize, cur_value: usize) -> bool {
    if index == numbers.len() - 1 {
        return (cur_value + numbers[index] == target) || (cur_value * numbers[index] == target);
    }

    let mut result = false;
    if cur_value * numbers[index] <= target {
        result = result || check_compliant(target, numbers, index + 1, cur_value * numbers[index]);
    }
    if cur_value + numbers[index] <= target {
        result = result || check_compliant(target, numbers, index + 1, cur_value + numbers[index]);
    }

    return result;
}

fn num_concat(a: usize, b: usize) -> usize {
    let num_digits = num_digits(b);
    let mut result = a;
    for _ in 0..num_digits {
        result *= 10;
    }
    return result + b;
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

fn unbox_first_element<T: std::clone::Clone>(arr: &Vec<Vec<T>>) -> Vec<T> {
    let ret: Vec<T> = arr.iter().map(|row| row[0].clone()).collect();
    return ret;
}

fn remove_first_element<T>(arr: &mut Vec<Vec<T>>) -> () {
    for row in arr {
        row.remove(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "3749");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "11387");
    }

    #[test]
    fn test_simple_part2() {
        assert_eq!(part2("12345: 123 45"), "12345");
    }

    #[test]
    fn test_concat() {
        assert_eq!(num_concat(123, 45), 12345);
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(12345), 5);
    }
}
