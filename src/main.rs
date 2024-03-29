mod command;
mod commands;
pub mod services;

use std::collections::HashMap;
use std::env;
use crate::command::Command;
use crate::commands::init_command::Init;
use crate::commands::add_command::Add;
use crate::commands::commit_command::Commit;
use crate::commands::cat_file_command::CatFile;
use crate::commands::hash_object_command::HashObject;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
    commands.insert("init".to_string(), Box::new(Init));
    commands.insert("add".to_string(), Box::new(Add));
    commands.insert("commit".to_string(), Box::new(Commit));
    commands.insert("cat-file".to_string(), Box::new(CatFile));
    commands.insert("hash-object".to_string(), Box::new(HashObject));

    match commands.get(&args[1]) {
        Some(command) => command.execute(&args[2..]),
        None => anyhow::bail!("Invalid command."),
    }
}
