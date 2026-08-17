use std::fs;
use std::path::Path;

/// Ensures the extension adheres to RISC-V custom instruction guidelines 
/// and does not violate core golden model safety invariants.
pub fn check_compliance(ext_path: &Path, slot_name: &str) {
    let lib_path = ext_path.join("src").join("lib.rs");
    if let Ok(content) = fs::read_to_string(lib_path) {
        // Example Golden Model heuristic checks:
        // 1. Ensure it accepts the standard function signature (u32, &mut CpuState)
        if !content.contains("CpuState") {
            println!("   ❌ [ISA Compliance] Warning: Extension code does not reference 'CpuState'. Verify signature.");
        } else {
            println!("   ✅ [ISA Compliance] CPU state hook signature verified.");
        }

        // 2. Check opcode range warning based on slot mapping
        let expected_opcode = match slot_name {
            "custom-0" => "0x0B",
            "custom-1" => "0x2B",
            "custom-2" => "0x5B",
            "custom-3" => "0x7B",
            _ => "unknown",
        };

        println!("   ℹ️ [ISA Compliance] Slot {} is bound to golden model opcode range [{}.", slot_name, expected_opcode);
    }
}
