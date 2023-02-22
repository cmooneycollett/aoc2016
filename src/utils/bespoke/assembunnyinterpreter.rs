use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_CPY: Regex = Regex::new(r"^cpy ([abcd]|-?\d+) ([abcd])$").unwrap();
    static ref REGEX_INC: Regex = Regex::new(r"^inc ([abcd])$").unwrap();
    static ref REGEX_DEC: Regex = Regex::new(r"^dec ([abcd])$").unwrap();
    static ref REGEX_JNZ: Regex = Regex::new(r"^jnz ([abcd]|-?\d+) ([abcd]|-?\d+)$").unwrap();
    static ref REGEX_TGL: Regex = Regex::new(r"^tgl ([abcd]|-?\d+)$").unwrap();
}

/// Custom error type indicating that a specified register does not exist in the Assembunny
/// interpreter.
#[derive(Debug)]
pub struct RegisterDoesNotExist;

/// Custom error type indicating that parsing of Assembunny code has failed.
///
/// Examples of situations where this error could occur:
/// - Converting invalid raw input into an assembunny operation
/// - Trying to decode an OpArgument register that is a Value variant
#[derive(Debug)]
pub struct ParseAssembunnyError;

/// Represents an argument for an Assembunny operation that could be either a register-held value or
/// a raw value.
#[derive(Copy, Clone)]
enum OpArgument {
    Register { register: char },
    Value { value: isize },
}

impl FromStr for OpArgument {
    type Err = ParseAssembunnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<isize>() {
            return Ok(OpArgument::Value { value });
        } else if let Some(register) = s.chars().next() {
            return Ok(OpArgument::Register { register });
        }
        Err(ParseAssembunnyError)
    }
}

/// Represents a single Assembunny operation with arguments that could be register-held values or
/// raw values.
#[derive(Copy, Clone)]
enum Operation {
    /// Copy
    Cpy {
        arg: OpArgument,
        register: OpArgument,
    },
    /// Increase
    Inc { register: OpArgument },
    /// Decrease
    Dec { register: OpArgument },
    /// Jump if not zero
    Jnz {
        check: OpArgument,
        delta: OpArgument,
    },
    /// Toggle
    Tgl { delta: OpArgument },
}

/// Interpreter for the Assembunny code described in AOC 2016 Day 12 and Day 23.
#[derive(Clone)]
pub struct AssembunnyInterpreter {
    registers: HashMap<char, isize>,
    pc: usize,
    operations: Vec<Operation>,
}

impl AssembunnyInterpreter {
    pub fn new(raw_input: &str) -> Result<AssembunnyInterpreter, ParseAssembunnyError> {
        // Parse raw input into Assembunny operations
        let mut operations: Vec<Operation> = vec![];
        for line in raw_input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(Some(caps)) = REGEX_CPY.captures(line) {
                let arg = OpArgument::from_str(&caps[1])?;
                let register = OpArgument::from_str(&caps[2])?;
                operations.push(Operation::Cpy { arg, register });
            } else if let Ok(Some(caps)) = REGEX_INC.captures(line) {
                let register = OpArgument::from_str(&caps[1])?;
                operations.push(Operation::Inc { register });
            } else if let Ok(Some(caps)) = REGEX_DEC.captures(line) {
                let register = OpArgument::from_str(&caps[1])?;
                operations.push(Operation::Dec { register });
            } else if let Ok(Some(caps)) = REGEX_JNZ.captures(line) {
                let check = OpArgument::from_str(&caps[1])?;
                let delta = OpArgument::from_str(&caps[2])?;
                operations.push(Operation::Jnz { check, delta });
            } else if let Ok(Some(caps)) = REGEX_TGL.captures(line) {
                let delta = OpArgument::from_str(&caps[1])?;
                operations.push(Operation::Tgl { delta });
            } else {
                return Err(ParseAssembunnyError);
            }
        }
        // Construct the Assembunny interpreter
        Ok(AssembunnyInterpreter {
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
    pub fn execute(&mut self) -> Result<(), ParseAssembunnyError> {
        let mut halt = false;
        loop {
            // Check if the program has halted
            if halt || self.pc >= self.operations.len() {
                return Ok(());
            }
            // Process the current operation
            match self.operations[self.pc] {
                Operation::Cpy { arg, register } => {
                    let value = self.get_op_argument_value(&arg);
                    // Skip invalid instruction
                    let register = match self.get_op_argument_register(&register) {
                        Ok(register) => register,
                        Err(ParseAssembunnyError) => {
                            self.pc += 1;
                            continue;
                        }
                    };
                    self.registers.insert(register, value);
                }
                Operation::Inc { register } => {
                    // Skip invalid instruction
                    let register = match self.get_op_argument_register(&register) {
                        Ok(register) => register,
                        Err(ParseAssembunnyError) => {
                            self.pc += 1;
                            continue;
                        }
                    };
                    *self.registers.get_mut(&register).unwrap() += 1;
                }
                Operation::Dec { register } => {
                    // Skip invalid instruction
                    let register = match self.get_op_argument_register(&register) {
                        Ok(register) => register,
                        Err(ParseAssembunnyError) => {
                            self.pc += 1;
                            continue;
                        }
                    };
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
                Operation::Tgl { delta } => {
                    let delta = self.get_op_argument_value(&delta);
                    // Check if the toggle delta points outside of the interpreter instruction space
                    if delta.is_negative() && delta.unsigned_abs() > self.pc
                        || delta.is_positive()
                            && (delta.unsigned_abs() + self.pc >= self.operations.len())
                    {
                        self.pc += 1;
                        continue;
                    }
                    let i_toggle = delta.unsigned_abs() + self.pc;
                    self.operations[i_toggle] = match self.operations[i_toggle] {
                        Operation::Cpy { arg, register } => Operation::Jnz {
                            check: arg,
                            delta: register,
                        },
                        Operation::Inc { register } => Operation::Dec { register },
                        Operation::Dec { register } => Operation::Inc { register },
                        Operation::Jnz { check, delta } => Operation::Cpy {
                            arg: check,
                            register: delta,
                        },
                        Operation::Tgl { delta } => Operation::Inc { register: delta },
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

    /// Gets the register held in the OpArgument.
    fn get_op_argument_register(&self, arg: &OpArgument) -> Result<char, ParseAssembunnyError> {
        match arg {
            OpArgument::Register { register } => Ok(*register),
            OpArgument::Value { value: _ } => Err(ParseAssembunnyError),
        }
    }
}
