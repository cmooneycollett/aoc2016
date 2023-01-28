use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;

use aoc_utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Bathroom Security";
const PROBLEM_INPUT_FILE: &str = "./input/day02.txt";
const PROBLEM_DAY: u64 = 2;

lazy_static! {
    static ref PART1_KEYPAD: HashMap<Point2D, char> = HashMap::from([
        (Point2D::new(0, 0), '1'),
        (Point2D::new(1, 0), '2'),
        (Point2D::new(2, 0), '3'),
        (Point2D::new(0, 1), '4'),
        (Point2D::new(1, 1), '5'),
        (Point2D::new(2, 1), '6'),
        (Point2D::new(0, 2), '7'),
        (Point2D::new(1, 2), '8'),
        (Point2D::new(2, 2), '9'),
    ]);
    static ref PART2_KEYPAD: HashMap<Point2D, char> = HashMap::from([
        (Point2D::new(2, 0), '1'),
        (Point2D::new(1, 1), '2'),
        (Point2D::new(2, 1), '3'),
        (Point2D::new(3, 1), '4'),
        (Point2D::new(0, 2), '5'),
        (Point2D::new(1, 2), '6'),
        (Point2D::new(2, 2), '7'),
        (Point2D::new(3, 2), '8'),
        (Point2D::new(4, 2), '9'),
        (Point2D::new(1, 3), 'A'),
        (Point2D::new(2, 3), 'B'),
        (Point2D::new(3, 3), 'C'),
        (Point2D::new(2, 4), 'D'),
    ]);
}

/// Represents the four different movement directions used in AOC 2016 Day 02.
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Determines the corresponding Direction from the given character.
    fn from_char(c: char) -> Option<Direction> {
        match c {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

/// Processes the AOC 2016 Day 02 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 02 input file in the format required by the solver functions.
/// Returned value is vector containing sequence of directions for each instruction line.
fn process_input_file(filename: &str) -> Vec<Vec<Direction>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Direction::from_char(c).unwrap())
                .collect::<Vec<Direction>>()
        })
        .collect::<Vec<Vec<Direction>>>()
}

/// Solves AOC 2016 Day 02 Part 1 // Determines the keypad combination for the simple keypad.
fn solve_part1(instructions: &[Vec<Direction>]) -> String {
    process_keypad_instructions(&PART1_KEYPAD, instructions, &Point2D::new(1, 1))
}

/// Solves AOC 2016 Day 02 Part 2 // Determines the keypad combination for the complex keypad.
fn solve_part2(instructions: &[Vec<Direction>]) -> String {
    process_keypad_instructions(&PART2_KEYPAD, instructions, &Point2D::new(0, 2))
}

/// Processes the instructions for the keypad and determines the resulting keypad combination.
fn process_keypad_instructions(
    keypad: &HashMap<Point2D, char>,
    instructions: &[Vec<Direction>],
    start_loc: &Point2D,
) -> String {
    let mut combo = String::new();
    let mut loc = *start_loc;
    for line in instructions {
        // Process each step in the current instruction line
        for dirn in line {
            // Get the new location that would result from following the current step
            let new_loc = match dirn {
                Direction::Up => loc.peek_shift(0, -1),
                Direction::Down => loc.peek_shift(0, 1),
                Direction::Left => loc.peek_shift(-1, 0),
                Direction::Right => loc.peek_shift(1, 0),
            };
            // Only update the current location if the next location is on the keypad
            if keypad.contains_key(&new_loc) {
                loc = new_loc;
            }
        }
        // Add the button to the keypad combination
        combo.push(*keypad.get(&loc).unwrap());
    }
    combo
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 02 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day02_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("78985", solution);
    }

    /// Tests the Day 02 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day02_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!("57DD8", solution);
    }
}
