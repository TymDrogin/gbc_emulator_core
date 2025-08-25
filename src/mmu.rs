use crate::common::IO;
use crate::cartridge::Cartridge;

// Complete memory map can be found here https://gbdev.gg8.se/wiki/articles/Memory_Map

// ROM
const MMU_ROM_BANK_0_START: u16 = 0x0000;
const MMU_ROM_BANK_0_END: u16   = 0x3FFF;

const MMU_ROM_BANK_N_START: u16 = 0x4000;
const MMU_ROM_BANK_N_END: u16   = 0x7FFF;

// VRAM
const MMU_VRAM_START: u16 = 0x8000;
const MMU_VRAM_END: u16   = 0x9FFF;

// External RAM
const MMU_EXTERNAL_RAM_START: u16 = 0xA000;
const MMU_EXTERNAL_RAM_END: u16   = 0xBFFF;

// Work RAM
const MMU_WORK_RAM_0_START: u16 = 0xC000;
const MMU_WORK_RAM_0_END: u16   = 0xCFFF;

const MMU_WORK_RAM_N_START: u16 = 0xD000;
const MMU_WORK_RAM_N_END: u16   = 0xDFFF;

// Echo RAM
const MMU_ECHO_RAM_START: u16 = 0xE000;
const MMU_ECHO_RAM_END: u16   = 0xFDFF;

// OAM
const MMU_OAM_START: u16 = 0xFE00;
const MMU_OAM_END: u16   = 0xFE9F;

// Not usable
const MMU_NOT_USABLE_START: u16 = 0xFEA0;
const MMU_NOT_USABLE_END: u16   = 0xFEFF;

// IO Registers
const MMU_IO_REGISTERS_START: u16 = 0xFF00;
const MMU_IO_REGISTERS_END: u16   = 0xFF7F;

// High RAM
const MMU_HIGH_RAM_START: u16 = 0xFF80;
const MMU_HIGH_RAM_END: u16   = 0xFFFE;

// Interrupt Enable Register
const MMU_INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;


const KB: usize = 1024;

const ROM_BANK_SIZE: usize = 16 * KB; // 16 KB
const VRAM_SIZE: usize = 8 * KB;      // 8 KB
const ERAM_SIZE: usize = 8 * KB;      // 8 KB
const WRAM_BANK_SIZE: usize = 4 * KB; // 4 KB
const OAM_SIZE: usize = 160;          // 160 B
const HRAM_SIZE: usize = 127;         // 127 B


enum MMUError {
    InvalidAddress,
    ReadOnlyAddress,
    WriteOnlyAddress,
    CartridgeNotLoaded,
}



#[allow(unused)]
struct MMU {
    cartridge: Option<Cartridge>,
    // ROM
    rom_bank_0: [u8; ROM_BANK_SIZE], // 16 KB
    rom_bank_n: [u8; ROM_BANK_SIZE], // 16 KB
    // Video RAM
    vram: [u8; VRAM_SIZE],           // 8 KB
    // External RAM
    eram: [u8; ERAM_SIZE],           // 8 KB
    // Work RAM
    wram_bank_0: [u8; WRAM_BANK_SIZE], // 4 KB
    wram_bank_n: [u8; WRAM_BANK_SIZE], // 4 KB
    // OAM
    oam: [u8; OAM_SIZE],             // 160 B
    // High RAM
    hram: [u8; HRAM_SIZE],           // 127 B
    // Interrupt Enable Register
    ie: u8,                          // 1 B
}
#[allow(unused)]
impl MMU {
    pub fn reset(&mut self) {
        // Reset memory to initial state
    }
    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
    }
}
#[allow(unused)]
impl MMU {
    fn read(&self, addr: u16) -> Option<u8> {
        match addr {
            // ROM Bank 0
            MMU_ROM_BANK_0_START..=MMU_ROM_BANK_0_END => {
                let offset = addr as usize - MMU_ROM_BANK_0_START as usize;
                Some(self.rom_bank_0[offset])
            }

            // Switchable ROM Bank N
            MMU_ROM_BANK_N_START..=MMU_ROM_BANK_N_END => {
                let offset= addr - MMU_ROM_BANK_N_START;
                todo!()
            }

            // Video RAM
            MMU_VRAM_START..=MMU_VRAM_END => {
                let offset = addr as usize - MMU_VRAM_START as usize;
                Some(self.vram[offset])
            }

            // External RAM
            MMU_EXTERNAL_RAM_START..=MMU_EXTERNAL_RAM_END => {
                let offset = addr as usize - MMU_EXTERNAL_RAM_START as usize;
                Some(self.eram[offset])
            }

            // Work RAM Bank 0
            MMU_WORK_RAM_0_START..=MMU_WORK_RAM_0_END => {
                let offset = addr as usize - MMU_WORK_RAM_0_START as usize;
                Some(self.wram_bank_0[offset])
            }

            // Work RAM Bank N
            MMU_WORK_RAM_N_START..=MMU_WORK_RAM_N_END => {
                let offset = addr as usize - MMU_WORK_RAM_N_START as usize;
                Some(self.wram_bank_n[offset])
            }

            // Echo RAM (mirrors Work RAM 0 + N)
            MMU_ECHO_RAM_START..=MMU_ECHO_RAM_END => {
                let mirrored_addr = addr - MMU_ECHO_RAM_START;
                if mirrored_addr < WRAM_BANK_SIZE as u16 {
                    Some(self.wram_bank_0[mirrored_addr as usize])
                } else {
                    let offset = mirrored_addr as usize - WRAM_BANK_SIZE;
                    Some(self.wram_bank_n[offset])
                }
            }

            // OAM (sprite attributes)
            MMU_OAM_START..=MMU_OAM_END => {
                None // can be restricted based on GPU mode
            }

            // Not usable
            MMU_NOT_USABLE_START..=MMU_NOT_USABLE_END => None,

            // I/O registers
            MMU_IO_REGISTERS_START..=MMU_IO_REGISTERS_END => {
                // For now, return dummy or implement proper I/O later
                Some(0xFF)
            }

            // High RAM
            MMU_HIGH_RAM_START..=MMU_HIGH_RAM_END => {
                let offset = addr as usize - MMU_HIGH_RAM_START as usize;
                Some(self.hram[offset])
            }

            // Interrupt Enable Register
            MMU_INTERRUPT_ENABLE_REGISTER => Some(self.ie),

            _ => unreachable!("MMU read failed: address out of range {:x}", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // ROM Bank 0 — read-only, ignore writes
            MMU_ROM_BANK_0_START..=MMU_ROM_BANK_0_END => {
                // ignore writes or log warning
            }

            // Switchable ROM Bank N — typically writes go to cartridge (MBC)
            MMU_ROM_BANK_N_START..=MMU_ROM_BANK_N_END => {
                if let Some(cart) = &mut self.cartridge {
                    let offset = (addr as usize - MMU_ROM_BANK_N_START as usize) as u16;
                    cart.write(offset, value);
                }
                // else ignore write
            }

            // Video RAM
            MMU_VRAM_START..=MMU_VRAM_END => {
                let offset = addr as usize - MMU_VRAM_START as usize;
                self.vram[offset] = value;
            }

            // External RAM
            MMU_EXTERNAL_RAM_START..=MMU_EXTERNAL_RAM_END => {
                let offset = addr as usize - MMU_EXTERNAL_RAM_START as usize;
                self.eram[offset] = value;
            }

            // Work RAM Bank 0
            MMU_WORK_RAM_0_START..=MMU_WORK_RAM_0_END => {
                let offset = addr as usize - MMU_WORK_RAM_0_START as usize;
                self.wram_bank_0[offset] = value;
            }

            // Work RAM Bank N
            MMU_WORK_RAM_N_START..=MMU_WORK_RAM_N_END => {
                let offset = addr as usize - MMU_WORK_RAM_N_START as usize;
                self.wram_bank_n[offset] = value;
            }

            // Echo RAM — mirrors Work RAM
            MMU_ECHO_RAM_START..=MMU_ECHO_RAM_END => {
                let mirrored_addr = addr - MMU_ECHO_RAM_START;
                if mirrored_addr < WRAM_BANK_SIZE as u16 {
                    self.wram_bank_0[mirrored_addr as usize] = value;
                } else {
                    let offset = mirrored_addr as usize - WRAM_BANK_SIZE;
                    self.wram_bank_n[offset] = value;
                }
            }

            // OAM (sprite attributes)
            MMU_OAM_START..=MMU_OAM_END => {
                // Can be written depending on GPU mode, for now allow
                // Implement GPU restrictions later
            }

            // Not usable — ignore writes
            MMU_NOT_USABLE_START..=MMU_NOT_USABLE_END => {}

            // I/O registers
            MMU_IO_REGISTERS_START..=MMU_IO_REGISTERS_END => {
                // Implement proper I/O writes later
            }

            // High RAM
            MMU_HIGH_RAM_START..=MMU_HIGH_RAM_END => {
                let offset = addr as usize - MMU_HIGH_RAM_START as usize;
                self.hram[offset] = value;
            }

            // Interrupt Enable Register
            MMU_INTERRUPT_ENABLE_REGISTER => self.ie = value,

            _ => unreachable!("MMU write failed: address out of range {:x}", addr),
        }
    }
}
