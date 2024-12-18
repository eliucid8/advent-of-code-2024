use std::collections::{HashSet, VecDeque};

use aoc24::*;

fn main() {
    let binding = read_input(18);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    part1_var_size(input, 71, 1024).to_string()
}

fn part2(input: &str) -> String {
    part1_var_size(input, 71, 2957).to_string();

    part2_var_size(input, 71, 2957).to_string()
}

fn part2_var_size(input: &str, grid_size: usize, num_bytes: usize) -> String {
    let mut grid = make_grid(grid_size, true);
    let bytes: Vec<(usize, usize)> = parse_coords(input)
        .iter()
        .map(|c| -> (usize, usize) { (c.0 as usize, c.1 as usize) })
        .collect();
    println!("parsed bytes");

    fill_bytes(&mut grid, &bytes, num_bytes);
    for i in num_bytes..bytes.len() {
        grid[bytes[i].0][bytes[i].1] = false;
        let len = bfs_len(&grid);
        if len == 0 {
            println!("failed at byte {}", i);
            print_grid(&grid);
            return format!("{},{}", bytes[i].1, bytes[i].0);
        }
    }
    return "oops".to_string();
}

fn part1_var_size(input: &str, grid_size: usize, num_bytes: usize) -> String {
    let mut grid = make_grid(grid_size, true);
    println!("made grid");
    let bytes: Vec<(usize, usize)> = parse_coords(input)
        .iter()
        .map(|c| -> (usize, usize) { (c.0 as usize, c.1 as usize) })
        .collect();
    println!("parsed bytes");

    fill_bytes(&mut grid, &bytes, num_bytes);
    println!("filled bytes");
    print_grid(&grid);

    bfs_len(&grid).to_string()
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    let mut rep = String::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] {
                rep.push('.');
            } else {
                rep.push('#');
            }
        }
        rep.push('\n');
    }
    println!("{}", rep);
}

/**
* return how long it takes to get from top left to bottom right
*/
fn bfs_len(grid: &Vec<Vec<bool>>) -> usize {
    const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let end = (grid.len() - 1, grid[0].len() - 1);
    let mut explored = HashSet::new();
    let mut to_explore = VecDeque::new();
    to_explore.push_back(((0, 0), 0));

    while !to_explore.is_empty() {
        let (coord, dist) = to_explore.pop_front().unwrap();
        if explored.contains(&coord) {
            continue;
        }
        explored.insert(coord);
        if coord == end {
            return dist;
        }
        for direction in 0..4 {
            if probe_grid(&grid, coord, direction) {
                let new_explore = uadd_idirection(coord, DELTAS[direction]);
                to_explore.push_back((new_explore, dist + 1));
            }
        }
    }
    0
}

/**
* return true if the direction specified is navigable.
*/
fn probe_grid(grid: &Vec<Vec<bool>>, coord: (usize, usize), direction: usize) -> bool {
    const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let i = coord.0 as i32 + DELTAS[direction].0;
    let j = coord.1 as i32 + DELTAS[direction].1;

    if i >= 0 && i < grid.len() as i32 && j >= 0 && j < grid[0].len() as i32 {
        return grid[i as usize][j as usize];
    } else {
        return false;
    }
}

fn fill_bytes(grid: &mut Vec<Vec<bool>>, bytes: &Vec<(usize, usize)>, num: usize) {
    for i in 0..num {
        let byte = bytes[i];
        grid[byte.0][byte.1] = false;
    }
}

fn make_grid<T: Clone>(len: usize, fill: T) -> Vec<Vec<T>> {
    let mut ret = Vec::with_capacity(len);
    for _ in 0..len {
        let row = vec![fill.clone(); len];
        ret.push(row);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        assert_eq!(part1_var_size(TEST_INPUT, 7, 12), "22");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_var_size(TEST_INPUT, 7, 12), "6,1");
    }
}
