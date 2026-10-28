use aoc24::*;

fn main() {
    let binding = read_input(21);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

// invariants:
//  All arms must be pointing at A at the beginning/end of each line of input.

// I think I should just try to brute-force solve the puzzle right now?
//  When we punch a number in on the final keypad, all other arms must be on the A.
//  So we can memoize number-to-number costs at the very least.
//  Or directional costs.

// The base case is steering a single arm around an arrow pad. We memoize the costs of each
// pairwise arrangements of movements (there are 5P2 = 20)
// These can be calculated from the manhattan distance + 1, because we have control of the first
// remote

// On the next remote, we pathfind (A* maybe?) for each pair using the cost of each pair of keys
// from the first iteration
// and on and on

// For N keys and R layers of robotic arms, this ends up being an n^2 * L problem. We can decrease
// the constant time cost by only calculating the routes we need and memoizing and generating the
// possible paths from key to key more intelligently.
// If I solve the arrowpads by hand I can probably also find a mathematical relationship between
// the increasing arrowpad costs (thinking its matrix related possibly?)
// But no matter what we do we have to pathfind one way or another?
// Seems like there are smarter ways to go about this whole shtick.

fn part1(input: &str) -> String {
    "".to_string()
}

fn part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "126384");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "0");
    }
}
