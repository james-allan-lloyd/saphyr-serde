use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("Trailing characters")]
    TrailingCharacters,

    #[error("Invalid type")]
    TypeError,

    #[error("Unexpected element {event_name} at line {}, column {}", .span.start.line(), .span.start.col())]
    UnexpectedElement {
        event_name: String,
        span: saphyr_parser::Span,
    },

    #[error("Serde error")]
    SerdeError(String),
}

impl DeserializeError {
    pub(crate) fn unexpected(
        event: &saphyr_parser::Event,
        span: saphyr_parser::Span,
        location: &str,
    ) -> Self {
        Self::UnexpectedElement {
            event_name: format!("{:?} (in {})", event, location),
            span,
        }
    }
}

impl serde::de::Error for DeserializeError {
    #[doc = r" Raised when there is general error when deserializing a type."]
    #[doc = r""]
    #[doc = r" The message should not be capitalized and should not end with a period."]
    #[doc = r""]
    #[doc = r" ```edition2021"]
    #[doc = r" # use std::str::FromStr;"]
    #[doc = r" #"]
    #[doc = r" # struct IpAddr;"]
    #[doc = r" #"]
    #[doc = r" # impl FromStr for IpAddr {"]
    #[doc = r" #     type Err = String;"]
    #[doc = r" #"]
    #[doc = r" #     fn from_str(_: &str) -> Result<Self, String> {"]
    #[doc = r" #         unimplemented!()"]
    #[doc = r" #     }"]
    #[doc = r" # }"]
    #[doc = r" #"]
    #[doc = r" use serde::de::{self, Deserialize, Deserializer};"]
    #[doc = r""]
    #[doc = r" impl<'de> Deserialize<'de> for IpAddr {"]
    #[doc = r"     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>"]
    #[doc = r"     where"]
    #[doc = r"         D: Deserializer<'de>,"]
    #[doc = r"     {"]
    #[doc = r"         let s = String::deserialize(deserializer)?;"]
    #[doc = r"         s.parse().map_err(de::Error::custom)"]
    #[doc = r"     }"]
    #[doc = r" }"]
    #[doc = r" ```"]
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerdeError(format!("{}", msg))
    }
}

pub type Result<T> = std::result::Result<T, DeserializeError>;
