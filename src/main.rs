use std::env;
use std::fs;
use std::process::Command;
use std::process::Stdio;
use regex::Regex;

mod instructions;
use instructions::*;

fn const_val(c: &char) -> u16 {
    match c {
        '0' => 0x1c72,
        '1' => 0x14bc,
        '2' => 0xfc26,
        '3' => 0x7e37,
        '4' => 0xb53f,
        '5' => 0x4fda,
        '6' => 0x20fe,
        '7' => 0x445a,
        '8' => 0xb76a,
        '9' => 0x25e5,
        unexpected_char => {unreachable!("incorrect character '{unexpected_char}' provided to match function")}
    }
}


enum InstructionSection {
    NativeInstruction(char),
    OptimizedInstruction(OptimizedInstruction),
    //PushConstant() // used in math optimizations
}

enum OptimizedInstruction {

}


impl InstructionSection {
    fn gen_rust(&self) -> String {
        let preformat: String = match self {
            Self::OptimizedInstruction(_) => {todo!()}
            Self::NativeInstruction(c) => {
                match c {
                    'A' => {String::from(instr_A)}
                    'C' => {String::from(instr_C)}
                    'L' => {String::from(instr_L)}
                    'N' => {String::from(instr_N)}
                    'R' => {String::from(instr_R)}
                    'X' => {String::from(instr_X)}
                    'a' => {String::from(instr_a)}
                    'l' => {String::from(instr_l)}
                    'r' => {String::from(instr_r)}
                    '0'..='9' => {
                        let val = const_val(c);
                        format!("stack.push({val:#04x});")
                    }
                    unexpected_char => {panic!("Unexpected character {unexpected_char} in code string.")}
                }
            }
        };

        format!("\n\t// instr {}\n\t{}\n", self.name(), preformat)
    }

    fn name(&self) -> String {
        match self {
            Self::NativeInstruction(c) => {c.to_string()}
            Self::OptimizedInstruction(_) => {
                todo!()
            }
        }
    }

    fn create_native_instruction(c: char, index: usize) -> InstructionSection {
        match c {
            '0'..='9' | 'a'..='z' | 'A'..='Z' | '(' | ')' => {Self::NativeInstruction(c)}
            unexpected_char => {panic!("Unexpected character {unexpected_char} at char number {index} in code.")}
        }
    }
}



// felt like giving these fun names
const PREAMBLE: &'static str = 
"use std::io::{self, Write};

fn main() {
\tlet mut stack: Vec<u16> = Vec::new();
\tlet mut flag = false;\n";

const POSTSCRIPT: &'static str = "}";

const RUST_IR_FILE_PATH: &'static str = "16b64_ir.rs"; 

const RUSTC_ARGS: &'static str = "-C opt-level = 'z'";

fn main() {
    // TODO: add clap
    // --16b_no_opt - no optimizations (default)
    // --16b_std_opt - standard optimizations (some performance optimzations)
    // --16b_math_opt - math optimizations 
    // --16b_all_opt - all optimtizations
    // --O3 - all 16b optimizations and rust optimizations
    // --keep_rust_ir - keep rust IR

    // Get input args
    let args: Vec<String> = env::args().collect();

    // get name of output file
    let output_file_path = "c16b64_bin"; 

    // Read file to string
    let file_path = &args[1];
    let mut contents = fs::read_to_string(file_path)
        .expect(&format!("Could not read file {file_path}"));

    contents.push('\n'); // to make the regex work better

    // PARSER & LEXER ----------------------------------------------------------

    let re_remove_comments: Regex = Regex::new(r"([\w\(\)\s]+)#?.*\n").unwrap();

    let removed_comments = re_remove_comments
        .captures_iter(&contents)
        .map(|c| c.get(1).map_or("", |m| m.as_str()));

    // this one functional statement does all of the lexing and parsing we need.
    let mut code = removed_comments.map(|x| 
        x.chars()
        .filter(|x| !x.is_whitespace())
    ).flatten();

    // let code = code.by_ref().collect::<String>();
    // dbg!(&code);
    // let code = code.chars();
    
    // now we have our code string from the file. Let's tokenize it

    let instructions = code.enumerate().map(|(i, x)| InstructionSection::create_native_instruction(x, i));

    // OPTIMIZER ---------------------------------------------------------------


    


    // GENERATOR ---------------------------------------------------------------

    let generated_code: String = instructions.map(|instr|  instr.gen_rust()).collect();
    let full_code = format!("{PREAMBLE}{generated_code}{POSTSCRIPT}");

    fs::write(RUST_IR_FILE_PATH, full_code).expect("File");

    // COMPILER CALL -----------------------------------------------------------

    let rustc_result = Command::new("rustc")
                    .args([RUST_IR_FILE_PATH, "-o", output_file_path, RUSTC_ARGS])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .status()
                    .expect("failed to execute rustc");

    std::process::exit(rustc_result.code().unwrap_or(0));

}
