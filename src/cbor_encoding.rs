use kv::{Encoding, Error};
use std::io::{Read, Write};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_cbor::{from_reader, to_writer};

use super::CustomEncoding;

#[derive(Debug, Deserialize, Serialize)]
pub struct CborEncoding<T>(T);

impl<T> Encoding for CborEncoding<T>
where
    T: DeserializeOwned + Serialize,
{
    fn encode_to<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        to_writer(w, &self.0).map_err(|e| {
            error!("Error encoding: {}", e);
            Error::InvalidEncoding
        })
    }

    fn decode_from<R: Read>(r: &mut R) -> Result<Self, Error> {
        from_reader(r).map(CborEncoding).map_err(|e| {
            error!("Error decoding: {}", e);
            Error::InvalidEncoding
        })
    }
}

impl<T> CustomEncoding<T> for CborEncoding<T>
where
    T: DeserializeOwned + Serialize,
{
    fn from_value(value: T) -> Self {
        CborEncoding(value)
    }
}
