use std::io::File;

pub struct Rom {
    pub bytes: Vec<u8>
}

impl Rom {
    pub fn from_path(path: &Path) -> Rom {
        let bytes = File::open(path).read_to_end().unwrap();

        Rom {
            bytes: bytes
        }
    }

    pub fn catrigde_type(&self) -> u8 {
        self.bytes[0x147]
    }

    pub fn rom_size(&self) -> u8 {
        self.bytes[0x148]
    }

    pub fn ram_size(&self) -> u8 {
        self.bytes[0x149]
    }
}
