#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    LOAD,
    IGL,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    EQ,
    NEQ,
    GT,
    LT,
    GTE,
    LTE,
    JEQ,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::EQ,
            9 => Opcode::NEQ,
            10 => Opcode::GT,
            11 => Opcode::LT,
            12 => Opcode::GTE,
            13 => Opcode::LTE,
            14 => Opcode::JEQ,
            _ => Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
