use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;

use aoc_utils::cartography::{CardinalDirection, Point2D};

const PROBLEM_NAME: &str = "No Time for a Taxicab";
const PROBLEM_INPUT_FILE: &str = "./input/day01.txt";
const PROBLEM_DAY: u64 = 1;

/// Represents the two different turn directions possible.
enum Turn {
    Left,
    Right,
}

impl Turn {
    /// Gets the turn direction represented by the given character.
    fn from_char(c: char) -> Option<Turn> {
        match c {
            'L' => Some(Turn::Left),
            'R' => Some(Turn::Right),
            _ => None,
        }
    }
}

/// Processes the AOC 2016 Day 1 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 1 input file in the format required by the solver functions.
/// Returned value is vector of instructions containing a turn direction (L or R) and number of
/// steps as a tuple.
fn process_input_file(filename: &str) -> Vec<(Turn, i64)> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut instructions: Vec<(Turn, i64)> = vec![];
    let regex_element = Regex::new(r"([LR])(\d+)").unwrap();
    for element in raw_input.trim().split(", ") {
        if let Ok(Some(caps)) = regex_element.captures(element) {
            let turn = Turn::from_char(caps[1].chars().next().unwrap()).unwrap();
            let steps = caps[2].parse::<i64>().unwrap();
            instructions.push((turn, steps));
        } else {
            panic!("Bad element in input file! // {element}");
        }
    }
    instructions
}

/// Solves AOC 2016 Day 1 Part 1 // Processes each instruction and determines how far the
/// protagonist ends up from the origin.
fn solve_part1(instructions: &[(Turn, i64)]) -> u64 {
    let mut direction = CardinalDirection::North;
    let start_loc = Point2D::new(0, 0);
    let mut loc = start_loc;
    for (turn, steps) in instructions.iter() {
        // Conduct the left or right turn
        direction = match turn {
            Turn::Left => direction.rotate90_counterclockwise(1),
            Turn::Right => direction.rotate90_clockwise(1),
        };
        // Update the location by the number of steps conducted
        match direction {
            CardinalDirection::North => loc.shift(0, -steps),
            CardinalDirection::East => loc.shift(*steps, 0),
            CardinalDirection::South => loc.shift(0, *steps),
            CardinalDirection::West => loc.shift(-steps, 0),
        }
    }
    // Find the Manhattan distance between the end location and the start location
    start_loc.get_manhattan_distance(&loc)
}

/// Solves AOC 2016 Day 1 Part 2 // Determines the distance from the origin of the first location
/// that the protagonist visits twice.
fn solve_part2(instructions: &[(Turn, i64)]) -> u64 {
    let mut direction = CardinalDirection::North;
    let start_loc = Point2D::new(0, 0);
    let mut loc = start_loc;
    let mut visited: HashSet<Point2D> = HashSet::from([loc]);
    'outer: for (turn, steps) in instructions.iter() {
        // Conduct the left or right turn
        direction = match turn {
            Turn::Left => direction.rotate90_counterclockwise(1),
            Turn::Right => direction.rotate90_clockwise(1),
        };
        // Determine how to adjust location on each step
        let (dx, dy) = match direction {
            CardinalDirection::North => (0, -1),
            CardinalDirection::East => (1, 0),
            CardinalDirection::South => (0, 1),
            CardinalDirection::West => (-1, 0),
        };
        // Conduct each step and check if the location has already been visited
        for _ in 0..*steps {
            loc.shift(dx, dy);
            if !visited.insert(loc) {
                break 'outer;
            }
        }
    }
    // Find the Manhattan distance between the end location and the start location
    start_loc.get_manhattan_distance(&loc)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 1 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day01_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(332, solution);
    }

    /// Tests the Day 1 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day01_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(166, solution);
    }
}
