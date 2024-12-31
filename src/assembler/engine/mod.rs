use crate::psw::PswFlag;
use crate::{emulator::*, psw::Psw};
use crate::{ram, regs};

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
            let bank = (self.em.psw.get_flag(PswFlag::RS1) as usize) << 1
                | (self.em.psw.get_flag(PswFlag::RS0) as usize);

            let bank = ram::BANK_ADDRESSES[bank];

            // println!("opcode: {:x}", opcode);

            match opcode {
                0x00 => break,

                /* mov instruction */
                0x74..=0x7B => {
                    /*
                       mov Rn, #data
                    */
                    let reg = opcode - 0x74;
                    let data = self.em.rom[(*pc + 1) as usize];

                    let addr = bank + reg as usize;
                    // println!("mov R{:x}, #{:x}, {:x}", reg, data, addr);

                    self.em.ram.write(addr, data);
                    *pc += 1;
                }

                0x7C..=0x7F => {
                    /*
                       mov Rn, Rm
                    */
                    let n = (opcode - 0x7C) / 8;
                    let m = (opcode - 0x7C) % 8;

                    let addr_n = bank + n as usize;
                    let addr_m = bank + m as usize;

                    let m_data = self.em.ram.read(addr_m);

                    self.em.ram.write(addr_n, m_data);
                }

                0x84..=0x8B => {
                    /*
                       mov A, Rn
                    */
                    let n = opcode - 0x84;
                    let addr_n = bank + n as usize;

                    let n_data = self.em.ram.read(addr_n);

                    self.em.reg.a.set(n_data);
                }

                0x8C => {
                    /*
                       mov A, #data
                    */
                    let data = self.em.rom[(*pc + 1) as usize];
                    self.em.reg.a.set(data);

                    *pc += 1;
                }

                0x8D..=0x94 => {
                    /*
                       mov Rn, A
                    */
                    let n = opcode - 0x8D;
                    let addr_n = bank + n as usize;

                    self.em.ram.write(addr_n, self.em.reg.a.get());
                }

                0x95..=0x9C => {
                    /*
                       mov B, Rn
                    */
                    let n = opcode - 0x95;
                    let addr_n = bank + n as usize;

                    let data = self.em.ram.read(addr_n);
                    self.em.reg.b.set(data);
                }

                0x9D => {
                    /*
                       mov B, #data
                    */
                    let data = self.em.rom[(*pc + 1) as usize];
                    self.em.reg.b.set(data);

                    *pc += 1;
                }

                0x9E..=0xA5 => {
                    /*
                       mov Rn, B
                    */
                    let n = opcode - 0x9E;
                    let addr_n = bank + n as usize;

                    self.em.ram.write(addr_n, self.em.reg.b.get());
                }

                0xA6 => {
                    self.em.reg.a.set(self.em.reg.b.get());
                }

                0xA7 => {
                    self.em.reg.b.set(self.em.reg.a.get());
                }

                /* add instruction */
                0x50 => {
                    /*
                       add A, #data
                    */
                    let data = self.em.rom[(*pc + 1) as usize];
                    let d_a = self.em.reg.a.get();
                    let d_a = d_a + data;

                    self.em.reg.a.set(d_a);
                    *pc += 1;
                }

                0x51 => {
                    /*
                       add A, B
                    */
                    let d_b = self.em.reg.b.get();
                    let d_a = self.em.reg.a.get();

                    let d_a = d_a + d_b;
                    self.em.reg.a.set(d_a);
                }

                0x52..=0x59 => {
                    /*
                       add A, Rn
                    */
                    let n = opcode - 0x52;
                    let addr_n = bank + n as usize;

                    let data = self.em.ram.read(addr_n);
                    let data = data + self.em.reg.a.get();

                    self.em.reg.a.set(data);
                }

                0x28 => {
                    /*
                       sjmp addr_rel
                    */
                    let addr_rel = self.em.rom[(*pc + 1) as usize] as i8;
                    *pc = (*pc as i16 + 2 + addr_rel as i16) as u16;
                    continue;
                }

                0x29 => {
                    /*
                       ljmp addr
                    */
                    let addr = (self.em.rom[(*pc + 1) as usize] as u16) << 8
                        | self.em.rom[(*pc + 2) as usize] as u16;
                    *pc = addr;
                    continue;
                }

                _ => (),
            }

            *pc += 1;
        }
    }
}
