use std::fs;
use std::path::Path;

pub fn check(ext_path: &Path) {
    let lib_path = ext_path.join("src").join("lib.rs");
    if !lib_path.exists() {
        println!("   ❌ [Source] Missing src/lib.rs target!");
        return;
    }

    match fs::read_to_string(&lib_path) {
        Ok(content) => {
            if content.contains("unimplemented!") || content.contains("todo!") || content.contains("Default") {
                println!("   ℹ️ [Source] Status: **Stub / Unimplemented** template.");
            } else {
                println!("   ✨ [Source] Status: **Active Implementation** detected.");
            }
        }
        Err(_) => println!("   ❌ [Source] Failed to read src/lib.rs."),
    }
}
