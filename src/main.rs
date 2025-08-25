//! uurl CLI - Command line interface for URL manipulation

use clap::Parser as _;
use color_eyre::Result;
use uurl::{
    cli::Cli,
    util::{get_input, process_input},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let input_source = get_input(&cli)?;
    process_input(input_source, &cli)?;

    Ok(())
}
