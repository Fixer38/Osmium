use super::instruction::*;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>
}


impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
        }
    }

    pub fn run(&mut self) {
        loop {
            // break the program loop if pc gets higher than the amount of bytes allowed
            if self.pc >= self.program.len() {
                break
            }

            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("Halt encountered");
                    return;
                },
                Opcode::LOAD => {
                    let register = self.next_8_bits() as usize; // usize cast for array index
                    let number = self.next_16_bits();
                    self.registers[register] = number as i32;
                    continue;
                },
                _ => {
                    println!("Opcode not recognized");
                    return;
                }
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        println!("{:?}", result);
        self.pc += 2;
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
        assert_eq!(test_vm.pc, 0);
        assert!(test_vm.program.is_empty());
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        test_vm.program = vec![200, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }
}
