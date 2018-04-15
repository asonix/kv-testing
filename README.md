# KV-testing

[KV](https://github.com/zshipko/rust-kv) provides mechanisms for using bincode,
cbor, and json natively, but if you need to store another type, you can look
here for example implementations for custom serde-compatible types. KV's
`Encoding` trait provides a simple API to read and write data.

Example implementing a custom YAML encoding

```rust
use kv::{Encoding, Error, SerdeEncoding};
use std::io::{Read, Write};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_yaml::{from_reader, to_writer};

#[derive(Debug, Deserialize, Serialize)]
pub struct YamlEncoding<T>(T);

impl<T> Encoding for YamlEncoding<T>
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
        from_reader(r).map(YamlEncoding).map_err(|e| {
            error!("Error decoding: {}", e);
            Error::InvalidEncoding
        })
    }
}

// This impl is optional, but will help your custom type behave similarly to
// the provided types
impl<T> SerdeEncoding<T> for YamlEncoding<T>
where
    T: DeserializeOwned + Serialize,
{
    fn from_serde(value: T) -> Self {
        YamlEncoding(value)
    }

    fn to_serde(self) -> T {
        self.0
    }
}
```
