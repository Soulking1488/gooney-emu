use gooney_core::cpu::CpuState;
use gooney_core::decoder::ExecutionResult;

/// A sample custom extension instruction: GOONEY_ADD
/// Operation: rd = rs1 + rs2 + 0x42 (Magic Constant)
pub fn execute(instruction: u32, cpu: &mut CpuState) -> ExecutionResult {
    let rd = ((instruction >> 7) & 0x1F) as usize;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let rs2 = ((instruction >> 20) & 0x1F) as usize;

    let val1 = cpu.read_reg(rs1);
    let val2 = cpu.read_reg(rs2);

    let result = val1.wrapping_add(val2).wrapping_add(0x42);
    cpu.write_reg(rd, result);

    println!(
        "✨ [gooney-example] GOONEY_ADD executed! x{} = 0x{:X} + 0x{:X} + 0x42 = 0x{:X}",
        rd, val1, val2, result
    );

    cpu.pc += 4;
    ExecutionResult::Ok
}
