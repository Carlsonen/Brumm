mod assembler;
use assembler::*;
mod emulator;
use emulator::*;

fn main() {
    let filename = "fib";
    let code = assmeble_brumm(filename);
    let mut emulator = BrummCpuEmulator::new(&code);
    emulator.run_until_dont();
    println!("{:?}", emulator.get_ramfile());
    println!("{:?}", emulator.get_regfile());
    barrelcode_to_schematic(bytes_to_barrelcode(&code), filename);
}
