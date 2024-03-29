mod command;
mod commands;
pub mod services;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::commands::init_command::Init;
use crate::commands::add_command::Add;
use crate::commands::commit_command::Commit;
use crate::commands::cat_file_command::CatFile;
use crate::commands::hash_object_command::HashObject;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        CommandEnum::Init => {
            Init::execute()
        }
        CommandEnum::Add { file_path } => {
            Add::execute(&file_path)
        }
        CommandEnum::Commit { message } => {
            Commit::execute(&message)
        }
        CommandEnum::CatFile { pretty_print, object_hash } => {
            CatFile::execute(pretty_print, object_hash)
        }
        CommandEnum::HashObject { write, file } => {
            HashObject::execute(write, file)
        }
        _ => {
            anyhow::bail!("Invalid command.")
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: CommandEnum,
}

#[derive(Debug, Subcommand)]
enum CommandEnum {
    Init,
    Add {
        file_path: String,
    },
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        file: PathBuf,
    },
    LsTree {
        #[clap(long)]
        name_only: bool,

        tree_hash: String,
    },
    WriteTree,
    CommitTree {
        #[clap(short = 'm')]
        message: String,

        #[clap(short = 'p')]
        parent_hash: Option<String>,

        tree_hash: String,
    },
    Commit {
        #[clap(short = 'm')]
        message: String,
    },
}