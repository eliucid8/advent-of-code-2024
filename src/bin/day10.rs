use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use aoc24::*;

fn main() {
    let binding = read_input(10);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let grid = get_grid(input);
    let trailheads = find_trailheads(&grid);

    let mut sum = 0;
    for trailhead in trailheads {
        sum += find_peaks(&grid, trailhead).len()
    }
    sum.to_string()
}

fn part2(input: &str) -> String {
    let grid = get_grid(input);
    let trailheads = find_trailheads(&grid);

    let mut sum = 0;
    for trailhead in trailheads {
        sum += find_unique_trails(&grid, trailhead);
    }
    sum.to_string()
}

fn find_unique_trails(grid: &Vec<Vec<i32>>, start: (usize, usize)) -> usize {
    let deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut to_visit_unique: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
    let mut total_paths: usize = 0;

    to_visit_unique.insert(start);
    to_visit.push_back(start);
    visited.insert(start, 1);

    while !to_visit.is_empty() {
        // pop from front of queue
        let current = to_visit.pop_front().unwrap();
        to_visit_unique.remove(&current);
        // if at peak, add current_num_paths to total_paths
        let elevation = index_location(&grid, current).unwrap();
        let current_num_paths = visited.get(&current).unwrap().clone();
        if elevation == 9 {
            total_paths += current_num_paths;
        }
        // check all neighboring squares
        for delta in deltas {
            let new_location = (
                (current.0 as i32 + delta.0) as usize,
                (current.1 as i32 + delta.1) as usize,
            );
            let probe_result = index_location(&grid, new_location);
            // if we are in the grid
            if probe_result.is_some() {
                // if next square is higher
                if probe_result.unwrap() == elevation + 1 {
                    // increment visit counter
                    increment_counter(&mut visited, &new_location, current_num_paths);
                    // push new location to queue if it isn't in there already
                    if !to_visit_unique.contains(&new_location) {
                        to_visit.push_back(new_location);
                        to_visit_unique.insert(new_location);
                    }
                }
            }
        }
    }
    total_paths
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

/**
* dfs through grid, find # peaks reachable
*/
fn find_peaks(grid: &Vec<Vec<i32>>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: HashSet<(usize, usize)> = HashSet::new();
    let mut peaks: HashSet<(usize, usize)> = HashSet::new();
    to_visit.insert(start);

    while !to_visit.is_empty() {
        // set pop
        let current = to_visit.iter().next().unwrap().clone();
        to_visit.remove(&current);

        let elevation = index_location(&grid, current).unwrap();
        if elevation == 9 {
            peaks.insert(current);
        }

        for delta in deltas {
            let new_location = (
                (current.0 as i32 + delta.0) as usize,
                (current.1 as i32 + delta.1) as usize,
            );
            let probe_result = index_location(&grid, new_location);
            if !visited.contains(&new_location) && probe_result.is_some() {
                if probe_result.unwrap() == elevation + 1 {
                    to_visit.insert(new_location);
                }
            }
        }
        visited.insert(current);
    }
    peaks
}

fn index_location(grid: &Vec<Vec<i32>>, idx: (usize, usize)) -> Option<i32> {
    if
    /* idx.0 >= 0 && */
    idx.0 < grid.len() && /* idx.1 >= 0 && */ idx.1 < grid[0].len() {
        return Some(grid[idx.0][idx.1]);
    }
    return None;
}

/**
* precondition: Grid has at least 1 row
*/
fn find_trailheads(grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }
    trailheads
}

fn get_grid(input: &str) -> Vec<Vec<i32>> {
    parse_row_major::<i32>(input, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    static TEST_INPUT_2: &str = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "36");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "81");
    }

    #[test]
    fn test_part2_simple() {
        assert_eq!(part2(TEST_INPUT_2), "6");
    }
}
