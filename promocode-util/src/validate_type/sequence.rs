use crate::validate_type::error::is_empty_sequence::SequenceContentError;
use serde::{de::Error, Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    hash,
};

/// A nonempty [Vec]
#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[serde(transparent)]
pub struct NonEmptyVec<T>(Vec<T>);

impl<T> NonEmptyVec<T> {
    /// Create a new [`NonEmptyVec`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `vec` is empty.
    #[allow(unused)]
    pub fn new(vec: Vec<T>) -> Result<Self, SequenceContentError> {
        if vec.is_empty() {
            Err(SequenceContentError::Empty)
        } else {
            Ok(Self(vec))
        }
    }

    /// # Safety
    ///
    /// This function is unsafe because it assumes `vec` is nonempty.
    /// The caller must ensure that `vec` is not empty to avoid undesirable
    /// behavior.
    #[allow(unused)]
    pub unsafe fn new_unchecked(vec: Vec<T>) -> Self {
        Self(vec)
    }

    /// Returns the value as [String] type
    #[allow(unused)]
    pub fn get(self) -> Vec<T> {
        self.0
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for NonEmptyVec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer).map(Self::new)? {
            Ok(result) => Ok(result),
            Err(err) => Err(D::Error::custom(err)),
        }
    }
}

impl<T: Display> Display for NonEmptyVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|it| format!("{}", it).to_string())
            .collect::<Vec<String>>()
            .join(", ")
            .fmt(f)
    }
}

/// A nonempty [HashSet]
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct NonEmptyHashSet<T: Eq + hash::Hash>(HashSet<T>);

impl<T: Eq + hash::Hash> NonEmptyHashSet<T> {
    /// Create a new [`NonEmptyHashSet`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `hash_set` is empty.
    #[allow(unused)]
    pub fn new(hash_set: HashSet<T>) -> Result<Self, SequenceContentError> {
        if hash_set.is_empty() {
            Err(SequenceContentError::Empty)
        } else {
            Ok(Self(hash_set))
        }
    }

    /// # Safety
    ///
    /// This function is unsafe because it assumes `hash_set` is nonempty.
    /// The caller must ensure that `hash_set` is not empty to avoid undesirable
    /// behavior.
    #[allow(unused)]
    pub unsafe fn new_unchecked(hash_set: HashSet<T>) -> Self {
        Self(hash_set)
    }

    /// Returns the value as [String] type
    #[allow(unused)]
    pub fn get(self) -> HashSet<T> {
        self.0
    }
}

impl<'de, T: Deserialize<'de> + Eq + hash::Hash> Deserialize<'de> for NonEmptyHashSet<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer).map(Self::new)? {
            Ok(result) => Ok(result),
            Err(err) => Err(D::Error::custom(err)),
        }
    }
}

impl<T: Display + Eq + hash::Hash> Display for NonEmptyHashSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|it| format!("{}", it).to_string())
            .collect::<Vec<String>>()
            .join(", ")
            .fmt(f)
    }
}
