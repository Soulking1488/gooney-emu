##How Gooney Helps You Learn the RISC-V ISA
Learning an Instruction Set Architecture (ISA) like RISC-V often involves wading through dense specifications or heavy, complex simulators like QEMU or Spike, where the code can be obscured by massive enterprise codebases and OS emulation layers.

Gooney is built specifically to bridge the gap between theory and implementation for learners through several key design choices:

#Transparent, Clean Rust Implementation:
RISC-V instructions are mapped directly into readable Rust match arms and bitwise operations. When you study a file like imm.rs, you see the exact math and bit-shifting required for sign-extension and immediate decoding without abstraction layers.

#Bare-Metal Simplicity: 
By executing raw flat binary workloads (.bin) starting straight from a boot address (0x80000000), learners deal exclusively with CPU instructions, registers, and memory—mirroring how a real processor boots up.

#Immediate Visibility: 
The interactive TUI and detailed CLI output provide instant feedback on every register modification, letting you verify instruction outcomes (ADDI, SLLI, LUI, etc.) step-by-step.

#Modular Incremental Design: 
Because the decoder is broken down into clean submodules (arithmetic, control, memory), learners can implement and test instructions category-by-category (e.g., building up from basic I-type logic to full 64-bit word operations).

#Below is the complete ARCHITECTURE.md file for the project, detailing its internal design, component relationships, and execution pipeline.

#Architecture & System Design (ARCHITECTURE.md)
This document outlines the internal architecture, design principles, and component layout of the Gooney RISC-V Emulator (gooney-emu).

#🏛️ Architectural Philosophy
Gooney is engineered around two core principles:

Modularity: Separation of concerns between the core instruction-set simulator logic (gooney-core) and the user presentation/runner layer (gooney-tui).

Explicitness: Instruction semantics and CPU state transitions are implemented transparently using strict typing and pattern matching, making it ideal for educational exploration and extension.

#🧩 High-Level System Structure
The repository is structured as a Cargo workspace containing two primary crates:

Plaintext
gooney-emu/
├── crates/
│   ├── gooney-core/          # Core instruction set simulator (ISS)
│   │   └── src/
│   │       ├── cpu.rs        # CPU state, 64-bit registers, and memory bus
│   │       ├── decoder/      # Instruction parsing & execution engine
│   │       │   ├── mod.rs    # Main opcode dispatcher
│   │       │   ├── arithmetic/ # Arithmetic, shifts, and upper immediates
│   │       │   ├── control.rs # Conditional branches and jumps (JAL, JALR)
│   │       │   └── memory_instr.rs # Loads and stores (LW, SW, etc.)
│   │       └── lib.rs        # Core crate root
│   │
    └── gooney-tui/           # CLI runner and Terminal User Interface
        └── src/              # Event loops, rendering, and application state
#⚙️ Core Components (gooney-core)
1. CPU State & Memory (cpu.rs)
Register File: Manages 32 general-purpose 64-bit registers (x0 through x31), where x0 is hardwired to zero.

#Program Counter (PC): Tracks the currently executing instruction address, typically initialized at the standard RISC-V base address (0x80000000).

#Memory Bus: Provides a contiguous vector-backed memory space representing physical RAM, handling read and write operations for raw binary instruction feeds and data storage.

#2. Instruction Decoder & Dispatcher (decoder/)
The decoder follows a pipelined fetch-decode-execute loop. When an instruction word (u32) is fetched from memory, it is parsed according to standard RISC-V instruction formats to extract:

- opcode (bits [6:0])
- rd, rs1, rs2 (destination and source registers)
- funct3 and funct7 (sub-operation specifiers)

#Modular Submodule Routing (decoder/arithmetic/)
To prevent monolithic code files, arithmetic instructions are partitioned into specialized submodules:

#imm.rs: 
Handles Immediate operations (0x13), including ADDI, SLTI, SLTIU, XORI, ORI, ANDI, and immediate shifts (SLLI, SRLI, SRAI).

#reg.rs: 
Handles Register-Register operations (0x33), such as ADD and SUB.

#upper.rs: 
Handles Upper Immediate instructions (0x37 for LUI, 0x17 for AUIPC).

#🔄 The Instruction Execution Lifecycle
Fetch: The emulator reads 4 bytes from memory at the current address stored in cpu.pc.

#Decode: 
The raw 32-bit instruction is unpacked to identify its type (R, I, S, B, U, J) and corresponding opcode.

#Execute:
Operand values are retrieved via cpu.read_reg(rs1) and cpu.read_reg(rs2).

Arithmetic or logical operations are performed using Rust's safe wrapping arithmetic (wrapping_add, wrapping_shl, etc.) to match hardware overflow behavior.

#Writeback & PC Update:

The result is written back to destination register rd via cpu.write_reg(rd, val).

The program counter is advanced (cpu.pc += 4), unless modified by a control flow instruction (JAL, JALR, or branch).

Trap Handling: If an unsupported instruction or an ECALL is encountered, an ExecutionResult::Trap halts the execution loop and dumps the terminal CPU state.
