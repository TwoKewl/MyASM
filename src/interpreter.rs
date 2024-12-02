
use std::collections::HashMap;

use crate::instruction::Instruction;
use crate::instruction::InstructionType;

pub fn interpret_instructions(instructions: Vec<Instruction>) {
    let mut registers: HashMap<String, i64> = HashMap::with_capacity(100);


    for i in instructions {
        match i.instruction {
            InstructionType::MOV => {
                registers.insert(i.operands[0].clone(),i.operands[1].parse::<i64>().unwrap());
            }
            InstructionType::ADD => {
                let value1: i64 = registers.get(&i.operands[1]).unwrap().clone();
                let value2: i64 = registers.get(&i.operands[2]).unwrap().clone();
                registers.insert(i.operands[0].clone(), value1 + value2);
            }
            InstructionType::SUB => {
                let value1: i64 = registers.get(&i.operands[1]).unwrap().clone();
                let value2: i64 = registers.get(&i.operands[2]).unwrap().clone();
                registers.insert(i.operands[0].clone(), value1 - value2);
            }
            InstructionType::MUL => {
                let value1: i64 = registers.get(&i.operands[1]).unwrap().clone();
                let value2: i64 = registers.get(&i.operands[2]).unwrap().clone();
                registers.insert(i.operands[0].clone(), value1 * value2);
            }
            InstructionType::DIV => {
                let value1: i64 = registers.get(&i.operands[1]).unwrap().clone();
                let value2: i64 = registers.get(&i.operands[2]).unwrap().clone();
                registers.insert(i.operands[0].clone(), value1 / value2);
            }
            InstructionType::LOG => {
                let value: i64 = registers.get(&i.operands[0]).unwrap().clone();
                println!("{}", value);
            }
            _ => {}
        }
    }
}