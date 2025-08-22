mod cartridge;
mod common;
use crate::cartridge::ROMHeader;


fn main() {
    let cartridge = cartridge::Cartridge::new_from_file(r"D:\Projects\Programming\rust\gbc_emulator_core\games\SuperMarioBros.Deluxe(USA)(Rev-B).gbc").unwrap();

    cartridge.print_info();
}
