#[derive(Debug, PartialEq)]
pub enum Opcode {
    Hlt,
    Igl,
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction { opcode }
    }
}

impl Instruction {
    pub fn get_opcode_as_ref(&self) -> &Opcode {
        &self.opcode
    }
}

impl From<u8> for Opcode {
    fn from(from: u8) -> Self {
        match from {
            0 => Opcode::Hlt,
            _ => Opcode::Igl,
        }
    }
}
