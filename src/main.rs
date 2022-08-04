mod assembler;
use assembler::*;
mod emulator;
use emulator::*;

fn main() {
    let code = assmeble_brumm("test");
    let mut emulator = BrummCpuEmulator::new(code);
    emulator.run_until_dont();
}
