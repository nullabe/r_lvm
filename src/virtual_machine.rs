use crate::instruction::Opcode;

#[derive(Default)]
pub struct VirtualMachine {
    register: [i32; 32],
    bytes: Vec<u8>,
    byte_index: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            register: [0; 32],
            bytes: vec![],
            byte_index: 0,
        }
    }
}

impl VirtualMachine {
    pub fn get_register(&self) -> [i32; 32] {
        self.register
    }

    pub fn get_bytes_as_ref(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes = bytes;
    }

    pub fn get_byte_index(&self) -> usize {
        self.byte_index
    }
}

impl VirtualMachine {
    pub fn run(&mut self) {
        loop {
            if !self.tick() {
                break;
            }
        }
    }

    fn tick(&mut self) -> bool {
        if self.byte_index >= self.bytes.len() {
            println!("Over the bounds. Terminating.");
            return false;
        }

        match self.interprete_byte() {
            Opcode::Hlt => {
                println!("Hlt encountered.");
                false
            }
            Opcode::Igl => {
                println!("Ilegal code found. Terminating.");
                false
            }
        }
    }

    fn interprete_byte(&mut self) -> Opcode {
        let opcode = Opcode::from(self.bytes[self.byte_index]);

        self.byte_index += 1;

        opcode
    }
}
