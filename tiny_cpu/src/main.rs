use std::u8;

struct Cpu {
    register_a: u8,
    register_b: u8,
    register_f: u8,
    program_counter: u16,
    halted: bool,
}

struct Memory {
    data: [u8; 65536],
}

impl Memory {
    fn read_byte(&self, address: u16) -> u8 {
        return self.data[address as usize];
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

impl Cpu {
    const ZERO_FLAG: u8 = 0b1000_0000;
    const SUBTRACT_FLAG: u8 = 0b0100_0000;
    const HALF_FLAG: u8 = 0b0010_0000;
    const CARRY_FLAG: u8 = 0b0001_0000;

    fn fetch_byte(&mut self, mem: &Memory) -> u8 {
        let next = mem.read_byte(self.program_counter);
        self.program_counter += 1;
        return next;
    }

    fn fetch_two_bytes(&mut self, mem: &Memory) -> u16 {
        let low = self.fetch_byte(mem);
        let high = self.fetch_byte(mem);

        let new_pc = (high as u16) << 8 | (low as u16);
        return new_pc;
    }

    fn ld_a_immediate(&mut self, mem: &Memory) {
        self.register_a = self.fetch_byte(mem);
    }

    fn ld_b_immediate(&mut self, mem: &Memory) {
        self.register_b = self.fetch_byte(mem);
    }

    fn noop(&self) {}

    fn check_zero(&self) -> bool {
        return self.register_f & Self::ZERO_FLAG == 0;
    }

    fn check_half_carry(&self) -> bool {
        return self.register_f & Self::HALF_FLAG != 0;
    }

    fn set_zero(&mut self, value: bool) {
        if (value == true) {
            self.register_f |= Self::ZERO_FLAG;
        } else {
            self.register_f &= !Self::ZERO_FLAG;
        }
    }

    fn set_substract(&mut self, value: bool) {
        if (value == true) {
            self.register_f |= Self::SUBTRACT_FLAG;
        } else {
            self.register_f &= !Self::SUBTRACT_FLAG;
        }
    }

    fn set_half_carry(&mut self, value: bool) {
        if (value == true) {
            self.register_f |= Self::HALF_FLAG;
        } else {
            self.register_f &= !Self::HALF_FLAG;
        }
    }

    fn set_carry(&mut self, value: bool) {
        if (value == true) {
            self.register_f |= Self::CARRY_FLAG;
        } else {
            self.register_f &= !Self::CARRY_FLAG;
        }
    }

    fn inc_b(&mut self) {
        let half_carry = (self.register_b & 0b0000_1111) == 0b0000_1111;
        self.register_b = self.register_b.wrapping_add(1);
        self.set_half_carry(half_carry);
        self.set_zero(self.register_b == 0);
        self.set_substract(false);
    }

    fn dec_b(&mut self) {
        let half_carry = (self.register_b & 0b0000_0000) == 0b1111_0000;
        self.register_b = self.register_b.wrapping_sub(1);
        self.set_substract(true);
        self.set_zero(self.register_b == 0);
    }

    fn step(&mut self, mem: &Memory) {
        let opcode = self.fetch_byte(mem);
        match opcode {
            0x00 => self.noop(),
            0x3E => self.ld_a_immediate(mem),
            0x06 => self.ld_b_immediate(mem),
            0x04 => self.inc_b(),
            _ => panic!("Unknown instruction: {:#04X}", opcode),
        }
    }
}

fn main() {
    let mut cpu = Cpu {
        register_a: 0,
        register_b: 0,
        register_f: 0,
        program_counter: 0,
        halted: false,
    };

    let mut memory = Memory { data: [0; 65536] };

    memory.data[0] = 0x06;
    memory.data[1] = 0x0F;
    memory.data[2] = 0x04;

    cpu.step(&memory);

    println!("After step 1: B = {}", cpu.register_b);
    println!("PC = {}", cpu.program_counter);

    cpu.step(&memory);

    println!("After step 2: B = {}", cpu.register_b);
    println!("PC = {}", cpu.program_counter);
}
