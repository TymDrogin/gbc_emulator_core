import json
from pathlib import Path

path = Path("D:/Projects/Programming/rust/gbc_emulator_core/data/Opcodes.json")


with open(path, "r") as f:
    opcodes = json.load(f)

# Unique mnemonics per group
unprefixed_mnemonics = {
    entry["mnemonic"] for entry in opcodes["unprefixed"].values()
}
cb_prefixed_mnemonics = {
    entry["mnemonic"] for entry in opcodes["cbprefixed"].values()
}
operand_names = {
    operand["name"] 
    for opcode in opcodes["unprefixed"].values()
    for operand in opcode.get("operands", [])
}

print("/// Enum representing all possible opcode mnemonics.")
print("#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]")
print('#[serde(rename_all = "UPPERCASE")]')
print("pub enum OpcodeMnemonic {")

# Unprefixed
for mnemonic in sorted(unprefixed_mnemonics):
    print(f"    {mnemonic.lower().capitalize()},")

# CB-prefixed (with namespace)
print("    // CB-Prefixed Opcodes")
for mnemonic in sorted(cb_prefixed_mnemonics):
    print(f"    {mnemonic.lower().capitalize()},")

print("}")

print(operand_names)