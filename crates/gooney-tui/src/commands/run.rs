use gooneymart_emu::vm::VirtualMachine;
use std::fs;
use std::path::Path;

pub fn execute(path: &str) {
    let target_path = if Path::new(path).exists() {
        path.to_string()
    } else {
        let workload_path = format!("workloads/{}", path);
        if Path::new(&workload_path).exists() {
            workload_path
        } else {
            eprintln!("\n❌ ERROR: File not found!");
            eprintln!("   - Checked current dir: '{}'", path);
            eprintln!("   - Checked workloads dir: 'workloads/{}'\n", path);
            return;
        }
    };

    println!("▶️  Executing resolved file: {}", target_path);

    let bin_data = match fs::read(&target_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("❌ Failed to read file '{}': {}", target_path, e);
            return;
        }
    };

    let mut vm = VirtualMachine::new(16 * 1024 * 1024);

    if let Err(e) = vm.load_program(&bin_data) {
        eprintln!("❌ Failed to load program into memory: {}", e);
        return;
    }

    println!("🚀 Starting execution from PC 0x80000000...");
    if let Err(e) = vm.run(10000) {
        eprintln!("🔥 VM Runtime Error: {}", e);
        return;
    }

    vm.cpu.dump_registers();
}
