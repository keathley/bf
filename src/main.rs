extern crate rand;

use std::io::{self, Read};
use std::fmt;
use std::env;
use std::fs::File;

// type Op = String;

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

    // println!("Read in: {}", input);

    input
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input = String::new();

    if args.len() > 1 {
        File::open(&args[1])
            .expect("Unable to open file")
            .read_to_string(&mut input)
            .expect("Unable to read file");
    } else {
        input = read_input_stream();
    }
    println!("Parsing");
    let program = Program::parse(input);
    let mut pc = 0;
    let memory_allocation = 30000;
    let mut datapointer = 0;
    let mut memory: Vec<u8> = vec![0; memory_allocation];


    println!("Running");

    while pc < program.instructions.len() {
        let ref instruction = program.instructions[pc];

        match instruction.as_ref() {
            ">" => datapointer += 1,
            "<" => datapointer -= 1,

            "+" => {
                let old_value = memory[datapointer];

                if old_value == 255 {
                    memory[datapointer] = 0;
                } else {
                    memory[datapointer] = old_value + 1;
                }
            },
            "-" => {
                let old_value = memory[datapointer];

                if old_value == 0 {
                    memory[datapointer] = 255;
                } else {
                    memory[datapointer] = old_value - 1;
                }
            },

            "." => print!("{}", memory[datapointer] as char),

            "," => {
                // println!("Enter some input: ");

                let mut human_input = String::new();
                io::stdin().read_line(&mut human_input)
                    .expect("Failed to read from stdin");
                let bytes = human_input
                    .bytes()
                    .next()
                    .map(|byte| byte as u8)
                    .expect("Failed to read bytes");

                println!("Got some bytes: {}", bytes);

                memory[datapointer] = bytes;
                //     .expect("Error reading from stdin");
                // memory[datapointer] = human_input
                //     .bytes()
                //     .nth(0)
                //     .expect("Error getting bytes");
            },

            "[" => {
                if memory[datapointer] == 0 {
                    let mut bracket_nesting = 1;
                    let saved_pc = pc;

                    pc+=1;
                    while bracket_nesting > 0 && pc < program.instructions.len() {
                        if program.instructions[pc] == "]" {
                            bracket_nesting -= 1;
                        } else if program.instructions[pc] == "[" {
                            bracket_nesting += 1;
                        }
                        pc+=1;
                    }

                    if bracket_nesting > 0 {
                        panic!("unmatched '[' at pc={}", saved_pc);
                    }
                }
            },

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

                    if bracket_nesting > 0 {
                        panic!("unmatched ']' at pc={}", saved_pc);
                    }
                }
            },

            c   => panic!("Bad character: {}", c)
        }

        pc+=1;
    }
}
