use std::fmt;

pub struct Program {
    pub instructions: Vec<String>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program: {}", self.instructions.join(""))
    }
}

impl Program {
    pub fn parse(text: String) -> Program {
        let chars = text.split("")
            .map(|s| s.to_string())
            .filter(|s| Program::is_valid_symbol(&s))
            .collect();

        Program { instructions: chars }
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
}
