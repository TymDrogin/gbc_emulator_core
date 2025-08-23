use crate::cartridge::*;



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

struct Registers {
    a: u8,   
    b: u8,    
    c: u8,    
    d: u8,    
    e: u8,    
    f: u8,    
    h: u8,   
    l: u8,
    sp: u16,
    pc: u16,
}

#[allow(unused)]
impl Registers {
 // --- Getters ---
    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_b(&self) -> u8 { self.b }
    pub fn get_c(&self) -> u8 { self.c }
    pub fn get_d(&self) -> u8 { self.d }
    pub fn get_e(&self) -> u8 { self.e }
    pub fn get_f(&self) -> u8 { self.f }
    pub fn get_h(&self) -> u8 { self.h }
    pub fn get_l(&self) -> u8 { self.l }
    pub fn get_sp(&self) -> u16 { self.sp }
    pub fn get_pc(&self) -> u16 { self.pc }

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

    // --- Setters ---
    pub fn set_a(&mut self, value: u8) { self.a = value; }
    pub fn set_b(&mut self, value: u8) { self.b = value; }
    pub fn set_c(&mut self, value: u8) { self.c = value; }
    pub fn set_d(&mut self, value: u8) { self.d = value; }
    pub fn set_e(&mut self, value: u8) { self.e = value; }
    pub fn set_f(&mut self, value: u8) { self.f = value; }
    pub fn set_h(&mut self, value: u8) { self.h = value; }
    pub fn set_l(&mut self, value: u8) { self.l = value; }
    pub fn set_sp(&mut self, value: u16) { self.sp = value; }
    pub fn set_pc(&mut self, value: u16) { self.pc = value; }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xF0) as u8; // lower 4 bits of F are always zero in real HW
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



struct Instruction(u8);