# Gooney Emulator Testing & Verification Guide

This document outlines the testing strategy, test workloads, and verification methods used for the **Gooney** RISC-V emulator project.

## 1. Project Test Workloads (`workloads/`)

The `workloads/` directory contains raw RISC-V binary test files (`.bin`) and helper scripts used to validate CPU features, instruction decoding, and execution flows.

These workloads are used by the Gooney TUI executable to execute and observe instructions inside the emulator.

* **`arith_32_test.bin`** & **`reg_arith_test.bin`**: Validate 32-bit and 64-bit register arithmetic operations such as `ADD`, `SUB`, `ADDI`, etc.

* **`imm_logic_test.bin`** & **`shift_imm_test.bin`**: Test immediate logic instructions (`ANDI`, `ORI`, `XORI`) and shift operations (`SLLI`, `SRLI`, `SRAI`).

* **`jump_branch_test.bin`** & **`control_test.bin`**: Test control-flow mechanics, conditional branches (`BEQ`, `BNE`, etc.), and unconditional jumps (`JAL`, `JALR`).

* **`upper_test.bin`**: Tests upper-immediate generation instructions (`LUI`, `AUIPC`).

* **`illegal.bin`**: Used to verify that unsupported or invalid instructions trigger the proper traps/exceptions.

* **`decode.py`**: A helper script used for inspecting or generating instruction encodings.

### Running a Workload

Workloads can be executed through the Gooney TUI binary using:

```bash
cargo run --bin gooney-tui -- run workloads/imm_logic_test.bin
```

For example, to test the immediate logic instructions:

```bash
cargo run --bin gooney-tui -- run workloads/imm_logic_test.bin
```

To test arithmetic operations:

```bash
cargo run --bin gooney-tui -- run workloads/arith_32_test.bin
```

The same approach can be used with the other `.bin` files in the `workloads/` directory.

This provides an end-to-end verification path where the binary workload is loaded by the emulator and executed through the normal CPU instruction-fetch, decode, and execution pipeline.

---

## 2. Testing Memory Operations

Memory operations are tested separately from the workload-based instruction tests.

The memory load and store instructions:

```text
LB   LH   LW   LD
LBU  LHU  LWU
SB   SH   SW   SD
```

are verified using **isolated unit tests** inside:

```text
crates/gooney-core/src/decoder/memory_instr.rs
```

These tests use Cargo's built-in Rust testing framework.

### Running the Memory Tests

To run the complete test suite:

```bash
cargo test
```

To run tests specifically for the core emulation library:

```bash
cargo test -p gooney-core
```

The purpose of these tests is specifically to verify the correctness of memory instruction implementation rather than to execute the `.bin` workloads.

---

## 3. How We Fundamentally Tested Memory Operations

To verify memory load and store instructions, we implemented rigorous **isolated unit tests** inside:

```text
crates/gooney-core/src/decoder/memory_instr.rs
```

Rather than relying purely on external binary workloads, memory operations are tested from the ground up using a **white-box simulation approach**.

### Step 1: Hand-Crafted Machine Code (Raw Encodings)

We manually constructed the exact 32-bit instruction words for RISC-V load (I-type) and store (S-type) instructions.

This required accounting for instruction format variations.

For S-type instructions, the 12-bit immediate is split across:

```text
[31:25] and [11:7]
```

while I-type instructions store the immediate in:

```text
[31:20]
```

This allows the tests to verify that the decoder correctly reconstructs the immediate value before executing the memory operation.

### Step 2: Virtual State Setup

For each test case, we created clean instances of the core components:

* **`CpuState`**: Initialized general-purpose registers, including base pointer registers such as `x1 = 0x80000000` and data source registers such as `x2 = 0x12345678` or `0xDEADBEEFCAFEBABE`.

* **`Memory`**: Allocated a sandbox memory block mapped to the base address `0x80000000`.

This creates a predictable and isolated environment for each memory test.

### Step 3: Decoder & Execution Pipeline

The raw instruction bit pattern is passed directly into the decoder execution path:

```rust
let res = Decoder::decode_and_execute(instruction, &mut cpu, &mut memory);
```

This exercises the actual decode and execution pipeline, including:

1. Resolving the instruction opcode.
2. Extracting the required instruction fields.
3. Reconstructing the immediate value.
4. Computing the effective memory address using wrapping addition.
5. Dispatching the appropriate memory read or write handler.

For memory instructions, the relevant opcodes include:

```text
0x03 - Load instructions
0x23 - Store instructions
```

### Step 4: Dual-Sided State Assertions

Correctness is verified by inspecting the state on both sides of the memory operation.

#### Verifying Stores (`SB`, `SH`, `SW`, `SD`)

After executing a store instruction, the test directly queries the `Memory` subsystem at the target address.

For example:

```rust
memory.read_u32(...)
```

or:

```rust
memory.read_u64(...)
```

The returned value is compared against the expected result.

This verifies that:

* The correct memory address was calculated.
* The correct number of bytes was written.
* The expected value was stored.
* The store instruction was decoded and executed correctly.

#### Verifying Loads (`LB`, `LH`, `LW`, `LD`, `LBU`, `LHU`, `LWU`)

For load instructions, the test checks the destination CPU register:

```rust
cpu.read_reg(...)
```

The resulting register value is compared against the expected value.

This verifies both the memory read and the required RISC-V extension behavior.

For example, a signed byte containing:

```text
0xFF
```

loaded using `LB` should be sign-extended to:

```text
0xFFFFFFFFFFFFFFFF
```

while loading the same byte using `LBU` should perform zero-extension:

```text
0x00000000000000FF
```

---

## 4. Overall Verification Strategy

The Gooney emulator therefore uses **two complementary testing approaches**:

### Workload-Based Verification

The `.bin` files in `workloads/` are used to test instruction execution through the emulator's normal runtime path.

Example:

```bash
cargo run --bin gooney-tui -- run workloads/imm_logic_test.bin
```

This is primarily used for testing instruction functionality and execution flows.

### Unit-Based Memory Verification

Memory instructions are tested directly through Rust unit tests using:

```bash
cargo test -p gooney-core
```

This provides a controlled environment for verifying the detailed behavior of memory loads and stores, including:

* Instruction decoding
* Immediate extraction
* Effective address calculation
* Memory access width
* Register updates
* Sign extension
* Zero extension

Together, the workload-based tests and isolated memory unit tests provide coverage at both the **emulator runtime level** and the **individual instruction implementation level**.

