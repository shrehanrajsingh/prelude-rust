use crate::psw::PswFlag;
use crate::ram;
use crate::{emulator::*, psw::Psw};

pub struct AsmContext {
    pub em: Emulator,
}

impl AsmContext {
    pub fn new(em: Emulator) -> AsmContext {
        AsmContext { em }
    }

    pub fn run(&mut self) {
        let mut pc: &mut u16 = &mut self.em.reg.pc.value;

        loop {
            let opcode = self.em.rom[*pc as usize];
            // println!("opcode: {:x}", opcode);

            match opcode {
                0x00 => break,
                0x74..=0x7B => {
                    /*
                       mov Rn, #data
                    */
                    let reg = opcode - 0x74;
                    let data = self.em.rom[(*pc + 1) as usize];

                    let bank = (self.em.psw.get_flag(PswFlag::RS1) as u8) << 1
                        | self.em.psw.get_flag(PswFlag::RS0) as u8;

                    let addr = ram::BANK_ADDRESSES[bank as usize] + reg as usize;
                    // println!("mov R{:x}, #{:x}, {:x}", reg, data, addr);

                    let addr = addr + self.em.reg.sp.get() as usize;

                    self.em.ram.write(addr, data);
                }
                _ => (),
            }

            (*pc) += 1;
        }
    }
}
