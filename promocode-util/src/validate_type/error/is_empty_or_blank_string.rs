use std::fmt::{Debug, Display, Formatter};

pub enum StringContentError {
    Empty,
    Blank(String),
}

impl Debug for StringContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StringContentError::Empty => write!(f, "Empty"),
            StringContentError::Blank(value) => write!(f, "Blank(value = `{}`)", value.escape_debug()),
        }
    }
}

impl Display for StringContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StringContentError::Empty => "string is empty".to_string(),
                StringContentError::Blank(value) => format!("string is blank (content: `{}`)", value),
            }
        )
    }
}
