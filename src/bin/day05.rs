use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "How About a Nice Game of Chess?";
const PROBLEM_INPUT_FILE: &str = "./input/day05.txt";
const PROBLEM_DAY: u64 = 5;

/// Processes the AOC 2016 Day 05 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 05 input file in the format required by the solver functions.
/// Returned value is the string given in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().to_string()
}

/// Solves AOC 2016 Day 05 Part 1 // Determines the eight-character door passcode by finding eight
/// md5 hex digests starting with five zeroes and taking the sixth character.
fn solve_part1(seed: &str) -> String {
    let mut passcode = String::new();
    let mut i: u64 = 0;
    for _ in 0..8 {
        // Find next character for the passcode
        loop {
            // Calculate md5 hex digest and increment index
            let digest = md5::compute(format!("{seed}{i}").as_bytes());
            let hex_digest = format!("{digest:x}");
            i += 1;
            // Check if md5 hex digest starting with five zeroes has been found
            if hex_digest.starts_with("00000") {
                passcode.push(hex_digest.chars().nth(5).unwrap());
                break;
            }
        }
    }
    passcode
}

/// Solves AOC 2016 Day 05 Part 2 // ###
fn solve_part2(_seed: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 05 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day05_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("f77a0e6e", solution);
    }

    /// Tests the Day 05 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day05_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!("999828ec", solution);
    }
}
