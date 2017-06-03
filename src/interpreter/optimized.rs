use Program;
use std::io::{self, Read};

enum Op {
    IncPointer(usize),
    DecPointer(usize),
    IncData(u8),
    DecData(u8),
    Read,
    Write,
    JumpIfDataZero {offset: usize},
    JumpIfDataNotZero {offset: usize},
}

type Ops = Vec<Op>;

fn bytecode_to_string(ops: &Ops) -> String {
    ops.iter().map(|op| match op {
        &Op::IncPointer(count) => format!("inc pointer: {}", count),
        &Op::DecPointer(count) => format!("dec pointer: {}", count),
        &Op::IncData(count) => format!("inc data: {}", count),
        &Op::DecData(count) => format!("dec data: {}", count),
        &Op::Read           => format!("reading"),
        &Op::Write          => format!("writing"),
        &Op::JumpIfDataZero{offset} => format!("Jumping to {}", offset),
        &Op::JumpIfDataNotZero{offset} => format!("Jumping back to {}", offset)
    })
    .collect::<Vec<String>>()
    .join("\n")
}

fn gen_bytecode(instructions: Vec<String>) -> Ops {
    let mut pc = 0;
    let mut bytecode = vec!();
    let mut jumps = vec!();

    while pc < instructions.len() {
        match instructions[pc].as_ref() {
            "[" => {
                jumps.push(bytecode.len());
                bytecode.push(Op::JumpIfDataZero{offset: 0});
                pc += 1;
            },

            "]" => {
                if jumps.is_empty() {
                    panic!("Unmatched closing ] at {}", pc);
                }
                let offset = jumps.pop().unwrap();
                bytecode[offset] = Op::JumpIfDataZero{offset: bytecode.len()};
                bytecode.push(Op::JumpIfDataNotZero{offset: offset});
                pc += 1;
            },

            instruction => {
                let start = pc;
                pc += 1;

                while pc < instructions.len() && instructions[pc] == instruction {
                    pc += 1;
                }

                let count = pc - start;

                let op = match instruction {
                    ">" => Op::IncPointer(count),
                    "<" => Op::DecPointer(count),
                    "+" => Op::IncData(count as u8),
                    "-" => Op::DecData(count as u8),
                    "." => Op::Write,
                    "," => Op::Read,
                    _   => panic!("Cannot convert instruction to bytecode: {}", instruction),
                };
                bytecode.push(op);
            }
        }
    }

    bytecode
}

pub fn run(program: Program) {
    let bytecode = gen_bytecode(program.instructions);
    let memory_alloc = 30000;
    let mut pc = 0;
    let mut datapointer = 0;
    let mut memory: Vec<u8> = vec![0; memory_alloc];

    while pc < bytecode.len() {
        match bytecode[pc] {
            Op::IncPointer(count) => datapointer += count,
            Op::DecPointer(count) => datapointer -= count,

            Op::IncData(count) => {
                let current_value = memory[datapointer];

                if 255 - count > current_value {
                }
                memory[datapointer] = (memory[datapointer] + count) % 255;
            },

            Op::DecData(count) => {
                memory[datapointer] = (memory[datapointer] - count) % 255;
            },

            Op::JumpIfDataZero{offset} => {
                if memory[datapointer] == 0 {
                    pc = offset;
                }
            },

            Op::JumpIfDataNotZero{offset} => {
                if memory[datapointer] != 0 {
                    pc = offset;
                }
            },

            Op::Read => {
                let input = io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8)
                    .expect("Failed to read from stdin");

                memory[datapointer] = input;
            },

            Op::Write => {
                print!("{}", memory[datapointer] as char);
            }
        }

        pc += 1;
    }
}
