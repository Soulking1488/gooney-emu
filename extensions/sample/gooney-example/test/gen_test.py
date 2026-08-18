import struct
import os

# Ensure the directory exists
test_dir = "extensions/custom-0/gooney-example/test"
os.makedirs(test_dir, exist_ok=True)

# RISC-V 32-bit Instructions (Little-Endian)
instructions = [
    0x00a00093,  # addi x1, x0, 10
    0x01400113,  # addi x2, x0, 20
    0x0020818b,  # GOONEY_ADD x3, x1, x2
    0x0000006f,  # j . (infinite loop)
]

bin_path = os.path.join(test_dir, "test_instruction")
with open(bin_path, "wb") as f:
    for inst in instructions:
        f.write(struct.pack("<I", inst))

print(f"✨ Successfully generated binary at: {bin_path}")
