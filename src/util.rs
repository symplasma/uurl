//! uurl - A library for URL manipulation
//!
//! This library provides functionality to manipulate URLs from various input sources.

use color_eyre::{Result, eyre::Ok};
use csscolorparser::parse;
use linkify::{LinkFinder, LinkKind, Spans};
use std::io::{self, Read};
use webpage::{Webpage, WebpageOptions};
use yansi::{Paint, Style, hyperlink::HyperlinkExt};

use crate::{cli::Cli, url::Url};

/// Represents the different sources of input for the URL processor
#[derive(Debug, Clone)]
pub enum InputSource {
    // TODO make input on STDIN streaming to handle large inputs
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
pub fn get_input(opts: &Cli) -> Result<InputSource> {
    // Check if STDIN has data
    if !atty::is(atty::Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        if !buffer.trim().is_empty() {
            return Ok(InputSource::Stdin(buffer.trim().to_string()));
        }
    }

    // Check if args are provided (skip program name)
    if opts.input.len() > 1 {
        return Ok(InputSource::Args(opts.input.clone()));
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
pub fn process_input(input: InputSource, opts: &Cli) -> Result<()> {
    let text = match input {
        InputSource::Stdin(content) => content,
        InputSource::Args(args) => args.join("\n"),
        InputSource::Clipboard(content) => content,
    };

    let mut link_style = Style::default();
    if let Some(color_urls) = &opts.color_urls {
        let [r, g, b, _a] = parse(color_urls)?.to_rgba8();
        link_style = link_style.rgb(r, g, b);
    }
    for span in text_to_spans(&text) {
        match span.kind() {
            // Handle non-link text
            None => {
                if !opts.links_only {
                    print!("{}", span.as_str())
                }
            }

            // Handle links
            Some(link_kind) => {
                let string = match link_kind {
                    LinkKind::Url => {
                        let url = Url::parse(span.as_str())?;
                        if opts.as_markdown {
                            // get the title of the link
                            let info =
                                Webpage::from_url(url.as_url().as_str(), WebpageOptions::default())
                                    .expect("could not get info for url");

                            // TODO add whitespace and unicode normalization for metadata
                            // TODO add custom formatting options here
                            format!(
                                "[{}]({url}): {}",
                                info.html.title.unwrap_or_default(),
                                info.html.description.unwrap_or_default()
                            )
                        } else if opts.git_ssh {
                            url.to_git_ssh()
                        } else {
                            url.as_url().as_str().to_owned()
                        }
                    }
                    LinkKind::Email => span.as_str().to_owned(),

                    // LinkKind is marked as non-exhaustive so we must have this
                    _ => unimplemented!("This link kind has not been implemented yet."),
                };
                if opts.clickable {
                    print!("{}", string.link(&string).paint(link_style));
                } else {
                    print!("{}", string.paint(link_style));
                }

                if opts.links_only {
                    println!()
                }
            }
        }
    }

    // add a newline after the text
    // TODO we should probably make this configurable
    if !opts.links_only {
        println!()
    }

    Ok(())
}

/// Find URls in the input
pub fn text_to_spans(text: &'_ str) -> Spans<'_> {
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(false);
    finder.spans(text)
}

/// Returns all characters except the first
pub(crate) fn skip_first_char(s: &str) -> &str {
    // If there's a first character, get its length in bytes.
    // Otherwise, return an empty string.
    s.chars().next().map(|c| &s[c.len_utf8()..]).unwrap_or("")
}
