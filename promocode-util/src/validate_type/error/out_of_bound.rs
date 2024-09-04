use std::fmt::{Debug, Display, Formatter};

pub enum OutOfBoundsError<T> {
    High(T, T, T),
    Low(T, T, T),
}

impl<T: Display> Debug for OutOfBoundsError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutOfBoundsError::High(min, max, value) => write!(f, "High(min = {}, max = {}, value = {})", min, max, value),
            OutOfBoundsError::Low(min, max, value) => write!(f, "Low(min = {}, max = {}, value = {})", min, max, value),
        }
    }
}

impl<T: Display> Display for OutOfBoundsError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OutOfBoundsError::High(min, max, value) => format!("{} is too high (range: {}..{})", value, min, max),
                OutOfBoundsError::Low(min, max, value) => format!("{} is too low (range: {}..{})", value, min, max),
            }
        )
    }
}
