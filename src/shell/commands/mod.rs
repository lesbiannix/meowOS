pub trait Command {
    fn name(&self) -> &str;
    fn execute(&self, args: &[&str]);
}

pub mod clear;
pub mod echo;
pub mod help;
pub mod uname;
