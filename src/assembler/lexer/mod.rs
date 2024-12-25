#[derive(Debug)]
pub enum Instruction {
    OneArg {
        name: String,
        op: String,
        line: usize,
    },
    TwoArg {
        name: String,
        op1: String,
        op2: String,
        line: usize,
    },
    Label {
        name: String,
        line: usize,
    },
    End {
        line: usize,
    },
}

#[derive(Debug)]
pub struct LexerContext {
    pub code: String,
    pub dt: Vec<Instruction>,
}

impl LexerContext {
    pub fn new(code: String) -> LexerContext {
        LexerContext {
            code,
            dt: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut lines = self.code.lines();
        let mut curr_line = 0;

        while let Some(line) = lines.next() {
            curr_line += 1;

            if line.is_empty() {
                continue;
            }

            if line[line.len() - 1..].eq(":") {
                let label = line[..line.len() - 1].to_string();
                self.dt.push(Instruction::Label {
                    name: label,
                    line: curr_line,
                });
                continue;
            }

            if line == "end" {
                self.dt.push(Instruction::End { line: curr_line });
                break;
            }

            let mut words = line.split_once(' ');

            match words {
                Some((name, args)) => {
                    let mut args = args.split(",");

                    if args.clone().count() == 1 {
                        let op = args.next().unwrap().trim();

                        self.dt.push(Instruction::OneArg {
                            name: name.to_string(),
                            op: op.to_string(),
                            line: curr_line,
                        });
                    } else {
                        let op1 = args.next().unwrap().trim();
                        let op2 = args.next().unwrap().trim();

                        self.dt.push(Instruction::TwoArg {
                            name: name.to_string(),
                            op1: op1.to_string(),
                            op2: op2.to_string(),
                            line: curr_line,
                        });
                    }
                }

                None => {
                    continue;
                }
            }
        }
    }
}
