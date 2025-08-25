# Overview

`uurl` is a Rust library and a CLI app to manipulate URLs.

It should use the Rust 2024 edition.

## Features

### Input Handling

- Take input from Args, STDIN, and the system clipboard
  - Detect if input is being passed in via STDIN and prefer that source if so
  - If no STDIN input, check if there are any args being passed in
  - If neither of the above is true, then pull the contents of the system clipboard as input
- Might want to normalize input to valid and safe Unicode. See the [picleo](https://crates.io/crates/picleo) crate.
