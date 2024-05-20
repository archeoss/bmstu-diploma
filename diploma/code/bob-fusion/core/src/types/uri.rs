use std::ops::{Deref, DerefMut};

use nutype::nutype;

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Uri(#[serde(with = "uri_serde")] http::Uri);

impl Uri {
    #[must_use]
    pub const fn new(raw_value: http::Uri) -> Self {
        Self(raw_value)
    }

    #[inline]
    pub fn into_inner(self) -> http::Uri {
        self.0
    }
}

impl Deref for Uri {
    type Target = http::Uri;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uri {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

mod uri_serde {
    use http::uri::InvalidUri;
    use serde::{de::Error as _, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<http::Uri, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        let uri = string
            .parse()
            .map_err(|err: InvalidUri| D::Error::custom(err.to_string()))?;

        Ok(uri)
    }

    pub fn serialize<S>(uri: &http::Uri, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&uri.to_string())
    }
}
