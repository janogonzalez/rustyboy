pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8
}

static ENABLED_FLAG: u8 = 0b0000_0100;
static FREQUENCY_FLAG : u8 = 0b0000_0011;

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0x00,
            tima: 0x00,
            tma: 0x00,
            tac: 0x00
        }
    }

    pub fn step(&mut self, increment: u8) {
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => { self.div = 0x00; },
            0xFF05 => { self.tima = value; },
            0xFF06 => { self.tma = value; },
            0xFF07 => { self.tac = value; },
            _ => unreachable!()
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => unreachable!()
        }
    }

    fn is_enabled(&self) -> bool {
        (self.tac & ENABLED_FLAG) == ENABLED_FLAG
    }

    fn frequency(&self) -> bool {
        (self.tac & FREQUENCY_FLAG) == FREQUENCY_FLAG
    }
}
