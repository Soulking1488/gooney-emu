# Test program for gooney-example (GOONEY_ADD)
.global _start

_start:
    # Load test values into x1 and x2
    li x1, 10
    li x2, 20

    # Custom instruction: GOONEY_ADD x3, x1, x2
    # Opcode: 0x0B, Custom encoding format
    # This computes: x3 = x1 (10) + x2 (20) + 0x42 (66) = 96 (0x60)
    .word 0x0120818b   # Encoded instruction word for GOONEY_ADD x3, x1, x2

    # Infinite loop to halt execution
done:
    j done
