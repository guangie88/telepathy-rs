use emojicons::EmojiFormatter;
use serde::de::{Deserialize, Deserializer, Error};
use std::{char::ParseCharError, ops::Deref, str::FromStr};

#[derive(Clone, Copy, Debug)]
/// Wrapper type over unicode char to represent emoji
pub struct Emoji(char);

/// Implements Deref trait for Emoji
impl Deref for Emoji {
    type Target = char;

    /// Gets the unicode char in Emoji
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implements ToString trait for Emoji
impl ToString for Emoji {
    /// Formats emoji unicode char into String
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}

impl FromStr for Emoji {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        format!("{}", EmojiFormatter(&format!(":{}:", s)))
            .parse::<char>()
            .map(Emoji)
    }
}

/// Implements conversion from string name into the Emoji char for
/// deserialization
impl<'de> Deserialize<'de> for Emoji {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|s| Emoji::from_str(&s).map_err(D::Error::custom))
    }
}
