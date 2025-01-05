use super::{codegen, lexer::Instruction};
use std::collections::HashMap;

pub enum Destination {
    RegisterR(u8),
    RegisterA,
    RegisterB,
    Label(String),
}

pub enum Source {
    RegisterR(u8),
    RegisterA,
    RegisterB,
    Label(String),
    Immediate(u8),
}

pub struct IPContext {
    pub cg: Vec<u8>,
    pub raw: Vec<Instruction>,
    pub lb: HashMap<String, usize>,
    pub future_addrs: Vec<(usize, String, bool /* isAbsoluteAddress? */)>,
}

impl IPContext {
    pub fn new(raw: Vec<Instruction>) -> IPContext {
        IPContext {
            cg: Vec::new(),
            raw,
            lb: HashMap::new(),
            future_addrs: Vec::new(),
        }
    }

    pub fn parse_address(&self, op: &str, fa: &mut String) -> u8 {
        if self.lb.contains_key(op) {
            self.lb[op] as u8
        } else if op.starts_with("#") {
            parse_number(&op[1..])
        } else {
            // panic!("invalid address");
            *fa = String::from(op.to_string());
            0
        }
    }

    pub fn parse_address_16(&self, op: &str, fa: &mut String) -> u16 {
        if self.lb.contains_key(op) {
            self.lb[op] as u16
        } else if op.starts_with("#") {
            parse_number(&op[1..]) as u16
        } else {
            // panic!("invalid address");
            *fa = String::from(op.to_string());
            0
        }
    }

    pub fn run(&mut self) {
        let mut pc: u16 = 0;

        for ins in self.raw.iter() {
            self.lb.insert(String::from("$"), pc as usize);

            match ins {
                Instruction::OneArg { name, op, line } => match name.as_str() {
                    "org" => {
                        let addr = parse_number_16(op);

                        if addr >= pc {
                            for _ in 0..(addr - pc) {
                                self.cg.push(0);
                                pc += 1;
                            }
                        } else {
                            panic!("Invalid address for `org` directive");
                        }
                    }
                    "sjmp" => {
                        let mut fa: String = String::new();
                        let mut addr = self.parse_address(op, &mut fa);

                        if fa.len() > 0 {
                            self.future_addrs
                                .push((self.cg.len() + 1 as usize, fa, false));
                        }

                        if addr < pc as u8 + 2 {
                            addr = (pc as u8 + 2) - addr;
                            addr = !addr;
                            addr += 1;
                        } else {
                            addr = addr - (pc as u8 + 2);
                        }

                        self.cg.append(&mut codegen::sjmp(addr as u8));
                        pc += 2;
                    }
                    "ljmp" => {
                        let mut fa: String = String::new();
                        let addr = self.parse_address_16(op, &mut fa);

                        if fa.len() > 0 {
                            self.future_addrs
                                .push((self.cg.len() + 1 as usize, fa, true));
                        }

                        self.cg.append(&mut codegen::ljmp(addr));
                        pc += 3;
                    }
                    _ => (),
                },

                Instruction::TwoArg {
                    name,
                    op1,
                    op2,
                    line,
                } => match name.as_str() {
                    "mov" => {
                        let dest = parse_destination(op1);
                        let src = parse_source(op2);

                        match dest {
                            Destination::RegisterR(rn) => match src {
                                Source::Immediate(data) => {
                                    self.cg.append(&mut codegen::mov_rn_data(rn, data));
                                }
                                Source::RegisterA => {
                                    self.cg.append(&mut codegen::mov_rn_a(rn));
                                }
                                Source::RegisterB => {
                                    // TODO: implement
                                    self.cg.append(&mut codegen::mov_rn_b(rn));
                                }
                                Source::RegisterR(rm) => {
                                    self.cg.append(&mut codegen::mov_rn_rn(rn, rm));
                                }
                                Source::Label(name) => {
                                    // TODO: implement
                                }
                            },
                            Destination::RegisterA => match src {
                                Source::Immediate(data) => {
                                    self.cg.append(&mut codegen::mov_a_data(data));
                                }
                                Source::RegisterR(rn) => {
                                    self.cg.append(&mut codegen::mov_a_rn(rn));
                                }
                                Source::RegisterB => {
                                    self.cg.append(&mut codegen::mov_a_b());
                                }
                                Source::Label(_name) => {
                                    // TODO: implement
                                }
                                Source::RegisterA => {
                                    panic!("Invalid source for `mov` instruction (mov A, A)");
                                }
                            },
                            Destination::RegisterB => match src {
                                Source::Immediate(data) => {
                                    self.cg.append(&mut codegen::mov_b_data(data));
                                }
                                Source::RegisterR(rn) => {
                                    self.cg.append(&mut codegen::mov_b_rn(rn));
                                }
                                Source::RegisterB => {
                                    panic!("Invalid source for `mov` instruction (mov B, B)");
                                }
                                Source::Label(_name) => {
                                    // TODO: implement
                                }
                                Source::RegisterA => {
                                    self.cg.append(&mut codegen::mov_b_a());
                                }
                            },
                            _ => {}
                        }

                        pc += 2;
                    }
                    "add" => {
                        let dest = parse_destination(op1);
                        let src = parse_source(op2);

                        assert!(matches!(dest, Destination::RegisterA));

                        match src {
                            Source::RegisterR(rn) => {
                                self.cg.append(&mut codegen::add_a_rn(rn));
                            }
                            Source::Immediate(data) => {
                                self.cg.append(&mut codegen::add_a_data(data));
                            }
                            Source::RegisterA => {
                                self.cg.append(&mut codegen::add_a_a());
                            }
                            Source::RegisterB => {
                                self.cg.append(&mut codegen::add_a_b());
                            }
                            Source::Label(_name) => {}
                        }

                        pc += 2;
                    }
                    _ => (),
                },

                Instruction::Label { name, line } => {
                    self.lb.insert(name.clone(), pc as usize);
                }

                Instruction::End { line } => {
                    self.cg.push(0);
                }
                _ => (),
            }
        }

        for (addr, name, isAbs) in self.future_addrs.iter() {
            let a = self.lb[name] as usize;

            if *isAbs {
                self.cg[*addr] = (a >> 8) as u8;
                self.cg[*addr + 1] = a as u8;
            } else {
                self.cg[*addr] = (a - *addr) as u8;
            }
        }
    }
}

pub fn parse_number(s: &str) -> u8 {
    let mut isneg = false;
    if s[0..1].eq("-") {
        isneg = true;
        panic!("negative numbers are not supported.");
    }

    let mut p = s;
    if isneg {
        p = &p[1..];
    }

    let mut res: u8;
    if p.ends_with('H') || p.ends_with('h') {
        res = u8::from_str_radix(&s[..p.len() - 1], 16).unwrap()
    } else if p.ends_with('B') || p.ends_with('b') {
        res = u8::from_str_radix(&s[..p.len() - 1], 2).unwrap()
    } else if p.ends_with('O') || p.ends_with('o') {
        res = u8::from_str_radix(&s[..p.len() - 1], 8).unwrap()
    } else if p.ends_with('D') || p.ends_with('d') {
        res = u8::from_str_radix(&s[..p.len() - 1], 10).unwrap()
    } else {
        res = p.parse::<u8>().unwrap()
    }

    // if isneg {
    //     res = res - 1;
    //     res = !res;

    //     if res < 128 {
    //         panic!("negative number out of 8-bit bounds.");
    //     }
    // } else {
    //     if res > 127 {
    //         panic!("number out of 8-bit bounds.");
    //     }
    // }

    res
}

pub fn parse_number_16(s: &str) -> u16 {
    let mut isneg = false;
    if s[0..1].eq("-") {
        isneg = true;
        panic!("negative numbers are not supported.");
    }

    let mut p = s;
    if isneg {
        p = &p[1..];
    }

    let mut res: u16;
    if p.ends_with('H') || p.ends_with('h') {
        res = u16::from_str_radix(&s[..p.len() - 1], 16).unwrap()
    } else if p.ends_with('B') || p.ends_with('b') {
        res = u16::from_str_radix(&s[..p.len() - 1], 2).unwrap()
    } else if p.ends_with('O') || p.ends_with('o') {
        res = u16::from_str_radix(&s[..p.len() - 1], 8).unwrap()
    } else if p.ends_with('D') || p.ends_with('d') {
        res = u16::from_str_radix(&s[..p.len() - 1], 10).unwrap()
    } else {
        res = p.parse::<u16>().unwrap()
    }

    // if isneg {
    //     res = res - 1;
    //     res = !res;

    //     if res < 128 {
    //         panic!("negative number out of 8-bit bounds.");
    //     }
    // } else {
    //     if res > 127 {
    //         panic!("number out of 8-bit bounds.");
    //     }
    // }

    res
}

pub fn parse_destination(s: &str) -> Destination {
    if s.starts_with('R') || s.starts_with('r') {
        Destination::RegisterR(s[1..].parse::<u8>().unwrap())
    } else if s.eq("A") || s.eq("a") {
        Destination::RegisterA
    } else if s.eq("B") || s.eq("b") {
        Destination::RegisterB
    } else {
        Destination::Label(s.to_string())
    }
}

pub fn parse_source(s: &str) -> Source {
    if s.starts_with("#") {
        Source::Immediate(parse_number(&s[1..]))
    } else if s.starts_with('R') || s.starts_with('r') {
        Source::RegisterR(s[1..].parse::<u8>().unwrap())
    } else if s.eq("A") || s.eq("a") {
        Source::RegisterA
    } else if s.eq("B") || s.eq("b") {
        Source::RegisterB
    } else {
        Source::Label(s.to_string())
    }
}
