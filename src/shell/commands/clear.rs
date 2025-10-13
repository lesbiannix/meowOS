use crate::shell::commands::Command;
use crate::print;

pub struct Clear;

impl Command for Clear {
    fn name(&self) -> &str {
        "clear"
    }

    fn execute(&self, _args: &[&str]) {
        print!("\x1B[2J\x1B[H");
    }
}