use std::fmt::{Debug, Display, Formatter};

pub enum SequenceContentError {
    Empty,
}

impl Debug for SequenceContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SequenceContentError::Empty => write!(f, "Empty"),
        }
    }
}

impl Display for SequenceContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SequenceContentError::Empty => "sequence is empty".to_string(),
            }
        )
    }
}
