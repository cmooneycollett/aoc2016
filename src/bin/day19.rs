use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "An Elephant Named Joseph";
const PROBLEM_INPUT_FILE: &str = "./input/day19.txt";
const PROBLEM_DAY: u64 = 19;

/// Processes the AOC 2016 Day 19 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 19 input file in the format required by the solver functions.
/// Returned value is number given in the input file.
fn process_input_file(filename: &str) -> usize {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().parse::<usize>().unwrap()
}

/// Solves AOC 2016 Day 19 Part 1 // Determines which elf ends up with all of the presents when the
/// gift exchange game ends (where elves in play steal the presents from the elf on their left).
/// The game has been modelled on the Josephus problem with k=2
/// (https://en.wikipedia.org/wiki/Josephus_problem).
fn solve_part1(num_elves: &usize) -> usize {
    solve_josephus_k2(*num_elves)
}

/// Solves AOC 2016 Day 19 Part 2 // Determines which elf ens up with all of the presents when the
/// gift exchange game ends (where the elves in play steal the presents from the elf directly
/// opposite them in the circle).
fn solve_part2(num_elves: &usize) -> usize {
    solve_elf_steal_opposite(*num_elves)
}

/// Provides the number of the last remaining place when the Josephus problem is solved for n with
/// k=2.
fn solve_josephus_k2(n: usize) -> usize {
    2 * (n - usize::pow(2, usize::ilog2(n))) + 1
}

/// Determines the place number of the last elf remaining at the end of the gift exchange game,
/// where elves steal gifts from the elf opposite them in the circle.
fn solve_elf_steal_opposite(n: usize) -> usize {
    // Create the left and right halves of the circle
    let mut right = VecDeque::from_iter(1..n / 2);
    let mut left = VecDeque::from_iter((n / 2..=n).rev());
    while !right.is_empty() && !left.is_empty() {
        // Steal present from elf
        if right.len() > left.len() {
            right.pop_back();
            if right.is_empty() {
                break;
            }
        } else {
            left.pop_back();
            if left.is_empty() {
                break;
            }
        }
        // Rotate elves counter-clockwise to account of the removed elf
        left.push_front(right.pop_front().unwrap());
        right.push_back(left.pop_back().unwrap());
    }
    // Return the place number of left remaining elf
    if right.is_empty() {
        left.pop_front().unwrap()
    } else {
        right.pop_front().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 19 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day19_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1808357, solution);
    }

    /// Tests the Day 19 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day19_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1407007, solution);
    }
}
