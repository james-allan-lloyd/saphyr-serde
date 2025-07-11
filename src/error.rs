use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
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

    #[error("Error during deserialization: {0}")]
    SerdeError(String),

    #[error("Unable to parse {text} as a {type_string} at line {}, column {}: {err}", .span.start.line(), .span.start.col())]
    NumberParseError {
        text: String,
        err: String,
        type_string: String,
        span: saphyr_parser::Span,
    },

    #[error("Unable to parse {text} as a boolean at line {}, column {}", .span.start.line(), .span.start.col())]
    BoolParseError {
        text: String,
        span: saphyr_parser::Span,
    },

    #[error("Unexpected early termination")]
    EarlyTermination,

    #[error("Scan error")]
    ScanError(#[from] saphyr_parser::ScanError),
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

    pub(crate) fn number_parse_failure(
        value: &str,
        span: saphyr_parser::Span,
        type_string: &str,
        parse_error: &str,
    ) -> DeserializeError {
        Self::NumberParseError {
            text: String::from(value),
            err: String::from(parse_error),
            type_string: String::from(type_string),
            span,
        }
    }

    pub(crate) fn not_a_bool(value: &str, span: saphyr_parser::Span) -> DeserializeError {
        Self::BoolParseError {
            text: String::from(value),
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
