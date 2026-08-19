use std::fs;
use std::path::Path;

pub fn execute(hdl_path: &str) -> Result<(), String> {
    println!("🔍 Analyzing HDL workspace at path: \"{}\"...", hdl_path);

    let path = Path::new(hdl_path);
    if !path.exists() {
        return Err(format!("❌ HDL directory not found at '{}'. Please create it and add your Veryl files.", hdl_path));
    }

    let mut file_count = 0;
    let mut found_sources = false;
    let mut found_filelists = 0;

    fn visit_dirs(dir: &Path, file_count: &mut usize, found_sources: &mut bool, found_filelists: &mut usize) -> Result<(), String> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, file_count, found_sources, found_filelists)?;
                } else {
                    *file_count += 1;
                    if let Some(ext) = path.extension() {
                        if ext == "veryl" || ext == "sv" || ext == "v" {
                            *found_sources = true;
                            println!("  📂 Found HDL source: {}", path.display());
                        } else if ext == "f" {
                            *found_filelists += 1;
                            println!("  📜 Found HDL Filelist: {}", path.display());
                            
                            // Optional: Read the .f file content to see referenced files
                            if let Ok(content) = fs::read_to_string(&path) {
                                for line in content.lines() {
                                    let line = line.trim();
                                    if !line.is_empty() && !line.starts_with('#') {
                                        println!("    ↳ Filelist entry: {}", line);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    visit_dirs(path, &mut file_count, &mut found_sources, &mut found_filelists)?;

    println!("📊 Scanned {} total files in workspace.", file_count);
    
    if found_filelists > 0 || found_sources {
        println!("✅ HDL structure check passed: Filelist / source assets detected.");
        println!("ℹ [Contract Check] Ready for opcode range and writeback pipeline verification.");
    } else {
        println!("⚠ Warning: No HDL source files or filelists found in '{}'.", hdl_path);
    }

    Ok(())
}
