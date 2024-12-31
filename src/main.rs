pub mod assembler;
pub mod emulator;
pub mod psw;
pub mod ram;
pub mod regs;

use assembler::codegen;
use assembler::engine;
use assembler::lexer::LexerContext;
use assembler::parser::IPContext;
use emulator::Emulator;

use engine::AsmContext;
use std::fs;

fn main() {
    let mut em = Emulator::new();

    // let mut bytes = Vec::new();

    // bytes.append(&mut codegen::mov_rn_data(0, 0x12)); // mov r0, #12H
    // bytes.append(&mut codegen::mov_rn_data(1, 0x34)); // mov r1, #34H

    // bytes.append(&mut codegen::mov_a_rn(0)); // mov A, R0
    // bytes.append(&mut codegen::add_a_rn(1)); // mov A, R1

    // bytes.push(0); // end

    // em.burn(bytes);

    // let mut asmctx = AsmContext::new(em);
    // asmctx.run();

    // println!("{:?}", asmctx.em.ram);
    // println!("{:?}", asmctx.em.reg);

    let contents = fs::read_to_string("test2.plasm").expect("invalid file path");
    let mut lc = LexerContext::new(contents);

    lc.run();
    let mut pc = IPContext::new(lc.dt);
    pc.run();

    // println!("{:?}", pc.cg);
    let mut asmctx = AsmContext::new(em);
    asmctx.em.burn(pc.cg);
    asmctx.run();

    println!("{:?}", asmctx.em.ram);
    println!("{:?}", asmctx.em.reg);
    println!("{:?}", asmctx.em.rom);
}
