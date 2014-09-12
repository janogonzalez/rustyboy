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
    8,12, 8, 8,12,12,12, 0, 8, 8, 8, 8, 4, 4, 8, 0, // 0x30
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x40
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x50
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x60
    8, 8, 8, 8, 8, 8, 0, 8, 4, 4, 4, 4, 4, 4, 8, 4, // 0x70
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x80
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0x90
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0xA0
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, // 0xB0
    8,16,12,16, 0,16, 8, 0, 8,16,12, 0, 0, 0, 8, 0, // 0xC0
    8,16,12, 0, 0,16, 8, 0, 8, 0,12, 0, 0, 0, 8, 0, // 0xD0
   12,16, 8, 0, 0,16, 8, 0, 0, 4,16, 0, 0, 0, 8, 0, // 0xE0
   12,16, 8, 4, 0,16, 8, 0, 0, 0,16, 4, 0, 0, 8, 0, // 0xF0
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

    fn step(&mut self) {
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

                if self.is_set(C_FLAG) { self.a |= 0x01 }

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

                if self.is_set(C_FLAG) { self.a |= 0x80 }

                self.set_flag(Z_FLAG, false);
                self.set_flag(N_FLAG, false);
                self.set_flag(H_FLAG, false);
                self.set_flag(C_FLAG, old & 0x01 == 0x01);
            },
            0x20 => { // JR NZ,+/-n
                let incr = self.read_next_byte() as i8;
                if !self.is_set(Z_FLAG) {
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
                if self.is_set(Z_FLAG) {
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
                if !self.is_set(C_FLAG) {
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
            0x35 => { // DEC (HL)
                let addr = self.hl();
                let val = self.memory.read_byte(addr);
                let dec = self.dec(val);
                self.memory.write_byte(addr, dec);
            },
            0x36 => { // LD (HL),n
                let addr = self.hl();
                let val = self.read_next_byte();
                self.memory.write_byte(addr, val);
            },

            0x38 => { // JR C,+/-n
                let incr = self.read_next_byte() as i8;
                if self.is_set(C_FLAG) {
                    self.pc = (self.pc as i16 + incr as i16) as u16;
                    self.cycles += 4;
                }
            },
            0x39 => { // ADD HL,SP
                let val = self.sp;
                self.add_hl(val);
            },
            0x3A => { // LD A,(HL-)
                let addr = self.hl();
                self.a = self.memory.read_byte(addr);
                self.set_hl(addr - 1);
            },
            0x3B => { // DEC SP
                self.sp -= 1;
            },
            0x3C => { // INC A
                let val = self.a;
                self.a = self.inc(val);
            },
            0x3D => { // DEC A
                let val = self.a;
                self.a = self.dec(val);
            },
            0x3E => { // LD A,n
                self.a = self.read_next_byte();
            },

            // LD B,...
            0x40 => { self.b = self.b; },
            0x41 => { self.b = self.c; },
            0x42 => { self.b = self.d; },
            0x43 => { self.b = self.e; },
            0x44 => { self.b = self.h; },
            0x45 => { self.b = self.l; },
            0x46 => { self.b = self.memory.read_byte(self.hl()); },
            0x47 => { self.b = self.a; },
            // LD C,...
            0x48 => { self.c = self.b; },
            0x49 => { self.c = self.c; },
            0x4A => { self.c = self.d; },
            0x4B => { self.c = self.e; },
            0x4C => { self.c = self.h; },
            0x4D => { self.c = self.l; },
            0x4E => { self.c = self.memory.read_byte(self.hl()); },
            0x4F => { self.c = self.a; },
            // LD D,...
            0x50 => { self.d = self.b; },
            0x51 => { self.d = self.c; },
            0x52 => { self.d = self.d; },
            0x53 => { self.d = self.e; },
            0x54 => { self.d = self.h; },
            0x55 => { self.d = self.l; },
            0x56 => { self.d = self.memory.read_byte(self.hl()); },
            0x57 => { self.d = self.a; },
            // LD E,...
            0x58 => { self.e = self.b; },
            0x59 => { self.e = self.c; },
            0x5A => { self.e = self.d; },
            0x5B => { self.e = self.e; },
            0x5C => { self.e = self.h; },
            0x5D => { self.e = self.l; },
            0x5E => { self.e = self.memory.read_byte(self.hl()); },
            0x5F => { self.e = self.a; },
            // LD H,...
            0x60 => { self.h = self.b; },
            0x61 => { self.h = self.c; },
            0x62 => { self.h = self.d; },
            0x63 => { self.h = self.e; },
            0x64 => { self.h = self.h; },
            0x65 => { self.h = self.l; },
            0x66 => { self.h = self.memory.read_byte(self.hl()); },
            0x67 => { self.h = self.a; },
            // LD L,...
            0x68 => { self.l = self.b; },
            0x69 => { self.l = self.c; },
            0x6A => { self.l = self.d; },
            0x6B => { self.l = self.e; },
            0x6C => { self.l = self.h; },
            0x6D => { self.l = self.l; },
            0x6E => { self.l = self.memory.read_byte(self.hl()); },
            0x6F => { self.l = self.a; },
            0x70 => { // LD (HL),B
                let addr = self.hl();
                self.memory.write_byte(addr, self.b);
            },
            0x71 => { // LD (HL),C
                let addr = self.hl();
                self.memory.write_byte(addr, self.c);
            },
            0x72 => { // LD (HL),D
                let addr = self.hl();
                self.memory.write_byte(addr, self.d);
            },
            0x73 => { // LD (HL),E
                let addr = self.hl();
                self.memory.write_byte(addr, self.e);
            },
            0x74 => { // LD (HL),H
                let addr = self.hl();
                self.memory.write_byte(addr, self.h);
            },
            0x75 => { // LD (HL),L
                let addr = self.hl();
                self.memory.write_byte(addr, self.l);
            },

            0x77 => { // LD (HL),A
                let addr = self.hl();
                self.memory.write_byte(addr, self.a);
            },
            // LD A,...
            0x78 => { self.a = self.b; },
            0x79 => { self.a = self.c; },
            0x7A => { self.a = self.d; },
            0x7B => { self.a = self.e; },
            0x7C => { self.a = self.h; },
            0x7D => { self.a = self.l; },
            0x7E => { self.a = self.memory.read_byte(self.hl()); },
            0x7F => { self.a = self.a; },

            0x80 => { // ADD A,B
                let val = self.b;
                self.add_a(val);
            },
            0x81 => { // ADD A,C
                let val = self.c;
                self.add_a(val);
            },
            0x82 => { // ADD A,D
                let val = self.d;
                self.add_a(val);
            },
            0x83 => { // ADD A,E
                let val = self.e;
                self.add_a(val);
            },
            0x84 => { // ADD A,H
                let val = self.h;
                self.add_a(val);
            },
            0x85 => { // ADD A,L
                let val = self.l;
                self.add_a(val);
            },
            0x86 => { // ADD A,(HL)
                let val = self.memory.read_byte(self.hl());
                self.add_a(val);
            },
            0x87 => { // ADD A,A
                let val = self.a;
                self.add_a(val);
            },
            0x88 => { // ADC A,B
                let val = self.b;
                self.adc_a(val);
            },
            0x89 => { // ADC A,C
                let val = self.c;
                self.adc_a(val);
            },
            0x8A => { // ADC A,D
                let val = self.d;
                self.adc_a(val);
            },
            0x8B => { // ADC A,E
                let val = self.e;
                self.adc_a(val);
            },
            0x8C => { // ADC A,H
                let val = self.h;
                self.adc_a(val);
            },
            0x8D => { // ADC A,L
                let val = self.l;
                self.adc_a(val);
            },
            0x8E => { // ADC A,(HL)
                let val = self.memory.read_byte(self.hl());
                self.adc_a(val);
            },
            0x8F => { // ADC A,A
                let val = self.a;
                self.adc_a(val);
            },
            0x90 => { // SUB A,B
                let val = self.b;
                self.sub_a(val);
            },
            0x91 => { // SUB A,C
                let val = self.c;
                self.sub_a(val);
            },
            0x92 => { // SUB A,D
                let val = self.d;
                self.sub_a(val);
            },
            0x93 => { // SUB A,E
                let val = self.e;
                self.sub_a(val);
            },
            0x94 => { // SUB A,H
                let val = self.h;
                self.sub_a(val);
            },
            0x95 => { // SUB A,L
                let val = self.l;
                self.sub_a(val);
            },
            0x96 => { // SUB A,(HL)
                let val = self.memory.read_byte(self.hl());
                self.sub_a(val);
            },
            0x97 => { // SUB A,A
                let val = self.a;
                self.sub_a(val);
            },
            0x98 => { // SBC A,B
                let val = self.b;
                self.sbc_a(val);
            },
            0x99 => { // SBC A,C
                let val = self.c;
                self.sbc_a(val);
            },
            0x9A => { // SBC A,D
                let val = self.d;
                self.sbc_a(val);
            },
            0x9B => { // SBC A,E
                let val = self.e;
                self.sbc_a(val);
            },
            0x9C => { // SBC A,H
                let val = self.h;
                self.sbc_a(val);
            },
            0x9D => { // SBC A,L
                let val = self.l;
                self.sbc_a(val);
            },
            0x9E => { // SBC A,(HL)
                let val = self.memory.read_byte(self.hl());
                self.sbc_a(val);
            },
            0x9F => { // SBC A,A
                let val = self.a;
                self.sbc_a(val);
            },
            0xA0 => { // AND A,B
                let val = self.b;
                self.and_a(val);
            },
            0xA1 => { // AND A,C
                let val = self.c;
                self.and_a(val);
            },
            0xA2 => { // AND A,D
                let val = self.d;
                self.and_a(val);
            },
            0xA3 => { // AND A,D
                let val = self.e;
                self.and_a(val);
            },
            0xA4 => { // AND A,H
                let val = self.h;
                self.and_a(val);
            },
            0xA5 => { // AND A,L
                let val = self.l;
                self.and_a(val);
            },
            0xA6 => { // AND A,(HL)
                let val = self.memory.read_byte(self.hl());
                self.and_a(val);
            },
            0xA7 => { // AND A,A
                let val = self.a;
                self.and_a(val);
            },
            0xA8 => { // XOR A,B
                let val = self.b;
                self.xor_a(val);
            },
            0xA9 => { // XOR A,C
                let val = self.c;
                self.xor_a(val);
            },
            0xAA => { // XOR A,D
                let val = self.d;
                self.xor_a(val);
            },
            0xAB => { // XOR A,E
                let val = self.e;
                self.xor_a(val);
            },
            0xAC => { // XOR A,H
                let val = self.h;
                self.xor_a(val);
            },
            0xAD => { // XOR A,L
                let val = self.l;
                self.xor_a(val);
            },
            0xAE => { // XOR A,(HL)
                let val = self.memory.read_byte(self.hl());
                self.xor_a(val);
            },
            0xAF => { // XOR A,A
                let val = self.a;
                self.xor_a(val);
            },
            0xB0 => { // OR A,B
                let val = self.b;
                self.or_a(val);
            },
            0xB1 => { // OR A,C
                let val = self.c;
                self.or_a(val);
            },
            0xB2 => { // OR A,D
                let val = self.d;
                self.or_a(val);
            },
            0xB3 => { // OR A,E
                let val = self.e;
                self.or_a(val);
            },
            0xB4 => { // OR A,H
                let val = self.h;
                self.or_a(val);
            },
            0xB5 => { // OR A,L
                let val = self.l;
                self.or_a(val);
            },
            0xB6 => { // OR A,(HL)
                let val = self.memory.read_byte(self.hl());
                self.or_a(val);
            },
            0xB7 => { // OR A,A
                let val = self.a;
                self.or_a(val);
            },
            0xB8 => { // CP A,B
                let val = self.b;
                self.cp_a(val);
            },
            0xB9 => { // CP A,C
                let val = self.c;
                self.cp_a(val);
            },
            0xBA => { // CP A,D
                let val = self.d;
                self.cp_a(val);
            },
            0xBB => { // CP A,E
                let val = self.e;
                self.cp_a(val);
            },
            0xBC => { // CP A,H
                let val = self.h;
                self.cp_a(val);
            },
            0xBD => { // CP A,L
                let val = self.l;
                self.cp_a(val);
            },
            0xBE => { // CP A,(HL)
                let val = self.memory.read_byte(self.hl());
                self.cp_a(val);
            },
            0xBF => { // CP A,A
                let val = self.a;
                self.cp_a(val);
            },
            0xC0 => { // RET NZ
                if !self.is_set(Z_FLAG) {
                    self.pc = self.memory.read_word(self.sp);
                    self.sp += 2;
                    self.cycles += 12;
                }
            },
            0xC1 => { // POP BC
                let value = self.pop();
                self.set_bc(value);
            },
            0xC2 => { // JP NZ,nn
                let addr = self.read_next_word();

                if !self.is_set(Z_FLAG) {
                    self.pc = addr;
                    self.cycles += 4;
                }
            },
            0xC3 => { // JP nn
                self.pc = self.read_next_word();
            },

            0xC5 => { // PUSH BC
                let value = self.bc();
                self.push(value);
            },
            0xC6 => { // ADD A,n
                let val = self.read_next_byte();
                self.add_a(val);
            },

            0xC8 => { // RET Z
                if self.is_set(Z_FLAG) {
                    self.pc = self.memory.read_word(self.sp);
                    self.sp += 2;
                    self.cycles += 12;
                }
            },
            0xC9 => { // RET
                self.pc = self.memory.read_word(self.sp);
                self.sp += 2;
            },

            0xCA => { // JP Z,nn
                let addr = self.read_next_word();

                if self.is_set(Z_FLAG) {
                    self.pc = addr;
                    self.cycles += 4;
                }
            },

            0xCE => { // ADC A,n
                let val = self.read_next_byte();
                self.adc_a(val);
            },

            0xD0 => { // RET NC
                if !self.is_set(C_FLAG) {
                    self.pc = self.memory.read_word(self.sp);
                    self.sp += 2;
                    self.cycles += 12;
                }
            },
            0xD1 => { // POP DE
                let value = self.pop();
                self.set_de(value);
            },
            0xD2 => { // JP NC,nn
                let addr = self.read_next_word();

                if !self.is_set(C_FLAG) {
                    self.pc = addr;
                    self.cycles += 4;
                }
            },

            0xD5 => { // PUSH DE
                let value = self.de();
                self.push(value);
            },
            0xD6 => { // SUB A,n
                let val = self.read_next_byte();
                self.sub_a(val);
            },

            0xD8 => { // RET C
                if self.is_set(C_FLAG) {
                    self.pc = self.memory.read_word(self.sp);
                    self.sp += 2;
                    self.cycles += 12;
                }
            },

            0xDA => { // JP C,nn
                let addr = self.read_next_word();

                if self.is_set(C_FLAG) {
                    self.pc = addr;
                    self.cycles += 4;
                }
            },

            0xDE => { // SBC A,n
                let val = self.read_next_byte();
                self.sbc_a(val);
            },

            0xE0 => { // LD (0xFF00+n),A
                let addr = 0xFF00 + self.read_next_byte() as u16;
                self.memory.write_byte(addr, self.a);
            },
            0xE1 => { // POP HL
                let value = self.pop();
                self.set_hl(value);
            },
            0xE2 => { // LD (0xFF00+C),A
                let addr = 0xFF00 + self.c as u16;
                self.memory.write_byte(addr, self.a);
            },

            0xE5 => { // PUSH HL
                let value = self.hl();
                self.push(value);
            },
            0xE6 => { // AND A,n
                let val = self.read_next_byte();
                self.and_a(val);
            },

            0xE9 => {
                self.pc = self.hl();
            },
            0xEA => { // LD (nn),A
                let addr = self.read_next_word();
                self.memory.write_byte(addr, self.a);
            }

            0xEE => { // XOR A,n
                let val = self.read_next_byte();
                self.xor_a(val);
            },

            0xF0 => { // LD A,(0xFF00+n)
                let addr = 0xFF00 + self.read_next_byte() as u16;
                self.a = self.memory.read_byte(addr);
            },
            0xF1 => { // POP AF
                let value = self.pop();
                self.set_af(value);
            },
            0xF2 => { // LD A,(0xFF00+C)
                let addr = 0xFF00 + self.c as u16;
                self.a = self.memory.read_byte(addr);
            }
            0xF3 => { // DI
                print!("implement interrupts stuff... ");
            },

            0xF5 => { // PUSH AF
                let value = self.af();
                self.push(value);
            },
            0xF6 => { // OR A,n
                let val = self.read_next_byte();
                self.or_a(val);
            },

            0xFA => { // LD A,(nn)
                let addr = self.read_next_word();
                self.a = self.memory.read_byte(addr);
            },
            0xFB => { // EI
                print!("implement interrupts stuff... ");
            },
            0xFE => { // CP A,n
                let val = self.read_next_byte();
                self.cp_a(val);
            },

            _ => fail!("Opcode not implemented: {:#04X}", opcode)
        }

        self.cycles += CPU_CYCLES[opcode as uint];

        self.print_info();
    }

    fn print_info(&self) {
        let flags = format!("{}{}{}{}",
                            if self.is_set(Z_FLAG) { "Z" } else { "-" },
                            if self.is_set(N_FLAG) { "N" } else { "-" },
                            if self.is_set(H_FLAG) { "H" } else { "-" },
                            if self.is_set(C_FLAG) { "C" } else { "-" });

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

    fn add_a(&mut self, value: u8) {
        let a = self.a;
        let result = a + value;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, (a & 0xF + value & 0xF) > 0xF);
        self.set_flag(C_FLAG, (a as u16 + value as u16) > 0xFF);

        self.a = result;
    }

    fn adc_a(&mut self, value: u8) {
        let a = self.a;
        let c = self.flag_value(C_FLAG);
        let result = a + value + c;

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

    fn sub_a(&mut self, value: u8) {
        let a = self.a;
        let result = a - value;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, true);
        self.set_flag(H_FLAG, a & 0xF < value & 0xF);
        self.set_flag(C_FLAG, a < value);

        self.a = result;
    }

    fn sbc_a(&mut self, value: u8) {
        let a = self.a;
        let c = self.flag_value(C_FLAG);
        let result = a - value - c;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, true);
        self.set_flag(H_FLAG, a & 0xF < (value & 0xF + c));
        self.set_flag(C_FLAG, (a as u16) < (value as u16 + c as u16));

        self.a = result;
    }

    fn and_a(&mut self, value: u8) {
        let result = self.a & value;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, true);
        self.set_flag(C_FLAG, false);

        self.a = result;
    }

    fn or_a(&mut self, value: u8) {
        let result = self.a | value;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, false);
        self.set_flag(C_FLAG, false);

        self.a = result;
    }

    fn xor_a(&mut self, value: u8) {
        let result = self.a ^ value;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, false);
        self.set_flag(C_FLAG, false);

        self.a = result;
    }

    fn cp_a(&mut self, value: u8) {
        let a = self.a;
        let result = a - value;

        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, true);
        self.set_flag(H_FLAG, a & 0xF < value & 0xF);
        self.set_flag(C_FLAG, a < value);
    }

    fn pop(&mut self) -> u16 {
        let result = self.memory.read_word(self.sp);
        self.sp += 2;
        result
    }

    fn push(&mut self, value: u16) {
        self.memory.write_word(self.sp, value);
        self.sp -= 2;
    }

    fn af(&self) -> u16 {
        self.a as u16 << 8 | self.f as u16
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

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
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

    fn is_set(&self, flag: u8) -> bool {
        (self.f & flag) == flag
    }

    fn flag_value(&self, flag: u8) -> u8 {
        if self.is_set(flag) { 0x01 } else { 0x00 }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        };
    }
}
