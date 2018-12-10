pub use ron::Value;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{Result, Write};

pub fn from_writer() {}

pub fn from_value<T: DeserializeOwned>(value: Value) -> ron::de::Result<T> {
    Deserialize::deserialize(value)
}

pub use ron::de::from_str;
pub use ron::ser::Serializer;

// pub fn from_str<'a, T>(contents: &'a str) -> ron::de::Result<T>
// where
//     T: Deserialize<'a>,
// {
//     ron::de::from_str(contents)
// }

pub fn to_writer_pretty<W: Write, T: Serialize>(writer: W, value: &T) -> Result<()> {
    Ok(())
}
