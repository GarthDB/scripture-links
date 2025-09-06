//! Scripture Links CLI Application
//!
//! A command-line tool for converting scripture references to URLs and processing text
//! containing scripture references.

use clap::Parser;
use scripture_links::cli::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
