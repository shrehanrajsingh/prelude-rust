use crate::psw;
use crate::ram;
use crate::regs;

use regs::Register16;
use regs::Register8;

use psw::Psw;
use ram::Ram;

pub struct AllRegs {
    pub a: regs::Register8,
    pub b: regs::Register8,
    pub dptr: regs::Register16,
    pub pc: regs::Register16,
    pub sp: regs::Register8,
}

pub struct Emulator {
    pub psw: Psw,
    pub ram: Ram,
    pub reg: AllRegs,
    pub rom: Vec<u8>,
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut res = Emulator {
            psw: Psw::new(),
            ram: Ram::new(),
            reg: AllRegs {
                a: Register8::new(),
                b: Register8::new(),
                dptr: Register16::new(),
                pc: Register16::new(),
                sp: Register8::new(),
            },
            rom: Vec::new(),
        };

        /*
           SP is first incremented
           then data is added to that location
           Kind of like
           stack[++sp] = data
        */
        res.reg.sp.set(0x07);

        res
    }

    pub fn burn(&mut self, bytes: Vec<u8>) {
        self.rom = bytes;
    }
}
