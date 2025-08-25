use clap::Parser;

#[derive(Parser)]
#[command(name = "uurl")]
#[command(about = "A CLI tool for URL manipulation")]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Input URLs or text to process
    #[arg(trailing_var_arg = true)]
    pub(crate) input: Vec<String>,
}
