use std::fmt;

const ROM_HEADER_START_ADDRESS: usize = 0x0100;
const ROM_HEADER_END_ADDRESS: usize = 0x014F;


#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ROMHeader {
    pub entry_point: [u8; 4],
    pub nintendo_logo: [u8; 48],
    pub title: [u8; 15],              // title (15 bytes, CGB interpretation)
    pub cgb_flag: u8,                 // 0x143
    pub new_licensee_code: [u8; 2],   // 0x144–0x145
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
    pub fn get_new_licensee_code(&self) -> String {
        String::from_utf8_lossy(&self.new_licensee_code).to_string()
    }
    pub fn get_cartridge_type(&self) -> u8 {
        self.cartridge_type
    }
    pub fn get_rom_size_kb(&self) -> u8 {
        32 << self.rom_size
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
    pub fn get_destination_code(&self) -> u8 {
        self.destination_code
    }
    pub fn get_old_licensee_code(&self) -> u8 {
        self.old_licensee_code
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
}

impl fmt::Display for ROMHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "----=====ROM Header====----")?;
        writeln!(f, "  Title: {}", self.get_title())?;
        writeln!(f, "  CGB Flag: 0x{:02X}", self.get_cgb_flag())?;
        writeln!(f, "  New Licensee Code: {}", self.get_new_licensee_code())?;
        writeln!(f, "  Cartridge Type: 0x{:02X}", self.get_cartridge_type())?;
        writeln!(f, "  ROM Size: {} KB", self.get_rom_size_kb())?;
        match self.get_ram_size_kb() {
            Some(size) => writeln!(f, "  RAM Size: {} KB", size)?,
            None => writeln!(f, "  RAM Size: Unknown")?,
        }
        writeln!(f, "  Destination Code: 0x{:02X}", self.get_destination_code())?;
        writeln!(f, "  Old Licensee Code: 0x{:02X}", self.get_old_licensee_code())?;
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
        let header = ROMHeader::from_bytes(&rom_data[..]);



        Cartridge { header: *header, rom_data }
    }
    pub fn new_from_file(file_path: &str) -> std::io::Result<Self> {
        let rom_data = std::fs::read(file_path)?;
        
        Ok(Cartridge::new(rom_data))
    }

    pub fn display_info(&self) {
        println!("{}", self.header);
    }
}
