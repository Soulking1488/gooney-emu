
<div align="center">

# Gooney RISC-V Emulator (`gooney-emu`)

[![License: GPL v2](https://img.shields.io/badge/License-GPL_v2-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Architecture](https://img.shields.io/badge/ISA-RV64I-green.svg)](https://riscv.org/)

**A modular, lightweight RV64I RISC-V instruction-set simulator written in Rust.**  
*Designed for education, transparency, and hands-on pipeline debugging.*

</div>

A modular, lightweight RV64I RISC-V emulator written in Rust, featuring a terminal user interface (TUI) for interactive debugging and workload execution.

#🎯 Purpose & Overview
Gooney is designed as an educational and high-performance instruction-set simulator (ISS) targeting the RISC-V 64-bit Base Integer Instruction Set (RV64I). Its primary goal is to provide a clean, modular, and extensible architecture for emulating raw binary workloads, understanding CPU pipeline states, and debugging execution step-by-step through an integrated terminal interface.

#Phase 1 Focus
Phase 1 establishes the core emulation engine, including:

RV64I Decoder & Execution Core: Supporting arithmetic, logical shifts, upper immediates, control flow (JAL, JALR, branches), loads/stores, and environment traps (ECALL).

Modular Architecture: Clean separation between CPU state management, instruction decoding, and user interface rendering.

Workload Runner: Capability to load raw flat binary machine code (.bin) and trace register state changes to completion.

#📂 Project Directory Structure
Plaintext
gooney-emu/
├── Cargo.toml                # Workspace configuration
├── crates/
│   ├── gooney-core/          # Core emulator engine (CPU state, memory, decoder)
│   │   └── src/
│   │       ├── cpu.rs        # Register file and memory bus management
│   │       ├── decoder/      # Instruction decoding and opcode submodules
│   │       │   ├── arithmetic/# Modular arithmetic execution (imm, reg, upper)
│   │       │   ├── control.rs # Branches and jumps
│   │       │   └── memory_instr.rs # Loads and stores
│   │       └── ...
│   └── gooney-tui/           # Terminal User Interface & CLI runner
│       └── src/              # TUI views, event loops, and command handlers
└── workloads/                # Test binaries and assembly test suites (.bin)

#✨ Key Features
Modular Submodule Decoding: Instruction decoders are cleanly partitioned into dedicated modules (arithmetic, control, memory) for maintainability.

Full 64-Bit Register File: Implements all 32 RISC-V general-purpose registers (x0 through x31) with 64-bit word support.

TUI & CLI Integration: Built using Rust's robust ecosystem (clap, ratatui/crossterm) to support both automated batch execution and interactive visualization.

Extensible Test Suite: Flat binary workload execution enables direct verification against known RISC-V behavioral expectations.

#🚀 Building the Project
Ensure you have the Rust toolchain installed. Clone the repository and build the workspace in debug or release mode:

Bash
# Clone the repository
git clone https://github.com/soulking/gooney-emu.git
cd gooney-emu

# Build all workspace packages
cargo build --all
🕹️ Executing Workloads
Gooney includes a CLI/TUI runner to execute compiled raw binary machine code files placed in the workloads/ directory.

Running a Test Workload
To execute a specific workload binary (e.g., imm_logic_test.bin or shift_imm_test.bin):

Bash
cargo run --bin gooney-tui -- run workloads/imm_logic_test.bin
Execution Output
Upon completion or when an ECALL halts the program, Gooney outputs the final CPU state, showing all register values and the terminal program counter (PC):

Plaintext
▶️  Executing resolved file: workloads/imm_logic_test.bin
🚀 Starting execution from PC 0x80000000...
🛑 Execution Halted.
=== CPU State ===
PC: 0x000000008000001C
x00: 0x0000000000000000  x01: 0x000000000000000A  x02: 0x0000000000000001  x03: 0x0000000000000000
x04: 0x00000000000000F5  x05: 0x00000000000000FA  x06: 0x000000000000000A  x07: 0x0000000000000000
...
=================
📜 License
This project is open-source software licensed under the GNU General Public License v2.0 (GPLv2).

You may copy, distribute, and modify this software under the terms of the GPLv2 license. See the LICENSE file for more details.

Gooney-emu is developed by Gooneymart™. For corporate licensing, commercial embedding, or custom support inquiries, please email us at gooneymart@gmail.com.
