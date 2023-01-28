use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Squares With Three Sides";
const PROBLEM_INPUT_FILE: &str = "./input/day03.txt";
const PROBLEM_DAY: u64 = 3;

/// Processes the AOC 2016 Day 03 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 03 input file in the format required by the solver functions.
/// Returned value is vector of three-tuples of values from the input file lines.
fn process_input_file(filename: &str) -> Vec<(u64, u64, u64)> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut triangles: Vec<(u64, u64, u64)> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let tri = line
            .split_ascii_whitespace()
            .map(|elem| elem.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        if tri.len() != 3 {
            panic!("Invalid input file line! // {line}");
        }
        triangles.push((tri[0], tri[1], tri[2]));
    }
    triangles
}

/// Solves AOC 2016 Day 03 Part 1 // Determines how many of the triangles are possible under the
/// problem rules (i.e., the sum of any two sides is greater than the remaining side).
fn solve_part1(triangles: &[(u64, u64, u64)]) -> usize {
    triangles
        .iter()
        .filter(|tri| is_triangle_valid(tri))
        .count()
}

/// Solves AOC 2016 Day 03 Part 2 // ###
fn solve_part2(_triangles: &[(u64, u64, u64)]) -> usize {
    0
}

/// Checks if the sum of any two elements is greater than the remaining element.
fn is_triangle_valid(tri: &(u64, u64, u64)) -> bool {
    tri.0 + tri.1 > tri.2 && tri.0 + tri.2 > tri.1 && tri.1 + tri.2 > tri.0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 03 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day03_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(862, solution);
    }

    /// Tests the Day 03 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day03_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1577, solution);
    }
}
