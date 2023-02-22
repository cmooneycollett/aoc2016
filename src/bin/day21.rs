use std::str::FromStr;
use std::time::Instant;
use std::{fs, iter};

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Scrambled Letters and Hash";
const PROBLEM_INPUT_FILE: &str = "./input/day21.txt";
const PROBLEM_DAY: u64 = 21;

const PART1_PASSWORD: &str = "abcdefgh";
const PART2_PASSWORD: &str = "fbgdceah";

lazy_static! {
    static ref REGEX_SWAP_POSITION: Regex =
        Regex::new(r"^swap position (\d+) with position (\d+)$").unwrap();
    static ref REGEX_SWAP_LETTER: Regex =
        Regex::new(r"^swap letter ([a-z]) with letter ([a-z])$").unwrap();
    static ref REGEX_ROTATE_LEFT: Regex = Regex::new(r"^rotate left (\d+) step[s]?$").unwrap();
    static ref REGEX_ROTATE_RIGHT: Regex = Regex::new(r"^rotate right (\d+) step[s]?$").unwrap();
    static ref REGEX_ROTATE_BASED_LETTER: Regex =
        Regex::new(r"^rotate based on position of letter ([a-z])$").unwrap();
    static ref REGEX_REVERSE_POSITIONS: Regex =
        Regex::new(r"^reverse positions (\d+) through (\d+)$").unwrap();
    static ref REGEX_MOVE_POSITIONS: Regex =
        Regex::new(r"^move position (\d+) to position (\d+)$").unwrap();
}

/// Custom error type to indicate that the parsing of an Operation from given string has failed.
#[derive(Debug)]
struct ParseOperationError;

/// Custom error type to indicate that a scramble or unscramble operation has failed.
#[derive(Debug)]
struct ScrambleOperationError;

/// Represents the different operations in the scrambling function.
#[derive(Clone, Copy)]
enum Operation {
    SwapPosition { pos_x: usize, pos_y: usize },
    SwapLetter { letter_x: char, letter_y: char },
    RotateLeft { steps: usize },
    RotateRight { steps: usize },
    RotateBasedLetter { letter: char },
    ReversePositions { start: usize, end: usize },
    MovePosition { pos_x: usize, pos_y: usize },
}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(Some(caps)) = REGEX_SWAP_POSITION.captures(s) {
            let pos_x = caps[1].parse::<usize>().unwrap();
            let pos_y = caps[2].parse::<usize>().unwrap();
            return Ok(Operation::SwapPosition { pos_x, pos_y });
        } else if let Ok(Some(caps)) = REGEX_SWAP_LETTER.captures(s) {
            let letter_x = caps[1].chars().next().unwrap();
            let letter_y = caps[2].chars().next().unwrap();
            return Ok(Operation::SwapLetter { letter_x, letter_y });
        } else if let Ok(Some(caps)) = REGEX_ROTATE_LEFT.captures(s) {
            let steps = caps[1].parse::<usize>().unwrap();
            return Ok(Operation::RotateLeft { steps });
        } else if let Ok(Some(caps)) = REGEX_ROTATE_RIGHT.captures(s) {
            let steps = caps[1].parse::<usize>().unwrap();
            return Ok(Operation::RotateRight { steps });
        } else if let Ok(Some(caps)) = REGEX_ROTATE_BASED_LETTER.captures(s) {
            let letter = caps[1].chars().next().unwrap();
            return Ok(Operation::RotateBasedLetter { letter });
        } else if let Ok(Some(caps)) = REGEX_REVERSE_POSITIONS.captures(s) {
            let start = caps[1].parse::<usize>().unwrap();
            let end = caps[2].parse::<usize>().unwrap();
            return Ok(Operation::ReversePositions { start, end });
        } else if let Ok(Some(caps)) = REGEX_MOVE_POSITIONS.captures(s) {
            let pos_x = caps[1].parse::<usize>().unwrap();
            let pos_y = caps[2].parse::<usize>().unwrap();
            return Ok(Operation::MovePosition { pos_x, pos_y });
        }
        Err(ParseOperationError)
    }
}

/// Processes the AOC 2016 Day 21 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 21 input file in the format required by the solver functions.
/// Returned value is vector of Operation structs given in the lines of the input file.
fn process_input_file(filename: &str) -> Vec<Operation> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Operation::from_str(line).unwrap())
        .collect::<Vec<Operation>>()
}

/// Solves AOC 2016 Day 21 Part 1 // Determines the result of applying the scrambling operations to
/// the string "abcdefgh".
fn solve_part1(operations: &[Operation]) -> String {
    apply_scramble_operations(PART1_PASSWORD, operations).unwrap()
}

/// Solves AOC 2016 Day 21 Part 2 // Determines the result of unscrambling the string "fbgdceah".
fn solve_part2(operations: &[Operation]) -> String {
    apply_unscramble_operations(PART2_PASSWORD, operations).unwrap()
}

/// Applies the scramble operations to the input string and returns the result.
fn apply_scramble_operations(
    s: &str,
    operations: &[Operation],
) -> Result<String, ScrambleOperationError> {
    let mut output = s.chars().collect::<Vec<char>>();
    for &op in operations.iter() {
        match op {
            Operation::SwapPosition { pos_x, pos_y } => {
                swap_positions(&mut output, pos_x, pos_y)?;
            }
            Operation::SwapLetter { letter_x, letter_y } => {
                swap_letters(&mut output, letter_x, letter_y)?;
            }
            Operation::RotateLeft { steps } => {
                rotate_left_by_steps(&mut output, steps);
            }
            Operation::RotateRight { steps } => {
                rotate_right_by_steps(&mut output, steps);
            }
            Operation::RotateBasedLetter { letter } => {
                rotate_based_on_letter_position(&mut output, letter)?;
            }
            Operation::ReversePositions { start, end } => {
                reverse_positions_in_slice(&mut output, start, end)?;
            }
            Operation::MovePosition { pos_x, pos_y } => {
                move_positions(&mut output, pos_x, pos_y)?;
            }
        }
    }
    Ok(output.iter().collect::<String>())
}

/// Applies the inverse of the given operations to unscrable the input string s.
fn apply_unscramble_operations(
    s: &str,
    operations: &[Operation],
) -> Result<String, ScrambleOperationError> {
    let letter_rotation_mapping = determine_letter_rotation_mapping(s.len());
    let mut output = s.chars().collect::<Vec<char>>();
    // Apply the inverse of the scramble operations in reverse order to unscramble input string.
    for &op in operations.iter().rev() {
        match op {
            Operation::SwapPosition { pos_x, pos_y } => {
                swap_positions(&mut output, pos_x, pos_y)?;
            }
            Operation::SwapLetter { letter_x, letter_y } => {
                swap_letters(&mut output, letter_x, letter_y)?;
            }
            Operation::RotateLeft { steps } => {
                rotate_right_by_steps(&mut output, steps);
            }
            Operation::RotateRight { steps } => {
                rotate_left_by_steps(&mut output, steps);
            }
            Operation::RotateBasedLetter { letter } => {
                unscramble_rotate_based_on_letter_position(
                    &mut output,
                    letter,
                    &letter_rotation_mapping,
                )?;
            }
            Operation::ReversePositions { start, end } => {
                reverse_positions_in_slice(&mut output, start, end)?;
            }
            Operation::MovePosition { pos_x, pos_y } => {
                move_positions(&mut output, pos_y, pos_x)?;
            }
        }
    }
    Ok(output.iter().collect::<String>())
}

/// Determines how many right-rotation steps were undertaken for a character to end up at an index
/// within a string of the given length.
fn determine_letter_rotation_mapping(length: usize) -> Vec<usize> {
    let mut output: Vec<usize> = iter::repeat(0).take(length).collect::<Vec<usize>>();
    for pos in 0..length {
        let steps = pos + 1 + (if pos >= 4 { 1 } else { 0 });
        let i = (pos + steps) % length;
        output[i] = steps;
    }
    output
}

/// Swaps the letters at the two positions.
fn swap_positions(
    output: &mut [char],
    pos_x: usize,
    pos_y: usize,
) -> Result<(), ScrambleOperationError> {
    if pos_x >= output.len() || pos_y >= output.len() {
        return Err(ScrambleOperationError);
    }
    let (letter_x, letter_y) = (output[pos_x], output[pos_y]);
    output[pos_y] = letter_x;
    output[pos_x] = letter_y;
    Ok(())
}

/// Swap the two letters, irrespective of their location in the output.
fn swap_letters(
    output: &mut [char],
    letter_x: char,
    letter_y: char,
) -> Result<(), ScrambleOperationError> {
    let pos_x = output.iter().position(|c| *c == letter_x);
    let pos_y = output.iter().position(|c| *c == letter_y);
    if pos_x.is_none() || pos_y.is_none() {
        return Err(ScrambleOperationError);
    }
    let (pos_x, pos_y) = (pos_x.unwrap(), pos_y.unwrap());
    output[pos_y] = letter_x;
    output[pos_x] = letter_y;
    Ok(())
}

/// Rotates the output buffer to the left by the given number of steps.
fn rotate_left_by_steps(output: &mut [char], steps: usize) {
    for _ in 0..steps {
        output.rotate_left(1);
    }
}

/// Rotates the output buffer to the right by the given number of steps.
fn rotate_right_by_steps(output: &mut [char], steps: usize) {
    for _ in 0..steps {
        output.rotate_right(1);
    }
}

/// Reverses the positions of the characters in the slice bounded by the start and end indices
/// (inclusive).
fn reverse_positions_in_slice(
    output: &mut [char],
    start: usize,
    end: usize,
) -> Result<(), ScrambleOperationError> {
    if start > end || start >= output.len() || end >= output.len() {
        return Err(ScrambleOperationError);
    }
    output[start..=end].reverse();
    Ok(())
}

/// Rotates the output buffer to the right based on the index of the given letter prior to rotations
/// being applied.
fn rotate_based_on_letter_position(
    output: &mut [char],
    letter: char,
) -> Result<(), ScrambleOperationError> {
    let pos = output.iter().position(|c| *c == letter);
    if pos.is_none() {
        return Err(ScrambleOperationError);
    }
    let pos = pos.unwrap();
    let steps = pos + 1 + (if pos >= 4 { 1 } else { 0 });
    for _ in 0..steps {
        output.rotate_right(1);
    }
    Ok(())
}

/// Removes the letter at position x and reinserts it at position y.
fn move_positions(
    output: &mut Vec<char>,
    pos_x: usize,
    pos_y: usize,
) -> Result<(), ScrambleOperationError> {
    if pos_x >= output.len() || pos_y >= output.len() {
        return Err(ScrambleOperationError);
    }
    let letter = output.remove(pos_x);
    output.insert(pos_y, letter);
    Ok(())
}

/// Applies the inverse of a ScrambedBasedLetter operation to the output buffer.
fn unscramble_rotate_based_on_letter_position(
    output: &mut [char],
    letter: char,
    letter_rotation_mapping: &[usize],
) -> Result<(), ScrambleOperationError> {
    let pos = output.iter().position(|c| *c == letter);
    if pos.is_none() {
        return Err(ScrambleOperationError);
    }
    let pos = pos.unwrap();
    let steps = letter_rotation_mapping[pos];
    rotate_left_by_steps(output, steps);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 21 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day21_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("gfdhebac", solution);
    }

    /// Tests the Day 21 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day21_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!("dhaegfbc", solution);
    }
}
