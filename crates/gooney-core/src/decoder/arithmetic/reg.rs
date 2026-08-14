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
    
    let val = match (funct3, funct7) {
        (0x0, 0x00) => r1.wrapping_add(r2), // ADD
        (0x0, 0x20) => r1.wrapping_sub(r2), // SUB
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
