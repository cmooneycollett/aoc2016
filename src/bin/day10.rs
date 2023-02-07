use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;

const PROBLEM_NAME: &str = "Balance Bots";
const PROBLEM_INPUT_FILE: &str = "./input/day10.txt";
const PROBLEM_DAY: u64 = 10;

/// Represents a different part of the two-part solution.
#[derive(Copy, Clone, PartialEq, Eq)]
enum SolutionPart {
    PartOne,
    PartTwo,
}

/// Represents a single entity that can receive microchips.
#[derive(Copy, Clone)]
enum Entity {
    Robot,
    Output,
}

impl Entity {
    /// Returns the Entity corresponding to the given string.
    fn from_string(s: &str) -> Option<Entity> {
        match s {
            "bot" => Some(Entity::Robot),
            "output" => Some(Entity::Output),
            _ => None,
        }
    }
}

/// Represents a single instruction for transfer of microchips from a robot.
#[derive(Copy, Clone)]
struct Instruction {
    low_target: Entity,
    low_id: u64,
    high_target: Entity,
    high_id: u64,
}

type ProblemInput = (
    HashMap<u64, Instruction>,
    HashMap<u64, Vec<u64>>,
    HashMap<u64, Vec<u64>>,
);

/// Processes the AOC 2016 Day 10 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 10 input file in the format required by the solver functions.
/// Returned value is tuple containing the: robot IDs mapped to instructions, initial state of
/// robots and initial stat of output bins.
fn process_input_file(filename: &str) -> ProblemInput {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_bot =
        Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$")
            .unwrap();
    let regex_value = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    let mut bot_instructions: HashMap<u64, Instruction> = HashMap::new();
    let mut bot_held: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut output_held: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(Some(caps)) = regex_value.captures(line) {
            let value = caps[1].parse::<u64>().unwrap();
            let bot_id = caps[2].parse::<u64>().unwrap();
            if let Entry::Vacant(e) = bot_held.entry(bot_id) {
                e.insert(vec![value]);
            } else {
                bot_held.get_mut(&bot_id).unwrap().push(value);
            }
        } else if let Ok(Some(caps)) = regex_bot.captures(line) {
            let bot_id = caps[1].parse::<u64>().unwrap();
            let low_target = Entity::from_string(&caps[2]).unwrap();
            let low_id = caps[3].parse::<u64>().unwrap();
            let high_target = Entity::from_string(&caps[4]).unwrap();
            let high_id = caps[5].parse::<u64>().unwrap();
            // Initialise holder for low target
            match low_target {
                Entity::Output => _ = output_held.insert(low_id, vec![]),
                Entity::Robot => {
                    if let Entry::Vacant(e) = bot_held.entry(low_id) {
                        e.insert(vec![]);
                    }
                }
            }
            // Initialise holder for high target
            match high_target {
                Entity::Output => _ = output_held.insert(high_id, vec![]),
                Entity::Robot => {
                    if let Entry::Vacant(e) = bot_held.entry(high_id) {
                        e.insert(vec![]);
                    }
                }
            }
            // Record the instruction against the bot ID
            bot_instructions.insert(
                bot_id,
                Instruction {
                    low_target,
                    low_id,
                    high_target,
                    high_id,
                },
            );
        } else {
            panic!("Bad format line in input file! // {line}");
        }
    }
    (bot_instructions, bot_held, output_held)
}

/// Solves AOC 2016 Day 10 Part 1 // Find the ID of the bot that is responsible for comparing
/// value-17 microchips to value-17 microchips.
fn solve_part1(input: &ProblemInput) -> u64 {
    process_bot_instructions(input, SolutionPart::PartOne)
}

/// Solves AOC 2016 Day 10 Part 2 // Find the product of the values held in outputs 0, 1 and 2 when
/// each contains one microchip.
fn solve_part2(input: &ProblemInput) -> u64 {
    process_bot_instructions(input, SolutionPart::PartTwo)
}

/// Processes the robot instructions based on the start state and returns the result required by the
/// solution part.
fn process_bot_instructions(input: &ProblemInput, solution_part: SolutionPart) -> u64 {
    let bot_instructions = input.0.clone();
    let mut bot_held = input.1.clone();
    let mut output_held = input.2.clone();
    loop {
        let mut check_outputs = false;
        for (id, instr) in bot_instructions.iter() {
            // Check for Part Two solution
            if solution_part == SolutionPart::PartTwo && check_outputs {
                let output0 = output_held.get(&0).unwrap();
                let output1 = output_held.get(&1).unwrap();
                let output2 = output_held.get(&2).unwrap();
                if output0.len() == 1 && output1.len() == 1 && output2.len() == 1 {
                    return output0[0] * output1[0] * output2[0];
                }
                check_outputs = false;
            }
            if bot_held.get(id).unwrap().len() < 2 {
                continue;
            }
            // Get the low and high microchip values held by the bot
            let (bot_low, bot_high) = {
                let bot_values = bot_held.get_mut(id).unwrap();
                (bot_values[0], bot_values[1])
            };
            // Check for Part One solution
            if solution_part == SolutionPart::PartOne && bot_low == 17 && bot_high == 61 {
                return *id;
            }
            // Allocate the bot low-value and high-value microchips to a bot or output
            match instr.low_target {
                Entity::Output => {
                    output_held.get_mut(&instr.low_id).unwrap().push(bot_low);
                    check_outputs = true;
                }
                Entity::Robot => {
                    bot_held.get_mut(&instr.low_id).unwrap().push(bot_low);
                    bot_held.get_mut(&instr.low_id).unwrap().sort();
                }
            }
            match instr.high_target {
                Entity::Output => {
                    output_held.get_mut(&instr.high_id).unwrap().push(bot_high);
                    check_outputs = true;
                }
                Entity::Robot => {
                    bot_held.get_mut(&instr.high_id).unwrap().push(bot_high);
                    bot_held.get_mut(&instr.high_id).unwrap().sort();
                }
            }
            // Remove the microchips from the current bot and record the instruction for removal
            bot_held.insert(*id, vec![]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 10 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day10_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(98, solution);
    }

    /// Tests the Day 10 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day10_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(4042, solution);
    }
}
