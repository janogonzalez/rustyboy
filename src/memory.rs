use rom;

pub struct Memory {
    pub rom: rom::Rom,
    ram: [u8, ..0x4000],
    hram: [u8, ..0x79],
    ie: u8
}

impl Memory {
    pub fn new(rom: rom::Rom) -> Memory {
        Memory {
            rom: rom,
            ram: [0x00, ..0x4000],
            hram: [0x00, ..0x79],
            ie: 0x00
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..0x7FFF => self.rom.bytes[address as uint],
            0x8000..0x9FFF => {
                print!("Video RAM not implemented!");
                0x00
            },
            0xA000..0xBFFF => {
                print!("External RAM not implemented!");
                0x00
            },
            0xC000..0xDFFF => self.ram[address as uint - 0xC000],
            0xE000..0xFDFF => self.ram[address as uint - 0xE000],
            0xFE00..0xFE9F => {
                print!("Sprite attribute table not implemented!");
                0x00
            },
            0xFEA0..0xFEFF => fail!("0xFEA0..0xFEFF segment is no usable"),
            0xFF00..0xFF7F => {
                print!("I/O ports not implemented!");
                0x00
            },
            0xFF80..0xFFFE => self.hram[address as uint - 0xFF80],
            0xFFFF => self.ie,
            _ => unreachable!()
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..0x7FFF => {
                fail!("Address not writable in MBC0: {:#06x}", address);
            },
            0x8000..0x9FFF => {
                print!("Video RAM not implemented!");
            },
            0xA000..0xBFFF => {
                print!("External RAM not implemented!");
            }
            0xC000..0xDFFF => {
                self.ram[address as uint - 0xC000] = value;
            },
            0xE000..0xFDFF => {
                self.ram[address as uint - 0xE000] = value;
            },
            0xFE00..0xFE9F => {
                print!("Sprite attribute table not implemented!");
            },
            0xFEA0..0xFEFF => fail!("0xFEA0..0xFEFF segment is no usable"),
            0xFF00..0xFF7F => {
                print!("Implement I/O ports stuff... ");
            },
            0xFF80..0xFFFE => {
                self.hram[address as uint - 0xFF80] = value;
            },
            0xFFFF => {
                self.ie = value;
            },
            _ => unreachable!()
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        self.read_byte(address) as u16 |
            self.read_byte(address + 1) as u16 << 8
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }
}
