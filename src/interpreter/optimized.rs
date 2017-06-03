use Program;

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

fn gen_bytecode(instructions: Vec<String>) -> Vec<Op> {
    let mut pc = 0;
    let mut bytecode = vec!();

    while pc < instructions.len() {
        match instructions[pc].as_ref() {
            "[" => {
            },

            "]" => {
            },

            instruction => {
                let start = pc;

                while pc < instructions.len() && instructions[pc] == instruction {
                    pc += 1;
                }

                let count = pc - start;

                let op = match instruction {
                    ">" => Op::IncPointer(count),
                    "<" => Op::DecPointer(count),
                    "+" => Op::IncData(count as u8),
                    "-" => Op::DecData(count as u8),
                    "." => Op::Read,
                    "," => Op::Write,
                    _   => panic!("Cannot convert instruction to bytecode: {}", instruction),
                };
                bytecode.push(op);
            }

        }
        pc += 1;
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

            _ => panic!("Not done yet"),
        }

        pc += 1;
    }
}
