use std::fs;
use std::time::Instant;

use aoc2016::utils::bespoke::AssembunnyInterpreter;

const PROBLEM_NAME: &str = "Clock Signal";
const PROBLEM_INPUT_FILE: &str = "./input/day25.txt";
const PROBLEM_DAY: u64 = 25;

const TONE_SEQUENCE_LENGTH_TARGET: usize = 50;

/// Processes the AOC 2016 Day 25 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 25 input file in the format required by the solver functions.
/// Returned value is assembunny interpreter initialised with the operations contained in the input
/// file.
fn process_input_file(filename: &str) -> AssembunnyInterpreter {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    AssembunnyInterpreter::new(raw_input.trim()).unwrap()
}

/// Solves AOC 2016 Day 25 Part 1 // Determines the lowest positive integer value that the 'a'
/// register needs to be initialised to in order for the interpreter to produce the required clock
/// signal (indefinitely alternating sequence of 0 and 1).
fn solve_part1(interpreter: &AssembunnyInterpreter) -> isize {
    let mut seed = 0;
    'outer: loop {
        // Initialise the interpreter with the new seed value
        seed += 1;
        let mut interpreter = interpreter.clone();
        interpreter.set_register('a', seed).unwrap();
        let mut expected_tones = [0isize, 1isize].iter().cycle();
        // Check for sequence of good tones
        'inner: for _ in 0..TONE_SEQUENCE_LENGTH_TARGET {
            // Resume execution of the program and check that interpreter has not halted
            interpreter.execute().unwrap();
            if interpreter.is_halted() {
                continue 'outer;
            }
            // Check if next tone is expected value in 0/1 sequence
            if let Some(tone) = interpreter.get_next_transmit_value() {
                if tone == *expected_tones.next().unwrap() {
                    continue 'inner;
                }
            }
            continue 'outer;
        }
        return seed;
    }
}

/// Solves AOC 2016 Day 25 Part 2 // Christmas has been saved for 2016!
fn solve_part2(_interpreter: &AssembunnyInterpreter) -> bool {
    true
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 25 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day25_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(182, solution);
    }

    /// Tests the Day 25 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day25_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert!(solution);
    }
}
