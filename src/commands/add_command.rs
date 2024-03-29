use walkdir::WalkDir;
use std::path::Path;
use std::fs;
use crate::services::hash_service::hash;

pub struct Add;

impl Add {
    pub fn execute(file_path: &String) -> anyhow::Result<()> {
        if file_path == "." {
            for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    stage_file(entry.path());
                }
            }
        } else {
            stage_file(Path::new(file_path));
        }
        Ok(())
    }
}

fn stage_file(file_path: &Path) {
    let data = fs::read_to_string(file_path).expect("Could not read file");
    let hashed_data = hash(&data);
    let dir = format!(".git/objects/{}", &hashed_data[..2]);
    let file = format!("{}/{}", dir, &hashed_data[2..]);
    fs::create_dir_all(dir).expect("Could not create objects directory");
    fs::write(file, data).expect("Could not write object file");
    println!("Changes to file {:?} have been staged.", file_path);
}
