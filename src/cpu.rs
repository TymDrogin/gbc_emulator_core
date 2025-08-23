use crate::cartridge::*;
use std::collections::HashMap;




struct CPU {
    reg: Registers
}





// A - Accumulator
// F - Flags
// B, C, D, E, H, L - General purpose registers
// SP - Stack Pointer
// PC - Program Counter
// AF, BC, DE, HL - Register pairs
// Flags: Z - Zero, N - Subtract, H - Half Carry, C - Carry


#[allow(unused)]
struct Registers {
    pub a: u8,   
    pub b: u8,    
    pub c: u8,    
    pub d: u8,    
    pub e: u8,    
    pub f: u8,    
    pub h: u8,   
    pub l: u8,
    pub sp: u16,                    
    pub pc: u16,                    
}
#[allow(unused)]
impl Registers {
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xF0) as u8; 
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}