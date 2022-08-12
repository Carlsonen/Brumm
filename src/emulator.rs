use core::num;

type PortListener = fn(&mut BrummCpuEmulator, called_from: u8);
fn dummy_port_listener(b: &mut BrummCpuEmulator, called_from: u8) {}
pub struct BrummCpuEmulator {
    code: Vec<[u8; 4]>,

    pc: u8,
    registers: [u8; 11],
    ram: [u8; 64],
    data_stack: Vec<u8>,
    call_stack: Vec<u8>,

    jmp_info: [Option<u8>; 5],
    reg4io: [u8; 11],
    reg4io_buffer: [u8; 11],
    pointer: [u8; 4],
    fl_cout: bool,
    fl_odd: bool,
    fl_zero: bool,
    is_running: bool,
    io_in: [u8; 4],
    io_out: [u8; 4],

    port_listeners: [PortListener; 4],
}

impl BrummCpuEmulator {
    pub fn new() -> Self {
        BrummCpuEmulator {
            code: vec![],
            pc: 0,
            registers: [0; 11],
            ram: [0; 64],
            data_stack: vec![],
            call_stack: vec![],
            jmp_info: [None; 5],
            reg4io: [0; 11],
            reg4io_buffer: [0; 11],
            pointer: [0; 4],
            fl_cout: false,
            fl_odd: false,
            fl_zero: false,
            is_running: false,
            io_in: [0; 4],
            io_out: [0; 4],
            port_listeners: [dummy_port_listener; 4],
        }
    }
    pub fn set_code(&mut self, code: &Vec<[u8; 4]>) {
        self.code = code.clone();
    }
    pub fn tick(&mut self) {
        // (1) - Update Pipeline
        self.jmp_info[4] = self.jmp_info[3];
        self.jmp_info[3] = self.jmp_info[2];
        self.jmp_info[2] = self.jmp_info[1];
        self.jmp_info[1] = self.jmp_info[0];
        self.jmp_info[0] = None;

        self.registers[0] = 0;
        self.reg4io = self.reg4io_buffer.clone();
        self.reg4io_buffer = self.registers.clone();

        self.pointer[3] = self.pointer[2];
        self.pointer[2] = self.pointer[1];
        self.pointer[1] = self.pointer[0];
        self.pointer[0] = self.registers[9];
        // (2) - Make Shit
        let instr = self.code[self.pc as usize];

        match instr[0] {
            0 => {
                // add
                let (a, b) = self.get_registers(instr);

                let c = a + b;

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            1 => {
                // sub
                let (a, b) = self.get_registers(instr);

                let c = a + (256 - b);

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            2 => {
                // addci
                let (a, b) = self.get_registers(instr);

                let c = a + b + 1;

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            3 => {
                // addco
                let (a, b) = self.get_registers(instr);

                let c = a + b + self.fl_cout as u16;

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            4 => {
                // or
                let (a, b) = self.get_registers(instr);

                let c = a + b;

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            5 => {
                // and
                let (a, b) = self.get_registers(instr);

                let c = a & b;

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            6 => {
                // xor
                let (a, b) = self.get_registers(instr);

                let c = a ^ b;

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            7 => {
                // xnor
                let (a, b) = self.get_registers(instr);

                let c = a ^ !b;

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            8 => {
                // rshift
                let (a, b) = self.get_registers(instr);

                let c = (a | b) >> 1;

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            9 => {
                // ldi
                let c = instr[2] as u16 + ((instr[3] as u16) << 4);

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            10 => {
                // load
                let p = self.pointer[2] as usize;
                let p_stack = self.pointer[1] as usize;
                let c: u16 = match p {
                    0..=63 => self.ram[p] as u16,
                    123..=126 => self.io_in[(p - 123) as usize] as u16,
                    _ => 0,
                };
                let c = c | match p_stack {
                    127 => match self.data_stack.pop() {
                        Some(v) => v as u16,
                        None => 0,
                    },
                    _ => 0,
                };
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            11 => {
                // store
                let val = self.reg4io[instr[2] as usize];
                let p = self.pointer[1] as usize;
                match p {
                    127 => {
                        self.data_stack.push(val);
                    }
                    0..=63 => {
                        self.ram[p] = val;
                    }
                    123..=126 => {
                        let port_index = (p - 123) as usize;
                        self.io_out[port_index] = val;
                        self.port_listeners[port_index](self, p as u8);
                    }
                    _ => {}
                }
            }
            12 => {
                // jmp if
                let n = instr[1];

                let flag: bool = match instr[1] % 8 {
                    0 => self.fl_odd,
                    1 => self.fl_zero,
                    3 => self.fl_cout,
                    _ => false,
                };

                let flag = flag ^ (n >= 8); // if negate it

                if flag {
                    let num = instr[2] + ((instr[3]) << 4);
                    self.jmp_info[0] = Some(num);
                }
                self.update_flags(0);
            }
            13 => {
                // call
                let num = instr[2] + ((instr[3]) << 4);
                self.jmp_info[2] = Some(num);
                if instr[1] < 8 {
                    self.call_stack.push(self.pc + 3);
                }
                self.update_flags(0);
            }
            14 => {
                // return
                let num = match self.call_stack.pop() {
                    Some(v) => v,
                    None => 0,
                };
                self.jmp_info[2] = Some(num);
                self.update_flags(0);
            }
            15 => {
                // halt
                self.is_running = false;
                self.update_flags(0);
            }
            _ => {}
        }
        // (3) - PC Shit
        self.pc = match self.jmp_info[4] {
            Some(v) => v,
            None => self.pc + 1,
        }
    }
    fn get_registers(&mut self, instr: [u8; 4]) -> (u16, u16) {
        let a = self.registers[instr[2] as usize] as u16;
        let b = self.registers[instr[3] as usize] as u16;
        self.registers[10] = 0;
        (a, b)
    }
    fn update_flags(&mut self, num: u16) {
        self.fl_cout = num > 255;
        self.fl_odd = num % 2 == 1;
        self.fl_zero = (num % 256) == 0;
    }
    pub fn get_regfile(&self) -> [u8; 11] {
        self.registers.clone()
    }
    pub fn get_ramfile(&self) -> [u8; 64] {
        self.ram.clone()
    }
    pub fn run_until_dont(&mut self) {
        use std::time::Instant;
        let now = Instant::now();
        let mut cycles = 0;
        self.is_running = true;
        while self.is_running {
            self.tick();
            cycles += 1;
        }
        let elapsed = now.elapsed().as_nanos();
        let hz = (cycles * 1000) / elapsed;
        match elapsed {
            x if x > 10000 => {
                println!("{cycles} cycles in {} µs ({hz} MHz)", elapsed / 1000);
            }
            _ => {
                println!("{cycles} cycles in {} ns ({hz} MHz)", elapsed);
            }
        }
    }
    pub fn set_input(&mut self, port: u8, value: u8) {
        match port {
            123..=126 => self.io_in[(port - 123) as usize] = value,
            _ => {}
        }
    }
    pub fn get_output(&self, port: u8) -> u8 {
        match port {
            123..=126 => self.io_out[(port - 123) as usize],
            _ => 0,
        }
    }
    pub fn set_port_listener(&mut self, port: u8, func: PortListener) {
        match port {
            123..=126 => self.port_listeners[(port - 123) as usize] = func,
            _ => {}
        }
    }
    pub fn print_ram(&self) {
        for y in 0..8 {
            for x in 0..8 {
                let val = self.ram[8 * y + x];
                let num_str = format!("{val}");
                let n = num_str.len();
                for _ in 0..5 - n {
                    print!(" ");
                }
                print!("{val}");
            }
            print!("\n");
        }
    }
}
