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

    pub fn name(&self) -> String {
        let name = self.bytes.
            slice(0x134, 0x143).
            iter().
            take_while(|&n| *n > 0x0).
            map(|&n| n).
            collect::<Vec<u8>>();

        String::from_utf8(name).unwrap()
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
