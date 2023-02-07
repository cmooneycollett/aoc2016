use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Explosives in Cyberspace";
const PROBLEM_INPUT_FILE: &str = "./input/day09.txt";
const PROBLEM_DAY: u64 = 9;

lazy_static! {
    static ref REGEX_MARKER: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
}

/// Processes the AOC 2016 Day 09 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 09 input file in the format required by the solver functions.
/// Returned value is string given in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().to_string()
}

/// Solves AOC 2016 Day 09 Part 1 // Determines the decompressed length of the input string, where
/// nested marker sequences are not decompressed.
fn solve_part1(input: &str) -> usize {
    calculate_decompressed_length(input, false)
}

/// Solves AOC 2016 Day 09 Part 2 // Determines the decompressed length of the input string, where
/// nested marker sequences are decompressed (version two decompression).
fn solve_part2(input: &str) -> usize {
    calculate_decompressed_length(input, true)
}

/// Calculates the decompressed length of the given string, using the length and number of repeats
/// in marker sequences. Nested marker sequences are not decompressed unless the v2_decompression
/// parameter is set to true.
fn calculate_decompressed_length(s: &str, v2_decompression: bool) -> usize {
    let mut decompressed_length = 0;
    let mut index = 0;
    let chars = s.chars().collect::<Vec<char>>();
    while index < chars.len() {
        // Look for index at start of marker sequence
        if chars[index] != '(' {
            index += 1;
            decompressed_length += 1;
            continue;
        }
        // Look for end of marker sequence
        let mut index_la = index + 1;
        while index_la < chars.len() {
            if chars[index_la] == ')' {
                break;
            }
            index_la += 1;
        }
        // Extract sequence length and number of repeats from the marker
        let marker = chars[index..index_la + 1].iter().collect::<String>();
        let (length, repeats) = if let Ok(Some(caps)) = REGEX_MARKER.captures(&marker) {
            let length = caps[1].parse::<usize>().unwrap();
            let repeats = caps[2].parse::<usize>().unwrap();
            (length, repeats)
        } else {
            panic!("Bad marker format!");
        };
        // Calculate the decompressed length of the marker sequence
        if !v2_decompression {
            decompressed_length += length * repeats;
        } else {
            let sub_s = chars[index_la + 1..index_la + 1 + length]
                .iter()
                .collect::<String>();
            let length = calculate_decompressed_length(&sub_s, v2_decompression);
            decompressed_length += length * repeats;
        }
        // Update index position to next character after marker sequence
        index = index_la + 1 + length;
    }
    decompressed_length
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 09 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day09_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(98135, solution);
    }

    /// Tests the Day 09 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day09_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(10964557606, solution);
    }
}
