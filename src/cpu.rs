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
    4,12, 8, 8, 4, 4, 8, 4,20, 8, 8, 8, 4, 4, 8, 4, // 0x00
    0,12, 8, 8, 4, 4, 8, 4,12, 8, 8, 8, 4, 4, 8, 4, // 0x10
    8,12, 8, 8, 4, 4, 8, 0, 8, 8, 8, 8, 4, 4, 8, 0, // 0x20
    8,12, 8, 8,12,12,12, 0, 8, 0, 8, 8, 4, 4, 8, 0, // 0x30
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x40
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x50
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x60
    8, 8, 8, 8, 8, 8, 0, 8, 4, 4, 4, 4, 4, 4, 8, 4, // 0x70
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x80
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x90
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0xA0
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0xB0
    8, 0, 0,16, 0, 0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, // 0xC0
    8, 0, 0, 0, 0, 0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, // 0xD0
   12, 0, 8, 0, 0, 0, 8, 0, 0, 0,16, 0, 0, 0, 8, 0, // 0xE0
   12, 0, 8, 4, 0, 0, 8, 0, 0, 0,16, 4, 0, 0, 8, 0, // 0xF0
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

        print!("Executing: {:#04X} ", opcode);

        match opcode {
            0x00 => { // NOP
            },
            0x01 => { // LD BC,nn
                let val = self.read_next_word();
                self.set_bc(val);
            },
            0x02 => { // LD (BC),A
                let addr = self.bc();
                self.memory.write_byte(addr, self.a);
            },
            0x03 => { // INC BC
                let val = self.bc();
                self.set_bc(val + 1);
            },
            0x04 => { // INC B
                let val = self.b;
                self.b = self.inc(val);
            },
            0x05 => { // DEC B
                let val = self.b;
                self.b = self.dec(val);
            },
            0x06 => { // LD B,n
                self.b = self.read_next_byte();
            },
            0x07 => { // RLCA
                let old = self.a;
                self.a <<= 1;

                self.set_flag(Z_FLAG, false);
                self.set_flag(N_FLAG, false);
                self.set_flag(H_FLAG, false);

                if old & 0x80 == 0x80 {
                    self.a |= 0x01;
                    self.set_flag(C_FLAG, true);
                } else {
                    self.set_flag(C_FLAG, false);
                }
            },
            0x08 => { // LD (nn),SP
                let addr = self.read_next_word();
                self.memory.write_word(addr, self.sp);
            },
            0x09 => { // ADD HL,BC
                let val = self.bc();
                self.add_hl(val);
            },
            0x0A => { // LD A,(BC)
                self.a = self.memory.read_byte(self.bc());
            },
            0x0B => { // DEC BC
                let val = self.bc();
                self.set_bc(val - 1);
            },
            0x0C => { // INC C
                let val = self.c;
                self.c = self.inc(val);
            },
            0x0D => { // DEC C
                let val = self.c;
                self.c = self.dec(val);
            },
            0x0E => { // LD C,n
                self.c = self.read_next_byte();
            },
            0x0F => { // RRCA
                let old = self.a;
                self.a >>= 1;

                self.set_flag(Z_FLAG, false);
                self.set_flag(N_FLAG, false);
                self.set_flag(H_FLAG, false);

                if old & 0x01 == 0x01 {
                    self.a |= 0x80;
                    self.set_flag(C_FLAG, true);
                } else {
                    self.set_flag(C_FLAG, false);
                }
            },

            0x11 => { // LD DE,nn
                let val = self.read_next_word();
                self.set_de(val);
            },
            0x12 => { // LD (DE),A
                let addr = self.de();
                self.memory.write_byte(addr, self.a);
            },
            0x13 => { // INC DE
                let val = self.de();
                self.set_de(val + 1);
            },
            0x14 => { // INC D
                let val = self.d;
                self.d = self.inc(val);
            },
            0x15 => { // DEC D
                let val = self.d;
                self.d = self.dec(val);
            },
            0x16 => { // LD D,n
                self.d = self.read_next_byte();
            },
            0x17 => { // RLA
                let old = self.a;
                self.a <<= 1;

                if self.f & C_FLAG == C_FLAG { self.a |= 0x01 }

                self.set_flag(Z_FLAG, false);
                self.set_flag(N_FLAG, false);
                self.set_flag(H_FLAG, false);
                self.set_flag(C_FLAG, old & 0x80 == 0x80);
            },
            0x18 => { // JR +/-n
                let incr = self.read_next_byte() as i8;
                self.pc = (self.pc as i16 + incr as i16) as u16;
            },
            0x19 => { // ADD HL,DE
                let val = self.de();
                self.add_hl(val);
            },
            0x1A => { // LD A,(DE)
                self.a = self.memory.read_byte(self.de());
            },
            0x1B => { // DEC DE
                let val = self.de();
                self.set_de(val - 1);
            },
            0x1C => { // INC E
                let val = self.e;
                self.e = self.inc(val);
            },
            0x1D => { // DEC E
                let val = self.e;
                self.e = self.dec(val);
            },
            0x1E => { // LD E,n
                self.e = self.read_next_byte();
            },
            0x1F => { // RRA
                let old = self.a;
                self.a >>= 1;

                if self.f & C_FLAG == C_FLAG { self.a |= 0x80 }

                self.set_flag(Z_FLAG, false);
                self.set_flag(N_FLAG, false);
                self.set_flag(H_FLAG, false);
                self.set_flag(C_FLAG, old & 0x01 == 0x01);
            },
            0x20 => { // JR NZ,+/-n
                let incr = self.read_next_byte() as i8;
                if (self.f & Z_FLAG) == 0x00 {
                    self.pc = (self.pc as i16 + incr as i16) as u16;
                    self.cycles += 4;
                }
            },
            0x21 => { // LD HL,nn
                let val = self.read_next_word();
                self.set_hl(val);
            },
            0x22 => { // LD (HL+),A
                let addr = self.hl();
                self.memory.write_byte(addr, self.a);
                self.set_hl(addr + 1);
            },
            0x23 => { // INC HL
                let val = self.hl();
                self.set_hl(val + 1);
            },
            0x24 => { // INC H
                let val = self.h;
                self.h = self.inc(val);
            },
            0x25 => { // DEC H
                let val = self.h;
                self.h = self.dec(val);
            },
            0x26 => { // LD H,n
                self.h = self.read_next_byte();
            },

            0x28 => { // JR Z,+/-n
                let incr = self.read_next_byte() as i8;
                if (self.f & Z_FLAG) == Z_FLAG {
                    self.pc = (self.pc as i16 + incr as i16) as u16;
                    self.cycles += 4;
                }
            },
            0x29 => { // ADD HL,HL
                let val = self.hl();
                self.add_hl(val);
            },
            0x2A => { // LD A,(HL+)
                let addr = self.hl();
                self.a = self.memory.read_byte(addr);
                self.set_hl(addr + 1);
            },
            0x2B => { // DEC HL
                let val = self.hl();
                self.set_hl(val - 1);
            },
            0x2C => { // INC L
                let val = self.l;
                self.l = self.inc(val);
            },
            0x2D => { // DEC L
                let val = self.l;
                self.l = self.dec(val);
            },
            0x2E => { // LD L,n
                self.l = self.read_next_byte();
            },

            0x30 => { // JR NC,+/-n
                let incr = self.read_next_byte() as i8;
                if (self.f & C_FLAG) == 0x00 {
                    self.pc = (self.pc as i16 + incr as i16) as u16;
                    self.cycles += 4;
                }
            },
            0x31 => { // LD SP,nn
                let val = self.read_next_word();
                self.sp = val;
            },
            0x32 => { // LD (HL-),A
                let addr = self.hl();
                self.memory.write_byte(addr, self.a);
                self.set_hl(addr - 1);
            },
            0x33 => { // INC SP
                self.sp += 1;
            },
            0x34 => { // INC (HL)
                let addr = self.hl();
                let val = self.memory.read_byte(addr);
                let inc = self.inc(val);
                self.memory.write_byte(addr, inc);
            },
            0x3B => {
                self.sp -= 1;
            },

            0x3A => {
                let addr = self.hl();
                self.a = self.memory.read_byte(addr);
                self.set_hl(addr - 1);
            },


            0x38 => {
                let incr = self.read_next_byte() as i8;
                if (self.f & C_FLAG) == C_FLAG {
                    self.pc = (self.pc as i16 + incr as i16) as u16;
                    self.cycles += 4;
                }
            },

            0x3C => {
                let val = self.a;
                self.a = self.inc(val);
            },


            0x35 => {
                let addr = self.hl();
                let val = self.memory.read_byte(addr);
                let dec = self.dec(val);
                self.memory.write_byte(addr, dec);
            },
            0x3D => {
                let val = self.a;
                self.a = self.dec(val);
            },

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
            },

            0xC0 => {
                if (self.f & Z_FLAG) == 0x00 {
                    let incr = self.memory.read_word(self.sp);
                    self.pc += incr;
                    self.sp += 2;
                    self.cycles += 12;
                }
            },
            0xC8 => {
                if (self.f & Z_FLAG) == Z_FLAG {
                    let incr = self.memory.read_word(self.sp);
                    self.pc += incr;
                    self.sp += 2;
                    self.cycles += 12;
                }
            },
            0xD0 => {
                if (self.f & C_FLAG) == 0x00 {
                    let incr = self.memory.read_word(self.sp);
                    self.pc += incr;
                    self.sp += 2;
                    self.cycles += 12;
                }
            },
            0xD8 => {
                if (self.f & C_FLAG) == C_FLAG {
                    let incr = self.memory.read_word(self.sp);
                    self.pc += incr;
                    self.sp += 2;
                    self.cycles += 12;
                }
            },

            0xC3 => { self.pc = self.read_next_word(); },
            0xE0 => {
                let addr = 0xFF00 + self.read_next_byte() as u16;
                self.memory.write_byte(addr, self.a);
            },
            0xE2 => {
                let addr = 0xFF00 + self.c as u16;
                self.memory.write_byte(addr, self.a);
            },
            0xEA => {
                let addr = self.read_next_word();
                self.memory.write_byte(addr, self.a);
            }

            0xC6 => {
                let val = self.read_next_byte();
                self.add(val);
            },
            0xCE => {
                let val = self.read_next_byte();
                self.adc(val);
            },
            0xD6 => {
                let val = self.read_next_byte();
                self.sub(val);
            },
            0xDE => {
                let val = self.read_next_byte();
                self.sbc(val);
            },
            0xE6 => {
                let val = self.read_next_byte();
                self.and(val);
            },
            0xEE => {
                let val = self.read_next_byte();
                self.xor(val);
            },
            0xF6 => {
                let val = self.read_next_byte();
                self.or(val);
            },
            0xFE => {
                let val = self.read_next_byte();
                self.cp(val);
            },

            0xF0 => {
                let addr = 0xFF00 + self.read_next_byte() as u16;
                self.a = self.memory.read_byte(addr);
            },
            0xF2 => {
                let addr = 0xFF00 + self.c as u16;
                self.a = self.memory.read_byte(addr);
            }
            0xF3 => {
                print!("implement interrupts stuff... ");
            },
            0xFB => {
                print!("implement interrupts stuff... ");
            },
            0xFA => {
                let addr = self.read_next_word();
                self.a = self.memory.read_byte(addr);
            },
            _ => fail!("Opcode not implemented: {:#04X}", opcode)
        }

        self.cycles += CPU_CYCLES[opcode as uint];

        let flags = format!("{}{}{}{}",
                            if self.f & Z_FLAG == Z_FLAG { "Z" } else { "-" },
                            if self.f & N_FLAG == N_FLAG { "N" } else { "-" },
                            if self.f & H_FLAG == H_FLAG { "H" } else { "-" },
                            if self.f & C_FLAG == C_FLAG { "C" } else { "-" });

        println!("<A = {:#04X}, B = {:#04X}, C = {:#04X}, D = {:#04X} \
                   E = {:#04X}, H = {:#04X}, L = {:#04X}, FLAGS = {} \
                   PC = {:#08X} SP = {:#08X} Cycles = {}>",
                 self.a, self.b, self.c, self.d, self.e, self.h, self.l,
                 flags, self.pc, self.sp, self.cycles);
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

    fn inc(&mut self, value: u8) -> u8 {
        let result = value + 1;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, (value & 0xF) + 1 > 0xF);

        result
    }

    fn dec(&mut self, value: u8) -> u8 {
        let result = value - 1;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, true);
        self.set_flag(H_FLAG, value & 0xF == 0);

        result
    }

    fn add(&mut self, value: u8) {
        let result = self.a + value;
        let a = self.a;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, (a & 0xF + value & 0xF) > 0xF);
        self.set_flag(C_FLAG, (a as u16 + value as u16) > 0xFF);

        self.a = result;
    }

    fn adc(&mut self, value: u8) {
        let c = (self.f & C_FLAG) >> 4;
        let result = self.a + value + c;
        let a = self.a;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, (a & 0xF + value & 0xF + c) > 0xF);
        self.set_flag(C_FLAG, (a as u16 + value as u16 + c as u16) > 0xFF);

        self.a = result;
    }

    fn add_hl(&mut self, value: u16) {
        let result = self.hl() + value;
        let hl = self.hl();

        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, (hl & 0xFFF + value & 0xFFF) > 0xFFF);
        self.set_flag(C_FLAG, (hl as u32 + value as u32) > 0xFFFF);

        self.set_hl(result);
    }

    fn sub(&mut self, value: u8) {
        let result = self.a - value;
        let a = self.a;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, true);
        self.set_flag(H_FLAG, a & 0xF < value & 0xF);
        self.set_flag(C_FLAG, a < value);

        self.a = result;
    }

    fn sbc(&mut self, value: u8) {
        let c = (self.f & C_FLAG) >> 4;
        let result = self.a - value - c;
        let a = self.a;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, true);
        self.set_flag(H_FLAG, a & 0xF < (value & 0xF + c));
        self.set_flag(C_FLAG, (a as u16) < (value as u16 + c as u16));

        self.a = result;
    }

    fn and(&mut self, value: u8) {
        self.a &= value;
        let a = self.a;

        self.set_flag(Z_FLAG, a == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, true);
        self.set_flag(C_FLAG, false);
    }

    fn or(&mut self, value: u8) {
        self.a |= value;
        let a = self.a;

        self.set_flag(Z_FLAG, a == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, false);
        self.set_flag(C_FLAG, false);
    }

    fn xor(&mut self, value: u8) {
        self.a ^= value;
        let a = self.a;

        self.set_flag(Z_FLAG, a == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, false);
        self.set_flag(C_FLAG, false);
    }

    fn cp(&mut self, value: u8) {
        let result = self.a - value;
        let a = self.a;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, true);
        self.set_flag(H_FLAG, a & 0xF < value & 0xF);
        self.set_flag(C_FLAG, a < value);
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

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    fn set_flag(&mut self, flag: u8, set: bool) {
        if set {
            self.f |= flag;
        } else {
            self.f &= !flag;
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        };
    }
}
