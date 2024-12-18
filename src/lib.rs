use std::{char, fs::read_to_string};

use regex::Regex;

pub fn read_input(day: i32) -> String {
    let path = format!("inputs/day{}.txt", day);
    let content = read_to_string(path).unwrap();
    return content;
}

pub fn split_sections(input: &str) -> Vec<&str> {
    let sections: Vec<&str> = input.split("\n\n").filter(|&x| !x.is_empty()).collect();
    return sections;
}

pub fn parse_row_major<T: std::str::FromStr + std::default::Default>(
    input: &str,
    delimiter: &str,
) -> Vec<Vec<T>> {
    let lines = input.split("\n");
    let mut rows: Vec<Vec<T>> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            rows.push(
                line.split(delimiter)
                    .filter(|&x| !x.is_empty())
                    .map(|x| x.parse::<T>().unwrap_or_default())
                    .collect(),
            );
        }
    }
    return rows;
}

/**
* Returns a 2d vec array of chars from a string
*/
pub fn parse_string_array(input: &str) -> Vec<Vec<char>> {
    let lines = input.split("\n");
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() > 0 {
            rows.push(chars);
        }
    }
    return rows;
}

/**
 * Adds a i32 delta to a usize tuple. Useful for grid traversal.
 */
pub fn uadd_idirection(coord: (usize, usize), direction: (i32, i32)) -> (usize, usize) {
    (
        (coord.0 as i32 + direction.0) as usize,
        (coord.1 as i32 + direction.1) as usize,
    )
}

fn parse_coord(line: &str) -> (i32, i32) {
    let re = Regex::new(r"(?P<x>-?\d+),(?P<y>-?\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let x = caps.name("x").unwrap().as_str().parse::<i32>().unwrap();
    let y = caps.name("y").unwrap().as_str().parse::<i32>().unwrap();
    (y, x)
}

pub fn parse_coords(input: &str) -> Vec<(i32, i32)> {
    input.lines().map(|l| parse_coord(l)).collect()
}
