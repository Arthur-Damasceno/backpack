use std::io::stdin;

use crate::database::Item;

#[derive(Debug)]
pub enum Command {
    List,
    Add(Item),
    Delete { id: u32 },
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
}
