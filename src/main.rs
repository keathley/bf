extern crate rand;

use std::io::{self, Read};
use std::fmt;
use std::env;
use std::fs::File;

struct Program {
    instructions: Vec<String>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program: {}", self.instructions.join(""))
    }
}

fn is_valid_symbol(s: &String) -> bool {
    match s.as_ref() {
        "<" => true,
        ">" => true,
        "+" => true,
        "-" => true,
        "." => true,
        "," => true,
        "[" => true,
        "]" => true,
        _   => false
    }
}

impl Program {
    fn parse(text: String) -> Program {
        let chars = text.split("")
            .map(|s| s.to_string())
            .filter(|s| is_valid_symbol(&s))
            .collect();

        Program { instructions: chars }
    }
}

fn read_input_stream() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut input)
        .expect("Failed to read file");

    input
}

fn read_file(file: &String) -> String {
    let mut input = String::new();

    File::open(file)
        .expect("Unable to open file")
        .read_to_string(&mut input)
        .expect("Unable to read file");

    input
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { read_file(&args[1]) } else { read_input_stream() };
    let program = Program::parse(input);
    let memory_allocation = 30000;
    let mut pc = 0;
    let mut datapointer = 0;
    let mut memory: Vec<u8> = vec![0; memory_allocation];

    while pc < program.instructions.len() {
        let ref instruction = program.instructions[pc];

        // println!("Running instruction: {}, memory: {}, datapointer: {}", instruction, memory[datapointer], datapointer);

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
            },
            "-" => {
                let current_value = memory[datapointer];

                if current_value == 0 {
                    memory[datapointer] = 255;
                } else {
                    memory[datapointer] = current_value - 1;
                }
            },

            "." => {
                print!("{}", memory[datapointer] as char);
            },

            "," => {
                let mut human_input = String::new();
                io::stdin().read_line(&mut human_input)
                    .expect("Failed to read from stdin");
                let bytes = human_input
                    .bytes()
                    .next()
                    .map(|byte| byte as u8)
                    .expect("Failed to read bytes");

                // println!("Got some bytes: {}", bytes);

                memory[datapointer] = bytes;
                //     .expect("Error reading from stdin");
                // memory[datapointer] = human_input
                //     .bytes()
                //     .nth(0)
                //     .expect("Error getting bytes");
            },

            "[" => {
                // println!("Found a [, {}", memory[datapointer]);
                if memory[datapointer] == 0 {
                    // println!("Searching");
                    let mut bracket_nesting = 1;
                    let saved_pc = pc;

                    while bracket_nesting > 0 && pc < program.instructions.len()-1 {
                        pc+=1;
                        if program.instructions[pc] == "]" {
                            bracket_nesting -= 1;
                        } else if program.instructions[pc] == "[" {
                            bracket_nesting += 1;
                        }
                    }

                    if bracket_nesting != 0 {
                        panic!("unmatched '[' at pc={}", saved_pc);
                    }

                    // println!("Found a match");
                }
            },

            "]" => {
                // println!("Found a ], {}", memory[datapointer]);
                if memory[datapointer] != 0 {
                    // println!("Searching...");
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

                    // println!("Found a match")
                }
            },

            c   => panic!("Bad character: {}", c)
        }

        pc+=1;
    }
}
