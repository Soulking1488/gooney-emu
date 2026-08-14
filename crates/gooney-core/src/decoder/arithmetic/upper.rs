use crate::cpu::CpuState;
use crate::decoder::ExecutionResult;

pub fn execute(
    _opcode: u32,
    instruction: u32,
    rd: usize,
    cpu: &mut CpuState,
) -> ExecutionResult {
    let opcode = _opcode;
    match opcode {
        0x37 => { // LUI
            let imm = (((instruction as i32) >> 12) << 12) as i64;
            cpu.write_reg(rd, imm as u64);
            cpu.pc += 4;
            ExecutionResult::Ok
        }
        0x17 => { // AUIPC
            let imm = (((instruction as i32) >> 12) << 12) as i64;
            let val = (cpu.pc as i64).wrapping_add(imm) as u64;
            cpu.write_reg(rd, val);
            cpu.pc += 4;
            ExecutionResult::Ok
        }
        _ => ExecutionResult::Trap(format!("Unknown upper opcode: 0x{:X}", opcode)),
    }
}
