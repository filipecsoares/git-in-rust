use std::fs;
use crate::command::Command;
use crate::services::hash_service::hash;

pub struct Add;

impl Command for Add {
    fn execute(&self, args: &[String]) {
        let file_path = &args[2];
        let data = fs::read_to_string(file_path).expect("Could not read file");
        let hashed_data = hash(&data);
        fs::write(format!(".git/objects/{}", hashed_data), data).expect("Could not write object file");
        println!("Changes to file {} have been staged.", file_path);
    }
}