use std::io::Write;

use {
    serde::{Deserialize, Serialize},
    std::{fs::File, io::Read, path::PathBuf},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    name: String,
    quantity: u32,
}

#[derive(Debug)]
pub enum Database {
    Local { file: File, items: Vec<Item> },
    Memory(Vec<Item>),
}

impl Database {
    pub fn open(name: PathBuf) -> Self {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(name)
            .unwrap();

        let items = {
            let mut buf = Vec::new();

            file.read_to_end(&mut buf).unwrap();

            bincode::deserialize(&buf).unwrap_or_default()
        };

        Self::Local { file, items }
    }

    pub fn save(&mut self) {
        if let Self::Local { file, items } = self {
            let data = bincode::serialize(&items).unwrap();

            file.write_all(&data).unwrap();
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::Memory(Vec::new())
    }
}
