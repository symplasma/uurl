use clap::Parser;

#[derive(Parser)]
#[command(name = "uurl")]
#[command(about = "A CLI tool for URL manipulation")]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Input URLs or text to process
    #[arg(trailing_var_arg = true)]
    pub(crate) input: Vec<String>,

    /// Print URLs in color, any color notation valid in CSS works (defaults to blue)
    #[arg(short, long, num_args = 0..=1, default_missing_value = "blue")]
    pub(crate) color_urls: Option<String>,

    /// Make links clickable
    #[arg(long)]
    pub(crate) clickable: bool,

    /// Print only links, one per line
    #[arg(short = 'l', long)]
    pub(crate) links_only: bool,

    /// Prints URLs as Git SSH style urls
    #[arg(short = 's', long)]
    pub(crate) git_ssh: bool,

    /// Prints URLs as markdown links with their title and their description
    #[arg(short = 'm', long)]
    pub(crate) as_markdown: bool,

    /// Open each link with a program (defaults to system browser if no program specified)
    #[arg(short = 'o', long, num_args = 0..=1, default_missing_value = "")]
    pub(crate) open: Option<String>,
}
