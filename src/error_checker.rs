
use crate::instruction::InstructionType;
use crate::token::Token;
use crate::token::TokenType;
use crate::instruction::Instruction;

use crate::type_checker::get_token_type;

pub fn token_check(tokens: Vec<Token>) {
    for token in tokens {
        match token.token_type {
            TokenType::Unrecognised => {
                panic!("Unrecognised token: \"{}\".", token.value);
            },
            _ => {}
        }
    }
}

pub fn instruction_check(instructions: Vec<Instruction>) {
    for i in instructions {
        match i.instruction {
            InstructionType::MOV => {
                if i.operands.len() != 2 || get_token_type(i.operands[0].clone()) != TokenType::Register || get_token_type(i.operands[1].clone()) != TokenType::Integer {
                    panic!("MOV instruction requires 2 operands. \nUsage: MOV <destination> <value>");
                }
            }
            InstructionType::ADD => {
                if i.operands.len() != 3 || get_token_type(i.operands[0].clone()) != TokenType::Register || get_token_type(i.operands[1].clone()) != TokenType::Register || get_token_type(i.operands[2].clone()) != TokenType::Register {
                    panic!("ADD instruction requires 3 operands. \nUsage: ADD <destination> <source1> <source2>");
                }
            }
            InstructionType::SUB => {
                if i.operands.len() != 3 || get_token_type(i.operands[0].clone()) != TokenType::Register || get_token_type(i.operands[1].clone()) != TokenType::Register || get_token_type(i.operands[2].clone()) != TokenType::Register {
                    panic!("SUB instruction requires 3 operands. \nUsage: SUB <destination> <source1> <source2>");
                }
            }
            InstructionType::MUL => {
                if i.operands.len() != 3 || get_token_type(i.operands[0].clone()) != TokenType::Register || get_token_type(i.operands[1].clone()) != TokenType::Register || get_token_type(i.operands[2].clone()) != TokenType::Register {
                    panic!("MUL instruction requires 3 operands. \nUsage: MUL <destination> <source1> <source2>");
                }
            }
            InstructionType::DIV => {
                if i.operands.len() != 3 || get_token_type(i.operands[0].clone()) != TokenType::Register || get_token_type(i.operands[1].clone()) != TokenType::Register || get_token_type(i.operands[2].clone()) != TokenType::Register {
                    panic!("DIV instruction requires 3 operands. \nUsage: DIV <destination> <source1> <source2>");
                }
            }
            InstructionType::LOG => {
                if i.operands.len() != 1 || get_token_type(i.operands[0].clone()) != TokenType::Register {
                    panic!("LOG instruction requires 1 operand. \nUsage: LOG <register>");
                }
            }
            _ => {}
        }
    }
}