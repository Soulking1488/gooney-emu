use gooneymart_emu::cpu::CpuState;
use std::io::{self, Write};

pub fn execute() {
    println!("🛠️ Launching Gooney Interactive Shell...");
    println!("Type 'help' for a list of commands, or 'exit' to quit.\n");

    let mut cpu = CpuState::new();
    let stdin = io::stdin();

    loop {
        print!("gooney> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            println!("❌ Error reading input");
            break;
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmd = parts[0];

        match cmd {
            "exit" | "quit" => {
                println!("Exiting shell. Goodbye!");
                break;
            }
            "help" => {
                println!("Available commands:");
                println!("  regs             - Print all CPU general-purpose registers and PC");
                println!("  exec <hex>       - Execute a 32-bit hex instruction (e.g., exec 0x00000073)");
                println!("  reset            - Reset CPU state (PC and registers)");
                println!("  exit / quit      - Exit the interactive shell");
            }
            "regs" => {
                print_registers(&cpu);
            }
            "reset" => {
                cpu = CpuState::new();
                println!("✨ CPU state reset.");
            }
            "exec" => {
                if parts.len() < 2 {
                    println!("❌ Usage: exec <32-bit hex instruction>");
                    continue;
                }

                let hex_str = parts[1].strip_prefix("0x").unwrap_or(parts[1]);
                let instruction = match u32::from_str_radix(hex_str, 16) {
                    Ok(val) => val,
                    Err(_) => {
                        println!("❌ Invalid hex instruction: {}", parts[1]);
                        continue;
                    }
                };

                let opcode = instruction & 0x7F;
                
                let result = match opcode {
                    0x0F | 0x73 => gooneymart_emu::decoder::system::execute(opcode, instruction, &mut cpu),
                    _ => gooneymart_emu::decoder::ExecutionResult::Trap(format!("Unsupported opcode in shell: 0x{:X}", opcode)),
                };

                println!("⚙️ Executed instruction: 0x{:08X} (opcode: 0x{:X})", instruction, opcode);
                match result {
                    gooneymart_emu::decoder::ExecutionResult::Ok => {
                        println!("✅ Success. Next PC: 0x{:0X}", cpu.pc);
                    }
                    gooneymart_emu::decoder::ExecutionResult::Trap(ref msg) => {
                        println!("⚠️ Trap / Halt triggered: {}", msg);
                    }
                    gooneymart_emu::decoder::ExecutionResult::Halt => {
                        println!("🛑 Execution halted.");
                    }
                }
            }
            _ => {
                println!("❌ Unknown command: '{}'. Type 'help' for options.", cmd);
            }
        }
    }
}

fn print_registers(cpu: &CpuState) {
    println!("--------------------------------------------------");
    println!("PC: 0x{:016X}", cpu.pc);
    for i in (0..32).step_by(4) {
        println!(
            "x{:<2}: 0x{:016X}  |  x{:<2}: 0x{:016X}  |  x{:<2}: 0x{:016X}  |  x{:<2}: 0x{:016X}",
            i, cpu.read_reg(i),
            i + 1, cpu.read_reg(i + 1),
            i + 2, cpu.read_reg(i + 2),
            i + 3, cpu.read_reg(i + 3)
        );
    }
    println!("--------------------------------------------------");
}
