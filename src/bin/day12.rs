use std::collections::HashSet;

use aoc24::*;

fn main() {
    let binding = read_input(12);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let grid = parse_string_array(input);
    let mut visited = HashSet::new();
    let mut sum: usize = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let cur_loc = (i, j);
            if !visited.contains(&cur_loc) {
                let ap = bfs_part1(&grid, cur_loc, &mut visited);
                sum += ap.0 * ap.1;
            }
        }
    }

    sum.to_string()
}

/**
 * Now, we only count the number of sides
 * This means that contiguous, aligned edges count as a single side
 * My current idea is to keep track of fence sides, merging them if we find a contiguous fence
 * In this case, I belive we can simply keep track of all fence edges, and then the number of
 * contiguous fences in a single variable we decrement every time we coalesce a fence edge.
 */
fn part2(input: &str) -> String {
    let grid = parse_string_array(input);
    let mut visited = HashSet::new();
    let mut sum: usize = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let cur_loc = (i, j);
            if !visited.contains(&cur_loc) {
                let ap = bfs_part2(&grid, cur_loc, &mut visited);
                sum += ap.0 * ap.1;
            }
        }
    }

    sum.to_string()
}

// the directions that we search in--unit circle convention
static DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn bfs_part2(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut to_visit = HashSet::new();
    to_visit.insert(start);

    let mut area: usize = 0;
    let mut num_sides: usize = 0;
    let mut fences = HashSet::new();

    while !to_visit.is_empty() {
        let cur_loc = to_visit.iter().next().unwrap().clone();
        to_visit.remove(&cur_loc);
        let (neighbors, edges) = get_neighbors_and_edges(&grid, cur_loc);

        area += 1;
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                to_visit.insert(neighbor);
            }
        }
        for edge in edges {
            coalesce_fences(&mut fences, edge, &mut num_sides);
        }

        visited.insert(cur_loc);
    }

    (area, num_sides)
}

/**
* A clean way to do this would be to return the delta to be added to sides, but then that would
* involve weird conversions between usize and an i type which I don't like.
* Oh well, we're already modifying the hashset anyway
*/
fn coalesce_fences(
    fences: &mut HashSet<(usize, usize, usize)>,
    new_fence: (usize, usize, usize),
    num_sides: &mut usize,
) {
    fences.insert(new_fence);
    *num_sides += 1;
    let left_fence_direction = (new_fence.2 + 1) % 4;
    let left_fence_loc = loc_plus_delta((new_fence.0, new_fence.1), DELTAS[left_fence_direction]);
    let right_fence_direction = (new_fence.2 + 3) % 4;
    let right_fence_loc = loc_plus_delta((new_fence.0, new_fence.1), DELTAS[right_fence_direction]);

    if fences.contains(&(left_fence_loc.0, left_fence_loc.1, new_fence.2)) {
        *num_sides -= 1
    }
    if fences.contains(&(right_fence_loc.0, right_fence_loc.1, new_fence.2)) {
        *num_sides -= 1
    }
}

/**
 * adds a location plus a delta
 */
fn loc_plus_delta(loc: (usize, usize), delta: (i32, i32)) -> (usize, usize) {
    (
        (loc.0 as i32 + delta.0) as usize,
        (loc.1 as i32 + delta.1) as usize,
    )
}

/**
 * Returns: Area, Perimeter
 */
fn bfs_part1(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut to_visit = HashSet::new();
    to_visit.insert(start);

    let mut area: usize = 0;
    let mut perimeter: usize = 0;
    while !to_visit.is_empty() {
        let cur_loc = to_visit.iter().next().unwrap().clone();
        to_visit.remove(&cur_loc);
        let neighbors = get_neighbors(&grid, cur_loc);

        area += 1;
        perimeter += 4 - neighbors.len();
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                to_visit.insert(neighbor);
            }
        }

        visited.insert(cur_loc);
    }

    (area, perimeter)
}

/**
 * Precondition: loc is in bounds of grid
 * Perhaps it's time to start using OOP
 */
fn get_neighbors_and_edges(
    grid: &Vec<Vec<char>>,
    loc: (usize, usize),
) -> (Vec<(usize, usize)>, Vec<(usize, usize, usize)>) {
    let mut neighbors = Vec::new();
    let mut edges = Vec::new();
    let cur_crop = grid[loc.0][loc.1];
    for i in 0..4 {
        let delta = DELTAS[i];
        let newloc = loc_plus_delta(loc, delta);
        if newloc.0 < grid.len() && newloc.1 < grid[0].len() && grid[newloc.0][newloc.1] == cur_crop
        {
            neighbors.push(newloc);
        } else {
            edges.push((newloc.0, newloc.1, i));
        }
    }
    (neighbors, edges)
}

/**
 * Precondition: loc is in bounds of grid
 */
fn get_neighbors(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> Vec<(usize, usize)> {
    //static DELTAS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut neighbors = Vec::new();
    let cur_crop = grid[loc.0][loc.1];
    for delta in DELTAS {
        let newloc = loc_plus_delta(loc, delta);
        if newloc.0 < grid.len() && newloc.1 < grid[0].len() && grid[newloc.0][newloc.1] == cur_crop
        {
            neighbors.push(newloc);
        }
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "1930");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "1206");
    }
}
