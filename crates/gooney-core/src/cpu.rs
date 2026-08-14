pub struct CpuState {
    pub regs: [u64; 32],
    pub pc: u64,
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            regs: [0; 32],
            pc: 0x80000000, // Standard RISC-V start address for bare-metal binaries
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
