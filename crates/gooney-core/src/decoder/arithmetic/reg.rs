use crate::cpu::CpuState;
use crate::decoder::ExecutionResult;

pub fn execute(
    instruction: u32,
    rd: usize,
    rs1: usize,
    rs2: usize,
    funct3: u32,
    funct7: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    let r1 = cpu.read_reg(rs1);
    let r2 = cpu.read_reg(rs2);
    let shamt = (r2 & 0x3F) as u32; // Lower 6 bits for 64-bit shifts
    
    let val = match (funct3, funct7) {
        (0x0, 0x00) => r1.wrapping_add(r2),               // ADD
        (0x0, 0x20) => r1.wrapping_sub(r2),               // SUB
        (0x1, 0x00) => r1.wrapping_shl(shamt),            // SLL (Shift Left Logical)
        (0x2, 0x00) => {                                  // SLT (Set Less Than, Signed)
            if (r1 as i64) < (r2 as i64) { 1 } else { 0 }
        }
        (0x3, 0x00) => {                                  // SLTU (Set Less Than Unsigned)
            if r1 < r2 { 1 } else { 0 }
        }
        (0x4, 0x00) => r1 ^ r2,                           // XOR
        (0x5, 0x00) => r1.wrapping_shr(shamt),            // SRL (Shift Right Logical)
        (0x5, 0x20) => {                                  // SRA (Shift Right Arithmetic)
            ((r1 as i64).wrapping_shr(shamt)) as u64
        }
        (0x6, 0x00) => r1 | r2,                           // OR
        (0x7, 0x00) => r1 & r2,                           // AND
        _ => {
            return ExecutionResult::Trap(format!(
                "Illegal RV64I OP variant (funct3: 0x{:X}, funct7: 0x{:X}) for instruction 0x{:08X}",
                funct3, funct7, instruction
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
    rs2: usize,
    funct3: u32,
    funct7: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    let r1 = cpu.read_reg(rs1) as u32;
    let r2 = cpu.read_reg(rs2) as u32;
    let shamt = (r2 & 0x1F) as u32; // Lower 5 bits for 32-bit shifts
    
    let val = match (funct3, funct7) {
        (0x0, 0x00) => (r1.wrapping_add(r2)) as i32 as i64, // ADDW
        (0x0, 0x20) => (r1.wrapping_sub(r2)) as i32 as i64, // SUBW
        (0x1, 0x00) => (r1.wrapping_shl(shamt)) as i32 as i64, // SLLW
        (0x5, 0x00) => (r1.wrapping_shr(shamt)) as i32 as i64, // SRLW
        (0x5, 0x20) => ((r1 as i32).wrapping_shr(shamt)) as i64, // SRAW
        _ => {
            return ExecutionResult::Trap(format!(
                "Illegal RV64I OP-32 variant (funct3: 0x{:X}, funct7: 0x{:X}) for instruction 0x{:08X}",
                funct3, funct7, instruction
            ))
        }
    };

    cpu.write_reg(rd, val as u64);
    cpu.pc += 4;
    ExecutionResult::Ok
}
