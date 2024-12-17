pub struct Register8 {
    pub value: u8,
}

impl Register8 {
    pub fn new() -> Register8 {
        Register8 { value: 0 }
    }

    pub fn set_bit(&mut self, bit: u8, value: bool) {
        let mask = 1 << bit;
        if value {
            self.value |= mask;
        } else {
            self.value &= !mask;
        }
    }

    pub fn get_bit(&self, bit: u8) -> bool {
        let mask = 1 << bit;
        self.value & mask != 0
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }

    pub fn get(&self) -> u8 {
        self.value
    }
}

pub struct Register16 {
    pub value: u16,
}

impl Register16 {
    pub fn new() -> Register16 {
        Register16 { value: 0 }
    }

    pub fn set_bit(&mut self, bit: u8, value: bool) {
        let mask = 1 << bit;
        if value {
            self.value |= mask;
        } else {
            self.value &= !mask;
        }
    }

    pub fn get_bit(&self, bit: u8) -> bool {
        let mask = 1 << bit;
        self.value & mask != 0
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }

    pub fn get(&self) -> u16 {
        self.value
    }
}
