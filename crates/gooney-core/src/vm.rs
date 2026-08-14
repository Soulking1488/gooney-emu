use crate::cpu::CpuState;
use crate::memory::Memory;
use crate::decoder::{Decoder, ExecutionResult};

pub struct VirtualMachine {
    pub cpu: CpuState,
    pub memory: Memory,
}

impl VirtualMachine {
    pub fn new(mem_size: usize) -> Self {
        Self {
            cpu: CpuState::new(),
            memory: Memory::new(mem_size, 0x80000000),
        }
    }

    /// Load a raw binary into memory at the default start address (0x80000000)
    pub fn load_program(&mut self, bin: &[u8]) -> Result<(), &'static str> {
        self.memory.load_binary(0x80000000, bin)
    }

    /// Execute a single instruction cycle
    pub fn step(&mut self) -> Result<ExecutionResult, &'static str> {
        let pc = self.cpu.pc;
        
        // 1. Fetch 32-bit instruction from memory
        let instruction = self.memory.read_u32(pc)?;

        // 2. Decode and Execute
        let result = Decoder::decode_and_execute(instruction, &mut self.cpu, &mut self.memory);

        Ok(result)
    }

    /// Run until a trap, halt, or max cycles reached
    pub fn run(&mut self, max_cycles: usize) -> Result<(), &'static str> {
        for _ in 0..max_cycles {
            // Check if PC is out of bounds or halted
            match self.step()? {
                ExecutionResult::Ok => {}
                ExecutionResult::Trap(reason) => {
                    println!("⚠️ Execution Trapped at PC 0x{:016X}: {}", self.cpu.pc - 4, reason);
                    return Ok(());
                }
                ExecutionResult::Halt => {
                    println!("🛑 Execution Halted.");
                    return Ok(());
                }
            }
        }
        println!("⏱️ Reached max cycles limit ({})", max_cycles);
        Ok(())
    }
}
