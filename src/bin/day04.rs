use std::fs;
use std::time::Instant;

use fancy_regex::Regex;

use aoc2016::utils::bespoke::Room;

const PROBLEM_NAME: &str = "Security Through Obscurity";
const PROBLEM_INPUT_FILE: &str = "./input/day04.txt";
const PROBLEM_DAY: u64 = 4;

const TARGET_DECRYPTED_NAME: &str = "northpole object storage";

/// Processes the AOC 2016 Day 04 input file and solves both parts of the problem. Solutions are
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
    println!("AOC 2016 Day {} - \"{}\"", PROBLEM_DAY, PROBLEM_NAME);
    println!("[+] Part 1: {}", p1_solution);
    println!("[+] Part 2: {}", p2_solution);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {:.2?}", input_parser_duration);
    println!("[+] Part 1: {:.2?}", p1_duration);
    println!("[+] Part 2: {:.2?}", p2_duration);
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2016 Day 04 input file in the format required by the solver functions.
/// Returned value is ###.
fn process_input_file(filename: &str) -> Vec<Room> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_line = Regex::new(r"^([a-z\-]+)-(\d+)\[([a-z]{5})\]$").unwrap();
    let mut rooms: Vec<Room> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(Some(caps)) = regex_line.captures(line) {
            let name = &caps[1];
            let sector_id = caps[2].parse::<u32>().unwrap();
            let checksum = &caps[3];
            rooms.push(Room::new(name, sector_id, checksum));
        }
    }
    rooms
}

/// Solves AOC 2016 Day 04 Part 1 // Determines the sum of the sector IDs for the real rooms.
fn solve_part1(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .filter(|room| room.is_real_room())
        .map(|room| room.sector_id())
        .sum()
}

/// Solves AOC 2016 Day 04 Part 2 // ###
fn solve_part2(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .filter(|room| room.decrypted_name() == TARGET_DECRYPTED_NAME)
        .map(|room| room.sector_id())
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 04 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day04_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(173787, solution);
    }

    /// Tests the Day 04 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day04_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(548, solution);
    }
}
