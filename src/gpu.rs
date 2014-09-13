pub struct Gpu {
    vram: [u8, ..0x2000],
    oam: [u8, ..0xA0],
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    wy: u8,
    wx: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            vram: [0x00, ..0x2000],
            oam: [0x00, ..0xA0],
            lcdc: 0x91,
            stat: 0x00,
            scy: 0x00,
            scx: 0x00,
            ly: 0x00,
            lyc: 0x00,
            wy: 0x00,
            wx: 0x00,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..0x9FFF => {
                self.vram[address as uint - 0x8000] = value;
            },
            0xFE00..0xFE9F => {
                self.oam[address as uint - 0xFE00] = value;
            },
            0xFF40 => {
                self.lcdc = value;
            },
            0xFF41 => {
                self.stat = value;
            },
            0xFF42 => {
                self.scy = value;
            },
            0xFF43 => {
                self.scx = value;
            },
            0xFF44 => {
                self.ly = value;
            },
            0xFF45 => {
                self.lyc = value;
            },
            0xFF46 => {
                fail!("DMA transfer not implemented");
            },
            0xFF47 => {
                self.bgp = value;
            },
            0xFF48 => {
                self.obp0 = value;
            },
            0xFF49 => {
                self.obp1 = value;
            },
            0xFF4A => {
                self.wy = value;
            },
            0xFF4B => {
                self.wx = value;
            },
            _ => unreachable!()
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..0x9FFF => self.vram[address as uint - 0x8000],
            0xFE00..0xFE9F => self.oam[address as uint - 0xFE00],
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => unreachable!()
        }
    }
}
