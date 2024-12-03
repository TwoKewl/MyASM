
#[derive(Clone, Debug)]
pub enum InstructionType {
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    LOG,
    MOD,
    NUL
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub instruction: InstructionType,
    pub operands: Vec<String>
}