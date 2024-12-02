
#[derive(Clone, Debug)]
pub enum InstructionType {
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    LOG,
    NUL
}

#[derive(Clone)]
pub struct Instruction {
    pub instruction: InstructionType,
    pub operands: Vec<String>
}