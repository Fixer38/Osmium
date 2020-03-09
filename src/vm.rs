use super::instruction::*;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    psw: bool,
}


impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            psw: false,
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
                // LOAD format: LOAD $0, $1
                // $0 register storing the loaded value
                // $1 value to load into register on 16 bits
                let register = self.next_8_bits() as usize; // usize cast for array index
                let number = self.next_16_bits();
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                // ADD format: ADD $1, $2, $3
                // $1 and $2 operands to add from register
                // $3 addition destination register
                let operand1 = self.registers[self.next_8_bits() as usize];
                let operand2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = operand1 + operand2;
            },
            Opcode::SUB => {
                // SUB format: SUB $1, $2, $3
                // $1 and $2 operands to substract from register
                // $3 substraction destination register
                let operand1 = self.registers[self.next_8_bits() as usize];
                let operand2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = operand1 - operand2;
            }
            Opcode::MUL => {
                // MUL format: MUL $1, $2, $3
                // $1 and $2 operands to multiply from register
                // $3 multiplication destination register
                let operand1 = self.registers[self.next_8_bits() as usize];
                let operand2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = operand1 * operand2;
            }
            Opcode::DIV => {
                // DIV format: DIV $1, $2, $3
                // $1 and $2 operands to multiply from register
                // $3 division quotient destination register
                // Remainder will contain remainder of the division => i32 allowed in the registers
                let operand1 = self.registers[self.next_8_bits() as usize];
                let operand2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = operand1 / operand2;
                self.remainder = (operand1 % operand2) as u32;
            }
            Opcode::JMP => {
                // JMP format: JMP $1
                // $1 register to get the pc from => Change pc according to its value
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                // JMPF format: JMPF $1
                // $1 register to get the value pc should be incremented
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            }
            Opcode::EQ => {
                // EQ format: EQ $1, $2
                // $1 and $2 values from the register to compare
                let register1 = self.registers[self.next_8_bits() as usize] as i32;
                let register2 = self.registers[self.next_8_bits() as usize] as i32;
                self.psw = register1 == register2;
                // Read next 8 bits since they're not used in this instruction
                self.next_8_bits();
            }
            Opcode::JEQ => {
                // JEQ format: JEQ $1
                // $1 target to jump to
                let target = self.registers[self.next_8_bits() as usize];
                if self.psw {
                    self.pc = target as usize;
                }
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
        test_vm.execute_instruction();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        test_vm.program = vec![200, 0, 0, 0];
        test_vm.execute_instruction();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244]; // 1 in first for the LOAD. 1 and 244 = 500 (LE)
        test_vm.execute_instruction();
        assert_eq!(test_vm.registers[0], 500); // Expected result to be 500
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 255;
        test_vm.registers[1] = 2;
        test_vm.program = vec![2, 0, 1, 0];
        test_vm.execute_instruction();
        assert_eq!(test_vm.registers[0], 257);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        // Check results of quotient in register 0 from the division
        // Check remainder
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 2;
        test_vm.program = vec![5, 0, 1, 0];
        test_vm.execute_instruction();
        assert_eq!(test_vm.registers[0], 2);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_opcode_jmp() {
        // check if the pc is equal to the target register value
        let mut test_vm = VM::new();
        test_vm.registers[1] = 1;
        test_vm.program = vec![6, 1, 0, 0];
        test_vm.execute_instruction();
        assert_eq!(test_vm.pc, 1);
    }
    
    #[test]
    fn test_opcode_jumpf() {
        // Check if the pc got incremented properly
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0];
        test_vm.execute_instruction();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_eq() {
        // Check if psw is true after condition check for same value
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.registers[1] = 1;
        test_vm.program = vec![8, 0, 1, 0, 8, 0, 1, 0];
        test_vm.execute_instruction();
        assert_eq!(test_vm.psw, true);
        // Check if psw is false after condition check for different value
        test_vm.registers[1] = 2;
        test_vm.execute_instruction();
        assert_eq!(test_vm.psw, false);
    }
    
    #[test]
    fn test_opcode_jeq() {
        // Check if the pc is equal to the correct pc
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.psw = true;
        test_vm.program = vec![9, 0, 0, 0];
        test_vm.execute_instruction();
        assert_eq!(test_vm.pc, 1);
    }
}
