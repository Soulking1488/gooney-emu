use crate::cpu::CpuState;
use crate::decoder::ExecutionResult;

pub fn execute(
    instruction: u32,
    rd: usize,
    rs1: usize,
    funct3: u32,
    _funct7: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    let r1 = cpu.read_reg(rs1);
    let val = match funct3 {
        0x0 => { // ADDI
            let imm = ((instruction as i32) >> 20) as i64;
            r1.wrapping_add(imm as u64)
        }
        0x1 => { // SLLI (Shift Left Logical Immediate)
            let shamt = ((instruction >> 20) & 0x3F) as u32;
            r1.wrapping_shl(shamt)
        }
        0x2 => { // SLTI (Set Less Than Immediate, Signed)
            let imm = ((instruction as i32) >> 20) as i64;
            if (r1 as i64) < imm { 1 } else { 0 }
        }
        0x3 => { // SLTIU (Set Less Than Immediate, Unsigned)
            let imm = ((instruction as i32) >> 20) as i64;
            if r1 < (imm as u64) { 1 } else { 0 }
        }
        0x4 => { // XORI
            let imm = ((instruction as i32) >> 20) as i64;
            r1 ^ (imm as u64)
        }
        0x5 => { // SRLI or SRAI
            let shamt = ((instruction >> 20) & 0x3F) as u32;
            let top7 = (instruction >> 25) & 0x7F;
            match top7 {
                0x00 => r1.wrapping_shr(shamt),                  // SRLI
                0x20 => ((r1 as i64).wrapping_shr(shamt)) as u64, // SRAI
                _ => {
                    return ExecutionResult::Trap(format!(
                        "Unsupported SRLI/SRAI variant funct7: 0x{:X}",
                        top7
                    ))
                }
            }
        }
        0x6 => { // ORI
            let imm = ((instruction as i32) >> 20) as i64;
            r1 | (imm as u64)
        }
        0x7 => { // ANDI
            let imm = ((instruction as i32) >> 20) as i64;
            r1 & (imm as u64)
        }
        _ => {
            return ExecutionResult::Trap(format!(
                "Unsupported OP-IMM funct3: 0x{:X} for instruction 0x{:08X}",
                funct3, instruction
            ))
        }
    };

    cpu.write_reg(rd, val);
    cpu.pc += 4;
    ExecutionResult::Ok
}

pub fn execute_32(
    instruction: u32,
    rd: usize,
    rs1: usize,
    funct3: u32,
    _funct7: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    let r1 = cpu.read_reg(rs1) as u32; // Truncate to 32 bits
    let val = match funct3 {
        0x0 => { // ADDIW
            let imm = (instruction as i32) >> 20;
            (r1.wrapping_add(imm as u32)) as i32 as i64 // Sign-extend 32-bit result to 64-bit
        }
        0x1 => { // SLLIW (Shift Left Logical Immediate Word)
            let shamt = ((instruction >> 20) & 0x1F) as u32;
            (r1.wrapping_shl(shamt)) as i32 as i64
        }
        0x5 => { // SRLIW or SRAIW
            let shamt = ((instruction >> 20) & 0x1F) as u32;
            let top7 = (instruction >> 25) & 0x7F;
            match top7 {
                0x00 => (r1.wrapping_shr(shamt)) as i32 as i64,                  // SRLIW
                0x20 => ((r1 as i32).wrapping_shr(shamt)) as i64,             // SRAIW
                _ => {
                    return ExecutionResult::Trap(format!(
                        "Unsupported SRLIW/SRAIW variant funct7: 0x{:X}",
                        top7
                    ))
                }
            }
        }
        _ => {
            return ExecutionResult::Trap(format!(
                "Unsupported OP-IMM-32 funct3: 0x{:X}",
                funct3
            ))
        }
    };

    cpu.write_reg(rd, val as u64);
    cpu.pc += 4;
    ExecutionResult::Ok
}
