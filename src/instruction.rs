
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum OpCode {
  LOAD, // Load
  ADD,  // Add
  SUB,  // Subtract
  MUL,  // Multiply
  DIV,  // Divide
  JMP,  // Absolute Jump
  HLT,  // Halt
  PAD,  // Padding, does nothing when executed
  IGL   // Illegal
}

impl From<u8> for OpCode {
  fn from(v: u8) -> Self {
    match v {
      0   => OpCode::HLT,
      10  => OpCode::LOAD,
      11  => OpCode::ADD,
      12  => OpCode::SUB,
      13  => OpCode::MUL,
      14  => OpCode::DIV,
      15  => OpCode::JMP,
      255 => OpCode::PAD,
      _   => OpCode::IGL
    }
  }
}

impl From<OpCode> for u8 {
  fn from(c: OpCode) -> Self {
    match c {
      OpCode::HLT  => 0,
      OpCode::LOAD => 10,
      OpCode::ADD  => 11,
      OpCode::SUB  => 12,
      OpCode::MUL  => 13,
      OpCode::DIV  => 14,
      OpCode::JMP  => 15,
      OpCode::PAD  => 255,
      OpCode::IGL  => 100,
    }
  }
}

pub struct Instruction {
  opcode: OpCode,
}

impl Instruction {
  pub fn new(opcode: OpCode) -> Self {
    Instruction { opcode }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_hlt() {
    let oc = OpCode::HLT;
    assert_eq!(oc, OpCode::HLT);
  }

  #[test]
  fn test_create_instruction() {
    let instruction = Instruction::new(OpCode::HLT);
    assert_eq!(instruction.opcode, OpCode::HLT);
  }
}