use crate::command::Command;

pub struct Commit;

impl Command for Commit {
    fn execute(&self, _args: &[String]) {
        // Your commit code here...
        println!("Commit created.");
    }
}