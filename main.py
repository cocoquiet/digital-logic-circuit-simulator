import digital_logic_circuit_simulator as dlcs

for a in range(2):
    for b in range(2):
        print(f"a={a}, b={b}")
        print(f"AND: {dlcs.basic.gate_and(a, b)}")
        print(f"OR: {dlcs.basic.gate_or(a, b)}")
        print(f"NOT(a): {dlcs.basic.gate_not(a)}")
        print(f"NAND: {dlcs.basic.gate_nand(a, b)}")
        print(f"NOR: {dlcs.basic.gate_nor(a, b)}")
        print(f"XOR: {dlcs.basic.gate_xor(a, b)}")
        print(f"XNOR: {dlcs.basic.gate_xnor(a, b)}")
        print()