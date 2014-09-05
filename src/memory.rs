use rom;

pub struct Memory {
    pub rom: rom::Rom
}

impl Memory {
    pub fn new(rom: rom::Rom) -> Memory {
        Memory {
            rom: rom
        }
    }

    pub fn read_byte(&self, address: uint) -> u8 {
        match address {
            0x0000..0x3FFF => self.rom.bytes[address],
            _ => 0x00
        }
    }
}
