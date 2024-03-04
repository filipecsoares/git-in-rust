mod command;
mod commands;
pub mod services;

use std::collections::HashMap;
use std::env;
use crate::command::Command;
use crate::commands::init_command::Init;
use crate::commands::add_command::Add;
use crate::commands::commit_command::Commit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
    commands.insert("init".to_string(), Box::new(Init));
    commands.insert("add".to_string(), Box::new(Add));
    commands.insert("commit".to_string(), Box::new(Commit));

    match commands.get(&args[1]) {
        Some(command) => command.execute(&args[2..]),
        None => println!("Invalid command"),
    }
}
