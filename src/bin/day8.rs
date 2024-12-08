use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    mem::swap,
};

use aoc24::*;

fn main() {
    let binding = read_input(8);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let (grid, antennae) = get_antennae_list(input);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for matched in antennae.values() {
        add_uniq_nodes(matched, &mut antinodes, (grid.len(), grid[0].len()));
    }

    return antinodes.len().to_string();
}

fn part2(input: &str) -> String {
    let (grid, antennae) = get_antennae_list(input);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for matched in antennae.values() {
        add_lattice_points(matched, &mut antinodes, (grid.len(), grid[0].len()));
    }

    return antinodes.len().to_string();
}

fn get_antennae_list(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(i32, i32)>>) {
    let grid = parse_string_array(input);
    let mut antennae: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char != '.' {
                if !antennae.contains_key(char) {
                    antennae.insert(*char, Vec::new());
                }
                let list = antennae.get_mut(char).unwrap();
                list.push((i as i32, j as i32));
            }
        }
    }
    (grid, antennae)
}

// TODO: reduce diff coords down to irreducible form: divide by gcf
fn irreducible_coords(coord: (i32, i32)) -> (i32, i32) {
    let gcf = gcf(coord.0, coord.1);
    return (coord.0 / gcf, coord.1 / gcf);
}

fn gcf(a: i32, b: i32) -> i32 {
    let mut greater = max(a, b);
    let mut lesser = min(a, b);
    if lesser == 0 {
        return greater;
    }

    loop {
        greater = greater % lesser;
        if greater == 0 {
            return lesser;
        }

        swap(&mut greater, &mut lesser);
    }
}

/**
*
*/
fn add_lattice_points(
    antennae: &Vec<(i32, i32)>,
    locs: &mut HashSet<(i32, i32)>,
    bounds: (usize, usize),
) {
    for i in 0..antennae.len() {
        for j in i + 1..antennae.len() {
            let diff = irreducible_coords(sub_tup(antennae[j], antennae[i]));

            let mut cur_coord = antennae[i];
            while bounds_check(cur_coord, bounds) {
                locs.insert(cur_coord);
                cur_coord = add_tup(cur_coord, diff);
            }
            cur_coord = antennae[i];
            while bounds_check(cur_coord, bounds) {
                locs.insert(cur_coord);
                cur_coord = sub_tup(cur_coord, diff);
            }
        }
    }
}

fn add_uniq_nodes(
    antennae: &Vec<(i32, i32)>,
    locs: &mut HashSet<(i32, i32)>,
    bounds: (usize, usize),
) {
    for i in 0..antennae.len() {
        for j in i + 1..antennae.len() {
            let diff = sub_tup(antennae[j], antennae[i]);
            let node1 = add_tup(antennae[j], diff);
            let node2 = sub_tup(antennae[i], diff);

            if bounds_check(node1, bounds) {
                locs.insert(node1);
            }
            if bounds_check(node2, bounds) {
                locs.insert(node2);
            }
        }
    }
}

fn add_tup(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    return (a.0 + b.0, a.1 + b.1);
}

fn sub_tup(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    return (a.0 - b.0, a.1 - b.1);
}

fn bounds_check(coords: (i32, i32), bounds: (usize, usize)) -> bool {
    return coords.0 >= 0
        && coords.0 < bounds.0 as i32
        && coords.1 >= 0
        && coords.1 < bounds.1 as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "14");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "34");
    }
}
