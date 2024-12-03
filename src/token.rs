
use crate::instruction::InstructionType;


#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub instruction_type: InstructionType
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Instruction,
    Register,
    Integer,
    Unrecognised
}