mod parser;
mod interpreter;

use std::io::{self, Read};
use std::env;
use std::fs::File;

use parser::Program;

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
    // interpreter::naive::run(program);
    interpreter::optimized::run(program);
}
