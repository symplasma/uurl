//! uurl - A library for URL manipulation
//! 
//! This library provides functionality to manipulate URLs from various input sources.

use std::io::{self, Read};

/// Represents the different sources of input for the URL processor
#[derive(Debug, Clone)]
pub enum InputSource {
    Stdin(String),
    Args(Vec<String>),
    Clipboard(String),
}

/// Get input from the most appropriate source based on availability
/// 
/// Priority order:
/// 1. STDIN (if available)
/// 2. Command line arguments (if provided)
/// 3. System clipboard (as fallback)
pub fn get_input(args: Vec<String>) -> Result<InputSource, Box<dyn std::error::Error>> {
    // Check if STDIN has data
    if !atty::is(atty::Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        if !buffer.trim().is_empty() {
            return Ok(InputSource::Stdin(buffer.trim().to_string()));
        }
    }
    
    // Check if args are provided (skip program name)
    if args.len() > 1 {
        return Ok(InputSource::Args(args[1..].to_vec()));
    }
    
    // Fall back to clipboard
    let mut clipboard = arboard::Clipboard::new()?;
    let clipboard_content = clipboard.get_text()?;
    Ok(InputSource::Clipboard(clipboard_content))
}

/// Process the input and return the result
/// 
/// Currently just passes through the input as-is.
/// Future versions will add URL transformation logic.
pub fn process_input(input: InputSource) -> String {
    match input {
        InputSource::Stdin(content) => content,
        InputSource::Args(args) => args.join(" "),
        InputSource::Clipboard(content) => content,
    }
}
