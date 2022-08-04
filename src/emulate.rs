

pub struct brumm_cpu_emulator {
    code: Vec<[u8; 4]>,

    PC: u8,
    registers: [u8; 11],
    ram: [u8; 64],
    data_stack: Vec<u8>,
    call_stack: Vec<u8>,
    
    jmp_info: [Option<u8>; 4],
    reg4io: [u8; 11],
    pointer: [u8; 3],
    fl_cout: bool,
    fl_odd: bool,
    fl_zero: bool,
}

impl brumm_cpu_emulator {
    pub fn new(code: Vec<[u8; 4]>) -> Self {
        brumm_cpu_emulator { 
            code: code, PC: 0, registers: [0; 11], ram: [0; 64], data_stack: vec![], call_stack: vec![], jmp_info: [None; 4], 
            reg4io: [0; 11], pointer: [0; 3], fl_cout: false, fl_odd: false, fl_zero: false }
    }
    pub fn tick(&mut self) {
        // (1) - Update Pipeline
        self.jmp_info[3] = self.jmp_info[2];
        self.jmp_info[2] = self.jmp_info[1];
        self.jmp_info[1] = self.jmp_info[0];
        self.jmp_info[0] = None;

        self.registers[0] = 0;
        self.reg4io = self.registers.clone();

        self.pointer[2] = self.pointer[1];
        self.pointer[1] = self.pointer[0];
        self.pointer[0] = self.registers[8];
        // (2) - Make Shit
        let instr = self.code[self.PC as usize];

        match instr[0] {
            0 => { // add
                let (a, b) = self.get_registers(instr);

                let c = a + b;
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            1 => { // sub
                let (a, b) = self.get_registers(instr);

                let c = a + (256 - b);
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            2 => { // addci
                let (a, b) = self.get_registers(instr);

                let c = a + b + 1;
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            3 => { // addco
                let (a, b) = self.get_registers(instr);

                let c = a + b + self.fl_cout as u16;
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            4 => { // or
                let (a, b) = self.get_registers(instr);

                let c = a + b;
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            5 => { // and
                let (a, b) = self.get_registers(instr);

                let c = a & b;
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            6 => { // xor
                let (a, b) = self.get_registers(instr);

                let c = a ^ b;
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            7 => { // xnor
                let (a, b) = self.get_registers(instr);

                let c = a ^ !b;
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            8 => { // rshift
                let (a, b) = self.get_registers(instr);

                let c = (a | b) >> 1;
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            9 => { // ldi
                let c = instr[2] as u16 + (instr[3] as u16) << 4;
                
                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            10 => { // load
                let p = self.pointer[2] as usize;
                let c = match p {
                    127 => {
                        match self.data_stack.pop() {
                            Some(v) => {
                                v
                            }
                            None => {0}
                        }
                    }
                    0..=63 => {
                        self.ram[p];
                    }
                    _ => {}
                }
                let c = 

                self.update_flags(c);

                self.registers[instr[1] as usize] = c as u8;
            }
            11 => { // store
                let val = self.reg4io[instr[2] as usize];
                let p = self.pointer[1] as usize;
                match p {
                    127 => {
                        self.data_stack.push(val);
                    }
                    0..=63 => {
                        self.ram[p] = val;
                    }
                    _ => {}
                }
            }
            12 => { // jmp if
                
            }
            13 => { // call
                
            }
            14 => { // return
                
            }
            15 => { // halt
                
            }
            _ => {}
        }
        // (3) - PC Shit

        // (4) - Reset Flags
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
        self.fl_zero = num == 0;
    }
}

