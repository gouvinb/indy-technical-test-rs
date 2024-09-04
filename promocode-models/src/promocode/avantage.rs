use promocode_util::validate_type::number::BoundedU8;
use serde::{de::Error, Deserialize, Serialize};

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct Avantage {
    pub percent: BoundedU8<0, 100>,
}

impl Avantage {
    /// Create a new [`Avantage`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `percent` is not correct.
    pub fn new(percent: u8) -> Result<Self, String> {
        let percent: BoundedU8<0, 100> = match BoundedU8::new(percent) {
            Err(_) => {
                return Err("`percent` must be greater than 0 and lower than 101.".to_string());
            },
            Ok(value) => value,
        };

        Ok(Self { percent })
    }

    /// Create a new [Avantage] (unchecked)
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe`, meaning the caller must ensure that
    /// `percent` value between 0 and 100 inclusive.
    /// Misuse may lead to undefined behavior.
    #[allow(unused)]
    pub unsafe fn new_unchecked(percent: u8) -> Self {
        Self {
            percent: BoundedU8::new_unchecked(percent),
        }
    }

    /// Returns the percent as [u8] type
    #[allow(unused)]
    pub fn percent(&self) -> u8 {
        self.percent.get()
    }
}

impl<'de> Deserialize<'de> for Avantage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
        pub struct AvantageUnsafe {
            pub percent: u8,
        }

        match AvantageUnsafe::deserialize(deserializer) {
            Ok(data) => Avantage::new(data.percent).map_err(Error::custom),
            Err(err) => Err(Error::custom(err)),
        }
    }
}
