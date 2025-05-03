use crate::Version;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VersionVisitor;

        impl<'de> Visitor<'de> for VersionVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("semver version")
            }

            fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                #[cfg(feature = "serde-nofail")]
                return Ok(string.parse().unwrap_or_default());

                #[cfg(not(feature = "serde-nofail"))]
                string.parse().map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(VersionVisitor)
    }
}
