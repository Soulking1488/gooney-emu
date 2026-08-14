use crate::cpu::CpuState;
use crate::decoder::ExecutionResult;

pub mod imm;
pub mod reg;
pub mod upper;

pub fn execute(
    opcode: u32,
    instruction: u32,
    rd: usize,
    rs1: usize,
    rs2: usize,
    funct3: u32,
    funct7: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    match opcode {
        0x13 => imm::execute(instruction, rd, rs1, funct3, funct7, cpu),
        0x33 => reg::execute(instruction, rd, rs1, rs2, funct3, funct7, cpu),
        0x37 | 0x17 => upper::execute(opcode, instruction, rd, cpu),
        _ => ExecutionResult::Trap(format!("Unknown arithmetic opcode: 0x{:X}", opcode)),
    }
}
