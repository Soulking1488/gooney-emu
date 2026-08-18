pub mod manifest;
pub mod source;
pub mod isa;
pub mod workload;

use std::path::{Path, PathBuf};
use std::fs;

pub fn execute(extension: &str) {
    if extension == "all" || extension.is_empty() {
        println!("🔍 Running Golden Model Linter on all extension slots (custom-0 to custom-3)...\n");
        for i in 0..4 {
            let slot_name = format!("custom-{}", i);
            lint_slot(&slot_name);
            println!("{}", "----------------------------------------".to_string());
        }
        return;
    }

    lint_slot(extension);
}

fn lint_slot(name: &str) {
    let slot_path = Path::new("extensions").join(name);
    println!("📦 Linter inspection for slot: [ {} ]", name);

    if !slot_path.exists() {
        println!("   ⚠️ Slot directory does not exist.");
        return;
    }

    let project_path = match find_extension_dir(&slot_path) {
        Some(p) => {
            if p != slot_path {
                println!("   📂 Found nested project directory: {:?}", p.file_name().unwrap_or_default());
            }
            p
        }
        None => {
            println!("   ❌ [Manifest] No valid Cargo.toml found in slot root or subfolders.");
            return;
        }
    };

    // Run modular checks including workload/test asset validation
    manifest::check(&project_path);
    source::check(&project_path);
    isa::check_compliance(&project_path, name);
    workload::check(&project_path);
}

fn find_extension_dir(slot_path: &Path) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(slot_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("Cargo.toml").exists() {
                return Some(path);
            }
        }
    }
    if slot_path.join("Cargo.toml").exists() {
        return Some(slot_path.to_path_buf());
    }
    None
}
