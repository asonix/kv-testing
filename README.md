# KV-testing

[KV](https://github.com/zshipko/rust-kv) provides mechanisms for using bincode,
cbor, and json natively, but if you need to store another type, you can look
here for example implementations for custom serde-compatible types. KV's
`Encoding` trait provides a simple API to read and write data.

Example usage of Serde types with KV

```rust
// Write a value to LMDB with Bincode encoding type
fn write_bincode<T>(mgr: &mut Manager, cfg: Config, bucket_name: &str, item: T) -> Result<(), Error>
where
    C: Serde<T> + Debug,
{
    let handle = mgr.open(cfg)?;

    let store = handle.write()?;
    let bucket = store.bucket::<&str, ValueBuf<Bincode<T>>>(Some(bucket_name))?;
    let mut txn = store.write_txn()?;

    txn.set(&bucket, "key", Bincode::to_value_buf(item)?)?;
    txn.commit()?;
    info!("Stored in {}", bucket_name);
    Ok(())
}

// Read a value from LMDB with encoding type C
fn read<C, T>(mgr: &mut Manager, cfg: Config, bucket_name: &str) -> Result<(), Error>
where
    C: Serde<T> + Debug,
{
    let handle = mgr.open(cfg)?;

    let store = handle.read()?;
    let bucket = store.bucket::<&str, ValueBuf<C>>(Some(bucket_name))?;

    let txn = store.read_txn()?;

    let item = txn.get(&bucket, "key")?;

    info!("Item from {}: {:?}", bucket_name, item.inner()?);
    Ok(())
}

fn main() {
    let tmp = SomeSerdeType::new();
    let bincode_bucket = "bincode-encoding";

    let mut cfg = Config::default("kv-test");
    cfg.bucket(bincode_bucket, None);

    let mut mgr = Manager::new();

    write_bincode(&mut mgr, cfg.clone(), bincode_bucket, tmp.clone()).unwrap();
    read::<BincodeEncoding<_>, SomeSerdeType>(&mut mgr, cfg.clone(), bincode_bucket).unwrap();
}
```

Example implementing a custom YAML encoding

```rust
use kv::{Encoding, Error, Serde};
use std::io::{Read, Write};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_yaml::{from_reader, to_writer};

#[derive(Debug, Deserialize, Serialize)]
pub struct Yaml<T>(T);

impl<T> Encoding for Yaml<T>
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
        from_reader(r).map(Yaml).map_err(|e| {
            error!("Error decoding: {}", e);
            Error::InvalidEncoding
        })
    }
}

/// This part is optional, but it helps make your implementation consistent with
// the rest of the serde types
impl<T> Serde<T> for Yaml<T>
where
    T: DeserializeOwned + Serialize,
{
    fn from_serde(value: T) -> Self {
        Yaml(value)
    }

    fn to_serde(self) -> T {
        self.0
    }
}
```
