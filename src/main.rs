
pub mod vm;
pub mod instruction;

// A virtual machine that operates on 32-bit instructions

fn main() {
  let mut vm = vm::VM::new();
  let program = vec![
    10, 0, 0b0, 0b1100, // load 12 into register0
    10, 1, 0b0, 0b1010, // load 10 into register1
    11, 0, 1, 2,        // add register0 and register1 and store the result in register2
    14, 2, 1, 3,        // div register2 by register1 and store the result in register3
  ];

  vm.program = program;
  vm.run();

  println!("Program output\n(10 + 12) / 10 == {} R {}", vm.registers[3], vm.remainder);
}
