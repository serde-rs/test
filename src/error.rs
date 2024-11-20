use alloc::string::{String, ToString};
use core::fmt::{self, Display};
use serde::{de, ser};
#[cfg(feature = "std")]
use std::error;

#[derive(Clone, Debug)]
pub struct Error {
    msg: String,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error {
            msg: msg.to_string(),
        }
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.msg)
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl PartialEq<str> for Error {
    fn eq(&self, other: &str) -> bool {
        self.msg == other
    }
}
