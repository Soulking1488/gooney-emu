# Co-Simulation Verification Guide

This guide outlines how to configure, run, and interact with the active co-simulation harness (`gooney-tui cosim`) to verify your RV64I Veryl/SystemVerilog hardware core against the software execution oracle (`gooney-emu`).

## 1. Overview of Co-Simulation
The co-simulation engine runs your compiled Verilator RTL model in lockstep with the `gooney-emu` software model. At every instruction retirement boundary (`wb_commit`), the harness performs real-time telemetry and state verification:

- **Architectural State Tracking:** Synchronizes PC, Register File (x0-x31), and ALU Writeback results.
- **Flag Verification:** Validates internal CPU Condition Codes / EFlags against the software oracle.
- **Memory Bus Monitoring:** Tracks Load/Store operations, including memory addresses, read/write strobes, and data payloads.
- **Pipeline Integrity:** Ensures hazards, stalls, and pipeline flushes in hardware do not desynchronize from the golden model.

---

## 2. Prerequisites
Before running the co-simulation:
1. Ensure your HDL filelist is properly set up in `hdl/` (e.g., `gooneymart_abc_hdl.f`).
2. Build your hardware static archives: run `make sim_bridge` inside your HDL project directory to populate `sim/obj_dir/`.
3. Build or place your target test binary/firmware (e.g., `tests/firmware.bin`) in the project root.

---

## 3. How to Run the Co-Simulation
You can execute the co-simulation harness using the `gooney-tui` binary. Use the `--steps` flag to define your simulation budget:

```bash
# Run simulation for 50 cycles
cargo run --bin gooney-tui -- cosim --steps 50

## Telemetry Output
The CLI provides real-time commit telemetry:
- [Commit #N]: Tracks instruction progression.
- Reg x[n] <= 0x[VAL]: Validates writeback data.
- Flags: 0x[VAL]: Monitors architectural condition codes.
- Memory READ/WRITE: Displays memory bus activity (Addr/Data) when a load or store instruction is committed.

If the hardware state deviates from the gooney-emu software oracle, the harness will report a DIVERGENCE DETECTED error and halt to assist with pipeline debugging.


