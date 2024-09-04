use crate::validate_type::error::is_empty_or_blank_string::StringContentError;
use serde::{de::Error, Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A nonempty [String]
#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[serde(transparent)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Create a new [`NonEmptyString`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `string` is empty.
    #[allow(unused)]
    pub fn new(string: String) -> Result<Self, StringContentError> {
        if string.is_empty() {
            Err(StringContentError::Empty)
        } else {
            Ok(Self(string))
        }
    }

    /// # Safety
    ///
    /// This function assumes `string` is nonempty and does not check that. The
    /// caller must guarantee that the input string is not empty.
    #[allow(unused)]
    pub unsafe fn new_unchecked(string: String) -> Self {
        Self(string)
    }

    /// Returns the value as [String] type
    #[allow(unused)]
    pub fn get(self) -> String {
        self.0
    }
}

impl<'de> Deserialize<'de> for NonEmptyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer).map(Self::new)? {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::custom(err)),
        }
    }
}

impl Display for NonEmptyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A nonblank [String]
#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[serde(transparent)]
pub struct NonBlankString(String);

impl NonBlankString {
    /// Create a new [`NonBlankString`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `string` is empty.
    #[allow(unused)]
    pub fn new(string: String) -> Result<Self, StringContentError> {
        if string.trim().is_empty() {
            Err(StringContentError::Blank(string))
        } else {
            Ok(Self(string))
        }
    }

    /// # Safety
    ///
    /// This function assumes `string` is not blank and does not check that. The
    /// caller must guarantee that the input string is not whitespace or empty.
    #[allow(unused)]
    pub unsafe fn new_unchecked(string: String) -> Self {
        Self(string)
    }

    /// Returns the value as [String] type
    #[allow(unused)]
    pub fn get(self) -> String {
        self.0
    }
}

impl<'de> Deserialize<'de> for NonBlankString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer).map(Self::new)? {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::custom(err)),
        }
    }
}

impl Display for NonBlankString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
