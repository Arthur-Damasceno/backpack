use {
    serde::{Deserialize, Serialize},
    std::{
        fs::File,
        io::{Read, Seek, Write},
        path::PathBuf,
    },
};

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub quantity: u32,
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

    pub fn items(&self) -> &[Item] {
        match self {
            Database::Local { items, .. } => items,
            Database::Memory(items) => items,
        }
    }

    pub fn add(&mut self, item: Item) {
        match self {
            Database::Local { items, .. } => items,
            Database::Memory(items) => items,
        }
        .push(item);
    }

    pub fn delete(&mut self, id: usize) -> Result {
        let items = match self {
            Database::Local { items, .. } => items,
            Database::Memory(items) => items,
        };

        if id > items.len() {
            return Err(Error::NonExistentItem);
        }

        items.remove(id - 1);

        Ok(())
    }

    pub fn save(&mut self) -> Result {
        if let Self::Local { file, items } = self {
            let data = bincode::serialize(&items).unwrap();

            file.rewind()?;
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
