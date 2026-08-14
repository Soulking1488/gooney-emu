use crate::cpu::CpuState;
use super::ExecutionResult;

pub fn execute(
    opcode: u32,
    instruction: u32,
    rd: usize,
    rs1: usize,
    rs2: usize,
    funct3: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    match opcode {
        // JAL (Jump and Link) - Opcode 0x6F
        0x6F => {
            let imm20 = (instruction >> 31) & 0x1;
            let imm10_1 = (instruction >> 21) & 0x3FF;
            let imm11 = (instruction >> 20) & 0x1;
            let imm19_12 = (instruction >> 12) & 0xFF;

            let mut offset = (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);
            if (offset & 0x100000) != 0 {
                offset |= 0xFFE00000; // Sign-extend 21-bit
            }

            let return_addr = cpu.pc + 4;
            cpu.pc = (cpu.pc as i64).wrapping_add(offset as i64) as u64;
            cpu.write_reg(rd, return_addr);
            ExecutionResult::Ok
        }

        // JALR (Jump and Link Register) - Opcode 0x67
        0x67 => {
            let imm = ((instruction as i32) >> 20) as i64;
            let base = cpu.read_reg(rs1);
            let target = (base as i64).wrapping_add(imm) & !1;

            let return_addr = cpu.pc + 4;
            cpu.pc = target as u64;
            cpu.write_reg(rd, return_addr);
            ExecutionResult::Ok
        }

        // Branches - Opcode 0x63
        0x63 => {
            let r1 = cpu.read_reg(rs1);
            let r2 = cpu.read_reg(rs2);

            let imm12 = (instruction >> 31) & 0x1;
            let imm10_5 = (instruction >> 25) & 0x3F;
            let imm4_1 = (instruction >> 8) & 0xF;
            let imm11 = (instruction >> 7) & 0x1;

            let mut offset = (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1);
            if (offset & 0x1000) != 0 {
                offset |= 0xFFFFE000; // Sign-extend 13-bit
            }

            let condition = match funct3 {
                0x0 => r1 == r2,                       // BEQ
                0x1 => r1 != r2,                       // BNE
                0x4 => (r1 as i64) < (r2 as i64),      // BLT
                0x5 => (r1 as i64) >= (r2 as i64),     // BGE
                0x6 => r1 < r2,                        // BLTU
                0x7 => r1 >= r2,                       // BGEU
                _ => return ExecutionResult::Trap(format!("Unknown branch funct3: 0x{:X}", funct3)),
            };

            if condition {
                cpu.pc = (cpu.pc as i64).wrapping_add(offset as i64) as u64;
            } else {
                cpu.pc += 4;
            }
            ExecutionResult::Ok
        }

        _ => ExecutionResult::Trap(format!("Unknown control flow opcode: 0x{:X}", opcode)),
    }
}
