mod assembler;
use assembler::*;
mod emulator;
use emulator::*;

fn main() {
    let code = assmeble_brumm("primes");
    let mut emulator = BrummCpuEmulator::new(code);
    emulator.run_until_dont();
    println!("{:?}", emulator.get_ramfile());
    println!("{:?}", emulator.get_regfile());
}
