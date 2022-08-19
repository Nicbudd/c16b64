use std::env;
use std::fs;
use regex::Regex;

struct Program {
    contents: Codegroup
}

enum Codegroup {
    Loop(Loop),
    Statement(Vec<Codeword>),
}

struct Loop {
    contents: Vec<Codegroup>
}

enum Codeword {
    Op(Op),
    Const(Const),
    Unknown,
}

enum Op {
    A,
    O,
}

enum Const {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
}

impl Const {
    fn val(&self) -> u16 {
        match &self {
            Const::C0 => 0x1c72,
            Const::C1 => 0x14bc,
            Const::C2 => 0xfc26,
            Const::C3 => 0x7e37,
            Const::C4 => 0xb53f,
            Const::C5 => 0x4fda,
            Const::C6 => 0x20fe,
            Const::C7 => 0x445a,
            Const::C8 => 0xb76a,
            Const::C9 => 0x25e5,
        }
    }
}

fn main() {
    // Get input args
    let args: Vec<String> = env::args().collect();

    // Read file to string
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect(&format!("Could not read file {file_path}"));

    // LEXER -------------------------------------------------------------

    //let mut code_string = String::new();

    let REMOVE_COMMENTS: Regex = Regex::new(r"([\w\(\)\s]*)#?.*\n").unwrap();
    
    let mut code_string = String::new();

    for caps in REMOVE_COMMENTS.captures_iter(&contents) {
        let s: String = caps[1]
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        code_string.push_str(&s);
    }

    // now we have our code string from the file. Let's tokenize it

    let mut prog = Program {contents: Codegroup::Statement(vec![])};

    for char in code_string.chars() {

        //prog.contents.
        match char {
            '0' => Codeword::Const(Const::C0),
            _ => Codeword::Unknown,
        }


    }

    dbg!(code_string);

    // PARSER ------------------------------------------------------------
}
