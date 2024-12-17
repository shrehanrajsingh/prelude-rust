#[derive(Debug)]

pub enum PswFlag {
    P = 0,
    F1 = 1,
    OV = 2,
    RS0 = 3,
    RS1 = 4,
    F0 = 5,
    AC = 6,
    CY = 7,
}

/**
 * The PSW register is a special register that contains the following flags:
 * - CY: Carry flag
 * - AC: Auxiliary carry flag
 * - F0: User-defined flag 0
 * - RS1: Register bank select 1
 * - RS0: Register bank select 0
 * - OV: Overflow flag
 * - F1: User-defined flag 1
 * - P: Parity flag
 * Bits are 0 indexed starting from bottom
 * The PSW register is 8 bits wide
 * value[0] is P
 * value[1] is F1
 * value[2] is OV
 * and so on
 */
pub struct Psw {
    pub value: u8,
}

impl Psw {
    pub fn new() -> Psw {
        Psw { value: 0 }
    }

    pub fn set_flag(&mut self, flag: PswFlag, value: bool) {
        let mask = 1 << flag as u8;
        if value {
            self.value |= mask;
        } else {
            self.value &= !mask;
        }
    }

    pub fn get_flag(&self, flag: PswFlag) -> bool {
        let mask = 1 << flag as u8;
        self.value & mask != 0
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }

    pub fn get(&self) -> u8 {
        self.value
    }
}
