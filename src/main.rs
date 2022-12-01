mod command;
mod database;
mod error;

use {command::Command, database::Database, error::Result};

use {clap::Parser, std::path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Database file path to open or create.
    #[arg(short, long)]
    open: Option<PathBuf>,
}

fn main() -> Result {
    let Args { open } = Args::parse();

    let mut database = if let Some(name) = open {
        Database::open(name)?
    } else {
        Database::default()
    };

    println!(concat!(
        "Welcome to Backpack! Where you can handle your items\n",
        include_str!("commands.txt"),
    ));

    loop {
        if let Some(command) = Command::try_read() {
            command.execute(&mut database)?;
        } else {
            eprintln!(concat!(
                "Could not understand the command.\n",
                include_str!("commands.txt")
            ));
        }
    }
}
