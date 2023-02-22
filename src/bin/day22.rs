use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;

use aoc_utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Grid Computing";
const PROBLEM_INPUT_FILE: &str = "./input/day22.txt";
const PROBLEM_DAY: u64 = 22;

/// Represents the details for data held in a single node.
struct NodeData {
    _size: usize,     // Terabytes
    used: usize,      // Terabytes
    available: usize, // Terabytes
    _used_pct: usize,
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
                _used_pct: used_pct,
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

/// Solves AOC 2016 Day 22 Part 2 // ###
fn solve_part2(_input: &HashMap<Point2D, NodeData>) -> usize {
    0
}

/// Determines the number of viable pairs of nodes.
fn count_viable_pairs(nodes: &HashMap<Point2D, NodeData>) -> usize {
    let mut viable_pairs: usize = 0;
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
