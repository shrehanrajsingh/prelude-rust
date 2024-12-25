use super::{codegen, lexer::Instruction};
use core::panic;
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
}

impl IPContext {
    pub fn new(raw: Vec<Instruction>) -> IPContext {
        IPContext {
            cg: Vec::new(),
            raw,
            lb: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let mut pc = 0;

        for ins in self.raw.iter() {
            match ins {
                Instruction::OneArg { name, op, line } => match name.as_str() {
                    "org" => {
                        let addr = parse_number(&op);

                        if addr >= pc as u8 {
                            for _ in 0..(addr - pc) {
                                self.cg.push(0);
                            }
                        } else {
                            panic!("Invalid address for `org` directive");
                        }
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
                        let dest = parse_destination(&op1);
                        let src = parse_source(&op2);

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
                                    // self.cg.append(&mut codegen::mov_rn_b(rn));
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
                                Source::Label(name) => {
                                    // TODO: implement
                                }
                                Source::RegisterA => {
                                    panic!("Invalid source for `mov` instruction (mov A, A)");
                                }
                            },
                            _ => {}
                        }
                    }
                    "add" => {
                        let dest = parse_destination(&op1);
                        let src = parse_source(&op2);

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
                            Source::Label(name) => {}
                        }
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

            pc += 1;
        }
    }
}

pub fn parse_number(s: &str) -> u8 {
    if s.ends_with('H') || s.ends_with('h') {
        u8::from_str_radix(&s[..s.len() - 1], 16).unwrap()
    } else if s.ends_with('B') || s.ends_with('b') {
        u8::from_str_radix(&s[..s.len() - 1], 2).unwrap()
    } else if s.ends_with('O') || s.ends_with('o') {
        u8::from_str_radix(&s[..s.len() - 1], 8).unwrap()
    } else if s.ends_with('D') || s.ends_with('d') {
        u8::from_str_radix(&s[..s.len() - 1], 10).unwrap()
    } else {
        s.parse::<u8>().unwrap()
    }
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
