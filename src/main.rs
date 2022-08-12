mod assembler;
use assembler::*;
mod emulator;
use emulator::*;
mod optimizer;
use optimizer::*;

fn main() {
    let program_name = "primes";
    let code = assmeble_brumm(program_name, false);
    let mut emulator = BrummCpuEmulator::new();
    emulator.set_code(&code);
    emulator.set_port_listener(123, port_printer);
    emulator.run_until_dont();
}

fn port_printer(emulator: &mut BrummCpuEmulator, called_from: u8) {
    let val = emulator.get_output(called_from);
    println!("<I/O - {}>\t{}\t\t{:b}", called_from, val, val);
}
