//! uurl - A library for URL manipulation
//!
//! This library provides functionality to manipulate URLs from various input sources.

use color_eyre::{Result, eyre::Ok};
use csscolorparser::parse;
use linkify::{LinkFinder, LinkKind, Spans};
use std::io::{self, Read};
use std::process::Command;
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

/// Open a URL with the specified program or the system default browser
fn open_url(url: &str, program: &Option<String>) -> Result<()> {
    match program {
        Some(prog) if !prog.is_empty() => {
            // If the program contains spaces, run via shell
            if prog.contains(' ') {
                #[cfg(target_os = "windows")]
                {
                    Command::new("cmd")
                        .args(["/C", &format!("{prog} {url}")])
                        .spawn()?;
                }
                #[cfg(not(target_os = "windows"))]
                {
                    Command::new("sh")
                        .args(["-c", &format!("{prog} {url}")])
                        .spawn()?;
                }
            } else {
                Command::new(prog).arg(url).spawn()?;
            }
        }
        _ => {
            // Default to system browser
            open::that(url)?;
        }
    }
    Ok(())
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
                    print!("{}", span.as_str());
                }
            }

            // Handle links
            Some(link_kind) => {
                let link_option = match link_kind {
                    LinkKind::Url => {
                        let span_string = span.as_str();
                        match Url::parse(span_string) {
                            std::result::Result::Ok(url) => {
                                // Open the URL if --open is specified
                                if opts.open.is_some() {
                                    open_url(url.as_url().as_str(), &opts.open)?;
                                }

                                if opts.as_markdown {
                                    // get the title of the link
                                    // TODO maybe make link info fetching asynchronous
                                    if let std::result::Result::Ok(info) = Webpage::from_url(
                                        url.as_url().as_str(),
                                        WebpageOptions::default(),
                                    ) {
                                        // TODO add custom formatting options here
                                        Some(format!(
                                            "[{}]({url}): {}",
                                            info.html.title.unwrap_or_default(),
                                            info.html.description.unwrap_or_default()
                                        ))
                                    } else {
                                        // if we cannot retrieve the metadata, render it as a raw link
                                        Some(format!("<{url}>"))
                                    }
                                } else if opts.git_ssh {
                                    Some(url.to_git_ssh())
                                } else {
                                    Some(url.as_url().as_str().to_owned())
                                }
                            }
                            // handle links that the Url lib cannot parse e.g. "relative URL without a base"
                            Err(_) => {
                                if !opts.links_only {
                                    Some(span_string.to_owned())
                                } else {
                                    None
                                }
                            }
                        }
                    }

                    LinkKind::Email => Some(span.as_str().to_owned()),

                    // LinkKind is marked as non-exhaustive so we must have this
                    _ => unimplemented!("This link kind has not been implemented yet."),
                };

                if let Some(string) = link_option {
                    if opts.clickable {
                        print!("{}", string.link(&string).paint(link_style));
                    } else {
                        print!("{}", string.paint(link_style));
                    }

                    if opts.links_only {
                        println!();
                    }
                }
            }
        }
    }

    // add a newline after the text
    // TODO we should probably make this configurable
    if !opts.links_only {
        println!();
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
