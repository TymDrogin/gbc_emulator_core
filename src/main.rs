mod cartridge;
mod common;
mod cpu;
mod opcode;
use crate::cartridge::ROMHeader;
use crate::opcode::*;
use std::collections::HashMap;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = r#"
    {
        "0x07": {
            "mnemonic": "RLC",
            "bytes": 2,
            "cycles": [8],
            "operands": [{"name": "A", "immediate": true}],
            "immediate": true,
            "flags": {"Z": "Z", "N": "0", "H": "0", "C": "C"}
        }
    }
    "#;

    // Deserialize into intermediate map
    let map: HashMap<String, Opcode> = serde_json::from_str(json_data)?;

    // Convert into Vec<Opcode> with code filled
    let mut opcodes = Vec::new();
    for (k, v) in map {
        let code = u8::from_str_radix(k.trim_start_matches("0x"), 16).unwrap();
        opcodes.push(Opcode {
            code,
            mnemonic: v.mnemonic,
            bytes: v.bytes,
            cycles: v.cycles,
            operands: v.operands,
            flags: v.flags,
            immediate: v.immediate,
        });
    }

    println!("{:#?}", opcodes);

    Ok(())
}
