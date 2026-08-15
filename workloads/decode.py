import struct

with open("workloads/reg_arith_test.bin", "rb") as f:
    data = f.read()

print(f"{'Offset':<10} | {'Hex (LE)':<12} | {'Raw Instruction (32-bit)'}")
print("-" * 45)

for i in range(0, len(data), 4):
    chunk = data[i:i+4]
    if len(chunk) < 4:
        break
    inst = struct.unpack('<I', chunk)[0]
    print(f"0x{i:08X}   | 0x{inst:08X}   | 0b{bin(inst)[2:].zfill(32)}")
