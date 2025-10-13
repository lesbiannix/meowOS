use crate::shell::commands::Command;
use crate::{print, println};

pub struct Echo;

impl Command for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn execute(&self, args: &[&str]) {
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", arg);
        }
        println!();
    }
}