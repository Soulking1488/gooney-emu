use std::fs;
use std::path::Path;
use std::process::Command;
use crate::commands::lint;

pub fn execute(slot: &str) {
    println!("🔗 Initiating secure connection for slot [ {} ]...", slot);

    // 1. Run Linter Pre-flight Check first
    lint::execute(slot);

    // 2. Validate Slot Index & Directory Structure
    let slot_num = match slot {
        "custom-0" => 0,
        "custom-1" => 1,
        "custom-2" => 2,
        "custom-3" => 3,
        _ => {
            println!("❌ Error: Unknown slot '{}'. Valid slots: custom-0, custom-1, custom-2, custom-3", slot);
            return;
        }
    };

    let slot_dir = format!("extensions/{}", slot);
    let slot_path = Path::new(&slot_dir);

    if !slot_path.exists() {
        println!("❌ Error: Slot directory '{}' does not exist.", slot_dir);
        return;
    }

    // 3. Locate Nested Cargo Project
    let ext_path = match find_cargo_project(slot_path) {
        Some(p) => p,
        None => {
            println!("❌ Error: No valid Cargo project found inside {}", slot_dir);
            return;
        }
    };

    // 4. Compile Extension as Shared Library (Standalone)
    if !compile_extension(&ext_path) {
        return;
    }

    // 5. Package and Copy Artifact to Core Plugins
    if !package_plugin(&ext_path, slot_num) {
        return;
    }

    // 6. Write Connection Marker
    let marker_path = slot_path.join(".connected");
    if let Err(e) = fs::write(&marker_path, "connected") {
        println!("⚠️ Warning: Failed to write connection marker: {}", e);
    }

    println!("✨ Successfully connected slot [ {} ]! Compiled artifact packaged to core plugins.", slot);
}

fn find_cargo_project(slot_path: &Path) -> Option<std::path::PathBuf> {
    if let Ok(entries) = fs::read_dir(slot_path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() && entry_path.join("Cargo.toml").exists() {
                return Some(entry_path);
            }
        }
    }
    None
}

fn compile_extension(ext_path: &Path) -> bool {
    println!("📦 Compiling extension package at {:?}...", ext_path);
    let status = Command::new("cargo")
    .args(["build", "--release"])
    .current_dir(ext_path)
    .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Extension build successful!");
            true
        }
        _ => {
            println!("❌ Error: Failed to compile extension package.");
            false
        }
    }
}

fn package_plugin(ext_path: &Path, slot_num: usize) -> bool {
    let mut release_dirs = vec![ext_path.join("target/release")];
    // Also check workspace root target/release if available
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        release_dirs.push(Path::new(&manifest_dir).join("../../target/release"));
    }
    // Fallback to searching relative workspace target/release
    release_dirs.push(Path::new("target/release").to_path_buf());

    let mut compiled_lib = None;

    for release_dir in release_dirs {
        if let Ok(entries) = fs::read_dir(&release_dir) {
            for entry in entries.flatten() {
                let filename = entry.file_name();
                let name_str = filename.to_string_lossy();
                if (name_str.contains("gooney") || name_str.contains("example"))
                    && (name_str.ends_with(".so") || name_str.ends_with(".dylib") || name_str.ends_with(".dll")) {
                        compiled_lib = Some(entry.path());
                        break;
                    }
            }
        }
        if compiled_lib.is_some() {
            break;
        }
    }

    let lib_src = match compiled_lib {
        Some(p) => p,
        None => {
            println!("❌ Error: Could not locate compiled shared library artifact in target/release/");
            return false;
        }
    };

    let plugins_dir = Path::new("crates/gooney-core/plugins");
    if !plugins_dir.exists() {
        let _ = fs::create_dir_all(plugins_dir);
    }

 let dest = plugins_dir.join(format!("slot_{}.so", slot_num));
    if let Err(e) = fs::copy(&lib_src, &dest) {
        println!("❌ Error copying plugin: {}", e);
        return false;
    }

    println!("✅ Plugin packaged successfully to {:?}", dest);
    true
}
