use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, Write};
use std::path::PathBuf;
use anyhow::Result;

fn main() {
    println!("Hello, world!");
}

pub struct KvStore {
    path: PathBuf,
    // map generation number to the file reader.
    readers: HashMap<u64, BufReaderWithPos<File>>,
    // writer of the current log.
    writer: BufWriterWithPos<File>,
    current_gen: u64,
    index: BTreeMap<String, CommandPos>,
    // the number of bytes representing "stale" commands that could be
    // deleted during a compaction.
    uncompacted: u64,
}

struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut readers = HashMap::<u64,BufReaderWithPos<File>>::new();
        let mut index = BTreeMap::<String, CommandPos>::new();

        // 导入文件

    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        pass
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        pass
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        pass
    }

    pub fn compact() -> Result<()> {
        pass
    }
}
