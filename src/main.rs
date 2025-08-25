mod cartridge;
mod common;
mod cpu;
mod opcode;

use std::fs;
use crate::cartridge::ROMHeader;
use crate::opcode::*;
use std::collections::HashMap;

use std::path::Path;

fn main() {
    let op_json_path = r"D:\Projects\Programming\rust\gbc_emulator_core\data\Opcodes.json";
    
    if !Path::new(op_json_path).exists() {
        panic!("File does not exist at path: {}", op_json_path);
    }

    let json = fs::read_to_string(op_json_path)
        .unwrap_or_else(|e| panic!("Failed to read opcode JSON file: {}", e));

    let tables: OpcodeTables = serde_json::from_str(&json)
        .unwrap_or_else(|e| panic!("Failed to deserialize JSON: {}", e));

    println!("Unprefixed opcodes loaded: {}", tables.unprefixed.len());
    println!("CB-prefixed opcodes loaded: {}", tables.cbprefixed.len());

    if let Some(op) = tables.unprefixed.get(&0x00) {
        println!("Opcode 0x00: {:#?}", op);
    }
}