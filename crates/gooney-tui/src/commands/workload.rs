use std::fs;
use std::path::Path;

pub fn check(ext_path: &Path) {
    // Look for test or workloads directory inside the extension
    let test_dir = ext_path.join("test");
    if !test_dir.exists() {
        println!("   ℹ️ [Workload] No 'test/' directory found in extension.");
        return;
    }

    println!("   🧪 [Workload] Inspecting test directory: {:?}", test_dir);

    if let Ok(entries) = fs::read_dir(test_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                
                // Inspect binary size and magic/structure
                if let Ok(metadata) = fs::metadata(&path) {
                    let size = metadata.len();
                    println!("      - Found test asset: '{}' ({} bytes)", file_name, size);

                    if size == 0 {
                        println!("         ❌ [Workload] Error: Test binary is empty!");
                    } else if size % 4 != 0 {
                        println!("         ⚠️ [Workload] Warning: Binary size ({}) is not 4-byte aligned (RISC-V instructions must be 32-bit aligned).", size);
                    } else {
                        println!("         ✅ [Workload] Binary alignment check passed.");
                    }
                }
            }
        }
    }
}
