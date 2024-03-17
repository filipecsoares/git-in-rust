use crate::command::Command;
use std::{ffi::CStr, io::{prelude::*, BufReader}};
use flate2::read::ZlibDecoder;
use anyhow::{Context, Ok};

pub struct CatFile;

impl Command for CatFile {
	fn execute(&self, args: &[String]) -> anyhow::Result<()> {
		// TODO - support shortest unique object hashes
		let file_path = &args[0];
		let file = std::fs::File::open(format!(".git/objects/{}/{}", &file_path[..2], &file_path[2..])).expect("Could not find file.");
		let z = ZlibDecoder::new(file);
		let mut z = BufReader::new(z);
		let mut buf = Vec::new();
		z.read_until(0, &mut buf).expect("Could not read file");
		let header = CStr::from_bytes_with_nul(&buf)
                .expect("know there is exactly one nul, and it's at the end");
            let header = header
                .to_str()
                .context(".git/objects file header isn't valid UTF-8")?;
            let Some((kind, size)) = header.split_once(' ') else {
                anyhow::bail!(
                    ".git/objects file header did not start with a known type: '{header}'"
                );
            };
            let kind = match kind {
                "blob" => Kind::Blob,
                _ => anyhow::bail!("we do not yet know how to print a '{kind}'"),
            };
            let size = size
                .parse::<u64>()
                .context(".git/objects file header has invalid size: {size}")?;
            let mut z = z.take(size);
            match kind {
                Kind::Blob => {
                    let stdout = std::io::stdout();
                    let mut stdout = stdout.lock();
                    let n = std::io::copy(&mut z, &mut stdout)
                        .context("write .git/objects file to stdout")?;
                    anyhow::ensure!(n == size, ".git/object file was not the expected size (expected: {size}, actual: {n})");
                }
            }
			Ok(())
	}
}

enum Kind {
    Blob,
}