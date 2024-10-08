use crate::cli::*;
use clap::Parser;

pub mod cli;
pub mod data;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
