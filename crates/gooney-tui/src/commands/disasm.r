use std::fs;
use gooney_core::decoder; // Assuming your decoder can parse instructions

pub fn execute(path: &str) {
    println!("📜 Disassembling binary: {}", path);
    
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("❌ Failed to read file: {}", e);
            return;
        }
    };

    // Iterate through the binary 4 bytes at a time (standard 32-bit RISC-V instructions)
    for (i, chunk) in bytes.chunks(4).enumerate() {
        if chunk.len() < 4 {
            break;
        }
        let instruction = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let pc = (i * 4) as u64;

        // Basic instruction printing (you can expand this with a full disassembly formatter)
        println!("  [0x{:08X}] 0x{:08X}", pc, instruction);
    }
}
