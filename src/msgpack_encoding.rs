use kv::{Encoding, Error, Serde};
use rmp_serde::{decode::from_read, encode::write};
use std::io::{Read, Write};
use serde::{de::DeserializeOwned, ser::Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Msgpack<T>(T);

impl<T> Encoding for Msgpack<T>
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
        from_read(r).map(Msgpack).map_err(|e| {
            error!("Error decoding: {}", e);
            Error::InvalidEncoding
        })
    }
}

impl<T> Serde<T> for Msgpack<T>
where
    T: DeserializeOwned + Serialize,
{
    fn from_serde(t: T) -> Self {
        Msgpack(t)
    }

    fn to_serde(self) -> T {
        self.0
    }
}
