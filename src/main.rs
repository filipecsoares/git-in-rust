use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args[1] == "init" {
        fs::create_dir(".git").expect("Could not create .git folder");
        fs::create_dir(".git/objects").expect("Could not create .git/objects folder");
        fs::create_dir(".git/refs").expect("Could not create .git/refs folder");
        fs::write(".git/HEAD", "ref: refs/heads/master\n").expect("Could not create HEAD file");
        println!("Initialized git directory");
    }
}
