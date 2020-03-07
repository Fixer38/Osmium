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
        let mut is_done = false;
        while is_done == false {
            is_done = self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::HLT => {
                println!("Halt encountered");
                return false
            },
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize; // usize cast for array index
                let number = self.next_16_bits();
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                let register = self.next_8_bits() as usize;
                let arg1 = self.next_8_bits() as i32;
                let arg2 = self.next_8_bits() as i32;
                self.registers[register] = arg1 + arg2;
            }
            Opcode::IGL => println!("unknown instruction encountered")
        }
        true
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
        // Having to shift first 8 bits into the u16 to fill up the last 8 bits with the remainder.
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
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244]; // 1 in first for the LOAD. 1 and 244 = 500 (LE)
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500); // Expected result to be 500
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.program = vec![2, 0, 2, 255];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 257);
    }
}
