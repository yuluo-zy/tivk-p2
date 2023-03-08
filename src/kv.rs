use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
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

impl<R: Read + Seek> BufReaderWithPos<R> {
    pub fn new(mut inner: R) -> Result<BufReaderWithPos<R>>{

        let pos= inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos{
            reader: BufReader::new(inner),
            pos
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let pos = self.reader.read(buf)?;
        self.pos += pos;
        Ok(pos)
    }
}

impl <R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        let mut readers = HashMap::<u64, BufReaderWithPos<File>>::new();
        let mut index = BTreeMap::<String, CommandPos>::new();
        let gen_list = read_file_gen_list(&path)?;
        // 导入文件
        let mut uncompacted = 0;

        for &item in &gen_list {
            let reader = BufReaderWithPos::new(File::open(log_file_path_gen(&path,item))?)?;
            readers.insert(item, reader);
            uncompacted +=
        }

        Ok()
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        todo!()
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        todo!()
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        todo!()
    }

    pub fn compact() -> Result<()> {
        todo!()
    }
}

fn read_file_gen_list(path: &Path) -> Result<Vec<u64>> {
    let mut path_list :Vec<u64>= fs::read_dir(&path)?
        .flat_map(|_dir| -> Result<_> { Ok(_dir?.path() )})
        .filter(|_path: &PathBuf| _path.is_file() && _path.extension() == Some("log".as_ref()))
        .flat_map(|file| {
            file.file_name().and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        }).flatten().collect();
    path_list.sort_unstable();
    Ok(path_list)
}

fn log_file_path_gen(path: &PathBuf, path_number: u64) -> PathBuf {
    path.join(format!("{}.log", path_number))
}
fn load ( gen: u64,
          reader: &mut BufReaderWithPos<File>,
          index: &mut BTreeMap<String, CommandPos>,) {
    let pos = reader.seek(SeekFrom::Start(0));


}
#[test]
fn test_file_gen_list() {
    let path = Path::new("/home/yuluo");
    read_file_gen_list(path);
}