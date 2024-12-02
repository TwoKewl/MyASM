
use crate::token::Token;
use crate::token::TokenType;

use crate::instruction::Instruction;

pub fn parse(tokens: Vec<Token>) -> Vec<Instruction> { 
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut index: i32 = 0;

    while index < tokens.len() as i32 {
        let token: &Token = &tokens[index as usize];
        
        match token.token_type {
            TokenType::Instruction => {
                let mut instruction: Instruction = Instruction {
                    instruction: token.instruction_type.clone(),
                    operands: Vec::new()
                };
                index += 1;

                while index < tokens.len() as i32 && matches!(tokens[index as usize].token_type, TokenType::Register | TokenType::Integer) {
                    instruction.operands.push(tokens[index as usize].value.clone());
                    index += 1;
                }

                instructions.push(instruction);
            },
            _ => {
                panic!("Unexpected token: \"{}\".", token.value);
            }
        }
    }

    instructions
}