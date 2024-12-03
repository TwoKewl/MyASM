
use std::fs::File;
use std::io::Read;

use crate::token::Token;
use crate::token::TokenType;

use crate::instruction::InstructionType;

fn instruction_from_string(value: &str) -> InstructionType {
    match value {
        "MOV" => InstructionType::MOV,
        "ADD" => InstructionType::ADD,
        "SUB" => InstructionType::SUB,
        "MUL" => InstructionType::MUL,
        "DIV" => InstructionType::DIV,
        "LOG" => InstructionType::LOG,
        "MOD" => InstructionType::MOD,
        _ => InstructionType::NUL
    }
}

fn get_token(token_value: String) -> Token {
    let instruction_type: InstructionType = instruction_from_string(&token_value);

    match instruction_type {
        InstructionType::NUL => {
            let mut is_integer: bool = true;

            for c in token_value.chars() {
                if !c.is_numeric() {
                    is_integer = false;
                    break;
                }
            }

            if is_integer {
                return Token {
                    token_type: TokenType::Integer,
                    value: token_value,
                    instruction_type,
                }
            }

            let token_type: TokenType;
            let mut chars: Vec<char> = Vec::new();
            for c in token_value.chars() {
                chars.push(c);
            }

            if chars[0] == 'L' && chars[1].is_numeric() {
                token_type = TokenType::Register;
            } else {
                token_type = TokenType::Unrecognised;
            }


            Token {
                token_type,
                value: token_value.to_string(),
                instruction_type,
            }
        }
        _ => Token {
                token_type: TokenType::Instruction,
                value: token_value.to_string(),
                instruction_type,
            }
        
    }
}

pub fn tokenize(filename: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut file: File = File::open(filename).expect("File not found");

    file.read_to_end(&mut buffer).expect("Error reading file");
    buffer.push(0x20);

    let mut token_value: String = String::from("");

    for i in 0..buffer.len() {
        let byte: u8 = buffer[i];

        if buffer[i] == 0x0D { // carriage return
            continue;
        }        

        if byte != 0x20 && byte != 0x0A {
            token_value.push(byte as char);
        } else {
            if token_value.len() == 0 {
                token_value = "".to_string();
                continue;
            }

            let token: Token = get_token(token_value);

            tokens.push(token);
            token_value = "".to_string();
        }
    }

    tokens
}