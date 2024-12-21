use std::collections::HashMap;

use crate::assembler::lexer::LexerContext;

pub struct InstructionParserContext {
    pub cg: Vec<u8>,
    pub raw: Vec<LexerContext>,
    pub lb: HashMap<String, usize>,
}

impl InstructionParserContext {
    pub fn new(cg: Vec<u8>) -> InstructionParserContext {
        InstructionParserContext {
            cg,
            raw: Vec::new(),
            lb: HashMap::new(),
        }
    }

    pub fn run(&mut self, lc: &mut LexerContext) {
        /* TODO */
    }
}
