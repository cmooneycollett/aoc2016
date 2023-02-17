use std::fs;
use std::time::Instant;

use fancy_regex::Regex;

const PROBLEM_NAME: &str = "Timing is Everything";
const PROBLEM_INPUT_FILE: &str = "./input/day15.txt";
const PROBLEM_DAY: u64 = 15;

/// Represents a single disc containing multiple positions, one of which has the hole in it.
struct Disc {
    id: u64,
    total_positions: u64,
    offset: u64,
}

impl Disc {
    pub fn new(id: u64, total_positions: u64, start_position: u64) -> Disc {
        let offset = total_positions - start_position;
        Disc {
            id,
            total_positions,
            offset,
        }
    }

    /// Checks if the ball would fall through the hole in the disc if dropped at the specified time.
    pub fn validate_time(&self, time: u64) -> bool {
        if time + self.id < self.offset {
            return false;
        }
        (time + self.id - self.offset) % self.total_positions == 0
    }
}

/// Processes the AOC 2016 Day 15 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 15 input file in the format required by the solver functions.
/// Returned value is vector of Discs specified by the lines of the input file.
fn process_input_file(filename: &str) -> Vec<Disc> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_disc =
        Regex::new(r"^Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).$")
            .unwrap();
    let mut discs: Vec<Disc> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(Some(caps)) = regex_disc.captures(line) {
            let id = caps[1].parse::<u64>().unwrap();
            let total_positions = caps[2].parse::<u64>().unwrap();
            let start_position = caps[3].parse::<u64>().unwrap();
            discs.push(Disc::new(id, total_positions, start_position));
        }
    }
    discs
}

/// Solves AOC 2016 Day 15 Part 1 // Determines the first time at which the ball could be dropped
/// and still pass through the hole in each disc.
fn solve_part1(discs: &[Disc]) -> u64 {
    find_first_valid_drop_time(discs)
}

/// Solves AOC 2016 Day 15 Part 2 // ###
fn solve_part2(_discs: &[Disc]) -> u64 {
    unimplemented!();
}

/// Finds the first time at which the ball could be dropped and still pass through the hole in each
/// disc.
fn find_first_valid_drop_time(discs: &[Disc]) -> u64 {
    let mut time: u64 = 0;
    loop {
        let mut valid_time = true;
        for disc in discs {
            if !disc.validate_time(time) {
                valid_time = false;
                break;
            }
        }
        if !valid_time {
            time += 1;
            continue;
        }
        return time;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 15 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day15_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(203660, solution);
    }

    /// Tests the Day 15 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day15_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(2408135, solution);
    }
}
