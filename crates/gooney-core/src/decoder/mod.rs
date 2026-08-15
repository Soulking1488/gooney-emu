pub mod arithmetic;
pub mod control;
pub mod memory_instr;
pub mod system;

use crate::cpu::CpuState;
use crate::memory::Memory;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionResult {
    Ok,
    Halt,
    Trap(String),
}

pub struct Decoder;

impl Decoder {
    pub fn decode_and_execute(
        instruction: u32,
        cpu: &mut CpuState,
        memory: &mut Memory,
    ) -> ExecutionResult {
        // Check for explicit ecall/halt instruction (0x00000073)
        if instruction == 0x00000073 {
            return ExecutionResult::Halt;
        }

        let opcode = instruction & 0x7F;
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;
        let funct3 = (instruction >> 12) & 0x7;
        let funct7 = (instruction >> 25) & 0x7F;

        match opcode {
            // Arithmetic & Upper Immediates
            0x13 | 0x33 | 0x1B | 0x3B | 0x37 | 0x17 => {
                arithmetic::execute(opcode, instruction, rd, rs1, rs2, funct3, funct7, cpu)
            }
            // Control Flow & Branching
            0x63 | 0x6F | 0x67 => {
                control::execute(opcode, instruction, rd, rs1, rs2, funct3, cpu)
            }
            0x03 | 0x23 => {
                memory_instr::execute(opcode, instruction, rd, rs1, rs2, funct3, cpu, memory)
            }
            0x0F => system::execute(opcode, instruction, cpu),
            0x73 => system::execute(opcode, instruction, cpu),
            _ => ExecutionResult::Trap(format!("Unknown opcode: 0x{:X}", opcode)),
        }
    }
}
