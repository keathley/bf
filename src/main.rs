extern crate clap;

use clap::{App, Arg};
use std::io::{self, Read};
use std::fs::File;

mod program;
mod interpreter;

use program::Program;

fn read_input_stream() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut input)
        .expect("Failed to read file");

    input
}

fn read_file(file: &str) -> String {
    let mut input = String::new();

    File::open(file)
        .expect("Unable to open file")
        .read_to_string(&mut input)
        .expect("Unable to read file");

    input
}

fn main() {
    let matches = App::new("brainfuck")
        .version("v1")
        .author("Chris Keathley")
        .about("A semi-optimized Brainfuck interpreter")
        .arg(Arg::with_name("naive")
             .short("n")
             .long("naive-mode")
             .help("Executes the code in naive mode"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .index(1))
        .get_matches();

    let input = match matches.value_of("INPUT") {
        Some(file) => read_file(file),
        None       => read_input_stream()
    };
    let program = Program::parse(&input);

    if matches.is_present("naive") {
        interpreter::naive::run(program);
    } else {
        interpreter::optimized::run(program);
    };
}
