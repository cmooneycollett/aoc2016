use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;

use aoc_utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Two Steps Forward";
const PROBLEM_INPUT_FILE: &str = "./input/day17.txt";
const PROBLEM_DAY: u64 = 17;

lazy_static! {
    static ref OPEN_CHARS: HashSet<char> = HashSet::from(['b', 'c', 'd', 'e', 'f']);
    static ref LOC_START: Point2D = Point2D::new(0, 0);
    static ref LOC_TARGET: Point2D = Point2D::new(3, 3);
}

/// Represents the current state of navigating through the grid leading to the vault.
struct PathState {
    loc: Point2D,
    path: String,
}

/// Processes the AOC 2016 Day 17 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 17 input file in the format required by the solver functions.
/// Returned value is the vault passcode given in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().to_string()
}

/// Solves AOC 2016 Day 17 Part 1 // Determines the shortest path string to reach the vault.
fn solve_part1(vault_code: &str) -> String {
    find_shortest_path_to_vault(vault_code, &LOC_START, &LOC_TARGET).unwrap()
}

/// Solves AOC 2016 Day 17 Part 2 // Determines the length of the longest path that reaches the
/// vault location from the start location.
fn solve_part2(vault_code: &str) -> usize {
    find_longest_path_length_to_vault(vault_code, &LOC_START, &LOC_TARGET).unwrap()
}

/// Determines the shortest path string needed to go from the start location to the vault location.
/// Uses a breadth-first search method.
fn find_shortest_path_to_vault(
    vault_code: &str,
    loc_start: &Point2D,
    loc_vault: &Point2D,
) -> Option<String> {
    // Create the state representing the starting point of the search
    let initial_state = PathState {
        loc: *loc_start,
        path: String::new(),
    };
    // The initial state is the first to be visited
    let mut state_queue: VecDeque<PathState> = VecDeque::from([initial_state]);
    while !state_queue.is_empty() {
        // Get the next state to be visited and check if the vault location has been reached
        let state = state_queue.pop_front().unwrap();
        if state.loc == *loc_vault {
            return Some(state.path);
        }
        // Visit all open rooms from the current room
        for next_state in find_next_valid_states(vault_code, &state) {
            state_queue.push_back(next_state);
        }
    }
    None
}

/// Determines the length of the longest path that reaches the vault location from the start
/// location. Uses depth-first search method.
fn find_longest_path_length_to_vault(
    vault_code: &str,
    loc_start: &Point2D,
    loc_vault: &Point2D,
) -> Option<usize> {
    // Create the state representing the starting point of the search
    let initial_state = PathState {
        loc: *loc_start,
        path: String::new(),
    };
    // The initial state is the first to be visited
    let mut state_stack: VecDeque<PathState> = VecDeque::from([initial_state]);
    let mut longest_path_length: Option<usize> = None;
    while !state_stack.is_empty() {
        // Get the next state to be visited and check if the vault location has been reached
        let state = state_stack.pop_front().unwrap();
        if state.loc == *loc_vault {
            // Check if a new longest path length has been found
            let path_length = state.path.len();
            if longest_path_length.is_none() || longest_path_length.unwrap() < path_length {
                longest_path_length = Some(path_length);
            }
            continue;
        }
        // Go to the first of the next open rooms, if vault location not yet reached
        for next_state in find_next_valid_states(vault_code, &state) {
            state_stack.push_front(next_state);
        }
    }
    longest_path_length
}

/// Determines the next valid states from the current state. Fixed walls are taken into account,
/// which limit the (x,y) values to a minimum of 0 and a maximum of 3 each.
fn find_next_valid_states(vault_code: &str, state: &PathState) -> Vec<PathState> {
    let mut valid_states: Vec<PathState> = vec![];
    // Generate MD5 hash for current room and take first four characters of the hexdigest
    let digest = md5::compute(format!("{vault_code}{}", state.path).as_bytes());
    let check_chars = format!("{digest:x}").chars().take(4).collect::<Vec<char>>();
    // UP - 'U'
    if OPEN_CHARS.contains(&check_chars[0]) && state.loc.y() > 0 {
        valid_states.push(PathState {
            loc: state.loc.peek_shift(0, -1),
            path: state.path.to_string() + "U",
        });
    }
    // DOWN - 'D'
    if OPEN_CHARS.contains(&check_chars[1]) && state.loc.y() < 3 {
        valid_states.push(PathState {
            loc: state.loc.peek_shift(0, 1),
            path: state.path.to_string() + "D",
        });
    }
    // LEFT - 'L'
    if OPEN_CHARS.contains(&check_chars[2]) && state.loc.x() > 0 {
        valid_states.push(PathState {
            loc: state.loc.peek_shift(-1, 0),
            path: state.path.to_string() + "L",
        });
    }
    // RIGHT - 'R'
    if OPEN_CHARS.contains(&check_chars[3]) && state.loc.x() < 3 {
        valid_states.push(PathState {
            loc: state.loc.peek_shift(1, 0),
            path: state.path.to_string() + "R",
        });
    }
    valid_states
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 17 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day17_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("RLDRUDRDDR", solution);
    }

    /// Tests the Day 17 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day17_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(498, solution);
    }
}
