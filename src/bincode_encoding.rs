use bincode::{deserialize_from, serialize_into};
use kv::{Encoding, Error};
use std::io::{Read, Write};
use serde::{de::DeserializeOwned, ser::Serialize};

use super::CustomEncoding;

#[derive(Debug, Deserialize, Serialize)]
pub struct BincodeEncoding<T>(T);

impl<T> Encoding for BincodeEncoding<T>
where
    T: DeserializeOwned + Serialize,
{
    fn encode_to<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        serialize_into(w, &self.0).map_err(|e| {
            error!("Error encoding: {}", e);
            Error::InvalidEncoding
        })
    }

    fn decode_from<R: Read>(r: &mut R) -> Result<Self, Error> {
        deserialize_from(r).map(BincodeEncoding).map_err(|e| {
            error!("Error decoding: {}", e);
            Error::InvalidEncoding
        })
    }
}

impl<T> CustomEncoding<T> for BincodeEncoding<T>
where
    T: DeserializeOwned + Serialize,
{
    fn from_value(value: T) -> Self {
        BincodeEncoding(value)
    }
}
