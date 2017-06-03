use Program;
use std::io::{self, Read};

pub fn run(program: Program) {
    let memory_allocation = 30000;
    let mut pc = 0;
    let mut datapointer = 0;
    let mut memory: Vec<u8> = vec![0; memory_allocation];

    while pc < program.instructions.len() {
        let ref instruction = program.instructions[pc];

        match instruction.as_ref() {
            ">" => datapointer += 1,
            "<" => datapointer -= 1,

            "+" => {
                let current_value = memory[datapointer];

                if current_value == 255 {
                    memory[datapointer] = 0;
                } else {
                    memory[datapointer] = current_value + 1;
                }
            }
            "-" => {
                let current_value = memory[datapointer];

                if current_value == 0 {
                    memory[datapointer] = 255;
                } else {
                    memory[datapointer] = current_value - 1;
                }
            }

            "." => {
                print!("{}", memory[datapointer] as char);
            }

            "," => {
                let input = io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8)
                    .expect("Failed to read from stdin");

                memory[datapointer] = input;
            }

            "[" => {
                if memory[datapointer] == 0 {
                    let mut bracket_nesting = 1;
                    let saved_pc = pc;

                    while bracket_nesting > 0 && pc < program.instructions.len() - 1 {
                        pc += 1;
                        if program.instructions[pc] == "]" {
                            bracket_nesting -= 1;
                        } else if program.instructions[pc] == "[" {
                            bracket_nesting += 1;
                        }
                    }

                    if bracket_nesting != 0 {
                        panic!("unmatched '[' at pc={}", saved_pc);
                    }
                }
            }

            "]" => {
                if memory[datapointer] != 0 {
                    let mut bracket_nesting = 1;
                    let saved_pc = pc;

                    while bracket_nesting > 0 && pc > 0 {
                        pc -= 1;
                        if program.instructions[pc] == "[" {
                            bracket_nesting -= 1;
                        } else if program.instructions[pc] == "]" {
                            bracket_nesting += 1;
                        }
                    }

                    if bracket_nesting != 0 {
                        panic!("unmatched ']' at pc={}", saved_pc);
                    }
                }
            }

            c => panic!("Bad character: {}", c),
        }

        pc += 1;
    }
}
