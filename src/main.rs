mod assembler;
use assembler::*;
mod emulator;
use emulator::*;

fn main() {
    let code = assmeble_brumm("primes", false);
    let mut emulator = BrummCpuEmulator::new();
    emulator.run_until_dont();
}
