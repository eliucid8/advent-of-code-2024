use std::collections::HashSet;

use aoc24::*;

fn main() {
    let binding = read_input(4);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input))
}

fn part1(input: &str) -> String {
    let grid = parse_string_array(input);
    let deltas: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut xmascount = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            for delta in &deltas {
                if check_word(
                    &grid,
                    (i.try_into().unwrap(), j.try_into().unwrap()),
                    *delta,
                    "XMAS",
                ) {
                    xmascount += 1
                }
            }
        }
    }

    return xmascount.to_string();
}

fn part2(input: &str) -> String {
    let grid = parse_string_array(input);
    let deltas: Vec<(i32, i32)> = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];

    let mut xmascount = 0;
    let mut mas_locs: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            for delta in &deltas {
                if check_word(
                    &grid,
                    (i.try_into().unwrap(), j.try_into().unwrap()),
                    *delta,
                    "MAS",
                ) {
                    // this is the location of the common A in a X-MAS formation
                    let a_loc = (
                        TryInto::<i32>::try_into(i).unwrap() + delta.0,
                        TryInto::<i32>::try_into(j).unwrap() + delta.1,
                    );
                    if !mas_locs.insert(a_loc) {
                        xmascount += 1;
                    }
                }
            }
        }
    }

    return xmascount.to_string();
}

fn check_word(grid: &Vec<Vec<char>>, coords: (i32, i32), delta: (i32, i32), word: &str) -> bool {
    let letters: Vec<char> = word.chars().collect();

    let mut cur_coords = coords;
    for letter in letters {
        let cur_char = probe(grid, cur_coords);
        if cur_char.is_some() && cur_char.unwrap() == letter {
            cur_coords = (cur_coords.0 + delta.0, cur_coords.1 + delta.1)
        } else {
            return false;
        }
    }
    return true;
}

fn probe(grid: &Vec<Vec<char>>, coords: (i32, i32)) -> Option<char> {
    if coords.0 < 0 || coords.1 < 0 {
        return None;
    }

    let (c0, c1) = (
        usize::try_from(coords.0).unwrap(),
        usize::try_from(coords.1).unwrap(),
    );

    if c0 >= grid.len() || c1 >= grid[0].len() {
        return None;
    }

    return Some(grid[c0][c1]);
}

//fn part2(input: &str) -> String {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(test_input), "18");
    }

    #[test]
    fn test_part2() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part2(test_input), "9");
    }
}
