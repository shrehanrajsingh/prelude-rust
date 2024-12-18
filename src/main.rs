pub mod assembler;
pub mod emulator;
pub mod psw;
pub mod ram;
pub mod regs;

use assembler::codegen;
use assembler::engine;
use emulator::Emulator;

use engine::AsmContext;

fn main() {
    let mut em = Emulator::new();

    let mut bytes = Vec::new();

    bytes.append(&mut codegen::mov_rn_data(0, 12)); // mov r0, #12H
    bytes.append(&mut codegen::mov_rn_data(1, 34)); // mov r1, #34H

    bytes.append(&mut codegen::mov_a_rn(0));
    bytes.append(&mut codegen::add_a_rn(1));

    bytes.push(0); // mov a, #56H

    em.burn(bytes);

    let mut asmctx = AsmContext::new(em);
    asmctx.run();

    println!("{:?}", asmctx.em.ram);
    println!("{:?}", asmctx.em.reg);
}
