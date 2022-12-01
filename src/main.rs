use {clap::Parser, std::path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Database file name to read.
    #[arg(short, long)]
    read: Option<PathBuf>,
    /// File name to create as a database.
    #[arg(short, long)]
    create: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("{args:#?}");
}
