use crate::shell::commands::Command;
use crate::shell::commands::uname::Uname;
use crate::shell::commands::help::Help;
use crate::shell::commands::clear::Clear;
use crate::shell::commands::echo::Echo;
use heapless::Vec;
use crate::println;

pub struct CommandExecutor {
    commands: Vec<&'static dyn Command, 16>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        let mut commands: Vec<&'static dyn Command, 16> = Vec::new();
        if commands.push(&Uname).is_err() {
            println!("WARNING: command queue full; dropping command");
        }
        if commands.push(&Help).is_err() {
            println!("WARNING: command queue full; dropping command");
        }
        if commands.push(&Clear).is_err() {
            println!("WARNING: command queue full; dropping command");
        }
        if commands.push(&Echo).is_err() {
            println!("WARNING: command queue full; dropping command");
        }
        CommandExecutor { commands }
    }

    pub fn execute(&self, line: &str) {
        let mut parts = line.split_whitespace();
        if let Some(command_name) = parts.next() {
            if let Some(command) = self.commands.iter().find(|c| c.name() == command_name) {
                let args: Vec<&str, 16> = parts.collect();
                command.execute(&args);
            } else {
                println!("Command not found: {}", command_name);
            }
        }
    }
}