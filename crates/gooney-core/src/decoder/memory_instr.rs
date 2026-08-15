use crate::cpu::CpuState;
use crate::memory::Memory;
use super::ExecutionResult;

pub fn execute(
    opcode: u32,
    instruction: u32,
    _rd: usize,
    rs1: usize,
    rs2: usize,
    funct3: u32,
    cpu: &mut CpuState,
    memory: &mut Memory,
) -> ExecutionResult {
    let rd = _rd;
    let base_addr = cpu.read_reg(rs1);

    // Extract immediate based on instruction format (I-type for loads, S-type for stores)
    let imm = match opcode {
        0x03 => {
            // I-type immediate: bits [31:20] sign-extended
            ((instruction as i32) >> 20) as i64
        }
        0x23 => {
            // S-type immediate: imm[11:5] at [31:25], imm[4:0] at [11:7]
            let imm_11_5 = (instruction >> 25) & 0x7F;
            let imm_4_0 = (instruction >> 7) & 0x1F;
            let combined = (imm_11_5 << 5) | imm_4_0;
            // Sign-extend from 12 bits to 32/64 bits
            (((combined as i32) << 20) >> 20) as i64
        }
        _ => 0,
    };

    let addr = base_addr.wrapping_add(imm as u64);

    match opcode {
        0x23 => {
            // Store instructions (SB, SH, SW, SD)
            let val = cpu.read_reg(rs2);
            let res = match funct3 {
                0x0 => memory.write_u8(addr, val as u8),
                0x1 => memory.write_u16(addr, val as u16),
                0x2 => memory.write_u32(addr, val as u32),
                0x3 => memory.write_u64(addr, val),
                _ => return ExecutionResult::Trap(format!("Unknown store funct3: 0x{:X}", funct3)),
            };

            match res {
                Ok(()) => ExecutionResult::Ok,
                Err(e) => ExecutionResult::Trap(e.to_string()),
            }
        }
        0x03 => {
            // Load instructions (LB, LH, LW, LD, LBU, LHU, LWU)
            let val = match funct3 {
                0x0 => {
                    match memory.read_u8(addr) {
                        Ok(v) => v as i8 as i64 as u64,
                        Err(e) => return ExecutionResult::Trap(e.to_string()),
                    }
                }
                0x1 => {
                    match memory.read_u16(addr) {
                        Ok(v) => v as i16 as i64 as u64,
                        Err(e) => return ExecutionResult::Trap(e.to_string()),
                    }
                }
                0x2 => {
                    match memory.read_u32(addr) {
                        Ok(v) => v as i32 as i64 as u64,
                        Err(e) => return ExecutionResult::Trap(e.to_string()),
                    }
                }
                0x3 => {
                    match memory.read_u64(addr) {
                        Ok(v) => v,
                        Err(e) => return ExecutionResult::Trap(e.to_string()),
                    }
                }
                0x4 => {
                    match memory.read_u8(addr) {
                        Ok(v) => v as u64,
                        Err(e) => return ExecutionResult::Trap(e.to_string()),
                    }
                }
                0x5 => {
                    match memory.read_u16(addr) {
                        Ok(v) => v as u64,
                        Err(e) => return ExecutionResult::Trap(e.to_string()),
                    }
                }
                0x6 => {
                    match memory.read_u32(addr) {
                        Ok(v) => v as u64,
                        Err(e) => return ExecutionResult::Trap(e.to_string()),
                    }
                }
                _ => return ExecutionResult::Trap(format!("Unknown load funct3: 0x{:X}", funct3)),
            };

            if rd != 0 {
                cpu.write_reg(rd, val);
            }
            ExecutionResult::Ok
        }
        _ => ExecutionResult::Trap(format!("Unknown memory opcode: 0x{:X}", opcode)),
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::CpuState;
    use crate::decoder::{Decoder, ExecutionResult};
    use crate::memory::Memory;

    #[test]
    fn test_lw_sw() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new(1024 * 1024, 0x80000000);

        cpu.write_reg(1, 0x80000000);
        cpu.write_reg(2, 0x12345678);

        let sw_instr: u32 = 0x0020A023;
        let res = Decoder::decode_and_execute(sw_instr, &mut cpu, &mut memory);
        assert_eq!(res, ExecutionResult::Ok);
        assert_eq!(memory.read_u32(0x80000000).unwrap(), 0x12345678);

        let lw_instr: u32 = 0x0000A183;
        let res = Decoder::decode_and_execute(lw_instr, &mut cpu, &mut memory);
        assert_eq!(res, ExecutionResult::Ok);
        assert_eq!(cpu.read_reg(3), 0x12345678);
    }

    #[test]
    fn test_ld_sd() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new(1024 * 1024, 0x80000000);

        cpu.write_reg(1, 0x80000100);
        cpu.write_reg(2, 0xDEADBEEFCAFEBABE);

        let sd_instr: u32 = 0x0020B023;
        let res = Decoder::decode_and_execute(sd_instr, &mut cpu, &mut memory);
        assert_eq!(res, ExecutionResult::Ok);
        assert_eq!(memory.read_u64(0x80000100).unwrap(), 0xDEADBEEFCAFEBABE);

        let ld_instr: u32 = 0x0000B203;
        let res = Decoder::decode_and_execute(ld_instr, &mut cpu, &mut memory);
        assert_eq!(res, ExecutionResult::Ok);
        assert_eq!(cpu.read_reg(4), 0xDEADBEEFCAFEBABE);
    }

    #[test]
    fn test_subword_access() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new(1024 * 1024, 0x80000000);

        cpu.write_reg(1, 0x80000200);
        cpu.write_reg(2, 0xFF);

        let sb_instr: u32 = 0x00208023;
        let _ = Decoder::decode_and_execute(sb_instr, &mut cpu, &mut memory);

        let lbu_instr: u32 = 0x0000C183;
        let _ = Decoder::decode_and_execute(lbu_instr, &mut cpu, &mut memory);
        assert_eq!(cpu.read_reg(3), 0xFF);

        let lb_instr: u32 = 0x00008283;
        let _ = Decoder::decode_and_execute(lb_instr, &mut cpu, &mut memory);
        assert_eq!(cpu.read_reg(5), 0xFFFFFFFFFFFFFFFF);
    }
}
