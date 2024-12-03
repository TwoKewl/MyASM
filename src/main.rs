
mod error_checker;
mod instruction;
mod interpreter;
mod parser;
mod token;
mod tokeniser;
mod type_checker;

use error_checker::{instruction_check, token_check};
use interpreter::interpret_instructions;

fn main() {
    let filename: String = "program.a".to_string();
    let tokens: Vec<token::Token> = tokeniser::tokenize(filename.to_string());
    token_check(tokens.clone());
    let instructions: Vec<instruction::Instruction> = parser::parse(tokens);
    instruction_check(instructions.clone());
    interpret_instructions(instructions);
} 