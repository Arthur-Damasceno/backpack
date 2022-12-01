use std::io::stdin;

use crate::{
    database::{Database, Item},
    error::Result,
};

#[derive(Debug)]
pub enum Command {
    List,
    Add(Item),
    Delete { id: usize },
    Save,
    Exit,
}

impl Command {
    pub fn try_read() -> Option<Self> {
        let mut buf = String::new();

        stdin().read_line(&mut buf).unwrap();

        let data = buf.trim();

        if data.starts_with('l') {
            Some(Self::List)
        } else if data.starts_with('s') {
            Some(Self::Save)
        } else if data.starts_with('e') {
            Some(Self::Exit)
        } else if data.starts_with('a') {
            if let Some((quantity, name)) = data
                .split_once(' ')
                .and_then(|(_, data)| data.split_once(' '))
            {
                if let Ok(quantity) = quantity.parse() {
                    return Some(Self::Add(Item {
                        quantity,
                        name: name.into(),
                    }));
                }
            }

            None
        } else if data.starts_with('d') {
            if let Some((_, data)) = data.split_once(' ') {
                if let Ok(id) = data.parse() {
                    return Some(Self::Delete { id });
                }
            }

            None
        } else {
            None
        }
    }

    pub fn execute(self, database: &mut Database) -> Result {
        match self {
            Self::List => {
                let items = database.items();

                if items.len() > 0 {
                    println!("The items in the backpack are:");

                    for (idx, item) in items.iter().enumerate() {
                        println!("{} {item:?}", idx + 1);
                    }
                } else {
                    println!("There are no items in the backpack");
                }
            }
            Self::Add(item) => {
                database.add(item);
                println!("The item has been added");
            }
            Self::Delete { id } => {
                database.delete(id)?;
                println!("The item has been deleted");
            }
            Self::Save => {
                database.save()?;
                let amount = database.items().len();
                println!("{amount} items were saved");
            }
            Self::Exit => std::process::exit(0),
        };

        Ok(())
    }
}
