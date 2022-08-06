mod assembler;
use assembler::*;
mod emulator;
use emulator::*;

fn main() {
    let program_name = "maze";
    let code = assmeble_brumm(program_name, false);
    let barrelcode = bytes_to_barrelcode(&code);
    barrelcode_to_schematic(&barrelcode, program_name);


    let mut emulator = BrummCpuEmulator::new();
    emulator.set_code(&code);
    emulator.run_until_dont();
    emulator.set_input(126, 4);
    emulator.run_until_dont();
    emulator.set_input(126, 8);
    emulator.run_until_dont();
}
