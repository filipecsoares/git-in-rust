use std::path::Path;
use std::fs;
use anyhow::Ok;

pub struct Init;

impl Init {
    pub fn execute() -> anyhow::Result<()>{
        if Path::new(".git").exists() {
            anyhow::bail!("The .git directory already exists.");
        } else {
            fs::create_dir(".git").expect("Could not create .git folder");
            fs::create_dir(".git/branches").expect("Could not create .git/branches folder");
            fs::create_dir(".git/hooks").expect("Could not create .git/hooks folder");
            fs::create_dir(".git/info").expect("Could not create .git/info folder");
            fs::create_dir(".git/objects").expect("Could not create .git/objects folder");
            fs::create_dir(".git/objects/info").expect("Could not create .git/objects/info folder");
            fs::create_dir(".git/objects/pack").expect("Could not create .git/objects/pack folder");
            fs::create_dir(".git/refs").expect("Could not create .git/refs folder");
            fs::create_dir(".git/refs/heads").expect("Could not create .git/refs/heads folder");
            fs::create_dir(".git/refs/tags").expect("Could not create .git/refs/tags folder");
            fs::write(".git/HEAD", "ref: refs/heads/main\n").expect("Could not create HEAD file");
            fs::write(".git/config", "[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n\tlogallrefupdates = true\n").expect("Could not create config file");
            fs::write(".git/description", "Unnamed repository; edit this file 'description' to name the repository.\n").expect("Could not create description file");
            println!("Initialized git directory");
        }
        Ok(())
    }
}