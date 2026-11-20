use std::collections::HashMap;
use std::{error, io};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
};
use wincode::{SchemaRead, SchemaWrite};

#[derive(Debug)]
struct DbErr {
    kind: DbErrKind
}

#[derive(Debug)]
enum DbErrKind {
    Io(io::Error),
    Miss()
}

#[derive(Debug, SchemaRead, SchemaWrite)]
enum Types {
    Int(i32),
    Float(f64),
    Bool(bool),
    Text(String),
}

#[derive(Debug, SchemaRead, SchemaWrite)]
struct KeyValue {
    key: String,
    value: Option<Types>,
}

struct LogIndex {
    index: HashMap<String, LogIndexEntry>,
}

struct LogIndexEntry {
    len: usize,
    offset: usize,
}

struct Writer {
    active_data_file: File,
}

struct DCask {
    writer: Writer,
    logindex: LogIndex
}

impl DCask {
    fn new(name: &str) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(name)
            .unwrap();
        let writer = Writer {
            active_data_file: file,
        };
        
        let logindex = LogIndex {
            index: HashMap::new()
        };
        DCask {
            writer: writer,
            logindex: logindex
        }
    }

    fn set(&mut self, key: &str, value: Types) {
        let byt = KeyValue {
            key: key.to_string(),
            value: Some(value),
        };
        let decoded = wincode::serialize(&byt).unwrap();
        self.write_active_data_file(decoded, key);
    }

    
    fn write_active_data_file(&mut self, bytes: Vec<u8>, key: &str) {
        self.writer.active_data_file.write_all(&bytes).unwrap();
        let index = LogIndexEntry {
            len: bytes.len(),
            offset: self.writer.active_data_file.stream_position().unwrap() as usize - bytes.len(),
        };
       self.logindex.index.insert(key.to_string(), index);
        
    }
    fn get(&mut self, key: &str) -> Result<Types, DbErr> {
        let log_index_entry = self.logindex.index.get(key).unwrap();
        let offset = log_index_entry.offset as u64;
        let len = log_index_entry.len;
       self.writer.active_data_file.seek(std::io::SeekFrom::Start(offset)).unwrap();
       let mut buf = Vec::new();
       self.writer.active_data_file.read_to_end(&mut buf).unwrap();
       let res: KeyValue = wincode::deserialize(&buf).unwrap();
       if let Some(i) = res.value {
           return Ok(i);
       } Err(DbErr {
           kind: DbErrKind::Miss()
       })
    }
}



fn main() {
    let mut db = DCask::new("name");
    db.set("key", Types::Int(1));
    println!("{:?}", db.get("key").unwrap());
    
}
