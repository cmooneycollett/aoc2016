use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Signals and Noise";
const PROBLEM_INPUT_FILE: &str = "./input/day06.txt";
const PROBLEM_DAY: u64 = 6;

/// Processes the AOC 2016 Day 06 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 06 input file in the format required by the solver functions.
/// Returned value is vector of strings given as the lines of the input file.
fn process_input_file(filename: &str) -> Vec<Vec<char>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .filter(|line| !line.is_empty())
        .collect::<Vec<Vec<char>>>()
}

/// Solves AOC 2016 Day 06 Part 1 // Determines the error-corrected message by taking the
/// most-common character at each index across all of the messages.
fn solve_part1(messages: &[Vec<char>]) -> String {
    let mut message_corrected = String::new();
    let pos_char_counts = get_position_character_counts(messages);
    for pos_count in pos_char_counts {
        // Get the most-common character at the current index
        let c = pos_count
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _v)| k)
            .unwrap();
        message_corrected.push(*c);
    }
    message_corrected
}

/// Solves AOC 2016 Day 06 Part 2 // ###
fn solve_part2(_messages: &[Vec<char>]) -> String {
    String::new()
}

/// Returns a vector of hashmaps containing the total number of times each character is observed at
/// each index across all of the messages.
fn get_position_character_counts(messages: &[Vec<char>]) -> Vec<HashMap<char, u64>> {
    let mut char_pos_counts: Vec<HashMap<char, u64>> = vec![];
    for message in messages {
        for (i, c) in message.iter().enumerate() {
            // Add a new empty hashmap if the current index hasn't been considered yet
            if char_pos_counts.len() <= i {
                char_pos_counts.push(HashMap::new());
            }
            // Initialise the character count or increment the character count
            if let Entry::Vacant(e) = char_pos_counts[i].entry(*c) {
                e.insert(1);
            } else {
                *char_pos_counts[i].get_mut(c).unwrap() += 1;
            }
        }
    }
    char_pos_counts
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 06 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day06_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("dzqckwsd", solution);
    }

    /// Tests the Day 06 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day06_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!("lragovly", solution);
    }
}
