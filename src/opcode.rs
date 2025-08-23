use std::collections::HashMap;
use serde::Deserialize;

type OpcodeTable = HashMap<u8, Opcode>;


// Opcode structure representing a CPU instruction
// All of opcodes are loaded from a json file at compile time
// The json file is located at "data/opcodes.json"


#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Opcode {
    #[serde(skip)]
    pub code: u8,                   // Opcode byte
    pub mnemonic: String,           // Mnemonic representation
    pub bytes: u8,                  // Number of bytes the instruction occupies
    pub cycles: Vec<u8>,                 // Number of cycles the instruction takes
    pub operands: Vec<Operand>,     // Operands for the instruction
    pub immediate: bool,         // Does the instruction use immediate values
    pub flags: Flags,               // Flags affected by the instruction

}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Operand {
    name: String,                   // Operand name (e.g., "A", "B", "(HL)", "d8", "a16")  

    #[serde(default)]
    bytes: Option<u8>,              // Number of bytes the operand occupies (if applicable)

    #[serde(default)]
    is_increment: bool,     // In the opcode json this value is either true, or does not exist     

    #[serde(rename = "immediate")]
    is_immediate: bool,             // Is the operand an immediate value  
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Flags {
    #[serde(rename = "Z")]
    z: char,                        // Zero Flag

    #[serde(rename = "N")]
    n: char,                        // Subtract Flag

    #[serde(rename = "H")]
    h: char,                        // Half Carry Flag

    #[serde(rename = "C")]
    c: char,                        // Carry Flag
}