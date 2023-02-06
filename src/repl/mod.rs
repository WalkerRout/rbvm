
use crate::vm::VM;
use std::io::Write;

pub struct Repl {
  command_buffer: Vec<String>,
  vm: VM
}

impl Repl {
  pub fn new() -> Self {
  	Repl { command_buffer: Vec::new(), vm: VM::new() }
  }

  pub fn run(&mut self) {
    println!("RBVM - Register Based Virtual Machine");

  	loop {
  		let mut buffer = String::new();

      print!("-> ");
      std::io::stdout().flush().expect("Unable to flush stdout...");
      std::io::stdin().read_line(&mut buffer).expect("Unable to read input from user");

      let buffer = buffer.trim();
      self.command_buffer.push(buffer.to_owned());

      match buffer {
        ".quit" => {
          println!("Terminating...");
          std::process::exit(0); // normal exit
        },

        ".spill" => {
          dump_data(self.command_buffer.as_slice());
        },

        _ => {
          println!("Unknown command!");
        }
      }
  	} // end of loop
  }
}

fn dump_data<T: std::fmt::Display>(data: &[T]) {
  for value in data {
    println!("- {}", value);
  }
}