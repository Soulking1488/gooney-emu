# Gooney Emu Custom Extensions

Gooney Emu provides a dedicated interface for experimenting with **RISC-V custom instructions, coprocessors, accelerators, and experimental ISA extensions** without requiring modifications to the core emulator.

Up to four custom extension slots are available:

| Slot       | RISC-V Opcode | Extension Directory    |
| ---------- | ------------: | ---------------------- |
| `CUSTOM-0` |        `0x0B` | `extensions/custom-0/` |
| `CUSTOM-1` |        `0x2B` | `extensions/custom-1/` |
| `CUSTOM-2` |        `0x5B` | `extensions/custom-2/` |
| `CUSTOM-3` |        `0x7B` | `extensions/custom-3/` |

Each slot can provide its own instruction decoder/execution handler.

---

## 1. Overview

A Gooney Emu custom extension consists of a Rust library implementing an execution handler:

```rust
fn execute(
    instruction: u32,
    cpu: &mut CpuState,
) -> ExecutionResult
```

The handler receives:

* The **raw 32-bit RISC-V instruction**.
* A mutable reference to the emulator's **CPU state**.

The extension can then:

* Decode custom instruction fields.
* Read and write integer registers.
* Access the program counter.
* Perform arithmetic or logical operations.
* Implement application-specific instructions.
* Emulate custom coprocessors or accelerators.
* Return an execution status to the emulator.

The core emulator does not need to be modified for the instruction implementation itself.

---

# 2. Custom Opcode Mapping

Gooney Emu reserves four RISC-V custom opcode spaces for extensions.

```text
31                         7 6      0
+---------------------------+--------+
|     Custom Instruction    | Opcode |
+---------------------------+--------+

CUSTOM-0 = 0x0B
CUSTOM-1 = 0x2B
CUSTOM-2 = 0x5B
CUSTOM-3 = 0x7B
```

The opcode determines which extension slot receives the instruction.

### Opcode Table

| Opcode | Slot       | Purpose                          |
| ------ | ---------- | -------------------------------- |
| `0x0B` | `CUSTOM-0` | General-purpose custom extension |
| `0x2B` | `CUSTOM-1` | General-purpose custom extension |
| `0x5B` | `CUSTOM-2` | General-purpose custom extension |
| `0x7B` | `CUSTOM-3` | General-purpose custom extension |

These correspond to the standard RISC-V custom opcode regions and are intended for implementation-specific instructions.

---

# 3. Directory Structure

The repository provides four extension slots and a reference implementation:

```text
gooney-emu/
├── crates/
│   ├── gooney-core/
│   └── gooney-tui/
│
├── extensions/
│   ├── sample/
│   │   └── ...
│   │
│   ├── custom-0/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── custom-1/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── custom-2/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   │
│   └── custom-3/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
│
└── ...
```

## 3.1 `sample/`

The `extensions/sample/` directory contains a reference implementation demonstrating the expected extension structure.

It is intended as a starting point for extension developers and is not necessarily connected to one of the four custom opcode slots.

## 3.2 `custom-X/`

Each `custom-X` directory represents one available extension slot.

For example:

```text
extensions/custom-0/
```

corresponds to:

```text
Opcode: 0x0B
```

while:

```text
extensions/custom-3/
```

corresponds to:

```text
Opcode: 0x7B
```

---

# 4. Git and Extension Workspace Layout

The base extension projects are intentionally kept in the main repository so that Gooney Emu can compile with all extension slots present.

The tracked files are:

```text
extensions/custom-X/Cargo.toml
extensions/custom-X/src/lib.rs
```

Developers may create a separate project inside an extension directory.

For example:

```text
extensions/
└── custom-0/
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs
    │
    └── my_crypto_coprocessor/
        ├── Cargo.toml
        └── src/
            └── lib.rs
```

Nested user workspaces are ignored by Git.

This makes it possible to experiment locally or maintain a larger extension project without adding the entire project to the Gooney Emu repository.

---

# 5. Creating a Custom Extension

There are two supported approaches.

## Option A — Modify the Existing Stub

For small extensions, the existing:

```text
extensions/custom-X/src/lib.rs
```

can be modified directly.

This is the simplest approach for experimentation.

## Option B — Create a Nested Extension Project

For larger extensions, create a separate workspace under the selected slot:

```text
extensions/custom-0/my_extension/
```

This approach is recommended when the extension contains multiple modules, tests, examples, or additional dependencies.

---

# 6. Cargo Configuration

A custom extension must depend on `gooney-core`.

A minimal extension manifest looks like this:

```toml
[package]
name = "custom-0"
version = "0.1.0"
edition = "2021"

[lib]
name = "custom_0"
path = "src/lib.rs"

[dependencies]
gooney-core = { path = "../../crates/gooney-core" }
```

For a nested project, adjust the relative path according to its location.

For example:

```text
extensions/custom-0/my_extension/
```

may require:

```toml
gooney-core = { path = "../../../crates/gooney-core" }
```

The important requirement is that the extension resolves the same `gooney-core` used by Gooney Emu.

---

# 7. Extension Execution API

Every custom extension must expose an execution function with the following signature:

```rust
pub fn execute(
    instruction: u32,
    cpu: &mut CpuState,
) -> ExecutionResult
```

Import the required types:

```rust
use gooney_core::cpu::CpuState;
use gooney_core::decoder::ExecutionResult;
```

The function receives the complete raw instruction:

```rust
instruction: u32
```

and mutable CPU state:

```rust
cpu: &mut CpuState
```

This allows the extension to inspect the instruction encoding and manipulate CPU state.

---

# 8. Decoding a Custom Instruction

RISC-V instructions contain several standard fields that can be extracted directly from the raw instruction.

For example:

```rust
let rd = ((instruction >> 7) & 0x1F) as usize;
let rs1 = ((instruction >> 15) & 0x1F) as usize;
let rs2 = ((instruction >> 20) & 0x1F) as usize;
let funct3 = ((instruction >> 12) & 0x07) as u32;
let funct7 = ((instruction >> 25) & 0x7F) as u32;
```

These fields can be used to define your own instruction encoding.

For example:

```text
31        25 24    20 19    15 14  12 11     7 6      0
+-----------+--------+--------+------+--------+--------+
|  funct7   |  rs2   |  rs1   |funct3|   rd   | custom |
+-----------+--------+--------+------+--------+--------+
```

The exact encoding is entirely up to the extension designer, provided it remains within the selected custom opcode space.

---

# 9. Reading and Writing Registers

The CPU state provides register access through its register API.

Read a register:

```rust
let value = cpu.read_reg(rs1);
```

Write a register:

```rust
cpu.write_reg(rd, value);
```

For example:

```rust
let rs1 = ((instruction >> 15) & 0x1F) as usize;
let rd = ((instruction >> 7) & 0x1F) as usize;

let value = cpu.read_reg(rs1);

cpu.write_reg(
    rd,
    value.wrapping_add(42),
);
```

Extensions should use the CPU register access methods rather than directly modifying the underlying register storage.

---

# 10. Program Counter Handling

A custom instruction occupies one 32-bit instruction word.

Therefore, an instruction that completes normally must advance the program counter by four bytes:

```rust
cpu.pc += 4;
```

A simple handler therefore follows this pattern:

```rust
pub fn execute(
    instruction: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    // Decode instruction
    // Perform operation
    // Write results

    cpu.pc += 4;

    ExecutionResult::Ok
}
```

If an extension implements control-flow behavior, such as a custom jump or branch, it may update the program counter according to the semantics of that instruction instead.

The extension is responsible for ensuring that its PC behavior matches the instruction it implements.

---

# 11. Returning Execution Results

The extension must return an `ExecutionResult`.

For a successfully executed instruction:

```rust
ExecutionResult::Ok
```

Example:

```rust
pub fn execute(
    instruction: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    let rd = ((instruction >> 7) & 0x1F) as usize;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;

    let value = cpu.read_reg(rs1);
    cpu.write_reg(rd, value.wrapping_add(42));

    cpu.pc += 4;

    ExecutionResult::Ok
}
```

Use the execution-result variants provided by the version of `gooney-core` used by the project. Do not create a separate result type inside the extension.

---

# 12. Complete Example

The following example implements a simple custom instruction:

```text
CUSTOM ADD42

rd = rs1 + 42
```

The instruction uses the following fields:

```text
rd  = bits 11:7
rs1 = bits 19:15
```

Implementation:

```rust
use gooney_core::cpu::CpuState;
use gooney_core::decoder::ExecutionResult;

pub fn execute(
    instruction: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let rd = ((instruction >> 7) & 0x1F) as usize;

    let value = cpu.read_reg(rs1);
    let result = value.wrapping_add(42);

    cpu.write_reg(rd, result);

    cpu.pc += 4;

    ExecutionResult::Ok
}
```

If `x5` contains:

```text
100
```

and the instruction specifies:

```text
rs1 = x5
rd  = x6
```

the result will be:

```text
x6 = 142
```

---

# 13. Registering an Extension

Implementing the extension is only half of the process.

The handler must also be registered with the CPU.

For example, to register `CUSTOM-0`:

```rust
let mut cpu = gooney_core::cpu::CpuState::new();

cpu.custom_handlers[0] = Some(custom_0::execute);
```

The slot index corresponds to the custom opcode:

```text
custom_handlers[0] → CUSTOM-0 → 0x0B
custom_handlers[1] → CUSTOM-1 → 0x2B
custom_handlers[2] → CUSTOM-2 → 0x5B
custom_handlers[3] → CUSTOM-3 → 0x7B
```

### Example

```rust
let mut cpu = gooney_core::cpu::CpuState::new();

cpu.custom_handlers[0] = Some(custom_0::execute);
cpu.custom_handlers[1] = Some(custom_1::execute);
```

Only the slots that are actually implemented need to be registered.

---

# 14. Extension Registration Architecture

The recommended architecture is:

```text
                    +----------------------+
                    |     RISC-V CPU       |
                    +----------+-----------+
                               |
                               v
                    +----------------------+
                    |      Instruction      |
                    |       Decoder        |
                    +----------+-----------+
                               |
                +--------------+--------------+
                |              |              |
             0x0B           0x2B           0x5B ... 0x7B
                |              |              |
                v              v              v
           CUSTOM-0       CUSTOM-1       CUSTOM-2/3
                |              |              |
                v              v              v
           execute()       execute()       execute()
```

The decoder selects the appropriate slot based on the instruction opcode.

The extension itself should therefore focus on instruction semantics rather than modifying the central decoder.

---

# 15. Keeping Extensions Independent

A custom extension should avoid modifying files under:

```text
crates/gooney-core/
```

unless the extension requires a new core API that does not currently exist.

Prefer implementing extension-specific behavior inside:

```text
extensions/custom-X/
```

This keeps the extension isolated and makes it easier to:

* Remove the extension.
* Replace the implementation.
* Maintain multiple experimental extensions.
* Move the extension to another repository.
* Test different implementations against the same emulator.

---

# 16. Designing an Instruction Encoding

Before implementing an extension, define its instruction format.

For example:

```text
31        25 24    20 19    15 14  12 11     7 6      0
+-----------+--------+--------+------+--------+--------+
|  funct7   |  rs2   |  rs1   |funct3|   rd   | 0x0B   |
+-----------+--------+--------+------+--------+--------+
```

You can then assign meanings to the remaining fields.

For example:

```text
funct7 = 0x01 → ADD42
funct7 = 0x02 → MUL
funct7 = 0x03 → CRC32
funct7 = 0x04 → CUSTOM_AES
```

The extension handler can dispatch based on these fields:

```rust
let funct7 = (instruction >> 25) & 0x7F;

match funct7 {
    0x01 => {
        // ADD42
    }

    0x02 => {
        // MUL
    }

    0x03 => {
        // CRC32
    }

    _ => {
        // Unknown custom instruction
    }
}
```

This allows one custom opcode slot to contain multiple custom instructions.

---

# 17. Testing an Extension

Every extension should be tested independently before being integrated into a larger application.

At minimum, test:

* Instruction decoding.
* Source register reads.
* Destination register writes.
* Immediate extraction.
* Arithmetic behavior.
* Overflow behavior.
* Program-counter advancement.
* Invalid or unsupported instruction encodings.

A useful test should verify both the CPU state and execution result.

Example structure:

```rust
#[test]
fn test_add42() {
    // Initialize CPU

    // Place the custom instruction in the test environment

    // Execute extension

    // Verify destination register
    // Verify PC
    // Verify execution result
}
```

---

# 18. `gooney-linter`

Gooney Emu provides a dedicated extension validation tool:

```bash
cargo run --bin gooney-linter
```

Run the linter after creating or modifying an extension.

The linter verifies the structural requirements expected by Gooney Emu.

## 18.1 Manifest Health

The linter checks that each extension slot contains the expected manifest:

```text
extensions/custom-0/Cargo.toml
extensions/custom-1/Cargo.toml
extensions/custom-2/Cargo.toml
extensions/custom-3/Cargo.toml
```

It also verifies that the extension correctly references `gooney-core`.

## 18.2 Stub Detection

The linter identifies whether a slot still contains the default stub implementation.

This helps distinguish:

```text
unused extension slot
```

from:

```text
implemented custom extension
```

## 18.3 Signature Verification

The extension handler must conform to:

```rust
fn(u32, &mut CpuState) -> ExecutionResult
```

The linter checks that the expected `execute` interface is present.

## 18.4 Opcode Funnel Integration

The linter verifies that the custom slots remain aligned with the expected opcode mapping:

```text
Slot 0 → 0x0B
Slot 1 → 0x2B
Slot 2 → 0x5B
Slot 3 → 0x7B
```

This helps detect accidental changes to the extension routing.

---

# 19. Recommended Development Workflow

A typical extension development cycle is:

```text
1. Choose a custom slot
        ↓
2. Define the instruction encoding
        ↓
3. Implement execute()
        ↓
4. Add unit tests
        ↓
5. Register the extension
        ↓
6. Run gooney-linter
        ↓
7. Build Gooney Emu
        ↓
8. Execute/test the custom instruction
        ↓
9. Iterate
```

Example:

```bash
# Check the extension structure
cargo run --bin gooney-linter

# Build the project
cargo build

# Run tests
cargo test
```

---

# 20. Multiple Extensions

Gooney Emu can register multiple custom extensions simultaneously.

For example:

```rust
cpu.custom_handlers[0] = Some(custom_0::execute);
cpu.custom_handlers[1] = Some(custom_1::execute);
cpu.custom_handlers[2] = Some(custom_2::execute);
cpu.custom_handlers[3] = Some(custom_3::execute);
```

This provides four independent custom opcode spaces:

```text
+----------+--------+--------------------------+
| Slot     | Opcode | Example Purpose          |
+----------+--------+--------------------------+
| CUSTOM-0 | 0x0B   | Cryptography             |
| CUSTOM-1 | 0x2B   | DSP / signal processing  |
| CUSTOM-2 | 0x5B   | AI / matrix acceleration |
| CUSTOM-3 | 0x7B   | Experimental ISA         |
+----------+--------+--------------------------+
```

These are only examples. The actual purpose of each slot is determined by the extension developer.

---

# 21. Common Mistakes

## Wrong `gooney-core` Path

Make sure the relative dependency path points to the Gooney Emu core crate:

```toml
gooney-core = { path = "../../crates/gooney-core" }
```

For nested projects, the path will be different.

---

## Wrong Handler Signature

This is incorrect:

```rust
pub fn execute(instruction: u32) {
}
```

The handler must receive mutable CPU state and return an `ExecutionResult`:

```rust
pub fn execute(
    instruction: u32,
    cpu: &mut CpuState,
) -> ExecutionResult {
    // ...
}
```

---

## Forgetting the PC Update

A normal 32-bit custom instruction must advance the PC:

```rust
cpu.pc += 4;
```

Failure to do so can cause the emulator to execute the same instruction repeatedly.

---

## Writing to the Wrong Register

Always extract `rd` from bits `11:7`:

```rust
let rd = ((instruction >> 7) & 0x1F) as usize;
```

and use the CPU register API:

```rust
cpu.write_reg(rd, result);
```

---

## Registering the Wrong Slot

Make sure the handler is registered against the correct custom slot.

```text
CUSTOM-0 → custom_handlers[0]
CUSTOM-1 → custom_handlers[1]
CUSTOM-2 → custom_handlers[2]
CUSTOM-3 → custom_handlers[3]
```

---

# 22. Extension Design Guidelines

When designing a custom instruction set, consider the following:

### Keep Instruction Encodings Documented

Document every field:

```text
31        25 24    20 19    15 14  12 11     7 6      0
+-----------+--------+--------+------+--------+--------+
|  funct7   |  rs2   |  rs1   |funct3|   rd   | opcode |
+-----------+--------+--------+------+--------+--------+
```

Do not rely on undocumented magic values.

### Use `funct3` and `funct7` for Sub-Operations

Rather than consuming another opcode slot for every instruction, use instruction fields to distinguish related operations.

### Keep Extensions Self-Contained

Place extension-specific decoding and execution logic inside the extension project whenever possible.

### Test Edge Cases

For arithmetic extensions, test:

* `0`
* Maximum integer values
* Minimum integer values
* Overflow
* Underflow
* Register `x0`
* Consecutive custom instructions

### Document the Software Interface

If your extension is intended for applications or assembly programmers, document:

* Instruction mnemonic.
* Opcode.
* Instruction format.
* Operand registers.
* Immediate format.
* Side effects.
* PC behavior.
* Exceptions/errors.
* Example assembly.

---

# 23. Example Extension Specification

A well-documented custom instruction might be described as:

```text
Instruction: ADD42
Opcode:      0x0B
funct7:      0x01
Format:      R-type

Operation:

    rd = rs1 + 42

Encoding:

    31        25 24    20 19    15 14  12 11     7 6      0
    +-----------+--------+--------+------+--------+--------+
    |  0000001  | 00000  |  rs1   | 000  |   rd   | 0001011|
    +-----------+--------+--------+------+--------+--------+

Effects:

    rd ← rs1 + 42
    PC ← PC + 4
```

This style makes the extension understandable to anyone implementing an assembler, compiler backend, test program, or hardware equivalent.

---

# 24. Summary

Gooney Emu's custom extension interface provides four dedicated RISC-V opcode slots:

```text
CUSTOM-0 → 0x0B
CUSTOM-1 → 0x2B
CUSTOM-2 → 0x5B
CUSTOM-3 → 0x7B
```

A custom extension generally consists of:

```text
Cargo.toml
    ↓
gooney-core dependency
    ↓
execute(instruction, cpu)
    ↓
instruction decoding
    ↓
CPU state manipulation
    ↓
ExecutionResult
```

The recommended implementation process is:

```bash
# 1. Implement the extension
extensions/custom-X/src/lib.rs

# 2. Register the handler
cpu.custom_handlers[X] = Some(custom_X::execute);

# 3. Validate the extension
cargo run --bin gooney-linter

# 4. Build and test
cargo build
cargo test
```

The custom extension system is intended to make Gooney Emu a practical environment for experimenting with **custom RISC-V instructions, experimental ISA designs, coprocessors, cryptographic accelerators, DSP operations, AI accelerators, and other specialized execution units** without coupling their implementation directly to the emulator core.

