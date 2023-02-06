
pub mod vm;
pub mod repl;
pub mod instruction;

// A virtual machine that operates on 32-bit instructions

fn main() {
  let mut repl = repl::Repl::new();
  repl.run();
}
