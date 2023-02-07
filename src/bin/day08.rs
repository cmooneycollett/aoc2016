use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use itertools::iproduct;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Two-Factor Authentication";
const PROBLEM_INPUT_FILE: &str = "./input/day08.txt";
const PROBLEM_DAY: u64 = 8;

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;
const CHAR_WIDTH: usize = 5;

lazy_static! {
    static ref REGEX_RECT: Regex = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
    static ref REGEX_ROTATE_ROW: Regex = Regex::new(r"^rotate row y=(\d+) by (\d+)$").unwrap();
    static ref REGEX_ROTATE_COL: Regex = Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();

    /// Maps the binary representation of the screen characters (5px wide by 6px tall) to the
    /// corresponding character displayed on the screen.
    static ref SCREEN_CHARS: HashMap<u32, char> = HashMap::from([
        (0x19297A52, 'A'),
        (0x392E4A5C, 'B'),
        (0x1D08420E, 'C'),
        (0x39294A5C, 'D'),
        (0x3D0F421E, 'E'),
        (0x3D0E4210, 'F'),
        (0x3D285A5E, 'G'),
        (0x252F4A52, 'H'),
        (0x3E42109F, 'I'),
        (0x0C210A4C, 'J'),
        (0x254C6292, 'K'),
        (0x2108421E, 'L'),
        (0x23BAC631, 'M'),
        (0x239ACE31, 'N'),
        (0x3D294A5E, 'O'),
        (0x39297210, 'P'),
        (0x192949C1, 'Q'),
        (0x39297292, 'R'),
        (0x1D08305C, 'S'),
        (0x3E421084, 'T'),
        (0x25294A4C, 'U'),
        (0x2318C544, 'V'),
        (0x231AD6BF, 'W'),
        (0x22A21151, 'X'),
        (0x22A21084, 'Y'),
        (0x3C22221E, 'Z'),
    ]);
}

/// Represents a single instruction used to operate on the pixels of the screen.
enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, amount: usize },
    RotateCol { col: usize, amount: usize },
}

impl Instruction {
    /// Converts the given string into an Instruction. Returns None if the given string does not
    /// match an expected format.
    fn from_string(s: &str) -> Option<Instruction> {
        if let Ok(Some(caps)) = REGEX_RECT.captures(s) {
            let width = caps[1].parse::<usize>().unwrap();
            let height = caps[2].parse::<usize>().unwrap();
            return Some(Instruction::Rect { width, height });
        } else if let Ok(Some(caps)) = REGEX_ROTATE_ROW.captures(s) {
            let row = caps[1].parse::<usize>().unwrap();
            let amount = caps[2].parse::<usize>().unwrap();
            return Some(Instruction::RotateRow { row, amount });
        } else if let Ok(Some(caps)) = REGEX_ROTATE_COL.captures(s) {
            let col = caps[1].parse::<usize>().unwrap();
            let amount = caps[2].parse::<usize>().unwrap();
            return Some(Instruction::RotateCol { col, amount });
        }
        None
    }
}

/// Processes the AOC 2016 Day 08 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 08 input file in the format required by the solver functions.
/// Returned value is vector of instructions given in the lines of the input file.
fn process_input_file(filename: &str) -> Vec<Instruction> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .filter_map(|line| Instruction::from_string(line.trim()))
        .collect::<Vec<Instruction>>()
}

/// Solves AOC 2016 Day 08 Part 1 // Returns the number of pixels that are lit after processing the
/// instructions for the 50px-by-6px screen starting with all pixels set to off.
fn solve_part1(instructions: &[Instruction]) -> usize {
    // Generate the initial screen and process the instructions
    let mut screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    process_screen_instructions(instructions, &mut screen);
    // Count the number of pixels that are lit
    screen
        .iter()
        .map(|row| row.iter().filter(|e| **e).count())
        .sum()
}

/// Solves AOC 2016 Day 08 Part 2 // Determines the 10-letter sequence displayed on the 50px-by-6px
/// screen after processing all of the instructions.
fn solve_part2(instructions: &[Instruction]) -> String {
    // Generate the initial screen and process the instructions
    let mut screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    process_screen_instructions(instructions, &mut screen);
    // Decode the letter sequence displayed on the screen
    decode_screen_letters(&screen)
}

/// Processes the instructions for the screen, updating the screen state by processing the
/// instructions.
fn process_screen_instructions(
    instructions: &[Instruction],
    screen: &mut [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
) {
    for instruct in instructions.iter() {
        match instruct {
            Instruction::Rect { width, height } => {
                for (y, x) in iproduct!(0..*height, 0..*width) {
                    screen[y][x] = true;
                }
            }
            Instruction::RotateRow { row, amount } => {
                let mut row_buffer: [bool; SCREEN_WIDTH] = [false; SCREEN_WIDTH];
                for (i, state) in screen[*row].iter().enumerate() {
                    row_buffer[(i + amount) % SCREEN_WIDTH] = *state;
                }
                screen[*row] = row_buffer;
            }
            Instruction::RotateCol { col, amount } => {
                let mut col_buffer: [bool; SCREEN_HEIGHT] = [false; SCREEN_HEIGHT];
                for (i, row) in screen.iter().enumerate() {
                    let state = row[*col];
                    col_buffer[(i + amount) % SCREEN_HEIGHT] = state;
                }
                for (i, row) in screen.iter_mut().enumerate() {
                    row[*col] = col_buffer[i];
                }
            }
        }
    }
}

/// Returns the letter sequence displayed by the screen by decoding the letters displayed by the
/// letter pixel groups (5px wide and 6px tall).
fn decode_screen_letters(screen: &[[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]) -> String {
    let mut decoded = String::new();
    for i in 0..(SCREEN_WIDTH / CHAR_WIDTH) {
        let mut key = 0;
        let mut power = (CHAR_WIDTH * SCREEN_HEIGHT) as u32;
        for (y, x) in iproduct!(0..SCREEN_HEIGHT, (i * CHAR_WIDTH)..((i + 1) * CHAR_WIDTH)) {
            power -= 1;
            if screen[y][x] {
                key += u32::pow(2, power);
            }
        }
        // Get the letter displayed in the current window
        if let Some(c) = SCREEN_CHARS.get(&key) {
            decoded.push(*c);
        } else {
            decoded.push('#');
        }
    }
    decoded
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 08 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day08_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(123, solution);
    }

    /// Tests the Day 08 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day08_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!("AFBUPZBJPS", solution);
    }
}
