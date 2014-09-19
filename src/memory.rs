use gpu;
use rom;
use timer;

pub struct Memory {
    pub rom: rom::Rom,
    pub gpu: gpu::Gpu,
    pub timer: timer::Timer,
    ram: [u8, ..0x4000],
    hram: [u8, ..0x79],
    if_: u8,
    ie: u8
}

impl Memory {
    pub fn new(rom: rom::Rom, gpu: gpu::Gpu, timer: timer::Timer) -> Memory {
        Memory {
            rom: rom,
            gpu: gpu,
            timer: timer,
            ram: [0x00, ..0x4000],
            hram: [0x00, ..0x79],
            if_: 0x00,
            ie: 0x00
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..0x7FFF => self.rom.bytes[address as uint],
            0x8000..0x9FFF => self.gpu.read_byte(address),
            0xA000..0xBFFF => {
                print!("External RAM not implemented!");
                0x00
            },
            0xC000..0xDFFF => self.ram[address as uint - 0xC000],
            0xE000..0xFDFF => self.ram[address as uint - 0xE000],
            0xFE00..0xFE9F => self.gpu.read_byte(address),
            0xFEA0..0xFEFF => fail!("0xFEA0..0xFEFF segment is no usable"),
            0xFF00..0xFF7F => {
                match address {
                    0xFF04..0xFF07 => {
                        self.timer.read_byte(address)
                    },
                    0xFF0F => {
                        self.if_
                    },
                    0xFF40..0xFF4B => {
                        self.gpu.read_byte(address)
                    },
                    _ => {
                        print!("Implement I/O ports stuff... ");
                        0x00
                    }
                }
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
                self.gpu.write_byte(address, value);
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
                self.gpu.write_byte(address, value);
            },
            0xFEA0..0xFEFF => fail!("0xFEA0..0xFEFF segment is no usable"),
            0xFF00..0xFF7F => {
                match address {
                    0xFF01 => {
                        println!("SB: {:c}", value as char);
                    },
                    0xFF04..0xFF07 => {
                        self.timer.write_byte(address, value);
                    },
                    0xFF0F => {
                        self.if_ = value;
                    },
                    0xFF40..0xFF4B => {
                        self.gpu.write_byte(address, value);
                    },
                    _ => print!("Implement I/O ports stuff... ")
                }
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
