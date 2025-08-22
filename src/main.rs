mod cartridge;
mod common;
mod cpu;
use crate::cartridge::ROMHeader;


fn main() {

    let cartridge = cartridge::Cartridge::new_from_file("/home/tim/github/gbc_emulator_core/games/Zelda.gbc")
        .unwrap()
        .print_info();

}
