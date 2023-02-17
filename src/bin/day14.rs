use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "One-Time Pad";
const PROBLEM_INPUT_FILE: &str = "./input/day14.txt";
const PROBLEM_DAY: u64 = 14;

/// We are looking for the 64th valid one-time pad key.
const TARGET_OTP_ORD: usize = 64;
const HASH_BUFFER_LEN: usize = 1000;

lazy_static! {
    static ref REGEX_THREE_GROUP: Regex = Regex::new(r"([0-9a-f])\1\1").unwrap();
    static ref REGEX_FIVE_GROUP: Regex = Regex::new(r"([0-9a-f])\1\1\1\1").unwrap();
}

/// Represents the details extracted from an MD5 hash, being the characters that are involved in any
/// groups of the same character three-in-a-row or five-in-a-row.
struct Md5HashDetails {
    /// Index of the MD5 hash
    index: usize,
    /// First character in a group-of-three that the MD5 hash contains
    three_group: Option<char>,
    /// Any characters contained in group-of-five of same characters
    five_groups: HashSet<char>,
}

/// Processes the AOC 2016 Day 14 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 14 input file in the format required by the solver functions.
/// Returned value is the salt string given in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().to_string()
}

/// Solves AOC 2016 Day 14 Part 1 // Determines the index that produces the 64th one-time pad key.
fn solve_part1(salt: &str) -> usize {
    find_index_of_target_ord_otp_key(salt, TARGET_OTP_ORD)
}

/// Solves AOC 2016 Day 14 Part 2 // ###
fn solve_part2(_salt: &str) -> usize {
    0
}

/// Determins the index of the one-time pad key that is the nth valid key.
fn find_index_of_target_ord_otp_key(salt: &str, nth_key: usize) -> usize {
    let mut details_queue: VecDeque<Md5HashDetails> = VecDeque::new();
    let mut five_groups_enqueued: HashSet<char> = HashSet::new();
    // Initialise the buffer of MD5 hash details
    for index in 0..HASH_BUFFER_LEN {
        let md5_hash_details = calculate_md5_hash_details(salt, index, false);
        five_groups_enqueued.extend(md5_hash_details.five_groups.iter());
        details_queue.push_back(md5_hash_details);
    }
    let mut valid_otp_keys_found = 0;
    loop {
        // Pop key from front and adjust five-groups enqueued
        let key_details = details_queue.pop_front().unwrap();
        for c in key_details.five_groups {
            five_groups_enqueued.remove(&c);
        }
        // Generate next md5 hash details and adjust five-groups enqueue
        let new_md5_hash_details =
            calculate_md5_hash_details(salt, key_details.index + HASH_BUFFER_LEN, false);
        five_groups_enqueued.extend(new_md5_hash_details.five_groups.iter());
        details_queue.push_back(new_md5_hash_details);
        // Check if the current key is a valid key
        if let Some(c) = key_details.three_group {
            if five_groups_enqueued.contains(&c) {
                valid_otp_keys_found += 1;
            }
            if valid_otp_keys_found == nth_key {
                return key_details.index;
            }
        }
    }
}

/// Calculates the MD5 hash details for the given salt and index.
fn calculate_md5_hash_details(
    salt: &str,
    index: usize,
    use_key_stretching: bool,
) -> Md5HashDetails {
    // Calculate MD5 hash
    let digest = calculate_md5_hexadecimal_digest(salt, index, use_key_stretching);
    // Calculate three-groups and five-groups
    let mut three_group: Option<char> = None;
    let mut five_groups: HashSet<char> = HashSet::new();
    if let Ok(Some(caps)) = REGEX_THREE_GROUP.captures(&digest) {
        three_group = Some(caps[1].chars().next().unwrap());
    }
    for caps in REGEX_FIVE_GROUP.captures_iter(&digest) {
        let caps = caps.unwrap();
        five_groups.insert(caps[1].chars().next().unwrap());
    }
    Md5HashDetails {
        index,
        three_group,
        five_groups,
    }
}

/// Caluclates the MD5 hexadecimal digest for the given salt and index. Key stretching is applied
/// if use_key_stretching is set to true.
fn calculate_md5_hexadecimal_digest(salt: &str, index: usize, use_key_stretching: bool) -> String {
    let mut digest = format!("{:x}", md5::compute(format!("{salt}{index}")));
    if use_key_stretching {
        for _ in 0..2016 {
            digest = format!("{:x}", md5::compute(digest.as_bytes()));
        }
    }
    digest
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 14 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day14_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(25427, solution);
    }

    /// Tests the Day 14 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day14_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(22045, solution);
    }
}
