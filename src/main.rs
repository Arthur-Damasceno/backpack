mod database;

use database::Database;

use {clap::Parser, std::path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Database file path to open or create.
    #[arg(short, long)]
    open: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut database = if let Some(name) = args.open {
        Database::open(name)
    } else {
        Database::default()
    };

    database.save();
}
