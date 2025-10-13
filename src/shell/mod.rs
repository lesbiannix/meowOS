use crate::{print, println};
use crate::keyboard;
mod parser;
pub mod commands;

pub struct Shell {
    command_executor: parser::CommandExecutor,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            command_executor: parser::CommandExecutor::new(),
        }
    }

    pub fn run(&mut self) {
        let mut buffer = [0u8; 128];
        let mut len = 0;
        println!("Welcome to the new OS shell!");
        loop {
            print!("> ");
            let line = self.read_line(&mut buffer, &mut len);
            if !line.is_empty() {
                self.command_executor.execute(line);
            }
        }
    }

    fn read_line<'a>(&self, buffer: &'a mut [u8], len: &mut usize) -> &'a str {
        *len = 0;
        loop {
            let c = keyboard::read_char();
            match c {
                '\n' => {
                    println!();
                    let line = core::str::from_utf8(&buffer[..*len]).unwrap();
                    return line;
                }
                '\x08' => { // backspace
                    if *len > 0 {
                        *len -= 1;
                        print!("\x08 \x08");
                    }
                }
                _ => {
                    if *len < buffer.len() {
                        buffer[*len] = c as u8;
                        *len += 1;
                        print!("{}", c);
                    }
                }
            }
        }
    }
}