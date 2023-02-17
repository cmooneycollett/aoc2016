use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Dragon Checksum";
const PROBLEM_INPUT_FILE: &str = "./input/day16.txt";
const PROBLEM_DAY: u64 = 16;

/// Processes the AOC 2016 Day 16 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 16 input file in the format required by the solver functions.
/// Returned value is seed sequence given in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().to_string()
}

/// Solves AOC 2016 Day 16 Part 1 // Determines the checksum of the modified dragon curve data
/// needed to fill a disk with size 272 units.
fn solve_part1(seed: &str) -> String {
    let blob = generate_dragon_curve_data(seed, 272);
    generate_dragon_curve_checksum(&blob)
}

/// Solves AOC 2016 Day 16 Part 2 // ###
fn solve_part2(_seed: &str) -> String {
    String::new()
}

/// Processes the dragon curve data blob using the checksum calculation until the checksum has an
/// off number of characters.
fn generate_dragon_curve_checksum(blob: &str) -> String {
    let mut checksum = blob.to_string();
    while checksum.len() % 2 == 0 {
        checksum = apply_checksum_iteration(&checksum);
    }
    checksum
}

/// Applies a single iteration of the dragon curve checksum calculation to the dragon curve data
/// blob.
fn apply_checksum_iteration(blob: &str) -> String {
    if blob.len() % 2 == 1 {
        return blob.to_string();
    }
    let blob_chars = blob.chars().collect::<Vec<char>>();
    let mut checksum = String::new();
    for (i, c) in blob_chars.iter().enumerate().step_by(2) {
        let c1 = blob_chars[i + 1];
        match c.eq(&c1) {
            true => checksum.push('1'),
            false => checksum.push('0'),
        }
    }
    checksum
}

/// Generates a blob of dragon curve data from the given seed that is the same length as the given
/// disk length.
fn generate_dragon_curve_data(seed: &str, disk_length: usize) -> String {
    let mut blob = seed.to_string();
    while blob.len() < disk_length {
        blob = apply_dragon_curve_iteration(&blob);
    }
    blob.chars().take(disk_length).collect::<String>()
}

/// Generates a new dragon curve blob using the given blob as input to the iteration.
fn apply_dragon_curve_iteration(blob: &str) -> String {
    let b_half = blob
        .chars()
        .rev()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect::<String>();
    format!("{blob}0{b_half}")
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 16 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day16_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("00000100100001100", solution);
    }

    /// Tests the Day 16 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day16_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!("00011010100010010", solution);
    }
}
