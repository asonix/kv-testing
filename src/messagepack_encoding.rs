use kv::{Encoding, Error};
use rmp_serde::{decode::from_read, encode::write};
use std::io::{Read, Write};
use serde::{de::DeserializeOwned, ser::Serialize};

use super::CustomEncoding;

#[derive(Debug, Deserialize, Serialize)]
pub struct MessagepackEncoding<T>(T);

impl<T> Encoding for MessagepackEncoding<T>
where
    T: DeserializeOwned + Serialize,
{
    fn encode_to<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        write(w, &self.0).map_err(|e| {
            error!("Error encoding: {}", e);
            Error::InvalidEncoding
        })
    }

    fn decode_from<R: Read>(r: &mut R) -> Result<Self, Error> {
        from_read(r).map(MessagepackEncoding).map_err(|e| {
            error!("Error decoding: {}", e);
            Error::InvalidEncoding
        })
    }
}

impl<T> CustomEncoding<T> for MessagepackEncoding<T>
where
    T: DeserializeOwned + Serialize,
{
    fn from_value(value: T) -> Self {
        MessagepackEncoding(value)
    }
}
