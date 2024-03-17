use crate::command::Command;

pub struct Commit;

impl Command for Commit {
    fn execute(&self, _args: &[String]) -> anyhow::Result<()>{
        // Your commit code here...
        println!("Commit created.");
        Ok(())
    }
}