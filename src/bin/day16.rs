use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
};

use aoc24::*;

fn main() {
    let binding = read_input(16);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let grid = get_grid(input);
    let (start, end) = get_start_end(&grid);
    let dist = dijkstra(&grid, (start.0, start.1, 1), end);

    dist.to_string()
}

fn part2(input: &str) -> String {
    let grid = get_grid(input);
    let (start, end) = get_start_end(&grid);
    let (dag, end_orientations) = dijkstra_dag(&grid, (start.0, start.1, 1), end);
    //println!("{:?}", dag.from);
    let start_orientation = (start.0, start.1, 1);
    let mut seat_set = HashSet::new();
    for orientation in end_orientations {
        seat_set = seat_set
            .union(&find_dag_paths(
                &dag,
                start_orientation,
                (end.0, end.1, orientation),
            ))
            .copied()
            .collect();
    }

    print_grid_traversed(grid, &seat_set);
    seat_set.len().to_string()
}

fn print_grid_traversed(grid: Vec<Vec<char>>, seats: &HashSet<(usize, usize)>) {
    let mut grid_string = String::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if seats.contains(&(i, j)) {
                grid_string.push('O');
            } else {
                grid_string.push(grid[i][j]);
            }
        }
        grid_string.push('\n');
    }
    println!("{}", grid_string);
}

fn find_dag_paths(
    dag: &DAG<(usize, usize, usize)>,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
) -> HashSet<(usize, usize)> {
    let mut to_explore = Vec::new();
    let mut explored = HashSet::new();
    let mut seats = HashSet::new();
    to_explore.push(end);

    while !to_explore.is_empty() {
        let cur = to_explore.pop().unwrap();
        explored.insert(cur);
        seats.insert((cur.0, cur.1));
        //println!("{:?}", cur);
        for link in &dag.from[&cur] {
            if !explored.contains(link) && *link != start {
                to_explore.push(*link);
            }
        }
    }

    seats
}

/**
* Returns a DAG constructed from dijkstra's and the orientation(s) with the lowest score
*/
fn dijkstra_dag(
    grid: &Vec<Vec<char>>,
    start: (usize, usize, usize),
    end: (usize, usize),
) -> (DAG<(usize, usize, usize)>, Vec<usize>) {
    const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut explored: HashSet<(usize, usize, usize)> = HashSet::new();
    // map from distances to values. When a distance is to be updated, just add a new one.
    //let mut to_explore: BTreeMap<usize, (usize, usize, usize)> = BTreeMap::new();
    let mut to_explore: PQueue = PQueue::new();
    to_explore.insert(0, start);

    let mut dag: DAG<(usize, usize, usize)> = DAG::new();
    let mut finish_up_dist = 0;
    let mut finish_right_dist = 0;

    while !to_explore.is_empty() {
        let (cur_dist, cur_coord) = to_explore.pop_min();
        //println!("{:?}", cur_coord);
        if !explored.contains(&cur_coord) {
            // set the finish dists
            if cur_coord == (end.0, end.1, 2) {
                finish_up_dist = cur_dist;
            }
            if cur_coord == (end.0, end.1, 1) {
                finish_right_dist = cur_dist;
            }

            explored.insert(cur_coord);
            let straight = uadd_idirection((cur_coord.0, cur_coord.1), DELTAS[cur_coord.2]);

            if grid[straight.0][straight.1] != '#' {
                let straight_with_orientation = (straight.0, straight.1, cur_coord.2);

                if to_explore.contains_greater(cur_dist + 1, straight_with_orientation) {
                    dag.clear_into(straight_with_orientation);
                }

                if !to_explore.contains_less(cur_dist + 1, straight_with_orientation) {
                    to_explore.insert(cur_dist + 1, straight_with_orientation);
                    dag.add(cur_coord, straight_with_orientation);
                }
            }

            let turn_right = (cur_coord.0, cur_coord.1, (cur_coord.2 + 1) % 4);
            if to_explore.contains_greater(cur_dist + 1000, turn_right) {
                dag.clear_into(turn_right);
            }
            if !to_explore.contains_less(cur_dist + 1000, turn_right) {
                to_explore.insert(cur_dist + 1000, turn_right);
                dag.add(cur_coord, turn_right);
            }

            let turn_left = (cur_coord.0, cur_coord.1, (cur_coord.2 + 3) % 4);
            if to_explore.contains_greater(cur_dist + 1000, turn_left) {
                dag.clear_into(turn_left);
            }
            if !to_explore.contains_less(cur_dist + 1000, turn_left) {
                to_explore.insert(cur_dist + 1000, turn_left);
                dag.add(cur_coord, turn_left);
            }
        }
    }

    let mut finish_orientations = Vec::new();
    if finish_up_dist <= finish_right_dist {
        finish_orientations.push(2);
    }
    if finish_right_dist <= finish_up_dist {
        finish_orientations.push(1);
    }

    return (dag, finish_orientations);
}

// dijkstra, but keeping track of rotation as well.
fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize, usize), end: (usize, usize)) -> usize {
    const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut explored: HashSet<(usize, usize, usize)> = HashSet::new();
    // map from distances to values. When a distance is to be updated, just add a new one.
    //let mut to_explore: BTreeMap<usize, (usize, usize, usize)> = BTreeMap::new();
    let mut to_explore: PQueue = PQueue::new();

    to_explore.insert(0, start);

    while !to_explore.is_empty() {
        let (cur_dist, cur_coord) = to_explore.pop_min();
        //println!("{:?}", cur_coord);
        if !explored.contains(&cur_coord) {
            if (cur_coord.0, cur_coord.1) == end {
                return cur_dist;
            }
            explored.insert(cur_coord);
            let straight = uadd_idirection((cur_coord.0, cur_coord.1), DELTAS[cur_coord.2]);

            if grid[straight.0][straight.1] != '#' {
                let straight_with_orientation = (straight.0, straight.1, cur_coord.2);
                to_explore.insert(cur_dist + 1, straight_with_orientation);
            }

            to_explore.insert(
                cur_dist + 1000,
                (cur_coord.0, cur_coord.1, (cur_coord.2 + 1) % 4),
            );

            to_explore.insert(
                cur_dist + 1000,
                (cur_coord.0, cur_coord.1, (cur_coord.2 + 3) % 4),
            );
        }
    }
    return 0;
}

struct DAG<T: Copy> {
    to: HashMap<T, HashSet<T>>,
    from: HashMap<T, HashSet<T>>,
}

impl<T: Eq + Hash + Copy> DAG<T> {
    fn new() -> Self {
        DAG::<T> {
            to: HashMap::new(),
            from: HashMap::new(),
        }
    }

    fn add(&mut self, from: T, to: T) {
        let to_entry = self.to.entry(from).or_insert(HashSet::new());
        to_entry.insert(to);
        let from_entry = self.from.entry(to).or_insert(HashSet::new());
        from_entry.insert(from);
    }

    /**
     * Clears all graph edges going in to a certain vertex.
     */
    fn clear_into(&mut self, to: T) {
        if !self.from.contains_key(&to) {
            return;
        }
        let from_entry = &self.from[&to];
        for origin_node in from_entry {
            let origin_set = self.to.get_mut(&origin_node).unwrap();
            origin_set.remove(&to);
        }
        self.from.remove(&to);
    }
}

struct PQueue {
    treemap: BTreeMap<usize, Vec<(usize, usize, usize)>>,
    dist_mappings: HashMap<(usize, usize, usize), usize>,
}

impl PQueue {
    fn new() -> Self {
        PQueue {
            treemap: BTreeMap::new(),
            dist_mappings: HashMap::new(),
        }
    }

    fn insert(&mut self, key: usize, value: (usize, usize, usize)) {
        if self.treemap.contains_key(&key) {
            self.treemap.get_mut(&key).unwrap().push(value);
        } else {
            let entry = vec![value];
            self.treemap.insert(key, entry);
        }
        // add/update dist mapping if it's smaller
        if !self.dist_mappings.contains_key(&value) || self.dist_mappings[&value] > key {
            self.dist_mappings.insert(value, key);
        }
    }

    fn is_empty(&self) -> bool {
        return self.treemap.is_empty();
    }

    fn pop_min(&mut self) -> (usize, (usize, usize, usize)) {
        let mut entry = self.treemap.first_entry().unwrap();
        let dist = *entry.key();
        let locations = entry.get_mut();
        if locations.len() == 1 {
            return (dist, entry.remove()[0]);
        } else {
            return (dist, locations.pop().unwrap());
        }
    }

    /**
     * Return true if we update the distance to a node to be explored.
     */
    fn contains_greater(&self, cur_dist: usize, orientation: (usize, usize, usize)) -> bool {
        self.dist_mappings.contains_key(&orientation) && self.dist_mappings[&orientation] > cur_dist
    }

    /**
     * Return true if we shouldn't add to dag
     */
    fn contains_less(&self, cur_dist: usize, orientation: (usize, usize, usize)) -> bool {
        self.dist_mappings.contains_key(&orientation) && self.dist_mappings[&orientation] < cur_dist
    }
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    parse_string_array(input)
}

fn get_start_end(grid: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    return ((grid.len() - 2, 1), (1, grid[0].len() - 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    static TEST_INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    static TEST_INPUT_3: &str = "#####
#..E#
#.#.#
#...#
#S..#
#####";

    #[test]
    fn test_part1_small() {
        assert_eq!(part1(TEST_INPUT_1), "7036");
    }

    #[test]
    fn test_part1_large() {
        assert_eq!(part1(TEST_INPUT_2), "11048");
    }

    #[test]
    fn test_part2_small() {
        assert_eq!(part2(TEST_INPUT_1), "45");
    }

    #[test]
    fn test_part2_large() {
        assert_eq!(part2(TEST_INPUT_2), "64");
    }

    #[test]
    fn test_both_orientations() {
        assert_eq!(part2(TEST_INPUT_3), "6");
    }
}
