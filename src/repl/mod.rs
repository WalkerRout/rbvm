
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

        ".program" => {
          println!("Program instructions:");

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
          println!("{:#?}", self.vm.registers);
          println!(" -- End of Register Listing -- ")
        },

        _ => {
          let bytes = parse_hex(buffer);

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
    } // end of loop
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