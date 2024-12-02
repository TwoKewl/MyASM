
use crate::token::TokenType;

fn is_instruction(value: String) -> bool {
    match value.as_str() {
        "MOV" => true,
        "ADD" => true,
        "SUB" => true,
        "MUL" => true,
        "DIV" => true,
        "LOG" => true,
        _ => false
    }
}

pub fn get_token_type(value: String) -> TokenType {
    let chars: Vec<char> = value.chars().collect();

    if chars[0] == 'L' && chars[1].is_numeric() {
        return TokenType::Register;
    } else if chars[0].is_numeric() {
        return TokenType::Integer;
    } else if is_instruction(value) {
        return TokenType::Instruction;
    }

    TokenType::Unrecognised
}