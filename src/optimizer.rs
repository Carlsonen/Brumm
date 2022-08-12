pub fn optimize(bytecode: &Vec<[u8; 4]>) -> Vec<[u8; 4]> {
    let mut blobs: Vec<CodeBlob> = vec![];
    for code in bytecode.iter() {
        blobs.push(CodeBlob::from(&code));
    }
    for code in bytecode.iter() {
        match code[0] {
            12 | 13 => {
                let lower = code[2];
                let upper = code[3];
                let label_id = lower + (upper << 4);
                blobs[label_id as usize].set_label(label_id);
            }
            _ => {}
        }
    }
    for blob in blobs.iter() {
        println!("{}", blob);
    }
    vec![]
}

struct CodeBlob {
    bytes: [u8; 4],
    label: Option<u8>,
}
impl CodeBlob {
    pub fn from(code: &[u8; 4]) -> Self {
        CodeBlob {
            bytes: code.clone(),
            label: None,
        }
    }
    pub fn set_label(&mut self, id: u8) {
        self.label = Some(id);
    }
}
impl std::fmt::Display for CodeBlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.label {
            Some(l) => {
                write!(f, "{}:\t{:?})", l, self.bytes)
            }
            None => {
                write!(f, "\t{:?})", self.bytes)
            }
        }
    }
}

struct Node {
    blob: CodeBlob,
    connections: Vec<Connection>,
}
struct Connection {
    id: usize,
    weight: u8,
}
