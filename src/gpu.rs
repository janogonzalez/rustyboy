pub struct Gpu {
    pub vram: [u8, ..0x2000],
    pub oam: [u8, ..0xA0]
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            vram: [0x00, ..0x2000],
            oam: [0x00, ..0xA0]
        }
    }
}
