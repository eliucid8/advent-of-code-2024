use std::collections::{HashMap, HashSet, VecDeque};

use aoc24::*;

fn main() {
    let binding = read_input(5);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&TEST_INPUT));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let sections = split_sections(input);
    let rules = parse_row_major::<usize>(sections[0], "|");
    let updates = parse_row_major::<usize>(sections[1], ",");

    let mut sum = 0;
    for update in updates {
        if check_all_rules(&rules, &update) {
            sum += middle_page(&update);
        }
    }

    return sum.to_string();
}

fn part2(input: &str) -> String {
    // topo sort rules
    let sections = split_sections(input);
    let rules = parse_row_major::<usize>(sections[0], "|");
    let mut updates = parse_row_major::<usize>(sections[1], ",");
    // Rules may have cycles...
    // filter out correctly-sorted updates
    updates.retain(|v| !check_all_rules(&rules, &v));
    let mut sum = 0;
    // grab correct ordering of rules from topo-sorted list
    for update in &updates {
        //println!("{:#?}", update);
        let relevant_rules = get_relevant_rules(&rules, &update);
        //println!("{:#?}", relevant_rules);
        let mut sorted_update = topo_sort_pages(&relevant_rules);
        //println!("{:#?}", sorted_update);
        sorted_update.retain(|x| update.contains(x));
        //println!("{:#?}", sorted_update);

        // return middle
        sum += middle_page(&sorted_update);
    }

    return sum.to_string();
}

fn get_relevant_rules(all_rules: &Vec<Vec<usize>>, update: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut relevant_rules = all_rules.clone();
    relevant_rules.retain(|v| update.contains(&v[0]) && update.contains(&v[1]));

    return relevant_rules;
}

fn topo_sort_pages(rules: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut outgoing_edges: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut incoming_edges: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut no_incoming: HashSet<usize> = HashSet::new();

    for rule in rules {
        no_incoming.insert(rule[0]);

        if !outgoing_edges.contains_key(&rule[0]) {
            outgoing_edges.insert(rule[0], HashSet::new());
        }
        outgoing_edges.get_mut(&rule[0]).unwrap().insert(rule[1]);
    }

    for rule in rules {
        no_incoming.remove(&rule[1]);

        if !incoming_edges.contains_key(&rule[1]) {
            incoming_edges.insert(rule[1], HashSet::new());
        }
        incoming_edges.get_mut(&rule[1]).unwrap().insert(rule[0]);
    }

    let mut to_explore: VecDeque<usize> = VecDeque::from_iter(no_incoming);
    let mut sorted: Vec<usize> = Vec::new();

    while !to_explore.is_empty() {
        let cur_node = to_explore.pop_front().unwrap();
        sorted.push(cur_node);

        let nodes = outgoing_edges.remove(&cur_node).unwrap_or_default();

        for node in nodes {
            let node_inc = incoming_edges.get_mut(&node).unwrap();
            node_inc.remove(&cur_node);
            if node_inc.is_empty() {
                incoming_edges.remove(&node);
                to_explore.push_back(node);
            }
        }
    }
    return sorted;
}

fn check_all_rules(rules: &Vec<Vec<usize>>, update: &Vec<usize>) -> bool {
    for rule in rules {
        if !check_rule(rule, update) {
            return false;
        }
    }
    return true;
}

fn check_rule(rule: &Vec<usize>, update: &Vec<usize>) -> bool {
    let index1 = update.iter().position(|&n| n == rule[0]);
    let index2 = update.iter().position(|&n| n == rule[1]);
    if index1.is_some() && index2.is_some() {
        return index1.unwrap() < index2.unwrap();
    }
    return true;
}

fn middle_page(update: &Vec<usize>) -> usize {
    if update.len() == 0 {
        return 0;
    }
    return update[update.len() / 2];
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "143");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "123");
    }

    #[test]
    fn test_topo() {
        let rules = parse_row_major::<usize>(split_sections(TEST_INPUT)[0], "|");
        let topo = topo_sort_pages(&rules);
        assert_eq!(check_all_rules(&rules, &topo), true);
    }

    #[test]
    fn test_topo_simple() {
        let input_simple = "1|2\n2|4\n3|4\n\n1,4,2";
        assert_eq!(part2(input_simple), "2");
    }
}
