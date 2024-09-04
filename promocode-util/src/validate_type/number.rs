use crate::validate_type::error::out_of_bound::OutOfBoundsError;
use serde::{de::Error, Deserialize, Serialize};
use std::fmt::{Display, Formatter};

macro_rules! generate_bounded_num {
    ($module:ident, $name:ident, $type_name:ident) => {
        /// A [`$name`]($name) that's bounded between two values (inclusive)
        #[repr(transparent)]
        #[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[serde(transparent)]
        pub struct $name<const MIN: $type_name, const MAX: $type_name>($type_name);

        impl<const MIN: $type_name, const MAX: $type_name> $name<MIN, MAX> {
            /// Create a new [`$name`](Self)
            ///
            /// # Errors
            ///
            /// This function fails if `num` is outside [MIN] and [MAX]
            #[allow(unused)]
            pub fn new(num: $type_name) -> Result<Self, OutOfBoundsError<$type_name>> {
                if MIN > num {
                    Err(OutOfBoundsError::Low(MIN, MAX, num))
                } else if num > MAX {
                    Err(OutOfBoundsError::High(MIN, MAX, num))
                } else {
                    Ok(Self(num))
                }
            }

            /// Create a new clamped `$name` (unchecked). Assumes `num` is already clamped between `MIN` and `MAX` (inclusive).
            ///
            /// # Safety
            ///
            /// This function is marked unsafe because it assumes that `num` is
            /// already checked to be within the range from `MIN` to `MAX` (inclusive).
            /// The caller must ensure that this assumption is upheld.
            #[allow(unused)]
            pub unsafe fn new_unchecked(num: $type_name) -> Self {
                Self(num)
            }

            /// Returns the value as a primitive type
            #[allow(unused)]
            pub fn get(self) -> $type_name {
                self.0
            }
        }

        impl<'de, const MIN: $type_name, const MAX: $type_name> Deserialize<'de> for $name<MIN, MAX> {
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

        impl<const MIN: $type_name, const MAX: $type_name> Display for $name<MIN, MAX> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

generate_bounded_num!(bounded_i8, BoundedI8, i8);
generate_bounded_num!(bounded_i16, BoundedI16, i16);
generate_bounded_num!(bounded_i32, BoundedI32, i32);
generate_bounded_num!(bounded_i64, BoundedI64, i64);
generate_bounded_num!(bounded_i128, BoundedI128, i128);
generate_bounded_num!(bounded_isize, BoundedIsize, isize);

generate_bounded_num!(bounded_u8, BoundedU8, u8);
generate_bounded_num!(bounded_u16, BoundedU16, u16);
generate_bounded_num!(bounded_u32, BoundedU32, u32);
generate_bounded_num!(bounded_u64, BoundedU64, u64);
generate_bounded_num!(bounded_u128, BoundedU128, u128);
generate_bounded_num!(bounded_usize, BoundedUsize, usize);

// generate_bounded_num!(bounded_f32, BoundedF32, f32);
// generate_bounded_num!(bounded_f64, BoundedF64, f64);
