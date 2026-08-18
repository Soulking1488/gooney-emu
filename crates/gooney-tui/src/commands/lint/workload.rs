use std::fs;
use std::path::Path;

pub fn check(ext_path: &Path) {
    let test_dir = ext_path.join("test");
    if !test_dir.exists() {
        println!("   ℹ️ [Workload] No 'test/' directory found in extension.");
        return;
    }

    println!("   🧪 [Workload] Inspecting test directory: {:?}", test_dir);

    let mut has_source_generator = false;
    let mut has_binary_workload = false;

    if let Ok(entries) = fs::read_dir(&test_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let file_name_cow = path.file_name().unwrap_or_default().to_string_lossy();
                let file_name = file_name_cow.as_ref();
                
                if file_name.ends_with(".py") || file_name.ends_with(".S") || file_name.ends_with(".rs") {
                    println!("      - Found source/script asset: '{}'", file_name);
                    if file_name == "gen_test.py" || file_name.ends_with(".S") {
                        has_source_generator = true;
                    }
                    continue;
                }

                has_binary_workload = true;
                inspect_binary(&path, file_name);
            }
        }
    }

    // Golden Model Architectural Insight
    if has_source_generator && !has_binary_workload {
        println!("\n   💡 [Golden Model Diagnosis]:");
        println!("      Found test source or generation scripts, but **no compiled binary workload** was found in test/.");
        println!("      -> Action: Run your generator script (e.g., python3 extensions/.../gen_test.py) or build your assembly file before executing 'gooney run'.");
    }
}

fn inspect_binary(path: &Path, label: &str) {
    if let Ok(metadata) = fs::metadata(path) {
        let size = metadata.len();
        println!("         - Inspecting binary [{}]: {} bytes", label, size);

        if size == 0 {
            println!("            ❌ Error: Binary is empty!");
        } else if size % 4 != 0 {
            println!("            ⚠️ Warning: Size ({}) is not 4-byte aligned.", size);
        } else {
            println!("            ✅ Alignment check passed.");
        }

        if let Ok(bytes) = fs::read(path) {
            if bytes.len() >= 4 {
               let first_inst = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
               let opcode = first_inst & 0x7F;
               println!("            ℹ️ First instruction word: 0x{:08X} (Opcode: 0x{:02X})", first_inst, opcode);
               
               if opcode == 0x69 {
                   println!("            ❌ FOUND IT: Opcode 0x69! The binary starts with ASCII 'i' (0x69) or corrupt data instead of a valid RISC-V instruction.");
               }
            }
        }
    }
}
