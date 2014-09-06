use rom;

pub struct Memory {
    pub rom: rom::Rom,
    ram: [u8, ..0x1000],
    hram: [u8, ..0x80]
}

impl Memory {
    pub fn new(rom: rom::Rom) -> Memory {
        Memory {
            rom: rom,
            ram: [0x00, ..0x1000],
            hram: [0x00, ..0x80]
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..0x3FFF => self.rom.bytes[address as uint],
            0xC000..0xDFFF => self.ram[address as uint - 0xC000],
            0xFF80..0xFFFF => self.hram[address as uint - 0xFF80],
            _ => 0x00
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xC000..0xDFFF => {
                self.ram[address as uint - 0xC000] = value
            },
            0xFF00..0xFF7F => {
                print!("Implement I/O ports stuff... ")
            },
            0xFF80..0xFFFF => {
                self.hram[address as uint - 0xFF80] = value
            },
            _ => println!("Can't write to memory address {:#06x}", address)
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        self.read_byte(address) as u16 |
            self.read_byte(address + 1) as u16 << 8
    }
}
