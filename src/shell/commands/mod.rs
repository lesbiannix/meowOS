pub trait Command {
    fn name(&self) -> &str;
    fn execute(&self, args: &[&str]);
}

pub mod uname;
pub mod help;
pub mod clear;
pub mod echo;