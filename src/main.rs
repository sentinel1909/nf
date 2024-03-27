// nf main.rs

// dependencies
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io::{self, Write};

// struct to represent command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

// main function, program entry point
fn main() -> Result<()> {
    // initialize color_eyre
    color_eyre::install()?;

    // get the command line arguments
    let args = Args::parse();

    // create a handle for writing output
    let mut stdout = io::stdout();

    // create the new file
    let new_file = File::create(args.file);

    // write a success message or an error if the file couldn't be created
    match new_file {
        Ok(file) => write!(stdout, "{:#?} successfully created", file)?,
        Err(err) => write!(stdout, "{:#?}", err)?,
    }

    Ok(())
}