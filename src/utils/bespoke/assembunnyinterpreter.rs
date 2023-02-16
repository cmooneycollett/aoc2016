use std::collections::hash_map::Entry;
use std::collections::HashMap;

use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_CPY: Regex = Regex::new(r"^cpy ([abcd]|-?\d+) ([abcd])$").unwrap();
    static ref REGEX_INC: Regex = Regex::new(r"^inc ([abcd])$").unwrap();
    static ref REGEX_DEC: Regex = Regex::new(r"^dec ([abcd])$").unwrap();
    static ref REGEX_JNZ: Regex = Regex::new(r"^jnz ([abcd]|-?\d+) ([abcd]|-?\d+)$").unwrap();
}

/// Custom error type indicating that a specified register does not exist in the Assembunny
/// interpreter.
#[derive(Debug)]
pub struct RegisterDoesNotExist;

/// Represents an argument for an Assembunny operation that could be either a register-held value or
/// a raw value.
#[derive(Copy, Clone)]
enum OpArgument {
    Register { register: char },
    Value { value: isize },
}

impl OpArgument {
    /// Converts the given string into an OpArgument.
    pub fn from_string(s: &str) -> Option<OpArgument> {
        if let Ok(value) = s.parse::<isize>() {
            return Some(OpArgument::Value { value });
        } else if let Some(register) = s.chars().next() {
            return Some(OpArgument::Register { register });
        }
        None
    }
}

/// Represents a single Assembunny operation with arguments that could be register-held values or
/// raw values.
#[derive(Copy, Clone)]
enum Operation {
    /// Copy
    Cpy { arg: OpArgument, register: char },
    /// Increase
    Inc { register: char },
    /// Decrease
    Dec { register: char },
    /// Jump if not zero
    Jnz {
        check: OpArgument,
        delta: OpArgument,
    },
}

/// Interpreter for the Assembunny code described in AOC 2016 Day 12
/// (https://adventofcode.com/2016/day/12).
#[derive(Clone)]
pub struct AssembunnyInterpreter {
    registers: HashMap<char, isize>,
    pc: usize,
    operations: Vec<Operation>,
}

impl AssembunnyInterpreter {
    pub fn new(raw_input: &str) -> Option<AssembunnyInterpreter> {
        // Parse raw input into Assembunny operations
        let mut operations: Vec<Operation> = vec![];
        for line in raw_input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(Some(caps)) = REGEX_CPY.captures(line) {
                let arg = OpArgument::from_string(&caps[1]).unwrap();
                let register = caps[2].chars().next().unwrap();
                operations.push(Operation::Cpy { arg, register });
            } else if let Ok(Some(caps)) = REGEX_INC.captures(line) {
                let register = caps[1].chars().next().unwrap();
                operations.push(Operation::Inc { register });
            } else if let Ok(Some(caps)) = REGEX_DEC.captures(line) {
                let register = caps[1].chars().next().unwrap();
                operations.push(Operation::Dec { register });
            } else if let Ok(Some(caps)) = REGEX_JNZ.captures(line) {
                let check = OpArgument::from_string(&caps[1]).unwrap();
                let delta = OpArgument::from_string(&caps[2]).unwrap();
                operations.push(Operation::Jnz { check, delta });
            } else {
                return None;
            }
        }
        // Construct the Assembunny interpreter
        Some(AssembunnyInterpreter {
            registers: HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0)]),
            pc: 0,
            operations,
        })
    }

    /// Gets the value held in the specified register.
    pub fn get_register(&self, register: char) -> Result<isize, RegisterDoesNotExist> {
        if let Some(value) = self.registers.get(&register) {
            Ok(*value)
        } else {
            // Given register does not exist in the Assembunny interpreter
            Err(RegisterDoesNotExist)
        }
    }

    /// Sets register to the given value.
    pub fn set_register(
        &mut self,
        register: char,
        value: isize,
    ) -> Result<(), RegisterDoesNotExist> {
        if let Entry::Occupied(mut e) = self.registers.entry(register) {
            e.insert(value);
            Ok(())
        } else {
            // Given register does not exist in the Assembunny interpreter.
            Err(RegisterDoesNotExist)
        }
    }

    /// Executes the program loaded into the Assembunny interpreter. Halts when the program counter
    /// is outside of the program instruction space.
    pub fn execute(&mut self) {
        let mut halt = false;
        loop {
            // Check if the program has halted
            if halt || self.pc >= self.operations.len() {
                return;
            }
            // Process the current operation
            match self.operations[self.pc] {
                Operation::Cpy { arg, register } => {
                    let value = self.get_op_argument_value(&arg);
                    self.registers.insert(register, value);
                }
                Operation::Inc { register } => {
                    *self.registers.get_mut(&register).unwrap() += 1;
                }
                Operation::Dec { register } => {
                    *self.registers.get_mut(&register).unwrap() -= 1;
                }
                Operation::Jnz { check, delta } => {
                    let check = self.get_op_argument_value(&check);
                    let delta = self.get_op_argument_value(&delta);
                    if check != 0 {
                        // Check if jump would move program counter to left of instruction space
                        if delta < 0 && delta.unsigned_abs() > self.pc {
                            self.pc = 0;
                            halt = true;
                            continue;
                        }
                        // Adjust program counter by jump
                        if delta < 0 {
                            self.pc -= delta.unsigned_abs();
                        } else {
                            self.pc += delta.unsigned_abs();
                        }
                        // Check if program counter is to right of instruction space
                        if self.pc >= self.operations.len() {
                            halt = true;
                            continue;
                        }
                        // Compensate for post instruction program counter increment
                        self.pc -= 1;
                    }
                }
            }
            // Go to the next instruction
            self.pc += 1;
        }
    }

    /// Looks up the value of the OpArgument in the Assembunny interpreter registers.
    fn get_op_argument_value(&self, arg: &OpArgument) -> isize {
        match arg {
            OpArgument::Value { value } => *value,
            OpArgument::Register { register } => *self.registers.get(register).unwrap(),
        }
    }
}
