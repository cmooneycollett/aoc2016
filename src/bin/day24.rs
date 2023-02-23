use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use itertools::Itertools;

use aoc_utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Air Duct Spelunking";
const PROBLEM_INPUT_FILE: &str = "./input/day24.txt";
const PROBLEM_DAY: u64 = 24;

/// Represents the different types of tiles that can exist in the grid.
enum TileType {
    Open,
    Wall,
}

type ProblemInput = (HashMap<Point2D, TileType>, HashMap<u64, Point2D>);

/// Processes the AOC 2016 Day 24 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 24 input file in the format required by the solver functions.
/// Returned value is tuple containing: hashmap mapping location to grid tile type, and hashmap
/// mapping number to its location in the grid.
fn process_input_file(filename: &str) -> ProblemInput {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut grid: HashMap<Point2D, TileType> = HashMap::new();
    let mut numbered_locations: HashMap<u64, Point2D> = HashMap::new();
    for (y, line) in raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            let loc = Point2D::new(x as i64, y as i64);
            match c {
                '#' => _ = grid.insert(loc, TileType::Wall),
                '.' => _ = grid.insert(loc, TileType::Open),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    grid.insert(loc, TileType::Open);
                    _ = numbered_locations.insert(c.to_digit(10).unwrap() as u64, loc);
                }
                _ => panic!("Bad character in input file! // x:{x}, y:{y} // char: {c}"),
            }
        }
    }
    (grid, numbered_locations)
}

/// Solves AOC 2016 Day 24 Part 1 // Determines the minimum number of steps required to visit every
/// non-0 number marked on the map at least once.
fn solve_part1(input: &ProblemInput) -> u64 {
    let (grid, numbered_locations) = input;
    determine_min_steps_to_visit_all_numbers(grid, numbered_locations, false).unwrap()
}

/// Solves AOC 2016 Day 24 Part 2 // ###
fn solve_part2(_input: &ProblemInput) -> u64 {
    0
}

/// Determines the minimum number of steps required to visit all of the numbered locations. Includes
/// the distance required to travel from the last location back to the '0' location if option is
/// given as true.
fn determine_min_steps_to_visit_all_numbers(
    grid: &HashMap<Point2D, TileType>,
    numbered_locations: &HashMap<u64, Point2D>,
    return_to_zero: bool,
) -> Option<u64> {
    // Determine the minimum distance between each pair of numbered locations
    let minimum_distances =
        determine_min_distances_between_numbered_locations(numbered_locations, grid);
    // Determine the possible orders in which the non-0 numbered locations can be visited in
    let orders = minimum_distances
        .keys()
        .filter(|k| **k != 0)
        .permutations(minimum_distances.len() - 1);
    // Calculate distance for each location order and check if distance is new overall minimum
    let mut min_steps: Option<u64> = None;
    for ord in orders {
        let ord = ord.into_iter().copied().collect::<Vec<u64>>();
        // Calculate distance required to visited all numbered locations in order, starting with '0'
        let mut current_steps = *minimum_distances.get(&0).unwrap().get(&ord[0]).unwrap();
        for i in 1..ord.len() {
            current_steps += minimum_distances
                .get(&ord[i - 1])
                .unwrap()
                .get(&ord[i])
                .unwrap();
        }
        // Include the distance for returning to '0' location if required
        if return_to_zero {
            current_steps += minimum_distances
                .get(ord.last().unwrap())
                .unwrap()
                .get(&0)
                .unwrap();
        }
        // Check if a new minimum distance has been found
        if min_steps.is_none() || min_steps.unwrap() > current_steps {
            min_steps = Some(current_steps);
        }
    }
    min_steps
}

/// For each numbered location, determines the minimum distance to each other numbered location.
/// Returns hashmap mapping the numbered location to hashmap containing destination location mapped
/// to distance in steps.
fn determine_min_distances_between_numbered_locations(
    numbered_locations: &HashMap<u64, Point2D>,
    grid: &HashMap<Point2D, TileType>,
) -> HashMap<u64, HashMap<u64, u64>> {
    let mut minimum_distances: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    // Find min distance between each different pair of numbered locations
    for (num_from, loc_start) in numbered_locations {
        let mut minimum_distances_from_num: HashMap<u64, u64> = HashMap::new();
        for (num_to, loc_end) in numbered_locations.iter().filter(|(k, _)| *k != num_from) {
            let min_dist = find_min_distance_between_locations(grid, loc_start, loc_end).unwrap();
            minimum_distances_from_num.insert(*num_to, min_dist);
        }
        minimum_distances.insert(*num_from, minimum_distances_from_num);
    }
    minimum_distances
}

/// Determines the minimum distance between the start location and the end location in the grid.
/// Returns none if start or end locations are not in the grid, or if the end location is not
/// reachable from the start location.
fn find_min_distance_between_locations(
    grid: &HashMap<Point2D, TileType>,
    loc_start: &Point2D,
    loc_end: &Point2D,
) -> Option<u64> {
    // Check if the start or end location is not contained in the grid
    if !grid.contains_key(loc_start) || !grid.contains_key(loc_end) {
        return None;
    }
    let mut visit_queue: VecDeque<(Point2D, u64)> = VecDeque::from([(*loc_start, 0)]);
    let mut visited: HashSet<Point2D> = HashSet::from([*loc_start]);
    while !visit_queue.is_empty() {
        let (loc, steps) = visit_queue.pop_front().unwrap();
        if loc == *loc_end {
            return Some(steps);
        }
        for next_loc in determine_next_reachable_locations(grid, &loc) {
            if visited.contains(&next_loc) {
                continue;
            }
            visited.insert(next_loc);
            visit_queue.push_back((next_loc, steps + 1));
        }
    }
    None
}

/// Determines the locations that can be reached in the grid from the current location.
fn determine_next_reachable_locations(
    grid: &HashMap<Point2D, TileType>,
    loc: &Point2D,
) -> Vec<Point2D> {
    let mut output: Vec<Point2D> = vec![];
    for next_loc in loc.get_adjacent_points() {
        match grid.get(&next_loc) {
            Some(TileType::Open) => output.push(next_loc),
            _ => continue,
        }
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 24 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day24_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(442, solution);
    }

    /// Tests the Day 24 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day24_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(660, solution);
    }
}
