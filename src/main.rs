use std::{fs::{File, OpenOptions}, process::Output};
use wincode::{SchemaRead, SchemaWrite, config::Config};
use wincode::config::ConfigCore;

#[derive(Debug)]
#[derive(SchemaRead, SchemaWrite)]
enum Types {
    Int(i32),
    Float(f64),
    Bool(bool),
    Text(String),
}

trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}
impl ToBytes for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl ToBytes for f64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl ToBytes for bool {
    fn to_bytes(&self) -> Vec<u8> {
        [*self as u8].to_vec()
    }
}
impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
impl ToBytes for &str {
     
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

trait EncodeBytes {
    type Output;
    fn encode_bytes(self) -> Output;
}


#[derive(SchemaRead, SchemaWrite)]
#[derive(Debug)]
struct LogIndex
{
    key: String,
    value: Types,
}

struct Writer {
    active_data_file: File,
}

struct DCask {
    writer: Writer,
}

impl DCask {
    fn new(name: &str) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .append(true)
            .open(name)
            .unwrap();
        let writer = Writer {
            active_data_file: file,
        };
        DCask { writer: writer }
    }

    fn set(&mut self, key: &str, value: Types)
   {
        let byt = LogIndex {
            key: key.to_string(),
            value: value,
        };
        let decoded = wincode::serialize(&byt).unwrap();
        let encoded: LogIndex<> = wincode::deserialize(&decoded).unwrap();
        println!("{:?}", &encoded.value);
    }
}

fn main() {
    let mut db = DCask::new("name");
    db.set("key", Types::Int(1));
}
