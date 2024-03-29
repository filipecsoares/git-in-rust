pub struct Commit;

impl Commit {
    pub fn execute(message: &String) -> anyhow::Result<()>{
        // Your commit code here...
        println!("Commit created.");
        println!("{}", message);
        Ok(())
    }
}