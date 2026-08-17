use crate::cpu::CpuState;
use crate::decoder::ExecutionResult;

pub fn execute_custom(opcode: u32, instruction: u32, cpu: &mut CpuState) -> ExecutionResult {
    let slot_index = match opcode {
        0x0B => 0,
        0x2B => 1,
        0x5B => 2,
        0x7B => 3,
        _ => return ExecutionResult::Trap(format!("Unknown custom opcode funnel route: 0x{:X}", opcode)),
    };

    if let Some(handler) = cpu.custom_handlers[slot_index] {
        handler(instruction, cpu)
    } else {
        println!("⚠️ [custom-{}] Unpopulated slot, skipping instruction 0x{:08X}", slot_index, instruction);
        cpu.pc += 4;
        ExecutionResult::Ok
    }
}
