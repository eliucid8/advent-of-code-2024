use std::collections::{HashMap, HashSet, VecDeque};

use aoc24::*;

fn main() {
    let binding = read_input(20);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let total = find_timesaves(input, 2, 100);

    total.to_string()
}

fn part2(input: &str) -> String {
    let total = find_timesaves(input, 20, 100);

    total.to_string()
}

fn find_timesaves(input: &str, cheat_time: usize, threshold: usize) -> usize {
    let grid = parse_string_array(input);
    let mut path = make_path_grid(&grid);
    let (start, end) = find_start_end(&grid);
    bfs_label(&grid, &mut path, start);

    let mut total = 0;

    let cheat_dict = compile_cheats(&path, cheat_time);
    for timesave in cheat_dict {
        if timesave.0 >= threshold {
            total += timesave.1.len();
        }
    }
    total
}

fn compile_cheats(path: &Vec<Vec<i32>>, cheat_time: usize) -> HashMap<usize, Vec<(usize, usize)>> {
    let mut cheat_counter = HashMap::new();
    for i in 0..path.len() {
        for j in 0..path[0].len() {
            if path[i][j] != -1 {
                let cheats = find_cheats(path, (i, j), cheat_time);
                if cheats.is_some() {
                    for cheat in cheats.unwrap() {
                        let entry: &mut Vec<(usize, usize)> =
                            cheat_counter.entry(cheat).or_default();
                        entry.push((i, j));
                    }
                }
            }
        }
    }

    cheat_counter
}

fn find_cheats(path: &Vec<Vec<i32>>, start: (usize, usize), limit: usize) -> Option<Vec<usize>> {
    let mut saved = Vec::new();
    let bounds = (path.len(), path[0].len());
    let startdist = path[start.0][start.1];
    for dist in 1..=limit {
        for x in 1..=dist {
            let y = dist - x;
            let mut offset = (x as i32, y as i32);
            for _ in 0..4 {
                let newcoord = add_delta(start, offset, bounds);
                if newcoord.is_some() {
                    let nc = newcoord.unwrap();
                    let time_diff = path[nc.0][nc.1] - startdist - dist as i32;
                    if time_diff > 0 {
                        saved.push(time_diff as usize);
                    }
                }
                offset = rotate_90(offset);
            }
        }
    }

    if saved.len() == 0 {
        return None;
    } else {
        return Some(saved);
    }
}

fn rotate_90(coord: (i32, i32)) -> (i32, i32) {
    return (-coord.1, coord.0);
}

fn find_cheat_2(path: &Vec<Vec<i32>>, start: (usize, usize)) -> Option<Vec<usize>> {
    let mut saved = Vec::new();
    const DELTAS1: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    const DELTAS2: [(i32, i32); 8] = [
        (2, 0),
        (1, 1),
        (1, -1),
        (0, 2),
        (0, -2),
        (-1, 1),
        (-1, -1),
        (-2, 0),
    ];
    let bounds = (path.len(), path[0].len());
    let startdist = path[start.0][start.1];
    for delta in DELTAS1 {
        let newcoord = add_delta(start, delta, bounds);
        if newcoord.is_some() {
            let nc = newcoord.unwrap();
            let time_diff = path[nc.0][nc.1] - startdist - 1;
            if time_diff > 0 {
                saved.push(time_diff as usize);
            }
        }
    }
    for delta in DELTAS2 {
        let newcoord = add_delta(start, delta, bounds);
        if newcoord.is_some() {
            let nc = newcoord.unwrap();
            let time_diff = path[nc.0][nc.1] - startdist - 2;
            if time_diff > 0 {
                saved.push(time_diff as usize);
            }
        }
    }

    if saved.len() == 0 {
        return None;
    } else {
        return Some(saved);
    }
}

pub fn add_delta(
    coord: (usize, usize),
    direction: (i32, i32),
    bounds: (usize, usize),
) -> Option<(usize, usize)> {
    if (coord.0 as i32 + direction.0) < 0 || (coord.1 as i32 + direction.1) < 0 {
        return None;
    }
    let newcoord = (
        (coord.0 as i32 + direction.0) as usize,
        (coord.1 as i32 + direction.1) as usize,
    );
    if newcoord.0 < bounds.0 && newcoord.1 < bounds.1 {
        return Some(newcoord);
    } else {
        return None;
    }
}

fn find_start_end(grid: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = (i, j);
            }
            if grid[i][j] == 'E' {
                end = (i, j);
            }
        }
    }
    (start, end)
}

fn bfs_label(grid: &Vec<Vec<char>>, distances: &mut Vec<Vec<i32>>, start: (usize, usize)) {
    let mut to_explore = VecDeque::new();
    let mut explored = HashSet::new();
    to_explore.push_back(start);
    const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut distance = 0;

    while !to_explore.is_empty() {
        let cur = to_explore.pop_front().unwrap();
        explored.insert(cur);
        distances[cur.0][cur.1] = distance;

        let mut num_neighbors = 0;
        for i in 0..4 {
            let next_explore = uadd_idirection(cur, DELTAS[i]);
            if !explored.contains(&next_explore) && grid[next_explore.0][next_explore.1] != '#' {
                num_neighbors += 1;
                if num_neighbors > 1 {
                    println!("more than 1 path: {:?}", next_explore);
                }
                to_explore.push_back(next_explore);
            }
        }
        distance += 1;
    }
}

fn make_path_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut path_grid = Vec::new();
    for row in grid {
        let mut path_row = Vec::new();
        for _ in row {
            path_row.push(-1);
        }
        path_grid.push(path_row);
    }
    path_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "0");
    }
}
