
use crate::instruction::OpCode;

pub struct VM {
  pub registers: [i32; 32],
  pub pc: usize, // program counter
  pub program: Vec<u8>,
  pub remainder: u32,
  pub equivalence_flag: bool,
}

impl VM {
  pub fn new() -> Self {
    VM { 
      registers: [0; 32], 
      pc: 0, program: Vec::new(), 
      remainder: 0, 
      equivalence_flag: false 
    }
  }

  pub fn run(&mut self) {
    loop {
      if !self.execute_instruction() { return; }
    } // end of loop
  }

  pub fn run_once(&mut self) {
    self.execute_instruction();
  }

  pub fn execute_instruction(&mut self) -> bool {
    if self.pc >= self.program.len() {
      false // something is wrong
    } else {
      self.step_opcode()
    }
  }

  pub fn step_opcode(&mut self) -> bool {
    let mut cont = true;

    match self.step_decode_opcode() {
      OpCode::LOAD => {
        let register = self.step_eight_bits() as usize;
        let number = self.step_sixteen_bits() as u16;

        self.registers[register] = number as i32;
      },

      OpCode::EQ => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        if register_a == register_b {
          self.equivalence_flag = true;
        } else {
          self.equivalence_flag = false;
        }

        self.step_eight_bits(); // ignore
      },

      OpCode::NEQ => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        if register_a != register_b {
          self.equivalence_flag = true;
        } else {
          self.equivalence_flag = false;
        }

        self.step_eight_bits(); // ignore
      },

      OpCode::LT => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        if register_a < register_b {
          self.equivalence_flag = true;
        } else {
          self.equivalence_flag = false;
        }

        self.step_eight_bits(); // ignore
      },

      OpCode::GT => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        if register_a > register_b {
          self.equivalence_flag = true;
        } else {
          self.equivalence_flag = false;
        }

        self.step_eight_bits(); // ignore
      },

      OpCode::LTE => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        if register_a <= register_b {
          self.equivalence_flag = true;
        } else {
          self.equivalence_flag = false;
        }

        self.step_eight_bits(); // ignore
      },

      OpCode::GTE => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        if register_a >= register_b {
          self.equivalence_flag = true;
        } else {
          self.equivalence_flag = false;
        }

        self.step_eight_bits(); // ignore
      },

      OpCode::JEQ => {
        let register = self.step_eight_bits() as usize;
        let target = self.registers[register];

        if self.equivalence_flag {
          self.pc = target as usize;
        }
      },

      OpCode::JMP => {
        let register = self.registers[self.step_eight_bits() as usize];
        self.pc = register as usize;
      },

      OpCode::JMPF => {
        let register = self.registers[self.step_eight_bits() as usize];
        self.pc += register as usize;
      },

      OpCode::JMPB => {
        let register = self.registers[self.step_eight_bits() as usize];
        self.pc -= register as usize;
      },

      OpCode::ADD => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        self.registers[self.step_eight_bits() as usize] = register_a + register_b;
      },

      OpCode::SUB => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        self.registers[self.step_eight_bits() as usize] = register_a - register_b;
      },

      OpCode::MUL => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        self.registers[self.step_eight_bits() as usize] = register_a * register_b;
      },

      OpCode::DIV => {
        let register_a = self.registers[self.step_eight_bits() as usize];
        let register_b = self.registers[self.step_eight_bits() as usize];

        self.registers[self.step_eight_bits() as usize] = register_a / register_b;
        self.remainder = (register_a % register_b) as u32;
      },

      OpCode::PAD => {
        
      },

      OpCode::HLT => {
        println!("HLT opcode found, halting...");
        cont = false; // terminate the loop
      },

      _ => {
        println!("Unrecognized opcode found, terminating...");
        cont = false; // terminate the loop
      }
    }
    cont
  }

  pub fn step_decode_opcode(&mut self) -> OpCode {
    let opcode = OpCode::from(self.program[self.pc]);
    self.pc += 1;
    return opcode;
  }

  fn step_eight_bits(&mut self) -> u8 {
    let bits = self.program[self.pc];
    self.pc += 1;
    return bits;
  }

  fn step_sixteen_bits(&mut self) -> u16 {
    // order is; first number in the first 8 bits, then second number in the next 8 bits
    let bits = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
    self.pc += 2;
    return bits;
  }
}

fn get_test_vm() -> VM {
  let mut test_vm = VM::new();

  test_vm.registers[0] = 5;
  test_vm.registers[1] = 10;

  test_vm
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_vm() {
    let test_vm = VM::new();

    assert_eq!(test_vm.registers[0], 0);
  }

  #[test]
  fn test_execute_instruction() {
    let mut test_vm = VM::new();

    test_vm.program = vec![
      0xFF, 0, 0, 0
    ];
    test_vm.run_once();

    assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_opcode_jeq() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 7;
    test_vm.equivalence_flag = true;
    test_vm.program = vec![
      24, 0, 0xFF, 0xFF, 
      0xFF, 0xFF, 0xFF, 0xFF
    ];
    test_vm.run_once();

    assert_eq!(test_vm.pc, 7);
  }

  #[test]
  fn test_opcode_gte() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    test_vm.program = vec![
      23, 0, 1, 0xFF, 
      23, 0, 1, 0xFF
    ];
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, true);

    test_vm.registers[1] = 20;
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, false);
  }

  #[test]
  fn test_opcode_lte() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    test_vm.program = vec![
      22, 0, 1, 0xFF, 
      22, 0, 1, 0xFF
    ];
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, true);

    test_vm.registers[1] = 5;
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, false);
  }

  #[test]
  fn test_opcode_gt() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    test_vm.program = vec![
      21, 0, 1, 0xFF, 
      21, 0, 1, 0xFF
    ];
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, false);

    test_vm.registers[0] = 20;
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, true);
  }

  #[test]
  fn test_opcode_lt() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    test_vm.program = vec![
      20, 0, 1, 0xFF, 
      20, 0, 1, 0xFF
    ];
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, false);

    test_vm.registers[1] = 20;
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, true);
  }

  #[test]
  fn test_opcode_neq() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    test_vm.program = vec![
      19, 0, 1, 0xFF, 
      19, 0, 1, 0xFF
    ];
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, false);

    test_vm.registers[1] = 20;
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, true);
  }

  #[test]
  fn test_opcode_eq() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    test_vm.program = vec![
      18, 0, 1, 0xFF, 
      18, 0, 1, 0xFF
    ];
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, true);

    test_vm.registers[1] = 20;
    test_vm.run_once();

    assert_eq!(test_vm.equivalence_flag, false);
  }

  #[test]
  fn test_opcode_jmpb() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 1;
    test_vm.program = vec![
      15, 0, 0, 0
    ]; // set pc to value in register0
    test_vm.run_once();

    assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_opcode_jmpf() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 2;
    test_vm.program = vec![
      16, 0, 0, 0, 
      10, 0, 0b0, 0b1000
    ];
    test_vm.run_once();

    assert_eq!(test_vm.pc, 4);
  }

  #[test]
  fn test_opcode_jmp() {
    let mut test_vm = get_test_vm();

    test_vm.registers[0] = 1;
    test_vm.program = vec![
      15, 0, 0, 0
    ]; // set pc to value in register0
    test_vm.run_once();

    assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_opcode_div_remainder() {
    let mut test_vm = get_test_vm();
    
    test_vm.registers[0] = 3;
    test_vm.program = vec![
      14, 1, 0, 2
    ];
    test_vm.run();
    
    assert_eq!(test_vm.registers[2], 3);
    assert_eq!(test_vm.remainder, 1);
  }

  #[test]
  #[should_panic]
  fn test_opcode_div_zero() {
    let mut test_vm = get_test_vm();
    
    test_vm.registers[0] = 0;
    test_vm.program = vec![
      14, 1, 0, 2
    ];
    test_vm.run();

    // expected to fail
  }

  #[test]
  fn test_opcode_div() {
    let mut test_vm = get_test_vm();
    
    test_vm.program = vec![
      14, 1, 0, 2
    ];
    test_vm.run();
    
    assert_eq!(test_vm.registers[2], 2);
  }

  #[test]
  fn test_opcode_mul() {
    let mut test_vm = get_test_vm();
    
    test_vm.program = vec![
      13, 1, 0, 2
    ];
    test_vm.run();
    
    assert_eq!(test_vm.registers[2], 50);
  }

  #[test]
  fn test_opcode_sub() {
    let mut test_vm = get_test_vm();
    
    test_vm.program = vec![
      12, 1, 0, 2
    ];
    test_vm.run();
    
    assert_eq!(test_vm.registers[2], 5);
  }

  #[test]
  fn test_opcode_add() {
    let mut test_vm = get_test_vm();
    
    test_vm.program = vec![
      11, 1, 0, 2
    ];
    test_vm.run();
    
    assert_eq!(test_vm.registers[2], 15);
  }

  #[test]
  fn test_opcode_load() {
    let mut test_vm = get_test_vm();
    
    test_vm.program = vec![
      0xFF, 0xFF, 0xFF, 0xFF, 
      10, 0, 0b1, 0b11110100, 0
    ];
    test_vm.run();
    
    assert_eq!(test_vm.registers[0], 500);
  }

  #[test]
  fn test_opcode_pad() {
    let mut test_vm = VM::new();
    
    test_vm.program = vec![
      0xFF, 0xFF, 0xFF, 0xFF, 0
    ];
    test_vm.run();
    
    assert_eq!(test_vm.registers[0], 0);
  }

  #[test]
    fn test_opcode_hlt() {
      let mut test_vm = VM::new();
      let test_bytes = vec![
        0, 0, 0, 0
      ];

      test_vm.program = test_bytes;
      test_vm.run();

      assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
      let mut test_vm = VM::new();
      let test_bytes = vec![
        200, 0, 0, 0
      ];

      test_vm.program = test_bytes;
      test_vm.run();

      assert_eq!(test_vm.pc, 1);
    }
}