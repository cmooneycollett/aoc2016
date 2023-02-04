use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Internet Protocol Version 7";
const PROBLEM_INPUT_FILE: &str = "./input/day07.txt";
const PROBLEM_DAY: u64 = 7;

lazy_static! {
    static ref REGEX_SUPERNET: Regex = Regex::new(r"([a-z]+\[|\][a-z]+\[|\][a-z]+)").unwrap();
    static ref REGEX_HYPERNET: Regex = Regex::new(r"\[([a-z]+)\]").unwrap();
    static ref REGEX_SQUARE_BRACE: Regex = Regex::new(r"\[|\]").unwrap();
    static ref REGEX_ABBA: Regex = Regex::new(r"([a-z])([a-z])\2\1").unwrap();
}

/// Processes the AOC 2016 Day 07 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 07 input file in the format required by the solver functions.
/// Returned value is vector of strings given as the lines of the input file.
fn process_input_file(filename: &str) -> Vec<String> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>()
}

/// Solves AOC 2016 Day 07 Part 1 // Determines the number of the given "IPv7" addresses that
/// support "TLS" (transport-layer snooping).
fn solve_part1(ipv7_addresses: &[String]) -> usize {
    ipv7_addresses
        .iter()
        .filter(|addr| check_tls_support(addr))
        .count()
}

/// Solves AOC 2016 Day 07 Part 2 // Determines the number of the given "IPv7" addresses that
/// support "SSL" (super-secret listening).
fn solve_part2(ipv7_addresses: &[String]) -> usize {
    ipv7_addresses
        .iter()
        .filter(|addr| check_ssl_support(addr))
        .count()
}

/// Checks if the given "IPv7" address supports "TLS" (transport-layer snooping).
fn check_tls_support(ipv7_address: &str) -> bool {
    let (supernets, hypernets) = extract_supernet_and_hypernet_sequences(ipv7_address);
    // Check that one of the supernet sequences contains an ABBA
    let mut supernet_check = false;
    for supernet in supernets {
        if let Ok(Some(caps)) = REGEX_ABBA.captures(&supernet) {
            // Check that the first two characters of the ABBA are different
            supernet_check = caps[1] != caps[2];
            if supernet_check {
                break;
            }
        }
    }
    if !supernet_check {
        return false;
    }
    // Check that none of the hypernet sequences contains an ABBA
    for hypernet in hypernets {
        if REGEX_ABBA.is_match(&hypernet).unwrap() {
            return false;
        }
    }
    true
}

/// Checks if the given "IPv7" address supports "SSL" (super-secret listening).
fn check_ssl_support(ipv7_address: &str) -> bool {
    let (supernets, hypernets) = extract_supernet_and_hypernet_sequences(ipv7_address);
    // Find the possible BAB candidates
    let mut bab_candidates: HashSet<String> = HashSet::new();
    for supernet in supernets.iter() {
        let supernet = supernet.chars().collect::<Vec<char>>();
        for (i, c) in supernet.iter().enumerate().take(supernet.len() - 2) {
            let c1 = supernet[i + 1];
            let c2 = supernet[i + 2];
            if *c == c2 && *c != c1 {
                bab_candidates.insert(format!("{c1}{c}{c1}"));
            }
        }
    }
    // Check if any of the hypernets contain one of the BAB candidates
    for hypernet in hypernets.iter() {
        for bab in bab_candidates.iter() {
            if hypernet.contains(bab) {
                return true;
            }
        }
    }
    false
}

/// Extracts the supernet and hypernet sequences from the given ipv7 address (assuming that there
/// are no hypernet sequences nested within other hypernet sequences).
fn extract_supernet_and_hypernet_sequences(ipv7_address: &str) -> (Vec<String>, Vec<String>) {
    // Extract supernets
    let supernets = REGEX_SUPERNET
        .find_iter(ipv7_address)
        .map(|cap| {
            REGEX_SQUARE_BRACE
                .replace_all(cap.unwrap().as_str(), "")
                .to_string()
        })
        .collect::<Vec<String>>();
    // Extract hypernets
    let hypernets = REGEX_HYPERNET
        .captures_iter(ipv7_address)
        .map(|cap| cap.unwrap()[1].to_string())
        .collect::<Vec<String>>();
    (supernets, hypernets)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 07 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day07_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(115, solution);
    }

    /// Tests the Day 07 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day07_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(231, solution);
    }
}
