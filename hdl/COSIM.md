This guide outlines how to configure, run, and interact with the active co-simulation harness (`gooney-tui cosim`) to verify your RV64I Veryl/SystemVerilog hardware core against the software execution oracle (`gooney-emu`).

## 1. Overview of Co-Simulation
The co-simulation engine runs your compiled Verilator RTL model in lockstep with the `gooney-emu` software model. At every instruction retirement boundary, the harness compares:
- **Register State & Writeback Value:** Matches `wb_result_out` against the software oracle's architectural register commits.
- **Pipeline Progression:** Ensures hazards, stalls, and flushing behave identically between hardware and the golden model.

---

## 2. Prerequisites
Before running the co-simulation:
1. Ensure your HDL filelist is properly set up in `hdl/` (e.g., `hdl/abc.f`).
2. Build or place your target test binary/firmware (e.g., `tests/firmware.bin`).

---

## 3. How to Run the Co-Simulation Command
You can execute the co-simulation harness using the `gooney-tui` binary:

```bash
cargo run --bin gooney-tui -- cosim
