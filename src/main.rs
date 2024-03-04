use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args[1] == "init" {
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
    } else if args[1] == "add" {
        let file_path = &args[2];
        let data = fs::read_to_string(file_path).expect("Could not read file");
        let hashed_data = hash(&data);
        fs::write(format!(".git/objects/{}", hashed_data), data).expect("Could not write object file");
        println!("Changes to file {} have been staged.", file_path);
    } else {
        println!("Invalid command");
    }
}

fn hash(data: &String) -> String {
    // TODO - You'll need to implement a hash function
    data.to_string()
}
