extern crate rand;

use std::io::{self, Read};
use std::fmt;

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
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read file");

    println!("Read in: {}", input);

    input
}

fn main() {
    let input = read_input_stream();
    let p = Program::parse(input);

    println!("This is your parsed program {}", p);
}
