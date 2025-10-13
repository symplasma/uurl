# Uurl

Universal url: a transformer and manipulator for Urls.

## Inspiration

- [Xurl](https://lib.rs/crates/xurl) ([GitHub](https://https//github.com/squioc/xurl)): A command-line utility to manipulate urls.
- [urlmatic](https://lib.rs/crates/urlmatic) ([GitHub](https://github.com/bww/urlmatic)): Slice and dice URLs on the command line.
- [trurl](https://github.com/curl/trurl) ([introductory blog post](https://daniel.haxx.se/blog/2023/04/03/introducing-trurl/)): trurl is a command line tool for URL parsing and manipulation.

## Features

- [ ] Find links in text using [linkify](https://crates.io/crates/linkify)
  - [x] Bare links
  - [ ] Git SSH links
  - [ ] Markdown links
- [x] Color links via ANSI codes
- [x] Make links clickable via OSC8 codes
- [x] Convert bare links to Markdown links with titles
- [ ] Add support for setting [webpage crate options](https://docs.rs/webpage/2.0.1/webpage/#options) when retrieving link metadata
- [ ] Add `--continue-on-error` support to skip links where metadata cannot be retrieved
- [ ] Extract various URL components
- [ ] Rewrite/modify various URL components
- [ ] Replace URLs based on template expressions
- [ ] Sort and otherwise normalize query strings
- [ ] Normalize and Clean URLs
  - [ ] This will require a URL classifier as different URL components are different based on their domain
  - [ ] Remove tracking parameters
  - [ ] Remove unnecessary parameters
  - [ ] Limit characters to valid characters
  - [ ] Git URLs
    - [ ] HTTP vs SSH
    - [ ] Be able to switch between various git providers
    - [ ] Truncate a github url to it's basic path i.e. ORG_OR_USER/REPO
- [ ] Switch between lib.rs and crates.io
- [ ] Add whitespace and unicode normalization for link metadata
- [ ] Add custom formatting options for markdown links
- [ ] Figure out how to grab the right title when Anubis or other anti-scripting measures are in place
