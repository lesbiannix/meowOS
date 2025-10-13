use crate::shell::commands::Command;
use crate::println;

pub struct Help;

impl Command for Help {
    fn name(&self) -> &str {
        "help"
    }

    fn execute(&self, _args: &[&str]) {
        println!("Available commands:");
        println!("  help - Display this help message");
        println!("  uname -a - Display system information");
        println!("  clear - Clear the screen");
        println!("  echo - Display a line of text");
    }
}