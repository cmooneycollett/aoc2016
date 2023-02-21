use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

const PROBLEM_NAME: &str = "Firewall Rules";
const PROBLEM_INPUT_FILE: &str = "./input/day20.txt";
const PROBLEM_DAY: u64 = 20;

/// Processes the AOC 2016 Day 20 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 20 input file in the format required by the solver functions.
/// Returned value is sorted vector of inclusive ranges given in the lines of the input file.
fn process_input_file(filename: &str) -> Vec<RangeInclusive<u32>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut ranges = raw_input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split('-')
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
        })
        .map(|split| split[0].parse::<u32>().unwrap()..=split[1].parse::<u32>().unwrap())
        .collect::<Vec<RangeInclusive<u32>>>();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    ranges
}

/// Solves AOC 2016 Day 20 Part 1 // Determines the lowest value that is not included in the given
/// ranges.
fn solve_part1(ranges: &[RangeInclusive<u32>]) -> u32 {
    find_lowest_value_not_included(ranges).unwrap()
}

/// Solves AOC 2016 Day 20 Part 2 // ###
fn solve_part2(_ranges: &[RangeInclusive<u32>]) -> usize {
    0
}

/// Finds the lowest value (u32) that is not included in the given ranges (sorted by start value).
/// Returns None if the entire range of u32 values is covered.
fn find_lowest_value_not_included(ranges: &[RangeInclusive<u32>]) -> Option<u32> {
    let mut highest_end: Option<u32> = None;
    for r in ranges {
        if highest_end.is_none() {
            // Check if there is a gap at the start of the u32 value range
            if *r.start() > 0 {
                return Some(0);
            }
            highest_end = Some(*r.end());
        } else {
            // Check if there is a gap between the highest end value and the range start
            if *r.start() > highest_end.unwrap()
                && u32::abs_diff(*r.start(), highest_end.unwrap()) >= 2
            {
                break;
            }
            // New highest end value has been observed
            if *r.end() > highest_end.unwrap() {
                highest_end = Some(*r.end());
            }
        }
    }
    // Check if there is a gap in u32 value range at the end of the ranges
    if let Some(value) = highest_end {
        if value < u32::MAX {
            return Some(highest_end.unwrap() + 1);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 20 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day20_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(22887907, solution);
    }

    /// Tests the Day 20 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day20_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(109, solution);
    }
}
