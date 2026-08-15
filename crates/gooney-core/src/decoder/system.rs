use crate::cpu::CpuState;
use crate::decoder::ExecutionResult;

pub fn execute(
    opcode: u32,
    instruction: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    match opcode {
        0x0F => {
            // FENCE and FENCE.I (NOPs for software emulation)
            cpu.pc += 4;
            ExecutionResult::Ok
        }
        0x73 => {
            let funct3 = (instruction >> 12) & 0x7;
            let rd = ((instruction >> 7) & 0x1F) as usize;
            let rs1 = ((instruction >> 15) & 0x1F) as usize;
            let csr = (instruction >> 20) & 0xFFF;

            // Handle ECALL / EBREAK
            if funct3 == 0 && rs1 == 0 && rd == 0 {
                if instruction == 0x00000073 {
                    return ExecutionResult::Trap("ECALL executed".to_string());
                } else if instruction == 0x00100073 {
                    return ExecutionResult::Trap("EBREAK executed".to_string());
                }
            }

            // Minimal Zicsr implementation for a golden testbench
            let csr_val: u64 = match csr {
                0xC00 | 0xC80 => cpu.pc, // cycle / cycleh
                0xC01 | 0xC81 => 0,      // time / timeh
                0xC02 | 0xC82 => 0,      // instret / instreth
                _ => 0,                  // Unknown CSRs return 0 to prevent faults
            };

            // Check if it's an immediate variant (funct3 bit 2 is set: 0x4, 0x5, 0x6, 0x7)
            let is_imm = (funct3 & 0x4) != 0;
            let src_val = if is_imm {
                rs1 as u64 // For immediate variants, the rs1 field holds the 5-bit zimm value
            } else {
                cpu.read_reg(rs1)
            };

            let write_val = match funct3 & 0x3 {
                1 => src_val,                  // CSRRW / CSRRWI (Write)
                2 => csr_val | src_val,        // CSRRS / CSRRSI (Set)
                3 => csr_val & !src_val,       // CSRRC / CSRRCI (Clear)
                _ => return ExecutionResult::Trap(format!("Unknown CSR funct3: 0x{:X}", funct3)),
            };

            // Write old CSR value to destination register if rd != x0
            if rd != 0 {
                cpu.write_reg(rd, csr_val);
            }

            let _ = write_val; // Silence unused warning for write value stub

            cpu.pc += 4;
            ExecutionResult::Ok
        }
        _ => ExecutionResult::Trap(format!("Unknown system opcode: 0x{:X}", opcode)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CpuState;
    use crate::decoder::ExecutionResult;

    #[test]
    fn test_fence() {
        let mut cpu = CpuState::new();
        let initial_pc = cpu.pc;
        let fence_instr = 0x0000000F;
        let res = execute(0x0F, fence_instr, &mut cpu);

        assert_eq!(res, ExecutionResult::Ok);
        assert_eq!(cpu.pc, initial_pc + 4);
    }

    #[test]
    fn test_ecall() {
        let mut cpu = CpuState::new();
        let ecall_instr = 0x00000073;
        let res = execute(0x73, ecall_instr, &mut cpu);

        assert_eq!(res, ExecutionResult::Trap("ECALL executed".to_string()));
    }
}
