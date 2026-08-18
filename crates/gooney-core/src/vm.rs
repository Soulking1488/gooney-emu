use crate::cpu::{CpuState, CustomHandler};
use crate::memory::Memory;
use crate::decoder::{Decoder, ExecutionResult};
use std::path::Path;

pub struct VirtualMachine {
    pub cpu: CpuState,
    pub memory: Memory,
    _library_handles: Vec<libloading::Library>,
}

impl VirtualMachine {
    pub fn new(mem_size: usize) -> Self {
        let mut cpu = CpuState::new();
        let mut library_handles = Vec::new();

        // Automatically scan plugin slots (0 to 3) for connected extensions and compiled artifacts
        for slot in 0..4 {
            let marker_path = format!("extensions/custom-{}/.connected", slot);
            if Path::new(&marker_path).exists() {
                let lib_path = format!("crates/gooney-core/plugins/slot_{}.so", slot);
                if Path::new(&lib_path).exists() {
                    unsafe {
                        match libloading::Library::new(&lib_path) {
                            Ok(lib) => {
                                match lib.get::<libloading::Symbol<CustomHandler>>(b"execute") {
                                    Ok(func) => {
                                        let raw_fn: CustomHandler = **func;
                                        cpu.custom_handlers[slot] = Some(raw_fn);
                                        library_handles.push(lib);
                                        println!("🔌 [Core] Dynamically loaded plugin for slot custom-{} from {}", slot, lib_path);
                                    }
                                    Err(e) => println!("⚠️ Failed to find 'execute' symbol in {}: {}", lib_path, e),
                                }
                            }
                            Err(e) => println!("⚠️ Failed to load library {}: {}", lib_path, e),
                        }
                    }
                }
            }
        }

        Self {
            cpu,
            memory: Memory::new(mem_size, 0x80000000),
            _library_handles: library_handles,
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
                    println!("⚠ Execution Trapped at PC 0x{:016X}: {}", self.cpu.pc - 4, reason);
                    return Ok(());
                }
                ExecutionResult::Halt => {
                    println!("🛑 Execution Halted.");
                    return Ok(());
                }
            }
        }
        println!("⏱ Reached max cycles limit ({})", max_cycles);
        Ok(())
    }
}
