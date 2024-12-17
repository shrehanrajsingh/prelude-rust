// constants for 8051 RAM
pub const RAM_SIZE: usize = 128;

#[derive(Debug)]
pub struct Ram {
    pub memory: Vec<u8>,
}

pub const BANK_ADDRESSES: [usize; 4] = [0x00, 0x08, 0x10, 0x18];

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: vec![0; RAM_SIZE],
        }
    }

    pub fn write(&mut self, address: usize, data: u8) {
        if address >= RAM_SIZE {
            panic!("RAM address out of bounds: {}", address);
        }
        self.memory[address] = data;
    }

    pub fn read(&self, address: usize) -> u8 {
        self.memory[address]
    }
}
