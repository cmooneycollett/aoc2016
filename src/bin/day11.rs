use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::Instant;

use fancy_regex::Regex;
use itertools::Itertools;

const PROBLEM_NAME: &str = "Radioisotope Thermoelectric Generators";
const PROBLEM_INPUT_FILE: &str = "./input/day11.txt";
const PROBLEM_DAY: u64 = 11;

/// Represents the two different types of Components found within the "Radioisotope Testing
/// Facility".
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ComponentType {
    Generator,
    Microchip,
}

/// Represents an individual Component found within the "Radioisotope Testing Facility".
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Component {
    comp_type: ComponentType,
    name: String,
}

impl Component {
    pub fn new(comp_type: ComponentType, name: &str) -> Component {
        Component {
            comp_type,
            name: name.to_string(),
        }
    }
}

/// Represents the current state of the "Radioisotope Testing Facility".
#[derive(Clone, Hash)]
struct FacilityState {
    /// Number of moves taken so far
    moves: usize,
    /// Current floor of the elevator
    elev_floor: usize,
    /// State of the floor comps
    floor_comps: Vec<BTreeSet<Component>>,
}

/// Processes the AOC 2016 Day 11 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2016 Day 11 input file in the format required by the solver functions.
/// Returned value is vector of Component collections representing the Components on each floor at
/// the start of the problem.
fn process_input_file(filename: &str) -> Vec<BTreeSet<Component>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut floor_comps: Vec<BTreeSet<Component>> = vec![];
    let regex_generator = Regex::new(r"([a-z]+) generator").unwrap();
    let regex_microchip = Regex::new(r"([a-z]+)-compatible microchip").unwrap();
    for line in raw_input.lines() {
        // Ignore empty lines from input
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut floor: BTreeSet<Component> = BTreeSet::new();
        // Find generators
        for caps in regex_generator.captures_iter(line) {
            let generator = &caps.unwrap()[1];
            floor.insert(Component::new(ComponentType::Generator, generator));
        }
        // Find microchips
        for caps in regex_microchip.captures_iter(line) {
            let microchip = &caps.unwrap()[1];
            floor.insert(Component::new(ComponentType::Microchip, microchip));
        }
        // Add floor to output
        floor_comps.push(floor);
    }
    floor_comps
}

/// Solves AOC 2016 Day 11 Part 1 // Calculates the minimum number of moves required to move all
/// the given Components to the top floor.
fn solve_part1(floor_comps: &[BTreeSet<Component>]) -> usize {
    calculate_minimum_moves_to_top_floor(floor_comps).unwrap()
}

/// Solves AOC 2016 Day 11 Part 2 // ###
fn solve_part2(_floor_comps: &[BTreeSet<Component>]) -> usize {
    0
}

/// Determines the minimum number of moves required to move all Components to the top floor.
fn calculate_minimum_moves_to_top_floor(floor_comps: &[BTreeSet<Component>]) -> Option<usize> {
    // Create the initial state of the facility
    let initial_state = FacilityState {
        moves: 0,
        elev_floor: 0,
        floor_comps: floor_comps.to_owned(),
    };
    // Enqueue the initial state and record the initial state as observed
    let mut state_queue: VecDeque<FacilityState> = VecDeque::from([initial_state.clone()]);
    let mut observed_states: HashSet<u64> =
        HashSet::from([calculate_facility_state_hash(&initial_state)]);
    while !state_queue.is_empty() {
        let state = state_queue.pop_front().unwrap();
        // Check if all components have been moved to the top floor
        if state.elev_floor == floor_comps.len() - 1
            && check_if_all_components_at_top_floor(&state.floor_comps)
        {
            return Some(state.moves);
        }
        // Find the possible next states and enqueue any states not already seen
        for next_state in get_next_states(&state) {
            let next_state_hash = calculate_facility_state_hash(&next_state);
            if !observed_states.contains(&next_state_hash) {
                observed_states.insert(next_state_hash);
                state_queue.push_back(next_state);
            }
        }
    }
    None
}

/// Determines the next possible states from the given facility state.
fn get_next_states(state: &FacilityState) -> Vec<FacilityState> {
    let mut next_states: Vec<FacilityState> = vec![];
    let move_options = itertools::chain(
        state.floor_comps[state.elev_floor].iter().combinations(2),
        state.floor_comps[state.elev_floor].iter().combinations(1),
    );
    let mut two_moved_up = false;
    let mut one_moved_down = false;
    for comps in move_options {
        for floor_delta in [1, -1] {
            // Skip move if at top or bottom floor and no floor to move to
            if state.elev_floor == 0 && floor_delta == -1
                || state.elev_floor == state.floor_comps.len() - 1 && floor_delta == 1
            {
                continue;
            }
            // Don't move one component up if two components can be moved up
            if floor_delta == 1 && two_moved_up && comps.len() == 1 {
                continue;
            }
            // Don't move two components down if one component can be moved down
            if floor_delta == -1 && one_moved_down && comps.len() == 1 {
                continue;
            }
            // Don't move down if all floors below are empty
            if floor_delta == -1 {
                let mut skip = true;
                for floor in state.floor_comps.iter().take(state.elev_floor) {
                    if !floor.is_empty() {
                        skip = false;
                        break;
                    }
                }
                if skip {
                    continue;
                }
            }
            // Modify next floor
            let mut next_state = state.clone();
            next_state.elev_floor = (state.elev_floor as i64 + floor_delta) as usize;
            next_state.moves += 1;
            for comp in comps.iter() {
                next_state.floor_comps[state.elev_floor].remove(comp);
                next_state.floor_comps[next_state.elev_floor].insert((*comp).clone());
            }
            // Validate affected floors
            if !validate_floor(&next_state.floor_comps[state.elev_floor])
                || !validate_floor(&next_state.floor_comps[next_state.elev_floor])
            {
                continue;
            }
            // We have now found a valid next state
            if floor_delta == 1 && comps.len() == 2 {
                two_moved_up = true;
            } else if floor_delta == -1 && comps.len() == 1 {
                one_moved_down = true;
            }
            next_states.push(next_state);
        }
    }
    next_states
}

/// Checks if the given floor represents a valid state. A floor is invalid if it contains a
/// microchip without its matching generator in the presence of a mismatched generator
fn validate_floor(floor: &BTreeSet<Component>) -> bool {
    // Valid if no Components on the floor
    if floor.is_empty() {
        return true;
    }
    // Extract the names of the generators and microchips
    let generators = floor
        .iter()
        .filter(|comp| comp.comp_type == ComponentType::Generator)
        .map(|comp| &comp.name)
        .collect::<HashSet<&String>>();
    let microchips = floor
        .iter()
        .filter(|comp| comp.comp_type == ComponentType::Microchip)
        .map(|comp| &comp.name)
        .collect::<HashSet<&String>>();
    // Valid if there is only one type of Component
    if generators.is_empty() || microchips.is_empty() {
        return true;
    }
    // Invalid if microchip is in the presence of a mismatched generator
    for chip in microchips {
        if !generators.contains(&chip) {
            return false;
        }
    }
    // Valid if all microchips have a matching generator
    true
}

/// Calculates the hash of the given state of the floor components.
fn calculate_facility_state_hash(state: &FacilityState) -> u64 {
    let mut hasher = DefaultHasher::new();
    state.hash(&mut hasher);
    hasher.finish()
}

/// Checks if all components are at the top floor.
fn check_if_all_components_at_top_floor(floor_items: &[BTreeSet<Component>]) -> bool {
    for floor in floor_items.iter().take(floor_items.len() - 1) {
        if !floor.is_empty() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 11 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day11_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(47, solution);
    }

    /// Tests the Day 11 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day11_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(71, solution);
    }
}
