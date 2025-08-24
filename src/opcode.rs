use std::collections::HashMap;
use serde::Deserialize;

type OpcodeTable = HashMap<u8, Opcode>;


#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Opcode {
    #[serde(skip)]
    pub code: u8,                   // Opcode byte, filled after deserialization
    
    pub mnemonic: Mnemonic,         // Mnemonic representation
    pub bytes: u8,                  // Number of bytes the instruction occupies
    pub cycles: Vec<u8>,            // Number of cycles the instruction takes
    pub operands: Vec<Operand>,     // Operands for the instruction
    
    #[serde(rename = "immediate")]
    pub is_immediate: bool,         // Does the instruction use immediate values
    pub flags: Flags,               // Flags affected by the instruction
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Operand {
    name: String,                   // Operand name (e.g., "A", "B", "(HL)", "d8", "a16")  

    #[serde(default)]
    bytes: Option<u8>,              // Number of bytes the operand occupies (if applicable)

    #[serde(default)]
    is_increment: bool,             // Is the operand an increment (e.g., "HL+")     

    #[serde(rename = "immediate")]
    is_immediate: bool,             // Is the operand an immediate value  
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Flags {
    z: char,                        // Zero Flag
    n: char,                        // Subtract Flag
    h: char,                        // Half Carry Flag
    c: char,                        // Carry Flag
}


// Generated using scripts/generate_opcode_enum.py
// Do not edit manually
#[allow(non_camel_case_types, unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Mnemonic {
    Adc,
    Add,
    And,
    Call,
    Ccf,
    Cp,
    Cpl,
    Daa,
    Dec,
    Di,
    Ei,
    Halt,
    Illegal_d3,
    Illegal_db,
    Illegal_dd,
    Illegal_e3,
    Illegal_e4,
    Illegal_eb,
    Illegal_ec,
    Illegal_ed,
    Illegal_f4,
    Illegal_fc,
    Illegal_fd,
    Inc,
    Jp,
    Jr,
    Ld,
    Ldh,
    Nop,
    Or,
    Pop,
    Prefix,
    Push,
    Ret,
    Reti,
    Rla,
    Rlca,
    Rra,
    Rrca,
    Rst,
    Sbc,
    Scf,
    Stop,
    Sub,
    Xor,
    // CB-Prefixed Opcodes
    Bit,
    Res,
    Rl,
    Rlc,
    Rr,
    Rrc,
    Set,
    Sla,
    Sra,
    Srl,
    Swap,
}
