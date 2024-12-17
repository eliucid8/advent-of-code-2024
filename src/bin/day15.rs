use std::collections::VecDeque;

use aoc24::*;

fn main() {
    let binding = read_input(15);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let (mut warehouse, directions) = parse_input(input);
    for direction in directions {
        warehouse.move_robot(direction);
    }

    warehouse.gps().to_string()
}

fn part1_visual(input: &str) -> String {
    let (mut warehouse, directions) = parse_input(input);
    println!("{:#?}", directions);
    static ARROWS: [char; 4] = ['v', '>', '^', '<'];
    for direction in directions {
        warehouse.move_robot(direction);
        println!("{}", ARROWS[direction]);
        println!("{}", warehouse.display());
    }

    warehouse.gps().to_string()
}

fn part2(input: &str) -> String {
    let (mut warehouse, directions) = parse_input2(input);
    for direction in directions {
        warehouse.move_robot(direction);
    }

    warehouse.gps().to_string()
}

fn parse_input2(input: &str) -> (Widehouse, Vec<usize>) {
    let sections = split_sections(input);
    let widehouse = Widehouse::from_string(sections[0]);
    let directions = parse_directions(sections[1]);
    (widehouse, directions)
}

// returns warehouse object, array of indices to direction array
fn parse_input(input: &str) -> (Warehouse, Vec<usize>) {
    let sections = split_sections(input);
    let warehouse = Warehouse::from_string(sections[0]);
    let directions = parse_directions(sections[1]);
    (warehouse, directions)
}

fn parse_directions(directions: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    //println!("{}", directions);
    let _ = directions.chars().for_each(|c| match c {
        'v' => indices.push(0),
        '>' => indices.push(1),
        '^' => indices.push(2),
        '<' => indices.push(3),
        _ => (),
    });
    indices
}

struct Widehouse {
    warehouse: Warehouse,
}

impl Widehouse {
    fn new(layout: &Vec<Vec<char>>) -> Self {
        Widehouse {
            warehouse: Warehouse::new(layout),
        }
    }

    fn convert_row(row: &str) -> Vec<char> {
        let mut ret = Vec::new();
        let _ = row.chars().for_each(|c| match c {
            '.' => {
                ret.push('.');
                ret.push('.');
            }
            '@' => {
                ret.push('@');
                ret.push('.');
            }
            '#' => {
                ret.push('#');
                ret.push('#');
            }
            'O' => {
                ret.push('[');
                ret.push(']');
            }
            _ => (),
        });
        ret
    }

    fn from_string(layout: &str) -> Self {
        let mut grid = Vec::new();
        layout
            .lines()
            .for_each(|line| grid.push(Self::convert_row(line)));
        Self::new(&grid)
    }

    fn gps(&self) -> usize {
        self.warehouse.gps_char('[')
    }

    // NOTE: The meat of the implementation here
    //
    // If going vertical, bfs for boxes, continuing if the spots above the current one are
    // occupied with another box (stop if we encounter a wall or all empty space)
    // Maintain stack of boxes so we can move them (literall move chars) in the order we encounter
    // them
    fn move_robot(&mut self, direction: usize) {
        if direction % 2 == 0 {
            self.move_robot_vertical(direction);
        } else {
            self.move_robot_horizontal(direction);
        }
    }

    fn move_robot_horizontal(&mut self, direction_idx: usize) {
        static DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let direction = DIRECTIONS[direction_idx];
        let grid = &mut self.warehouse.grid;

        let mut to_explore: VecDeque<(usize, usize)> = VecDeque::new();
        to_explore.push_back(Self::add_direction(self.warehouse.robot, direction));
        let mut to_push: Vec<(usize, usize)> = Vec::new(); // stack of blocks to move_box_char
        let mut move_robot: bool = true;

        while !to_explore.is_empty() {
            let cur_loc = to_explore.pop_front().unwrap();
            match grid[cur_loc.0][cur_loc.1] {
                '#' => {
                    to_explore.clear();
                    to_push.clear();
                    move_robot = false;
                    break;
                }
                '[' => {
                    let explore = Self::add_direction(cur_loc, direction);
                    if direction_idx == 1 {
                        Self::add_direction(cur_loc, direction);
                    }
                    to_explore.push_back(explore);
                    to_push.push(cur_loc);
                }
                ']' => {
                    let explore = Self::add_direction(cur_loc, direction);
                    if direction_idx == 3 {
                        Self::add_direction(cur_loc, direction);
                    }
                    to_explore.push_back(explore);
                    to_push.push(Self::add_direction(cur_loc, (0, -1)));
                }
                _ => (),
            }
        }

        while !to_push.is_empty() {
            self.move_box_char(to_push.pop().unwrap(), direction_idx);
        }

        if move_robot {
            let next_robot = Self::add_direction(self.warehouse.robot, direction);
            self.warehouse.grid[next_robot.0][next_robot.1] = '@';
            self.warehouse.grid[self.warehouse.robot.0][self.warehouse.robot.1] = '.';
            self.warehouse.robot = next_robot;
        }
    }

    fn move_robot_vertical(&mut self, direction_idx: usize) {
        static DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let direction = DIRECTIONS[direction_idx];
        let grid = &mut self.warehouse.grid;

        let mut to_explore: VecDeque<(usize, usize)> = VecDeque::new();
        to_explore.push_back(Self::add_direction(self.warehouse.robot, direction));
        let mut to_push: Vec<(usize, usize)> = Vec::new(); // stack of blocks to move_box_char
        let mut move_robot: bool = true;

        while !to_explore.is_empty() {
            let cur_loc = to_explore.pop_front().unwrap();
            match grid[cur_loc.0][cur_loc.1] {
                '#' => {
                    to_explore.clear();
                    to_push.clear();
                    move_robot = false;
                    break;
                }
                '[' => {
                    let left_explore = Self::add_direction(cur_loc, direction);
                    to_explore.push_back(left_explore);
                    // explore right as well.
                    to_explore.push_back(Self::add_direction(left_explore, (0, 1)));
                    to_push.push(cur_loc);
                }
                ']' => {
                    let right_explore = Self::add_direction(cur_loc, direction);
                    to_explore.push_back(right_explore);
                    // explore right as well.
                    to_explore.push_back(Self::add_direction(right_explore, (0, -1)));
                    to_push.push(Self::add_direction(cur_loc, (0, -1)));
                }
                _ => (),
            }
        }

        while !to_push.is_empty() {
            self.move_box_char(to_push.pop().unwrap(), direction_idx);
        }

        if move_robot {
            let next_robot = Self::add_direction(self.warehouse.robot, direction);
            self.warehouse.grid[next_robot.0][next_robot.1] = '@';
            self.warehouse.grid[self.warehouse.robot.0][self.warehouse.robot.1] = '.';
            self.warehouse.robot = next_robot;
        }
    }

    /**
     * Assume loc is the coordinate of the left side of the box.
     * Does not check if the move is legal.
     */
    fn move_box_char(&mut self, loc: (usize, usize), direction: usize) {
        let grid = &mut self.warehouse.grid;
        static DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        grid[loc.0][loc.1] = '.';
        grid[loc.0][loc.1 + 1] = '.';

        let new_loc = Self::add_direction(loc, DIRECTIONS[direction]);
        grid[new_loc.0][new_loc.1] = '[';
        grid[new_loc.0][new_loc.1 + 1] = ']';
    }

    fn add_direction(coord: (usize, usize), direction: (i32, i32)) -> (usize, usize) {
        Warehouse::add_direction(coord, direction)
    }

    fn display(&self) -> String {
        self.warehouse.display()
    }
}

struct Warehouse {
    grid: Vec<Vec<char>>,
    robot: (usize, usize),
}

trait WarehouseTrait {
    fn new(layout: &Vec<Vec<char>>) -> Self;

    fn from_string(layout: &str) -> Self;

    fn gps(&self) -> usize;

    fn move_robot(&mut self, direction: usize);

    fn add_direction(coord: (usize, usize), direction: (i32, i32)) -> (usize, usize);

    fn display(&self) -> String;
}

impl WarehouseTrait for Warehouse {
    fn new(layout: &Vec<Vec<char>>) -> Self {
        let mut robot = (0, 0);
        for i in 0..layout.len() {
            for j in 0..layout[0].len() {
                if layout[i][j] == '@' {
                    robot = (i, j);
                }
            }
        }

        Warehouse {
            grid: layout.clone(),
            robot,
        }
    }

    fn from_string(layout: &str) -> Self {
        Self::new(&parse_string_array(layout))
    }

    fn gps(&self) -> usize {
        self.gps_char('O')
    }

    fn move_robot(&mut self, direction: usize) {
        static DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut loc = Self::add_direction(self.robot, DIRECTIONS[direction]);
        let mut num_pushed = 0;
        while self.grid[loc.0][loc.1] == 'O' {
            loc = Self::add_direction(loc, DIRECTIONS[direction]);
            num_pushed += 1;
        }
        if self.grid[loc.0][loc.1] != '#' {
            if num_pushed > 0 {
                self.grid[loc.0][loc.1] = 'O';
            }
            let next_robot = Self::add_direction(self.robot, DIRECTIONS[direction]);
            self.grid[next_robot.0][next_robot.1] = '@';
            self.grid[self.robot.0][self.robot.1] = '.';
            self.robot = next_robot;
        }
    }

    fn add_direction(coord: (usize, usize), direction: (i32, i32)) -> (usize, usize) {
        (
            (coord.0 as i32 + direction.0) as usize,
            (coord.1 as i32 + direction.1) as usize,
        )
    }

    fn display(&self) -> String {
        let mut ret = String::new();
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                ret.push(self.grid[i][j]);
            }
            ret.push('\n');
        }
        ret
    }
}

impl Warehouse {
    fn gps_char(&self, c: char) -> usize {
        let mut sum = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == c {
                    sum += i * 100 + j;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small() {
        assert_eq!(part1(TEST_INPUT_1), "2028");
    }

    #[test]
    fn test_part1_big() {
        assert_eq!(part1(TEST_INPUT_2), "10092");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), "9021");
    }

    static TEST_INPUT_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    static TEST_INPUT_2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
}
