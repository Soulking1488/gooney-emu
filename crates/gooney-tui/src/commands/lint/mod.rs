pub mod manifest;
pub mod source;
pub mod isa;

use std::path::Path;

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
    let ext_path = Path::new("extensions").join(name);
    println!("📦 Linter inspection for slot: [ {} ]", name);

    if !ext_path.exists() {
        println!("   ⚠️ Slot directory does not exist.");
        return;
    }

    // Run modular checks
    manifest::check(&ext_path);
    source::check(&ext_path);
    isa::check_compliance(&ext_path, name);
}
