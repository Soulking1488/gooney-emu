use std::fs;
use std::path::Path;

pub fn execute() {
    println!("🔄 Resetting emulator core to vanilla state (disconnecting all custom extensions)...");

    let ext_root = Path::new("extensions");
    if ext_root.exists() {
        if let Ok(entries) = fs::read_dir(ext_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let marker = path.join(".connected");
                    if marker.exists() {
                        let _ = fs::remove_file(&marker);
                    }
                }
            }
        }
    }

    // Clean up compiled plugin artifacts in core
    let plugins_dir = Path::new("crates/gooney-core/plugins");
    if plugins_dir.exists() {
        if let Ok(entries) = fs::read_dir(plugins_dir) {
            for entry in entries.flatten() {
                let _ = fs::remove_file(entry.path());
            }
        }
    }

    println!("✨ Emulator core successfully reverted to vanilla state! All custom extensions are unlinked and plugins cleaned.");
}
