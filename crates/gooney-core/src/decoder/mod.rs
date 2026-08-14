use crate::cpu::CpuState;
use crate::memory::Memory;

pub mod arithmetic;
pub mod control;
pub mod memory_instr;

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Ok,
    Trap(String),
    Halt,
}

pub struct Decoder;

impl Decoder {
    pub fn decode_and_execute(instruction: u32, cpu: &mut CpuState, mem: &mut Memory) -> ExecutionResult {
        let opcode = instruction & 0x7F;
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;
        let funct3 = (instruction >> 12) & 0x7;
        let funct7 = (instruction >> 25) & 0x7F;

        match opcode {
            // Arithmetic & Upper Immediates
            0x13 | 0x33 | 0x37 | 0x17 => {
                arithmetic::execute(opcode, instruction, rd, rs1, rs2, funct3, funct7, cpu)
            }

            // Jumps & Branches
            0x6F | 0x67 | 0x63 => {
                control::execute(opcode, instruction, rd, rs1, rs2, funct3, cpu)
            }

            // Loads & Stores
            0x03 | 0x23 => {
                memory_instr::execute(opcode, instruction, rd, rs1, rs2, funct3, cpu, mem)
            }

            // System Instructions (ECALL)
            0x73 => {
                cpu.pc += 4;
                ExecutionResult::Halt
            }

            // Custom Opcode Slots
            0x0B | 0x2B | 0x5B | 0x7B => {
                cpu.pc += 4;
                ExecutionResult::Trap(format!("Trap: custom opcode unassigned (instruction: 0x{:08X})", instruction))
            }

            _ => {
                cpu.pc += 4;
                ExecutionResult::Trap(format!(
                    "Illegal Instruction: Unknown opcode 0x{:02X} in instruction 0x{:08X}",
                    opcode, instruction
                ))
            }
        }
    }
}
