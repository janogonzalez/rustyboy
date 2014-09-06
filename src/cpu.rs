use memory;

pub struct Cpu {
    pub memory: memory::Memory,
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
    cycles: uint
}

static CPU_CYCLES: [uint, ..256] = [
//  0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    4, 0, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, // 0x00
    0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, // 0x10
    0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, // 0x20
    0, 0, 0, 0, 0, 0,12, 0, 0, 0, 0, 0, 0, 0, 8, 0, // 0x30
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x40
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x50
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x60
    8, 8, 8, 8, 8, 8, 0, 8, 4, 4, 4, 4, 4, 4, 8, 4, // 0x70
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x80
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x90
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0xA0
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0xB0
    0, 0, 0,16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xC0
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xD0
   12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, // 0xE0
   12, 0, 0, 8, 0, 0, 0, 0, 0, 0,16, 0, 0, 0, 0, 0, // 0xF0
];

static Z_FLAG: u8 = 0b1000_0000;
static N_FLAG: u8 = 0b0100_0000;
static H_FLAG: u8 = 0b0010_0000;
static C_FLAG: u8 = 0b0001_0000;

impl Cpu {
    pub fn new(memory: memory::Memory) -> Cpu {
        Cpu {
            memory: memory,
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
            cycles: 0
        }
    }

    pub fn step(&mut self) {
        let opcode = self.read_next_byte();

        println!("Executing: {:#04X}", opcode);

        match opcode {
            0x00 => {},
            0x0A => { self.a = self.memory.read_byte(self.bc()); },
            0x1A => { self.a = self.memory.read_byte(self.de()); },

            0x06 => { self.b = self.read_next_byte(); },
            0x0E => { self.c = self.read_next_byte(); },
            0x16 => { self.d = self.read_next_byte(); },
            0x1E => { self.e = self.read_next_byte(); },
            0x26 => { self.h = self.read_next_byte(); },
            0x2E => { self.l = self.read_next_byte(); },
            0x36 => {
                let addr = self.hl();
                let val = self.read_next_byte();
                self.memory.write_byte(addr, val);
            },
            0x3E => { self.a = self.read_next_byte(); },

            0x40 => { self.b = self.b; },
            0x41 => { self.b = self.c; },
            0x42 => { self.b = self.d; },
            0x43 => { self.b = self.e; },
            0x44 => { self.b = self.h; },
            0x45 => { self.b = self.l; },
            0x46 => { self.b = self.memory.read_byte(self.hl()); },
            0x47 => { self.b = self.a; },

            0x48 => { self.c = self.b; },
            0x49 => { self.c = self.c; },
            0x4A => { self.c = self.d; },
            0x4B => { self.c = self.e; },
            0x4C => { self.c = self.h; },
            0x4D => { self.c = self.l; },
            0x4E => { self.c = self.memory.read_byte(self.hl()); },
            0x4F => { self.c = self.a; },

            0x50 => { self.d = self.b; },
            0x51 => { self.d = self.c; },
            0x52 => { self.d = self.d; },
            0x53 => { self.d = self.e; },
            0x54 => { self.d = self.h; },
            0x55 => { self.d = self.l; },
            0x56 => { self.d = self.memory.read_byte(self.hl()); },
            0x57 => { self.d = self.a; },

            0x58 => { self.e = self.b; },
            0x59 => { self.e = self.c; },
            0x5A => { self.e = self.d; },
            0x5B => { self.e = self.e; },
            0x5C => { self.e = self.h; },
            0x5D => { self.e = self.l; },
            0x5E => { self.e = self.memory.read_byte(self.hl()); },
            0x5F => { self.e = self.a; },

            0x60 => { self.h = self.b; },
            0x61 => { self.h = self.c; },
            0x62 => { self.h = self.d; },
            0x63 => { self.h = self.e; },
            0x64 => { self.h = self.h; },
            0x65 => { self.h = self.l; },
            0x66 => { self.h = self.memory.read_byte(self.hl()); },
            0x67 => { self.h = self.a; },

            0x68 => { self.l = self.b; },
            0x69 => { self.l = self.c; },
            0x6A => { self.l = self.d; },
            0x6B => { self.l = self.e; },
            0x6C => { self.l = self.h; },
            0x6D => { self.l = self.l; },
            0x6E => { self.l = self.memory.read_byte(self.hl()); },
            0x6F => { self.l = self.a; },

            0x70 => {
                let addr = self.hl();
                self.memory.write_byte(addr, self.b);
            },
            0x71 => {
                let addr = self.hl();
                self.memory.write_byte(addr, self.c);
            },
            0x72 => {
                let addr = self.hl();
                self.memory.write_byte(addr, self.d);
            },
            0x73 => {
                let addr = self.hl();
                self.memory.write_byte(addr, self.e);
            },
            0x74 => {
                let addr = self.hl();
                self.memory.write_byte(addr, self.h);
            },
            0x75 => {
                let addr = self.hl();
                self.memory.write_byte(addr, self.l);
            },

            0x77 => {
                let addr = self.hl();
                self.memory.write_byte(addr, self.a);
            },

            0x78 => { self.a = self.b; },
            0x79 => { self.a = self.c; },
            0x7A => { self.a = self.d; },
            0x7B => { self.a = self.e; },
            0x7C => { self.a = self.h; },
            0x7D => { self.a = self.l; },
            0x7E => { self.a = self.memory.read_byte(self.hl()); },
            0x7F => { self.a = self.a; },

            0x80 => {
                let val = self.b;
                self.add(val);
            },
            0x81 => {
                let val = self.c;
                self.add(val);
            },
            0x82 => {
                let val = self.d;
                self.add(val);
            },
            0x83 => {
                let val = self.e;
                self.add(val);
            },
            0x84 => {
                let val = self.h;
                self.add(val);
            },
            0x85 => {
                let val = self.l;
                self.add(val);
            },
            0x86 => {
                let val = self.memory.read_byte(self.hl());
                self.add(val);
            },
            0x87 => {
                let val = self.a;
                self.add(val);
            },

            0x88 => {
                let val = self.b;
                self.adc(val);
            },
            0x89 => {
                let val = self.c;
                self.adc(val);
            },
            0x8A => {
                let val = self.d;
                self.adc(val);
            },
            0x8B => {
                let val = self.e;
                self.adc(val);
            },
            0x8C => {
                let val = self.h;
                self.adc(val);
            },
            0x8D => {
                let val = self.l;
                self.adc(val);
            },
            0x8E => {
                let val = self.memory.read_byte(self.hl());
                self.adc(val);
            },
            0x8F => {
                let val = self.a;
                self.adc(val);
            },

            0x90 => {
                let val = self.b;
                self.sub(val);
            },
            0x91 => {
                let val = self.c;
                self.sub(val);
            },
            0x92 => {
                let val = self.d;
                self.sub(val);
            },
            0x93 => {
                let val = self.e;
                self.sub(val);
            },
            0x94 => {
                let val = self.h;
                self.sub(val);
            },
            0x95 => {
                let val = self.l;
                self.sub(val);
            },
            0x96 => {
                let val = self.memory.read_byte(self.hl());
                self.sub(val);
            },
            0x97 => {
                let val = self.a;
                self.sub(val);
            },

            0x98 => {
                let val = self.b;
                self.sbc(val);
            },
            0x99 => {
                let val = self.c;
                self.sbc(val);
            },
            0x9A => {
                let val = self.d;
                self.sbc(val);
            },
            0x9B => {
                let val = self.e;
                self.sbc(val);
            },
            0x9C => {
                let val = self.h;
                self.sbc(val);
            },
            0x9D => {
                let val = self.l;
                self.sbc(val);
            },
            0x9E => {
                let val = self.memory.read_byte(self.hl());
                self.sbc(val);
            },
            0x9F => {
                let val = self.a;
                self.sbc(val);
            },

            0xA0 => {
                let val = self.b;
                self.and(val);
            },
            0xA1 => {
                let val = self.c;
                self.and(val);
            },
            0xA2 => {
                let val = self.d;
                self.and(val);
            },
            0xA3 => {
                let val = self.e;
                self.and(val);
            },
            0xA4 => {
                let val = self.h;
                self.and(val);
            },
            0xA5 => {
                let val = self.l;
                self.and(val);
            },
            0xA6 => {
                let val = self.memory.read_byte(self.hl());
                self.and(val);
            },
            0xA7 => {
                let val = self.a;
                self.and(val);
            },

            0xA8 => {
                let val = self.b;
                self.xor(val);
            },
            0xA9 => {
                let val = self.c;
                self.xor(val);
            },
            0xAA => {
                let val = self.d;
                self.xor(val);
            },
            0xAB => {
                let val = self.e;
                self.xor(val);
            },
            0xAC => {
                let val = self.h;
                self.xor(val);
            },
            0xAD => {
                let val = self.l;
                self.xor(val);
            },
            0xAE => {
                let val = self.memory.read_byte(self.hl());
                self.xor(val);
            },
            0xAF => {
                let val = self.a;
                self.xor(val);
                println!("  a: {:#04X}", self.a);
                println!("  f: {:#08t}", self.f);
            },

            0xB0 => {
                let val = self.b;
                self.or(val);
            },
            0xB1 => {
                let val = self.c;
                self.or(val);
            },
            0xB2 => {
                let val = self.d;
                self.or(val);
            },
            0xB3 => {
                let val = self.e;
                self.or(val);
            },
            0xB4 => {
                let val = self.h;
                self.or(val);
            },
            0xB5 => {
                let val = self.l;
                self.or(val);
            },
            0xB6 => {
                let val = self.memory.read_byte(self.hl());
                self.or(val);
            },
            0xB7 => {
                let val = self.a;
                self.or(val);
            },

            0xB8 => {
                let val = self.b;
                self.cp(val);
            },
            0xB9 => {
                let val = self.c;
                self.cp(val);
            },
            0xBA => {
                let val = self.d;
                self.cp(val);
            },
            0xBB => {
                let val = self.e;
                self.cp(val);
            },
            0xBC => {
                let val = self.h;
                self.cp(val);
            },
            0xBD => {
                let val = self.l;
                self.cp(val);
            },
            0xBE => {
                let val = self.memory.read_byte(self.hl());
                self.cp(val);
            },
            0xBF => {
                let val = self.a;
                self.cp(val);
                println!("  a: {:#04X}", self.a);
                println!("  f: {:#08t}", self.f);
            },

            0xC3 => {
                self.pc = self.read_next_word();
                println!("  address: {:#06X}", self.pc);
            },
            0xE0 => {
                let addr = 0xFF00 + self.read_next_byte() as u16;
                self.memory.write_byte(addr, self.a);
                println!("  memory[{:#06X}] = A ({:#04X})", addr, self.a);
            },
            0xEE => {
                let val = self.read_next_byte();
                self.xor(val);
            },
            0xF0 => {
                let addr = 0xFF00 + self.read_next_byte() as u16;
                self.a = self.memory.read_byte(addr);
                println!("  A = memory[{:#06X}] ({:#04X})", addr, self.a);
            },
            0xF3 => {
                println!("  implement interrupts stuff...");
            },
            0xFA => {
                let addr = self.read_next_word();
                self.a = self.memory.read_byte(addr);
            },
            _ => fail!("Opcode not implemented: {:#04X}", opcode)
        }

        self.cycles += CPU_CYCLES[opcode as uint];
        println!("  cycles: {}", self.cycles);
    }

    fn read_next_byte(&mut self) -> u8 {
        let value = self.memory.read_byte(self.pc);
        self.pc += 1;
        value
    }

    fn read_next_word(&mut self) -> u16 {
        let value = self.memory.read_word(self.pc);
        self.pc += 2;
        value
    }

    fn add(&mut self, value: u8) {
        let result = self.a + value;
        self.f = 0x0000;
        if result == 0 { self.f |= Z_FLAG };
        if (self.a & 0xF + value & 0xF) > 0xF { self.f |= H_FLAG }
        if (self.a as u16 + value as u16) > 0xFF { self.f |= C_FLAG }
        self.a = result;
    }

    fn adc(&mut self, value: u8) {
        let c = (self.f & C_FLAG) >> 4;
        let result = self.a + value + c;
        self.f = 0x0000;
        if result == 0 { self.f |= Z_FLAG };
        if (self.a & 0xF + value & 0xF + c) > 0xF { self.f |= H_FLAG }
        if (self.a as u16 + value as u16 + c as u16) > 0xFF { self.f |= C_FLAG }
        self.a = result;
    }

    fn sub(&mut self, value: u8) {
        let result = self.a - value;
        self.f = N_FLAG;
        if result == 0 { self.f |= Z_FLAG };
        if self.a & 0xF < value & 0xF { self.f |= H_FLAG };
        if self.a < value { self.f |= C_FLAG }
        self.a = result;
    }

    fn sbc(&mut self, value: u8) {
        let c = (self.f & C_FLAG) >> 4;
        let result = self.a - value - c;
        self.f = N_FLAG;
        if result == 0 { self.f |= Z_FLAG };
        if self.a & 0xF < (value & 0xF + c) { self.f |= H_FLAG };
        if (self.a as u16) < (value as u16 + c as u16) { self.f |= C_FLAG };
        self.a = result;
    }

    fn and(&mut self, value: u8) {
        self.a &= value;
        self.f = H_FLAG;
        if self.a == 0 { self.f |= Z_FLAG };
    }

    fn or(&mut self, value: u8) {
        self.a |= value;
        self.f = if self.a == 0 { Z_FLAG } else { 0x0000 };
    }

    fn xor(&mut self, value: u8) {
        self.a ^= value;
        self.f = if self.a == 0 { Z_FLAG } else { 0x0000 };
    }

    fn cp(&mut self, value: u8) {
        let result = self.a - value;
        self.f = N_FLAG;
        if result == 0 { self.f |= Z_FLAG };
        if self.a & 0xF < value & 0xF { self.f |= H_FLAG };
        if self.a < value { self.f |= C_FLAG }
    }

    fn bc(&self) -> u16 {
        self.b as u16 << 8 | self.c as u16
    }

    fn de(&self) -> u16 {
        self.d as u16 << 8 | self.e as u16
    }

    fn hl(&self) -> u16 {
        self.h as u16 << 8 | self.l as u16
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
}
