use {
    serde::{Deserialize, Serialize},
    std::{
        fs::File,
        io::{Read, Write},
        path::PathBuf,
    },
};

use crate::error::Result;

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
    pub fn open(name: PathBuf) -> Result<Self> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(name)?;

        let items = {
            let mut buf = Vec::new();

            file.read_to_end(&mut buf)?;

            bincode::deserialize(&buf).unwrap_or_default()
        };

        Ok(Self::Local { file, items })
    }

    pub fn save(&mut self) -> Result {
        if let Self::Local { file, items } = self {
            let data = bincode::serialize(&items).unwrap();

            file.write_all(&data)?;
        }

        Ok(())
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::Memory(Vec::new())
    }
}
