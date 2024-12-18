fn dijkstra_counter(
    grid: &Vec<Vec<char>>,
    start: (usize, usize, usize),
    end: (usize, usize),
) -> usize {
    const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut explored: HashSet<(usize, usize, usize)> = HashSet::new();
    // map from distances to values. When a distance is to be updated, just add a new one.
    //let mut to_explore: BTreeMap<usize, (usize, usize, usize)> = BTreeMap::new();
    let mut to_explore: PQueue = PQueue::new();
    let mut counter = Counter::new();

    to_explore.insert(0, start);
    counter.increment(start, 1);

    while !to_explore.is_empty() {
        let (cur_dist, cur_coord) = to_explore.pop_min();
        //println!("{:?}", cur_coord);
        if !explored.contains(&cur_coord) {
            explored.insert(cur_coord);
            let num_paths = counter.get(&cur_coord).unwrap();

            let straight = uadd_idirection((cur_coord.0, cur_coord.1), DELTAS[cur_coord.2]);
            if grid[straight.0][straight.1] != '#' {
                let straight_with_orientation = (straight.0, straight.1, cur_coord.2);

                if to_explore.contains_greater(cur_dist + 1, straight_with_orientation) {
                    counter.clear(&straight_with_orientation);
                }

                to_explore.insert(cur_dist + 1, straight_with_orientation);
                counter.increment(straight_with_orientation, num_paths);
            }

            let turn_right = (cur_coord.0, cur_coord.1, (cur_coord.2 + 1) % 4);
            if to_explore.contains_greater(cur_dist + 1000, turn_right) {
                counter.clear(&turn_right);
            }
            to_explore.insert(cur_dist + 1000, turn_right);
            counter.increment(turn_right, num_paths);

            let turn_left = (cur_coord.0, cur_coord.1, (cur_coord.2 + 3) % 4);
            if to_explore.contains_greater(cur_dist + 1000, turn_left) {
                counter.clear(&turn_left);
            }
            to_explore.insert(cur_dist + 1000, turn_left);
            counter.increment(turn_left, num_paths);
        }
    }
    counter.get(&(end.0, end.1, 1)).unwrap_or(0) + counter.get(&(end.0, end.1, 2)).unwrap_or(0)
}

struct Counter<T: Hash + Eq> {
    counter: HashMap<T, usize>,
}

impl<T: Hash + Eq> Counter<T> {
    fn new() -> Self {
        Counter::<T> {
            counter: HashMap::new(),
        }
    }

    fn increment(&mut self, key: T, inc: usize) {
        let entry = self.counter.entry(key).or_insert(0);
        *entry += inc;
    }

    fn clear(&mut self, key: &T) {
        self.counter.remove(key);
    }

    fn get(&self, key: &T) -> Option<usize> {
        if self.counter.contains_key(key) {
            return Some(self.counter[key]);
        } else {
            return None;
        }
    }
}
