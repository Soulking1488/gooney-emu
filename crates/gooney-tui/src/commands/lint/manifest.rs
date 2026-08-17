use std::fs;
use std::path::Path;

pub fn check(ext_path: &Path) {
    let cargo_path = ext_path.join("Cargo.toml");
    if !cargo_path.exists() {
        println!("   ❌ [Manifest] Missing Cargo.toml!");
        return;
    }

    match fs::read_to_string(&cargo_path) {
        Ok(content) => {
            if content.contains("gooney-core") {
                println!("   ✅ [Manifest] Correctly links to 'gooney-core'.");
            } else {
                println!("   ⚠️ [Manifest] Warning: Cargo.toml does not reference 'gooney-core'.");
            }
        }
        Err(_) => println!("   ❌ [Manifest] Failed to read Cargo.toml."),
    }
}
