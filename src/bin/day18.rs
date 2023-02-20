use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Like a Rogue";
const PROBLEM_INPUT_FILE: &str = "./input/day18.txt";
const PROBLEM_DAY: u64 = 18;

const PART1_TOTAL_ROWS: usize = 40;
const _PART2_TOTAL_ROWS: usize = 400000;

lazy_static! {
    static ref REGEX_TRAP: Regex = Regex::new(r"\^\^\.|\.\^\^|\^\.\.|\.\.\^").unwrap();
}

/// Processes the AOC 2016 Day 18 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 18 input file in the format required by the solver functions.
/// Returned value is string given in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().to_string()
}

/// Solves AOC 2016 Day 18 Part 1 // Determines how many safe tiles there are in the first 40 rows.
fn solve_part1(first_row: &str) -> usize {
    calculate_total_safe_tiles(first_row, PART1_TOTAL_ROWS)
}

/// Solves AOC 2016 Day 18 Part 2 // ###
fn solve_part2(_first_row: &str) -> usize {
    0
}

/// Calculates the number of safe tiles there are in the given number of rows, starting from the
/// given first row.
fn calculate_total_safe_tiles(first_row: &str, total_rows: usize) -> usize {
    let mut total_safe_tiles = first_row.chars().filter(|c| *c == '.').count();
    if total_rows == 0 {
        return 0;
    } else if total_rows == 1 {
        return total_safe_tiles;
    }
    let mut prior_row = first_row.chars().collect::<Vec<char>>();
    for _ in 1..total_rows {
        let mut next_row: Vec<char> = vec![];
        for i in 0..prior_row.len() {
            let header = generate_header(&prior_row, i).unwrap();
            match REGEX_TRAP.is_match(&header).unwrap() {
                true => next_row.push('^'),
                false => next_row.push('.'),
            }
        }
        prior_row = next_row;
        total_safe_tiles += prior_row.iter().filter(|c| **c == '.').count();
    }
    total_safe_tiles
}

/// Genenerates the string representing the three characters from the prior row, centred around the
/// given index. Indices outside of the prior row are treated as safe tiles.
fn generate_header(prior_row: &[char], index: usize) -> Option<String> {
    if index >= prior_row.len() {
        return None;
    } else if index == 0 {
        return Some(format!(".{}{}", prior_row[index], prior_row[index + 1]));
    } else if index == prior_row.len() - 1 {
        return Some(format!("{}{}.", prior_row[index - 1], prior_row[index]));
    }
    Some(format!(
        "{}{}{}",
        prior_row[index - 1],
        prior_row[index],
        prior_row[index + 1]
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 18 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day18_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1974, solution);
    }

    /// Tests the Day 18 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day18_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(19991126, solution);
    }
}
