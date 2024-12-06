use std::collections::HashSet;

use aoc24::*;

fn main() {
    let binding = read_input(6);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let grid = parse_string_array(input);
    return orig_path(&grid).len().to_string();
}

fn orig_path(grid: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let mut loc = find_start(&grid);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    static DELTAS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;

    loop {
        visited.insert(loc);
        let newloc = (loc.0 + DELTAS[dir].0, loc.1 + DELTAS[dir].1);
        if !bounds_check(newloc, &grid) {
            break;
        }
        if grid[newloc.0 as usize][newloc.1 as usize] == '#' {
            dir = (dir + 1) % 4;
        } else {
            loc = newloc;
        }
    }

    return visited;
}

/**
 * Exhaustively search through all obstruction locations
 * Check to see if guard ends up in same location with same orientation
 * Keep track of visited locations with 3-tuple now, third representing orientation.
 */
fn part2(input: &str) -> String {
    let grid = parse_string_array(input);
    let start = find_start(&grid);
    let mut count = 0;

    for loc in orig_path(&grid) {
        if loop_check(start, (loc.0 as usize, loc.1 as usize), &grid) {
            count += 1;
        }
    }
    return count.to_string();
}

/**
* returns true if there's a loop
*/
fn loop_check(start: (i32, i32), obstacle: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    if grid[obstacle.0][obstacle.1] == '#' || grid[obstacle.0][obstacle.1] == '^' {
        return false;
    }
    let mut loc = start;
    let mut visited: HashSet<(i32, i32, usize)> = HashSet::new();

    static DELTAS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir: usize = 0;

    loop {
        if !visited.insert((loc.0, loc.1, dir)) {
            return true;
        }
        let newloc = (loc.0 + DELTAS[dir].0, loc.1 + DELTAS[dir].1);
        if !bounds_check(newloc, &grid) {
            return false;
        }
        if grid[newloc.0 as usize][newloc.1 as usize] == '#'
            || (newloc.0 as usize == obstacle.0 && newloc.1 as usize == obstacle.1)
        {
            dir = (dir + 1) % 4;
        } else {
            loc = newloc;
        }
    }
}

fn bounds_check(loc: (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    let grid_bounds: (i32, i32) = (grid.len() as i32, grid[0].len() as i32);
    return loc.0 >= 0 && loc.0 < grid_bounds.0 && loc.1 >= 0 && loc.1 < grid_bounds.1;
}

/**
 * kinda ugly way of finding start location but idc
 */
fn find_start(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                return (i.try_into().unwrap(), j.try_into().unwrap());
            }
        }
    }
    return (0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "41");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "6");
    }
}
