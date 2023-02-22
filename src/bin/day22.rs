use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;

use aoc_utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Grid Computing";
const PROBLEM_INPUT_FILE: &str = "./input/day22.txt";
const PROBLEM_DAY: u64 = 22;

/// Lower bound of used percentage for nodes considered as Wall tiles.
const WALL_NODE_USED_PCT: usize = 90;

/// Represents the details for data held in a single node.
#[derive(Copy, Clone)]
struct NodeData {
    _size: usize,     // Terabytes
    used: usize,      // Terabytes
    available: usize, // Terabytes
    used_pct: usize,
}

/// Used to model the the nodes based on their used percentage.
#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeType {
    Empty,       // Visitable
    PartialUsed, // Visitable
    Wall,        // Not visitable
}

/// Processes the AOC 2016 Day 22 input file and solves both parts of the problem. Solutions are
/// printed to stdout.
pub fn main() {
    let start = Instant::now();
    // Input processing
    let input = process_input_file(PROBLEM_INPUT_FILE);
    let input_parser_timestamp = Instant::now();
    let input_parser_duration = input_parser_timestamp.duration_since(start);
    // Solve part 1
    let p1_solution = solve_part1(&input);
    let p1_timestamp = Instant::now();
    let p1_duration = p1_timestamp.duration_since(input_parser_timestamp);
    // Solve part 2
    let p2_solution = solve_part2(&input);
    let p2_timestamp = Instant::now();
    let p2_duration = p2_timestamp.duration_since(p1_timestamp);
    // Print results
    println!("==================================================");
    println!("AOC 2016 Day {PROBLEM_DAY} - \"{PROBLEM_NAME}\"");
    println!("[+] Part 1: {p1_solution}");
    println!("[+] Part 2: {p2_solution}");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {input_parser_duration:.2?}");
    println!("[+] Part 1: {p1_duration:.2?}");
    println!("[+] Part 2: {p2_duration:.2?}");
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2016 Day 22 input file in the format required by the solver functions.
/// Returned value is hashmap mapping locations to the NodeData details for the data held at the
/// location.
fn process_input_file(filename: &str) -> HashMap<Point2D, NodeData> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_line =
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap();
    let mut output: HashMap<Point2D, NodeData> = HashMap::new();
    for line in raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .skip(2)
    {
        if let Ok(Some(caps)) = regex_line.captures(line) {
            // Extract location and NodeData details from the input line
            let x = caps[1].parse::<i64>().unwrap();
            let y = caps[2].parse::<i64>().unwrap();
            let size = caps[3].parse::<usize>().unwrap();
            let used = caps[4].parse::<usize>().unwrap();
            let available = caps[5].parse::<usize>().unwrap();
            let used_pct = caps[6].parse::<usize>().unwrap();
            // Create key and value
            let loc = Point2D::new(x, y);
            let node_data = NodeData {
                _size: size,
                used,
                available,
                used_pct,
            };
            output.insert(loc, node_data);
        } else {
            panic!("Bad format input line! // {line}");
        }
    }
    output
}

/// Solves AOC 2016 Day 22 Part 1 // Determines the number of viable pairs of nodes.
fn solve_part1(nodes: &HashMap<Point2D, NodeData>) -> usize {
    count_viable_pairs(nodes)
}

/// Solves AOC 2016 Day 22 Part 2 // Determines the minimum number of moves required to move the
/// data at the location with y=0 and the highest x value to the location (0, 0).
fn solve_part2(nodes: &HashMap<Point2D, NodeData>) -> usize {
    find_minimum_steps_from_goal_to_target(nodes)
}

/// Determines the number of viable pairs of nodes.
fn count_viable_pairs(nodes: &HashMap<Point2D, NodeData>) -> usize {
    let mut viable_pairs = 0;
    for (a_loc, a_node_data) in nodes.iter() {
        // Pair is node viable is Node A is empty
        if a_node_data.used == 0 {
            continue;
        }
        // Check if Node B has enough available space to fit the Node A used space
        for (_, b_node_data) in nodes.iter().filter(|(k, _)| *k != a_loc) {
            if b_node_data.available >= a_node_data.used {
                viable_pairs += 1;
            }
        }
    }
    viable_pairs
}

/// Determines the minimum number of moves required to move the data from the goal node (y=0 and
/// highest x value) to the target node (0, 0).
fn find_minimum_steps_from_goal_to_target(nodes: &HashMap<Point2D, NodeData>) -> usize {
    // Convert the node data map into the node tile map
    let node_tiles = convert_nodes_to_tiles(nodes);
    let mut steps: usize = 0;
    // Determine the shortest path between the goal data node and the target node
    let max_x = node_tiles.keys().map(|loc| loc.x()).max().unwrap();
    let mut loc_goal_data = Point2D::new(max_x, 0);
    let loc_target = Point2D::new(0, 0);
    let mut shortest_path =
        find_shortest_path(&node_tiles, &loc_goal_data, &loc_target, None).unwrap();
    shortest_path.pop_front();
    // Find the initial location of the empty node
    let mut loc_empty = *node_tiles
        .iter()
        .filter(|(_loc, tile)| **tile == NodeType::Empty)
        .map(|(loc, _tile)| loc)
        .next()
        .unwrap();
    // Keep moving the empty node to the next location in the goal shortest path to target
    while !shortest_path.is_empty() {
        // Find the shortest path between the empty location and next location on goal shortest path
        let sp_empty_to_goal = find_shortest_path(
            &node_tiles,
            &loc_empty,
            &shortest_path.pop_front().unwrap(),
            Some(&loc_goal_data),
        )
        .unwrap();
        // Move the goal data into the empty location, and update empty location
        loc_empty = loc_goal_data;
        loc_goal_data = *sp_empty_to_goal.back().unwrap();
        // Increase steps for empty node moving in front of goal, and goal moving into empty loc
        steps += sp_empty_to_goal.len();
    }
    // Move goal node to next location on shortest path
    steps
}

/// Converts the node data map into a node tile map.
fn convert_nodes_to_tiles(nodes: &HashMap<Point2D, NodeData>) -> HashMap<Point2D, NodeType> {
    let mut output: HashMap<Point2D, NodeType> = HashMap::new();
    for (&loc, &node_data) in nodes.iter() {
        if node_data.used_pct == 0 {
            output.insert(loc, NodeType::Empty);
        } else if node_data.used_pct < WALL_NODE_USED_PCT {
            output.insert(loc, NodeType::PartialUsed);
        } else {
            output.insert(loc, NodeType::Wall);
        }
    }
    output
}

/// Finds the shorted path between the start and end locations. Any nodes locations that are equal
/// to the exclude node or are wall tiles cannot be visited.
fn find_shortest_path(
    node_tiles: &HashMap<Point2D, NodeType>,
    loc_start: &Point2D,
    loc_end: &Point2D,
    exclude: Option<&Point2D>,
) -> Option<VecDeque<Point2D>> {
    let mut visit_queue: VecDeque<VecDeque<Point2D>> =
        VecDeque::from([VecDeque::from([*loc_start])]);
    let mut visited: HashSet<Point2D> = HashSet::from([*loc_start]);
    while !visit_queue.is_empty() {
        let path = visit_queue.pop_front().unwrap();
        for next_loc in get_next_valid_locations(node_tiles, path.back().unwrap()) {
            // Don't visit node already visited or the excluded node
            if visited.contains(&next_loc) || exclude.is_some() && *exclude.unwrap() == next_loc {
                continue;
            }
            // Create new path and check if the end location has been reached
            let mut new_path = path.clone();
            new_path.push_back(next_loc);
            if next_loc == *loc_end {
                return Some(new_path);
            }
            // Record the next location as visited
            visited.insert(next_loc);
            visit_queue.push_back(new_path);
        }
    }
    None
}

/// Gets the next valid locations when conducting BFS of node tile map.
fn get_next_valid_locations(
    node_tiles: &HashMap<Point2D, NodeType>,
    loc: &Point2D,
) -> Vec<Point2D> {
    let mut output: Vec<Point2D> = vec![];
    // Record Empty and PartialUsed locations as valid for visiting
    for next_loc in loc.get_adjacent_points() {
        if let Some(tile) = node_tiles.get(&next_loc) {
            match tile {
                NodeType::Empty | NodeType::PartialUsed => output.push(next_loc),
                NodeType::Wall => (),
            }
        }
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 22 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day22_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(960, solution);
    }

    /// Tests the Day 22 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day22_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(225, solution);
    }
}
