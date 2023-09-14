// nf main.rs

// dependencies
use clap::Parser;
use std::fs::File;
use std::io::{self, Write};

// struct to represent command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut stdout = io::stdout();
    let new_file = File::create(args.name);
    match new_file {
        Ok(file) => write!(stdout, "{:#?} successfully created", file),
        Err(err) => write!(stdout, "{:#?}", err),
    }
}
