use std::u8;

struct Cpu {
    register_a: u8,
    register_b: u8,
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

fn main() {
    let cpu = Cpu {
        register_a: 0,
        register_b: 0,
        program_counter: 0,
        halted: false,
    };
    println!("Hello, world!");
}
