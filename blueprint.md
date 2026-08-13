# Gooney-emu: Blueprint & Specification

`Gooney-emu` is a high-performance, minimalist RV64I Instruction Set Simulator (ISS) written in Rust. It is engineered specifically for silicon IP development and hardware verification, serving as a co-simulation golden model and a interactive TUI-driven validator for custom Veryl HDL cores featuring the `custom-0` opcode extension.

---

## 1. Core Architecture & Project Layout

The repository is structured as a workspace, separating the core simulation engine (which can be compiled as a C-FFI static library for Verilator) from the interactive TUI binary frontend.

```text
gooney-emu/
├── Cargo.toml                # Workspace definition
├── crates/
│   ├── gooney-core/          # Core RV64I ISA, memory, decoder, and C-FFI bindings
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── cpu.rs        # Register file (x0-x31), 64-bit PC state
│   │       ├── memory.rs     # Flat memory model / ELF loader
│   │       ├── decoder.rs    # RV64I + custom-0 opcode dispatch
│   │       ├── custom.rs     # Modular extension handlers (e.g., xzxt flags)
│   │       ├── trace.rs      # Retirement trace logger
│   │       └── ffi.rs        # #[no_mangle] extern "C" bindings for Verilator
│   └── gooney-tui/           # Terminal UI frontend (ratatui + crossterm)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── ui.rs         # Layout rendering (registers, disassembly, memory)
│           └── app.rs        # State machine and event loop
└── tests/                    # Integration and compliance test scripts
2. Technical Specifications
A. Architectural Core & Execution Engine (gooney-core)
Registers: 32 general-purpose 64-bit registers (x0–x31), with x0 strictly hardwired to zero. 64-bit Program Counter (pc).

Loaders: Supports raw flat binary ingestion and minimal ELF loading to bypass bootloaders/OS kernels.

Execution Model: Deterministic single-stepping loop enabling cycle-accurate or instruction-accurate inspection.

B. Custom Opcode & Extension Architecture
Dispatcher: Dedicated decoder logic matching the RISC-V reserved major opcode space custom-0 (0x0B / 0b0001011).

Bitfield Extraction: Helper utilities to cleanly parse R-Type fields (rd, rs1, rs2, funct3, funct7).

Plugin Interface: Encapsulates custom instruction mechanics (such as mapping custom execution states) inside isolated Rust traits or modules.

C. Verification & Co-Simulation Support (libgooneymart_emu.a)
C-FFI Bindings: Exports a stable C interface using #[repr(C)] and #[no_mangle]:

C
extern "C" {
    void* emu_init(const char* rom_path);
    int emu_step(void* handle);
    uint64_t emu_get_reg(void* handle, int reg_idx);
    void emu_free(void* handle);
}
Trace Exporter: Emits structured, line-by-line retirement logs (PC, Opcode, Mnemonic, Target Reg, Mutated Value) designed for automated file diffing (diff emu_trace.log rtl_trace.log).

D. Exception & Diagnostics Handling
Trap Handler: Catches illegal instruction patterns, unmapped sub-opcodes, or out-of-bounds register accesses, triggering an immediate diagnostic dump.

State Dumper: Formatted CLI and TUI outputs displaying complete register and flag states pre- and post-instruction execution.

3. Development Roadmap
Phase 1: Foundation (RV64I Base)

Implement flat memory management and CPU struct (x0-x31, pc).

Implement base integer instruction decoders (RV64I: ADD, SUB, ADDI, LD, SD, JAL, JALR, branches, etc.).

Write basic unit tests against self-contained raw binaries.

Phase 2: Custom Opcode & Tracing

Wire up the custom-0 (0x0B) major opcode dispatcher.

Implement structured retirement logging to text files.

Establish the C-FFI layer (libgooneymart_emu.a) and test integration with a mock C++ wrapper.

Phase 3: TUI Frontend (gooney-tui)

Build the terminal interface using ratatui and crossterm.

Add views for live register monitoring, disassembly inspection, and stepping controls.

Phase 4: Co-Simulation & Verification

Hook libgooneymart_emu.a into your Verilator testbench.

Run automated diff tests comparing Gooney-emu traces against your Veryl HDL simulations.
