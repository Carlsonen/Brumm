use std::collections::HashMap;
use std::fs;

use redstone_schem::world::{World, BlockPos};


pub fn assmeble_brumm(filename: &str, debug_mode: bool) -> Vec<[u8; 4]> {
    let filepath = format!("brumm_src/{}.brumm", filename);
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut bytecode: Vec<[u8; 4]> = vec![];
    let mut regs: HashMap<&str, u8> = HashMap::from([
        ("0", 0),
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("tmp", 10),
        ("tmp2", 11),
    ]);
    let flags: HashMap<&str, u8> = HashMap::from([
        ("odd", 0),
        ("true", 5),
        ("zero", 1),
        ("input", 5),
        ("cout", 3),
        ("greater", 3),
    ]);
    let mut labels: HashMap<&str, u8> = HashMap::from([]);
    {
        // get labels
        let mut n = 0;
        let mut reg_n = 1;
        for line in &lines {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if *line == "" || tokens[0] == "#" {
                continue;
            }

            match tokens[0] {
                "def" => {
                    labels.insert(tokens[1], n);
                }
                "use" => {
                    for i in 1..tokens.len() {
                        if i <= 8 {
                            regs.insert(tokens[i], reg_n);
                            reg_n += 1;
                        } else {
                            println!("too many registers! {i}");
                        }
                    }
                }
                _ => n += 1,
            }
        }
    }
    for line in &lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 0 {
            continue;
        } // ignore blank

        let opcode = tokens[0];

        if match opcode {
            // ignore other shit
            "use" | "def" | "#" => true,
            _ => false,
        } {
            continue;
        }

        let bytes = match opcode {
            "-" | "noop" => [0, 0, 0, 0],
            "add" => [0, regs[tokens[1]], regs[tokens[2]], regs[tokens[3]]],
            "sub" => [1, regs[tokens[1]], regs[tokens[2]], regs[tokens[3]]],
            "addci" => [2, regs[tokens[1]], regs[tokens[2]], regs[tokens[3]]],
            "addco" => [3, regs[tokens[1]], regs[tokens[2]], regs[tokens[3]]],
            "or" => [4, regs[tokens[1]], regs[tokens[2]], regs[tokens[3]]],
            "and" => [5, regs[tokens[1]], regs[tokens[2]], regs[tokens[3]]],
            "xor" => [6, regs[tokens[1]], regs[tokens[2]], regs[tokens[3]]],
            "xnor" => [7, regs[tokens[1]], regs[tokens[2]], regs[tokens[3]]],
            "rshift" => [8, regs[tokens[1]], regs[tokens[2]], regs[tokens[3]]],
            "ldi" => {
                let num: u8 = tokens[2].parse().unwrap();
                [9, regs[tokens[1]], num & 0xf, (num >> 4) & 0xf]
            }
            "load" => [10, regs[tokens[1]], 0, 0],
            "store" => [11, 0, regs[tokens[1]], 0],
            "if" => match tokens[1] {
                "not" => {
                    let num: u8 = labels[tokens[4]];
                    [12, 8 + flags[tokens[2]], num & 0xf, (num >> 4) & 0xf]
                }
                _ => {
                    let num: u8 = labels[tokens[3]];
                    [12, flags[tokens[1]], num & 0xf, (num >> 4) & 0xf]
                }
            },
            "call" => {
                let num: u8 = labels[tokens[1]];
                [13, 0, num & 0xf, (num >> 4) & 0xf]
            }
            "return" => [14, 0, 0, 0],
            "halt" => [15, 0, 0, 0],

            "add=" => [0, regs[tokens[1]], regs[tokens[1]], regs[tokens[2]]],
            "sub=" => [1, regs[tokens[1]], regs[tokens[1]], regs[tokens[2]]],
            "addci=" => [2, regs[tokens[1]], regs[tokens[1]], regs[tokens[2]]],
            "addco=" => [3, regs[tokens[1]], regs[tokens[1]], regs[tokens[2]]],
            "or=" => [4, regs[tokens[1]], regs[tokens[1]], regs[tokens[2]]],
            "and=" => [5, regs[tokens[1]], regs[tokens[1]], regs[tokens[2]]],
            "xor=" => [6, regs[tokens[1]], regs[tokens[1]], regs[tokens[2]]],
            "xnor=" => [7, regs[tokens[1]], regs[tokens[1]], regs[tokens[2]]],
            "rshift=" | ">>=" => [8, regs[tokens[1]], 0, regs[tokens[1]]],
            "lshift=" | "<<=" => [0, regs[tokens[1]], regs[tokens[1]], regs[tokens[1]]],

            "mov" => [0, regs[tokens[1]], 0, regs[tokens[2]]],
            "cmp" => [1, 0, regs[tokens[1]], regs[tokens[2]]],
            "goto" | "jmp" => {
                let num: u8 = labels[tokens[1]];
                [13, 8, num & 0xf, (num >> 4) & 0xf]
            }
            "inc" => [2, regs[tokens[1]], 0, regs[tokens[1]]],
            "check" => [0, 0, 0, regs[tokens[1]]],
            _ => {
                println!("wtf is this:\n{:?}\n", tokens);
                continue;
            }
        };
        bytecode.push(bytes);
    }
    if debug_mode {
        for bytes in &bytecode {
            println!("{:?}", bytes);
        }
        println!("{:?}", regs);
        println!("{:?}", labels);
    }
    bytecode
}
pub fn bytes_to_barrelcode(bytecode: &Vec<[u8; 4]>) -> (Vec<[u8; 8]>, Vec<[u8; 8]>) {
    let mut bytecode = bytecode.clone();
    while bytecode.len() < 256 {
        bytecode.push([0, 0, 0, 0]);
    }
    let mut barrels1: Vec<[u8; 8]> = vec![];
    let mut barrels2: Vec<[u8; 8]> = vec![];
    for x in 0..4 {
        for i in 0..16 {
            let mut barrel_column = [0, 0, 0, 0, 0, 0, 0, 0];
            for b in 0..8 {
                let mut barrel = 0;
                for j in 0..4 {
                    barrel += ((bytecode[i + 16 * j + 64 * x][b / 4] >> (b % 4)) & 1) << j;
                }
                barrel_column[b] = barrel;
            }
            barrels1.push(barrel_column);
        }
    }
    //println!("{:?}", barrels1);
    for x in 0..4 {
        for i in 0..16 {
            let mut barrel_column = [0, 0, 0, 0, 0, 0, 0, 0];
            for b in 0..8 {
                let mut barrel = 0;
                for j in 0..4 {
                    barrel += ((bytecode[i + 16 * j + 64 * x][b / 4 + 2] >> (b % 4)) & 1) << j;
                }
                barrel_column[b] = barrel;
            }
            barrels2.push(barrel_column);
        }
    }
    (barrels1, barrels2)
}

pub fn barrelcode_to_schematic(barrels: (Vec<[u8; 8]>, Vec<[u8; 8]>), filename: &str) {
    let mut world = World::new(87, 15, 33);
    let stone = world.add_block("minecraft:stone");

    for x in 0..4 {
        for z in 0..16 {
            for y in 0..8 {
                let b1 = barrels.0[z + 16 * x][y];
                let b2 = barrels.1[z + 16 * x][y];
                match b1 {
                    1..=15 =>   {world.set_barrel(BlockPos::new(4 * x, 2 * y, 2 * z + 2), b1.into());}
                    _ =>        {world.set_block(BlockPos::new(4 * x, 2 * y, 2 * z + 2), stone);}
                }
                match b2 {
                    1..=15 =>   {world.set_barrel(BlockPos::new(74 + 4 * x, 2 * y, 2 * z + 2), b2.into());}
                    _ =>        {world.set_block(BlockPos::new(74 + 4 * x, 2 * y, 2 * z + 2), stone);}
                }
            }
        }
    }
    let path = format!("C:/__SKIT__/.actual server/schems/{filename}.schem");
    world.save_schematic(path.as_str(), 0, -15, 0);
    println!("saved scematic!");
}
