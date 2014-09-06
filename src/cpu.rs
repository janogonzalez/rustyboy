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
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x00
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x10
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x20
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, // 0x30
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x40
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x50
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x60
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x70
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x80
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x90
    0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 8, 4, // 0xA0
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xB0
    0, 0, 0,16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xC0
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xD0
   12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, // 0xE0
   12, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xF0
];

static Z_FLAG: u8 = 0b1000_0000;

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
            0x3E => {
                self.a = self.read_next_byte();
                println!("  a: {:#04X}", self.a);
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

    fn xor(&mut self, value: u8) {
        self.a ^= value;
        self.f = if self.a == 0 { Z_FLAG } else { 0x0000 };
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
