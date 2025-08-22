use std::fmt;
use crate::common::*;

const ROM_HEADER_START_ADDRESS: usize = 0x0100;
const ROM_HEADER_END_ADDRESS: usize = 0x014F;


#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ROMHeader {
    pub entry_point: [u8; 4],
    pub nintendo_logo: [u8; 48],
    pub title: [u8; 15],              // title (15 bytes, CGB interpretation)
    pub cgb_flag: u8,                 // 0x143
    pub new_licensee_code: u16,   // 0x144–0x145
    pub sgb_flag: u8,                 // 0x146
    pub cartridge_type: u8,           // 0x147
    pub rom_size: u8,                 // 0x148
    pub ram_size: u8,                 // 0x149
    pub destination_code: u8,         // 0x14A
    pub old_licensee_code: u8,        // 0x14B
    pub mask_rom_version_number: u8,  // 0x14C
    pub header_checksum: u8,          // 0x14D
    pub global_checksum: u16,     // 0x14E–0x14F
}
impl ROMHeader {
    pub fn from_bytes(bytes: &[u8]) -> &Self {
        assert!(bytes.len() >= std::mem::size_of::<ROMHeader>());
        unsafe { &*(bytes.as_ptr() as *const ROMHeader) }
    }

    pub fn get_title(&self) -> String {
        let end = self.title.iter().position(|&c| c == 0).unwrap_or(self.title.len());
        String::from_utf8_lossy(&self.title[..end]).to_string()
    }
    pub fn get_cgb_flag(&self) -> u8 {
        self.cgb_flag
    }
    pub fn get_new_licensee_code(&self) -> u16 {
        return self.new_licensee_code;
    }
    pub fn get_cartridge_type(&self) -> &str {
        match get_cartridge_types().get(&self.cartridge_type) {
            Some(name) => name,
            None => "Unknown",
        }
    }
    pub fn get_rom_size_kb(&self) -> usize {
        32 * (1 << self.rom_size)
    }
    pub fn get_ram_size_kb(&self) -> Option<u8> {
        match self.ram_size {
            0 => Some(0),
            1 => Some(2),
            2 => Some(8),
            3 => Some(32),
            4 => Some(128),
            5 => Some(64),
            _ => None,
        }
    }
    pub fn get_destination_code(&self) -> &str {
        match self.destination_code {
            0x00 => "Japanese",
            0x01 => "Overseas only",
            _ => "Unknown",
        }
    }
    pub fn get_old_licensee_code(&self) -> &str {
        if self.old_licensee_code == 0x33 {
            return "New licensee code should be used";
        }
        match old_licensee_codes().get(&self.old_licensee_code) {
            Some(name) => name,
            None => "Unknown",
        }
    }
    pub fn get_mask_rom_version_number(&self) -> u8 {
        self.mask_rom_version_number
    }
    pub fn get_header_checksum(&self) -> u8 {
        self.header_checksum
    }
    pub fn get_global_checksum(&self) -> u16 {
        self.global_checksum
    }
    pub fn validate_nintendo_logo(&self) -> bool {
        const NINTENDO_LOGO: [u8; 48] = [
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
            0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
            0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
            0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
            0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
            0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];

        self.nintendo_logo == NINTENDO_LOGO
    }
}

impl fmt::Display for ROMHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "----=====ROM Header====----")?;
        writeln!(f, "  Title: {}", self.get_title())?;
        match self.validate_nintendo_logo() {
            true => writeln!(f, "  Nintendo Logo: Valid")?,
            false => writeln!(f, "  Nintendo Logo: Invalid")?,
        }
        writeln!(f, "  CGB Flag: 0x{:02X}", self.get_cgb_flag())?;
        writeln!(f, "  New Licensee Code: {}", self.get_new_licensee_code())?;
        writeln!(f, "  Cartridge Type: {}", self.get_cartridge_type())?;
        writeln!(f, "  ROM Size: {} KB", self.get_rom_size_kb())?;
        match self.get_ram_size_kb() {
            Some(size) => writeln!(f, "  RAM Size: {} KB", size)?,
            None => writeln!(f, "  RAM Size: Unknown")?,
        }
        writeln!(f, "  Destination Code: {}", self.get_destination_code())?;
        writeln!(f, "  Old Licensee Code: {}", self.get_old_licensee_code())?;
        writeln!(f, "  Mask ROM Version: 0x{:02X}", self.get_mask_rom_version_number())?;
        writeln!(f, "  Header Checksum: 0x{:02X}", self.get_header_checksum())?;
        writeln!(f, "  Global Checksum: 0x{:04X}", self.get_global_checksum())
    }
}

pub struct Cartridge {
    pub header: ROMHeader,
    pub rom_data: Vec<u8>,
}
impl Cartridge {
    pub fn new(rom_data: Vec<u8>) -> Self {
        let header_bytes = &rom_data[ROM_HEADER_START_ADDRESS..=ROM_HEADER_END_ADDRESS];
        let header = ROMHeader::from_bytes(header_bytes);

        Cartridge { header: *header, rom_data }
    }
    pub fn new_from_file(file_path: &str) -> std::io::Result<Self> {
        let rom_data = std::fs::read(file_path)?;
        
        Ok(Cartridge::new(rom_data))
    }

    pub fn print_info(&self) {
        println!("{}", self.header);
    }
}
