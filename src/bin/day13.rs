use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use aoc_utils::cartography::Point2D;

const PROBLEM_NAME: &str = "A Maze of Twisty Little Cubicles";
const PROBLEM_INPUT_FILE: &str = "./input/day13.txt";
const PROBLEM_DAY: u64 = 13;

/// Processes the AOC 2016 Day 13 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 13 input file in the format required by the solver functions.
/// Returned value is seed value given in the input file.
fn process_input_file(filename: &str) -> i64 {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().parse::<i64>().unwrap()
}

/// Solves AOC 2016 Day 13 Part 1 // Determines the fewest number of steps required to reach (31,39)
/// when starting at (1,1).
fn solve_part1(seed: &i64) -> usize {
    let loc_start = Point2D::new(1, 1);
    let loc_target = Point2D::new(31, 39);
    find_minimum_steps_to_target_location(*seed, &loc_start, &loc_target).unwrap()
}

/// Solves AOC 2016 Day 13 Part 2 // ###
fn solve_part2(_seed: &i64) -> usize {
    unimplemented!();
}

/// Finds the minimum number of steps to get from the starting location to the target location. The
/// seed value is used to dynamically determine if a particular location in the grid is a wall or
/// open space.
fn find_minimum_steps_to_target_location(
    seed: i64,
    loc_start: &Point2D,
    loc_target: &Point2D,
) -> Option<usize> {
    let mut visit_queue: VecDeque<(Point2D, usize)> = VecDeque::from([(*loc_start, 0)]);
    let mut visited: HashSet<Point2D> = HashSet::from([*loc_start]);
    while !visit_queue.is_empty() {
        // Check if the target location has been reached
        let (loc, steps) = visit_queue.pop_front().unwrap();
        if loc == *loc_target {
            return Some(steps);
        }
        // Get the next locations to visit
        for next_loc in get_next_locations(seed, &loc) {
            if !visited.contains(&next_loc) {
                visit_queue.push_back((next_loc, steps + 1));
                visited.insert(next_loc);
            }
        }
    }
    None
}

/// Gets the next locations that could be visited from the current location. Does not account for
/// any points that have already been visited.
fn get_next_locations(seed: i64, loc: &Point2D) -> Vec<Point2D> {
    let mut next_locations: Vec<Point2D> = vec![];
    for next_loc in loc.get_adjacent_points() {
        if next_loc.x() < 0 || next_loc.y() < 0 {
            continue;
        }
        if is_location_open(seed, &next_loc) {
            next_locations.push(next_loc);
        }
    }
    next_locations
}

/// Checks if the given location is open space. If not, it is a wall and cannot be visited.
fn is_location_open(seed: i64, loc: &Point2D) -> bool {
    let (x, y) = (loc.x(), loc.y());
    let value = x * x + 3 * x + 2 * x * y + y + y * y + seed;
    format!("{value:b}").chars().filter(|c| *c == '1').count() % 2 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 13 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day13_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(90, solution);
    }

    /// Tests the Day 13 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day13_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(90, solution);
    }
}
