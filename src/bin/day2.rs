use std::cmp;

use aoc24::*;

fn main() {
    let binding = read_input(2);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input))
}

fn part1(input: &str) -> String {
    let grid = parse_row_major::<i32>(input, " ");
    let mut num_safe = 0;
    for row in grid {
        let increasing = row[1] >= row[0];
        let mut safe = true;
        for i in 1..row.len() {
            if (row[i] > row[i - 1]) != increasing {
                safe = false;
                break;
            }
            let difference = row[i].abs_diff(row[i - 1]);
            if difference < 1 || difference > 3 {
                safe = false;
                break;
            }
        }
        if safe {
            num_safe += 1
        };
    }

    return num_safe.to_string();
}

fn part2_wrong(input: &str) -> String {
    let grid = parse_row_major::<i32>(input, " ");
    let mut num_safe = 0;
    for row in grid {
        if check_with_dampener(&row) {
            num_safe += 1;
        }
    }

    return num_safe.to_string();
}

fn part2(input: &str) -> String {
    let grid = parse_row_major::<i32>(input, " ");
    let mut num_safe = 0;
    for row in grid {
        for i in 0..row.len() {
            let mut row_cp = row.to_vec();
            row_cp.remove(i);
            if check_row_valid(&row_cp) {
                num_safe += 1;
                break;
            }
        }
    }
    return num_safe.to_string();
}

/**
 * @return (whether the sequence is valid, whether the sequence is increasing or not)
 */
fn check_increasing_dir(row: &Vec<i32>) -> (bool, bool) {
    let mut inc_count = (0, 0);
    for i in 1..row.len() {
        if row[i] > row[i - 1] {
            inc_count.0 += 1;
        } else {
            inc_count.1 += 1;
        }
    }
    // skip this row if there are more than 2 direction outliers.
    if cmp::min(inc_count.0, inc_count.1) > 1 {
        return (false, false);
    }
    let increasing = inc_count.0 > inc_count.1;
    return (true, increasing);
}

fn check_valid_pair(a: i32, b: i32, increasing: bool) -> bool {
    if (a > b) != increasing {
        return false;
    }
    let difference = a.abs_diff(b);
    if difference < 1 || difference > 3 {
        return false;
    }
    return true;
}

fn check_row_valid(row: &Vec<i32>) -> bool {
    let increasing = row[1] > row[0];
    for i in 1..row.len() {
        if !check_valid_pair(row[i], row[i - 1], increasing) {
            return false;
        }
    }
    return true;
}

fn check_with_dampener(row: &Vec<i32>) -> bool {
    let (dir_valid, increasing) = check_increasing_dir(&row);
    if !dir_valid {
        return false;
    }

    // set to false when we need to dampen an error.
    let mut dampener = true;

    let mut i = 1;
    while i < row.len() {
        if !check_valid_pair(row[i], row[i - 1], increasing) {
            if dampener {
                if i > 1 && check_valid_pair(row[i], row[i - 2], increasing) {
                    dampener = false;
                } else if i == row.len() - 1 {
                    return true;
                } else if check_valid_pair(row[i + 1], row[i - 1], increasing) {
                    dampener = false;
                    i += 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        i += 1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!(part1(test_input), "2");
    }

    #[test]
    fn tst_part2() {
        let test_input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!(part2(test_input), "4");
    }

    #[test]
    fn tst_part2_3() {
        let test_input = "1 2 7 8 9";
        assert_eq!(part2(test_input), "0");
    }

    #[test]
    fn tst_part2_1323() {
        let test_input2 = "1 3 2 3";
        assert_eq!(part2(test_input2), "1")
    }

    #[test]
    fn tst_part2_1023() {
        let test_input2 = "1 0 2 3";
        assert_eq!(part2(test_input2), "1")
    }
}
