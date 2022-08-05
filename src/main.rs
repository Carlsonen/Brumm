mod assembler;
use assembler::*;
mod emulator;
use emulator::*;

fn main() {
    use std::time::Instant;
    let code = assmeble_brumm("sort", false);
    let mut emulator = BrummCpuEmulator::new();
    emulator.set_code(code);

    let now = Instant::now();
    emulator.run_until_dont();
    println!(
        "execution time in microseconds: {}",
        now.elapsed().as_micros()
    );
}
