use crate::shell::commands::Command;
use crate::println;

pub struct Uname;

impl Command for Uname {
    fn name(&self) -> &str {
        "uname"
    }

    fn execute(&self, args: &[&str]) {
        if args.is_empty() || args[0] != "-a" {
            println!("Usage: uname -a");
            return;
        }
        println!("meowOS 0.1.0 {} x86_64 GNU/Linux {}", env!("GIT_HASH"), env!("RUSTC_BUILD_DATE"));
    }
}