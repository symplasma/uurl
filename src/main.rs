//! uurl CLI - Command line interface for URL manipulation

use clap::Parser as _;
use std::process;
use uurl::{
    cli::Cli,
    util::{get_input, process_input},
};

fn main() {
    let cli = Cli::parse();

    match get_input(&cli) {
        Ok(input_source) => {
            process_input(input_source, &cli);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}
