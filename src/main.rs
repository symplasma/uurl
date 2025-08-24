//! uurl CLI - Command line interface for URL manipulation

use clap::Parser;
use std::env;
use std::process;

#[derive(Parser)]
#[command(name = "uurl")]
#[command(about = "A CLI tool for URL manipulation")]
#[command(version = "0.1.0")]
struct Cli {
    /// Input URLs or text to process
    #[arg(trailing_var_arg = true)]
    input: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let args: Vec<String> = env::args().collect();
    
    match uurl::get_input(args) {
        Ok(input_source) => {
            let result = uurl::process_input(input_source);
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
