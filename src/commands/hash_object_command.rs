use std::{io::Write, path::PathBuf};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use anyhow::{Context, Ok};

use crate::command::Command;

pub struct HashObject;

impl Command for HashObject {
    fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        let file_path: PathBuf = (&args[0]).into();
        let stat = std::fs::metadata(&file_path).with_context(|| format!("stat {}", file_path.display()))?;

        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        write!(e, "blob ")?;
        write!(e, "{}\0", stat.len())?;
        let compressed = e.finish();
        let mut hasher = Sha1::new();
        println!("Not Ready yet!");

        Ok(())
    }
}

struct HashWriter<W> {
    writer: W,
    hasher: Sha1,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}