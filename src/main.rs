mod assembler;
use assembler::*;
mod emulator;
use emulator::*;
mod optimizer;
use optimizer::*;

fn main() {
    let program_name = "test";
    let code = assmeble_brumm(program_name, false);
    let mut emulator = BrummCpuEmulator::new();
    emulator.set_code(&code);
    emulator.set_port_listener(124, port_lamp_display);
    emulator.run_until_dont();
}

fn port_printer(emulator: &mut BrummCpuEmulator, called_from: u8) {
    let val = emulator.get_output(called_from);
    println!("<I/O - {}>\t{}\t\t{:b}", called_from, val, val);
}

fn port_rng(emulator: &mut BrummCpuEmulator, called_from: u8) {
    //use rand;
    let value: u8 = rand::random();
    emulator.set_input(called_from, value);
}

fn port_char_display(emulator: &mut BrummCpuEmulator, called_from: u8) {
    let chars =
        "0123456789abcdefghijklmnopqrstuvwxyzåäöABCDEFGHIJKLMNOPQRSTUVWXYZÅÄÖ \n".to_string();
    let val = emulator.get_output(called_from);
    let c = chars.chars().nth(val as usize).unwrap();
    print!("{c}");
}

fn port_lamp_display(emulator: &mut BrummCpuEmulator, called_from: u8) {
    let x = emulator.get_output(126);
    let y = emulator.get_output(125);
    let command = emulator.get_output(124);
    match command {
        0 => {
            emulator.display_put_pixel(x as u32, y as u32);
        }
        1 => {
            emulator.display_erase_pixel(x as u32, y as u32);
        }
        2 => {
            emulator.display_flood();
        }
        3 => {
            emulator.display_clear();
        }
        _ => {}
    }
    emulator.display_update();
}