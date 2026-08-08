//! XML Schema string wrapper types used by the generated models.

use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

macro_rules! encoded_string {
    ($name:ident, $length:expr, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            /// Returns the byte length of the represented data.
            #[must_use]
            pub fn len(&self) -> usize {
                $length(&self.0)
            }

            /// Returns `true` if the encoded string is empty.
            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Returns the encoded value as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl FromStr for $name {
            type Err = Infallible;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self(value.to_owned()))
            }
        }

        impl Deref for $name {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

encoded_string!(
    Base64String,
    |value: &str| value.trim_end_matches('=').len() * 3 / 4,
    "A `base64Binary` value stored in its encoded string representation."
);

encoded_string!(
    HexString,
    |value: &str| value.len() / 2,
    "A `hexBinary` value stored in its encoded string representation."
);
