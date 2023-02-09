
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

      self.execute_command(buffer);
    } // end of loop
  }

  fn execute_command(&mut self, command: &str) {
    match command {
      ".quit" => {
        println!("Terminating...");
        std::process::exit(0); // normal exit
      },

      ".spill" => { // eventually have a .dump to print ALL vm data
        dump_data(self.command_buffer.as_slice());
      },

      ".clear" => {
        print!("\x1B[2J\x1B[1;1H\n");
      },

      ".resetpc" => {
        self.vm.pc = 0;
      },

      ".reset" => {
        self.vm.pc = 0;
        self.vm.program = vec![];
        self.vm.registers = [0; 32];
        println!("Environment reset...");
      },

      ".program" => {
        println!("Program instructions (pc={}):", self.vm.pc);

        let mut i = 0;
        for instruction in &self.vm.program {
          if i % 4 == 0 { print!("\n"); }
          print!("0x{:02x} ", instruction);
          i += 1;
        }

        print!("\n");
        println!("-- End of Program Listing -- ");
      },

      ".registers" => {
        println!("Registers and Contents:");
        println!("[");
        for (i, register) in self.vm.registers.iter().enumerate() {
          println!("  {}\t{},", i, register);
        }
        println!("];");
        println!(" -- End of Register Listing -- ")
      },

      ".execute" => {
        self.vm.pc = 0;
        self.vm.registers = [0; 32];
        self.vm.run();
      },

      _ => {
        let bytes = parse_hex(command);

        match bytes {
          Ok(bytes) => {
            for byte in bytes {
              self.vm.add_byte(byte);
            }
          },
          Err(_) => {
            println!("Unable to process input, please enter 4 groups of 2 characters!");
          }
        };

        self.vm.run_once();
      }
    }
  }

}

fn parse_hex(i: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
  let initial_bytes: String = i
    .chars()
    .filter(|c| !c.is_whitespace())
    .enumerate()
    .flat_map(|(i, c)| {
        if i != 0 && i % 2 == 0 {
            Some(' ')
        } else {
            None
        }
        .into_iter()
        .chain(std::iter::once(c))
    })
    .collect();

  let bytes = initial_bytes.split(" ").collect::<Vec<&str>>();
  let mut results: Vec<u8> = vec![];
  
  for hex_string in bytes {
    let byte = u8::from_str_radix(&hex_string, 16);
    
    match byte {
      Ok(result) => {
        results.push(result);
      },
      Err(e) => {
        return Err(e);
      }
    }
  }

  Ok(results)
}

fn dump_data<T: std::fmt::Display>(data: &[T]) {
  for value in data {
    println!("- {}", value);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_hex() {
    let bytes = parse_hex("0A000001");
    assert_eq!(bytes, Ok(vec![0x0A, 0x00, 0x00, 0x01]));

    let bytes = parse_hex("0A 00 00 01");
    assert_eq!(bytes, Ok(vec![0x0A, 0x00, 0x00, 0x01]));

    let bytes = parse_hex("0A 00");
    assert_eq!(bytes, Ok(vec![0x0A, 0x00]));

    let bytes = parse_hex("ABCDEFGHIJKLMNOP");
    assert!(bytes.is_err());
  }
}