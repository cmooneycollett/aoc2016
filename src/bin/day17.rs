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

/// Solves AOC 2016 Day 17 Part 2 // ###
fn solve_part2(_input: &str) -> usize {
    unimplemented!();
}

/// Determines the shortest path string needed to go from the start location to the vault location.
fn find_shortest_path_to_vault(
    vault_code: &str,
    loc_start: &Point2D,
    loc_vault: &Point2D,
) -> Option<String> {
    let initial_state = PathState {
        loc: *loc_start,
        path: String::new(),
    };
    let mut state_queue: VecDeque<PathState> = VecDeque::from([initial_state]);
    while !state_queue.is_empty() {
        let state = state_queue.pop_front().unwrap();
        if state.loc == *loc_vault {
            return Some(state.path);
        }
        for next_state in find_next_valid_states(vault_code, &state) {
            state_queue.push_back(next_state);
        }
    }
    None
}

/// Determines the next valid states from the current state.
fn find_next_valid_states(vault_code: &str, state: &PathState) -> Vec<PathState> {
    let mut valid_states: Vec<PathState> = vec![];
    // Generate MD5 hash for current room and take first four characters of the hexdigest
    let digest = md5::compute(format!("{vault_code}{}", state.path).as_bytes());
    let check_chars = format!("{digest:x}").chars().take(4).collect::<Vec<char>>();
    // UP - 'U'
    if OPEN_CHARS.contains(&check_chars[0]) {
        valid_states.push(PathState {
            loc: state.loc.peek_shift(0, -1),
            path: format!("{}U", state.path),
        });
    }
    // DOWN - 'D'
    if OPEN_CHARS.contains(&check_chars[1]) {
        valid_states.push(PathState {
            loc: state.loc.peek_shift(0, 1),
            path: format!("{}D", state.path),
        });
    }
    // LEFT - 'L'
    if OPEN_CHARS.contains(&check_chars[2]) {
        valid_states.push(PathState {
            loc: state.loc.peek_shift(-1, 0),
            path: format!("{}L", state.path),
        });
    }
    // RIGHT - 'R'
    if OPEN_CHARS.contains(&check_chars[3]) {
        valid_states.push(PathState {
            loc: state.loc.peek_shift(1, 0),
            path: format!("{}R", state.path),
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
