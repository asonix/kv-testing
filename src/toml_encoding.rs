use kv::{Encoding, Error, Serde};
use std::io::{Read, Write};
use serde::{de::DeserializeOwned, ser::Serialize};
use toml::{from_slice, to_vec};

#[derive(Debug, Deserialize, Serialize)]
pub struct Toml<T>(T);

impl<T> Encoding for Toml<T>
where
    T: DeserializeOwned + Serialize,
{
    fn encode_to<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        let v = to_vec(&self.0).map_err(|e| {
            error!("Error encoding: {}", e);
            Error::InvalidEncoding
        })?;

        w.write_all(&v).map(|_| ()).map_err(|e| {
            error!("Error writing: {}", e);
            Error::IO(e)
        })
    }

    fn decode_from<R: Read>(r: &mut R) -> Result<Self, Error> {
        let mut v = Vec::new();

        r.read_to_end(&mut v).map_err(|e| {
            error!("Error reading: {}", e);
            Error::IO(e)
        })?;

        from_slice(&v).map(Toml).map_err(|e| {
            error!("Error decoding: {}", e);
            Error::InvalidEncoding
        })
    }
}

impl<T> Serde<T> for Toml<T>
where
    T: DeserializeOwned + Serialize,
{
    fn from_serde(t: T) -> Self {
        Toml(t)
    }

    fn to_serde(self) -> T {
        self.0
    }
}
