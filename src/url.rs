use std::fmt;
use std::str::FromStr;

/// A newtype wrapper around the `url` crate's `Url` type.
/// 
/// This provides a more controlled interface for URL handling within this crate
/// and allows for future extensions and customizations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Url(url::Url);

impl Url {
    /// Create a new `Url` from a string.
    pub fn parse(input: &str) -> Result<Self, url::ParseError> {
        url::Url::parse(input).map(Url)
    }

    /// Get the underlying `url::Url`.
    pub fn as_url(&self) -> &url::Url {
        &self.0
    }

    /// Convert into the underlying `url::Url`.
    pub fn into_url(self) -> url::Url {
        self.0
    }
}

impl FromStr for Url {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<url::Url> for Url {
    fn from(url: url::Url) -> Self {
        Url(url)
    }
}

impl From<Url> for url::Url {
    fn from(url: Url) -> Self {
        url.0
    }
}
