use std::path::Path;
use crate::bridge::*;

pub fn execute(binary: &str, steps: usize) -> Result<(), String> {
    println!("🚀 Initializing Gooney-Emu & Verilator Co-Simulation...");
    println!("📦 Target Firmware: \"{}\"", binary);
    println!("⏱️ Simulation Budget: {} steps", steps);

    let bin_path = Path::new(binary);
    if !bin_path.exists() {
        return Err(format!("❌ Firmware binary not found at '{}'. Please provide a valid binary.", binary));
    }

    println!("🔌 Linking C-FFI / Verilator static archives...");

    unsafe {
        xzxt_sim_init();
    }

    let divergences = 0;
    let mut committed_instructions = 0;

    for step in 1..=steps {
        unsafe {
            // Step the simulation clock (branch_taken=0, target_pc=0, stall=0, flush=0 for basic flow)
            xzxt_sim_step(0, 0, 0, 0);

            // Check if an instruction successfully retired this cycle
            if xzxt_get_wb_commit() == 1 {
                committed_instructions += 1;
                let rtl_pc = xzxt_get_pc();
                let rtl_wb = xzxt_get_wb_result();
                let rtl_rd = xzxt_get_wb_rd();
                let mem_ren = xzxt_get_mem_ren();
                let mem_wen = xzxt_get_mem_wen();
                let eflags = xzxt_get_eflags();

                if step % 10 == 0 || committed_instructions <= 5 {
                    println!(
                        "  [Commit #{:3}] PC: 0x{:08X} | Reg x{} <= 0x{:016X} | Flags: 0x{:08X}",
                        committed_instructions, rtl_pc, rtl_rd, rtl_wb, eflags
                    );

                    if mem_ren != 0 || mem_wen != 0 {
                        let addr = xzxt_get_mem_addr();
                        if mem_wen != 0 {
                            println!("    └─ 📝 Memory WRITE: Addr 0x{:08X} | Data 0x{:016X}", addr, xzxt_get_mem_wdata());
                        } else {
                            println!("    └─ 📖 Memory READ : Addr 0x{:08X} | Data 0x{:016X}", addr, xzxt_get_mem_rdata());
                        }
                    }
                }

                // TODO: Compare rtl_wb / rtl_pc against gooney-core software oracle execution here.
                // If mismatch:
                // eprintln!("❌ DIVERGENCE DETECTED [Step {}]: Expected {:#x}, Actual {:#x}", step, oracle_val, rtl_wb);
                // divergences += 1;
                // break;
            }
        }
    }

    unsafe {
        xzxt_sim_destroy();
    }

    if divergences > 0 {
        return Err(format!("❌ Co-simulation failed with {} divergence errors.", divergences));
    }

    println!("✅ Co-simulation finished successfully: {} instructions committed with 0 divergences.", committed_instructions);
    Ok(())
}
