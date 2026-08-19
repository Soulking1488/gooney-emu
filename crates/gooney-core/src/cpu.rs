use crate::decoder::ExecutionResult;

pub type CustomHandler = fn(u32, &mut CpuState) -> ExecutionResult;

pub struct CpuState {
    pub regs: [u64; 32],
    pub pc: u64,
    pub custom_handlers: [Option<CustomHandler>; 4],
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            regs: [0; 32],
            pc: 0x80000000, // Standard RISC-V start address for bare-metal binaries
            custom_handlers: [None; 4],
        }
    }

    /// Read a register value, ensuring x0 always returns 0
    pub fn read_reg(&self, reg: usize) -> u64 {
        if reg == 0 {
            0
        } else {
            self.regs[reg]
        }
    }

    /// Write a register value, dropping writes to x0
    pub fn write_reg(&mut self, reg: usize, val: u64) {
        if reg != 0 {
            self.regs[reg] = val;
        }
    }

    /// Pretty print register file states (ideal for diagnostics and TUI)
    pub fn dump_registers(&self) {
        println!("=== CPU State ===");
        println!("PC: 0x{:016X}", self.pc);
        for i in (0..32).step_by(4) {
            println!(
                "x{:02}: 0x{:016X}  x{:02}: 0x{:016X}  x{:02}: 0x{:016X}  x{:02}: 0x{:016X}",
                i, self.regs[i],
                i+1, self.regs[i+1],
                i+2, self.regs[i+2],
                i+3, self.regs[i+3]
            );
        }
        println!("=================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x0_hardwiring_native() {
        let mut cpu = CpuState::new();
        cpu.write_reg(0, 0xDEADBEEFCAFEBABE);
        assert_eq!(cpu.read_reg(0), 0, "read_reg(0) must return 0");
        assert_eq!(cpu.regs[0], 0, "write_reg must drop writes to x0");
    }

    #[test]
    fn test_sign_extension_helpers() {
        let raw_byte: u8 = 0x80;
        let sign_extended_64: u64 = (raw_byte as i8) as i64 as u64;
        
        assert_eq!(
            sign_extended_64, 
            0xFFFFFFFFFFFFFF80,
            "Sign-extension for LB failed!"
        );
    }

    #[test]
    fn test_divergence_reporter_formatting() {
        let cycle = 142;
        let reg_idx = 5;
        let expected_val: u64 = 42;
        let actual_val: u64 = 44;

        let divergence_msg = format!(
            "❌ DIVERGENCE DETECTED [Cycle {}]: Reg x{} mismatch. Expected (Oracle): {:#x}, Actual (RTL): {:#x}",
            cycle, reg_idx, expected_val, actual_val
        );

        assert!(divergence_msg.contains("Cycle 142"));
        assert!(divergence_msg.contains("Reg x5"));
        assert!(divergence_msg.contains("0x2a"));
    }
}
