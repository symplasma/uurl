# Uurl

Universal url: a transformer and manipulator for Urls.

## Inspiration

- [Xurl](https://lib.rs/crates/xurl) ([GitHub](https://https//github.com/squioc/xurl)): A command-line utility to manipulate urls.
- [urlmatic](https://lib.rs/crates/urlmatic) ([GitHub](https://github.com/bww/urlmatic)): Slice and dice URLs on the command line.
- [trurl](https://github.com/curl/trurl) ([introductory blog post](https://daniel.haxx.se/blog/2023/04/03/introducing-trurl/)): trurl is a command line tool for URL parsing and manipulation.

## Features

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
