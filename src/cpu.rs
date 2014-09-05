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
        let opcode = self.memory.read_byte(self.pc);
        self.pc += 1;
        self.cycles += 4;

        println!("Executing: {:#04X}", opcode);

        match opcode {
            0x00 => {},
            0xC3 => {
                self.pc = self.memory.read_byte(self.pc).to_u16().unwrap() |
                    self.memory.read_byte(self.pc + 1).to_u16().unwrap() << 8;
                println!("  address: {:#06X}", self.pc);
                self.cycles += 12;
            },
            _ => fail!("Opcode not implemented: {:#04X}", opcode)
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
}
