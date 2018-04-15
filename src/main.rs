extern crate env_logger;
#[macro_use]
extern crate log;
extern crate kv;
extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate toml;

use std::{env, fmt::Debug};

use kv::{Config, Error, Manager, Serde, ValueBuf, bincode::Bincode, cbor::Cbor, json::Json};

mod msgpack_encoding;
mod toml_encoding;
mod yaml_encoding;

pub use msgpack_encoding::Msgpack;
pub use toml_encoding::Toml;
pub use yaml_encoding::Yaml;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tmp {
    tmp: String,
}

// Write a value to LMDB with encoding type C
fn write<C, T>(mgr: &mut Manager, cfg: Config, bucket_name: &str, item: T) -> Result<(), Error>
where
    C: Serde<T> + Debug,
{
    let handle = mgr.open(cfg)?;

    let store = handle.write()?;
    let bucket = store.bucket::<&str, ValueBuf<C>>(Some(bucket_name))?;
    let mut txn = store.write_txn()?;

    txn.set(&bucket, "key", C::from_serde(item).encode()?)?;
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

fn test<C, T>(mgr: &mut Manager, cfg: Config, bucket_name: &str, item: T) -> Result<(), Error>
where
    C: Serde<T> + Debug,
{
    write::<C, _>(mgr, cfg.clone(), bucket_name, item)?;
    read::<C, _>(mgr, cfg, bucket_name)?;
    Ok(())
}

fn main() {
    env::set_var("RUST_LOG", "kv_test=info");
    env_logger::init();

    let bincode_bucket = "bincode-encoding";
    let cbor_bucket = "cbor-encoding";
    let json_bucket = "json-encoding";
    let msgpack_bucket = "msgpack-encoding";
    let toml_bucket = "toml-encoding";
    let yaml_bucket = "yaml-encoding";

    let tmp = Tmp {
        tmp: "Hello, world!".to_owned(),
    };

    let mut cfg = Config::default("kv-test");
    cfg.bucket(bincode_bucket, None);
    cfg.bucket(cbor_bucket, None);
    cfg.bucket(json_bucket, None);
    cfg.bucket(msgpack_bucket, None);
    cfg.bucket(toml_bucket, None);
    cfg.bucket(yaml_bucket, None);

    let mut mgr = Manager::new();

    test::<Bincode<_>, _>(&mut mgr, cfg.clone(), bincode_bucket, tmp.clone()).unwrap();
    test::<Cbor<_>, _>(&mut mgr, cfg.clone(), cbor_bucket, tmp.clone()).unwrap();
    test::<Json<_>, _>(&mut mgr, cfg.clone(), json_bucket, tmp.clone()).unwrap();
    test::<Msgpack<_>, _>(&mut mgr, cfg.clone(), msgpack_bucket, tmp.clone()).unwrap();
    test::<Toml<_>, _>(&mut mgr, cfg.clone(), toml_bucket, tmp.clone()).unwrap();
    test::<Yaml<_>, _>(&mut mgr, cfg.clone(), yaml_bucket, tmp.clone()).unwrap();
}
