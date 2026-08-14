use crate::cpu::CpuState;
use crate::memory::Memory;
use super::ExecutionResult;

pub fn execute(
    opcode: u32,
    instruction: u32,
    rd: usize,
    rs1: usize,
    rs2: usize,
    funct3: u32,
    cpu: &mut CpuState,
    mem: &mut Memory,
) -> ExecutionResult {
    match opcode {
        // LOAD instructions (e.g., LW - Opcode 0x03)
        0x03 => {
            let imm = ((instruction as i32) >> 20) as i64;
            let addr = (cpu.read_reg(rs1) as i64).wrapping_add(imm) as u64;

            let val = match funct3 {
                0x2 => { // LW (Load Word)
                    let word = match mem.read_u32(addr) {
                        Ok(w) => w,
                        Err(e) => return ExecutionResult::Trap(format!("Load trap: {}", e)),
                    };
                    (word as i32) as i64 as u64 // Sign-extend 32-bit to 64-bit
                }
                _ => return ExecutionResult::Trap(format!("Unsupported load funct3: 0x{:X}", funct3)),
            };

            cpu.write_reg(rd, val);
            cpu.pc += 4;
            ExecutionResult::Ok
        }

        // STORE instructions (e.g., SW - Opcode 0x23)
        0x23 => {
            let imm_11_5 = (instruction >> 25) & 0x7F;
            let imm_4_0 = (instruction >> 7) & 0x1F;
            let imm = (((((imm_11_5 << 5) | imm_4_0) as i32) << 20) >> 20) as i64;

            let addr = (cpu.read_reg(rs1) as i64).wrapping_add(imm) as u64;
            let val = cpu.read_reg(rs2);

            match funct3 {
                0x2 => { // SW (Store Word)
                    if let Err(e) = mem.write_u32(addr, val as u32) {
                        return ExecutionResult::Trap(format!("Store trap: {}", e));
                    }
                }
                _ => return ExecutionResult::Trap(format!("Unsupported store funct3: 0x{:X}", funct3)),
            }

            cpu.pc += 4;
            ExecutionResult::Ok
        }

        _ => ExecutionResult::Trap(format!("Unknown memory opcode: 0x{:X}", opcode)),
    }
}
