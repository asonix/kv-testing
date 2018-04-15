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

use kv::{Config, Manager, SerdeEncoding, ValueBuf, bincode::BincodeEncoding, cbor::CborEncoding,
         json::JsonEncoding};

mod messagepack_encoding;
mod toml_encoding;
mod yaml_encoding;

pub use messagepack_encoding::MessagepackEncoding;
pub use toml_encoding::TomlEncoding;
pub use yaml_encoding::YamlEncoding;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tmp {
    tmp: String,
}

// Write a value to LMDB with encoding type C
fn write<C, T>(mgr: &mut Manager, cfg: Config, bucket_name: &str, item: T)
where
    C: SerdeEncoding<T> + Debug,
{
    let handle = mgr.open(cfg).unwrap();

    let store = handle.write().unwrap();
    let bucket = store
        .bucket::<&str, ValueBuf<C>>(Some(bucket_name))
        .unwrap();
    let mut txn = store.write_txn().unwrap();

    txn.set(&bucket, "key", C::from_serde(item)).unwrap();
    txn.commit().unwrap();
    info!("Stored in {}", bucket_name);
}

// Read a value from LMDB with encoding type C
fn read<C, T>(mgr: &mut Manager, cfg: Config, bucket_name: &str)
where
    C: SerdeEncoding<T> + Debug,
{
    let handle = mgr.open(cfg).unwrap();

    let store = handle.read().unwrap();
    let bucket = store
        .bucket::<&str, ValueBuf<C>>(Some(bucket_name))
        .unwrap();

    let txn = store.read_txn().unwrap();

    let item = txn.get(&bucket, "key").unwrap();

    info!("Item from {}: {:?}", bucket_name, item.inner().unwrap());
}

fn test<C, T>(mgr: &mut Manager, cfg: Config, bucket_name: &str, item: T)
where
    C: SerdeEncoding<T> + Debug,
{
    write::<C, _>(mgr, cfg.clone(), bucket_name, item);
    read::<C, _>(mgr, cfg, bucket_name);
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

    test::<BincodeEncoding<_>, _>(&mut mgr, cfg.clone(), bincode_bucket, tmp.clone());
    test::<CborEncoding<_>, _>(&mut mgr, cfg.clone(), cbor_bucket, tmp.clone());
    test::<JsonEncoding<_>, _>(&mut mgr, cfg.clone(), json_bucket, tmp.clone());
    test::<MessagepackEncoding<_>, _>(&mut mgr, cfg.clone(), msgpack_bucket, tmp.clone());
    test::<TomlEncoding<_>, _>(&mut mgr, cfg.clone(), toml_bucket, tmp.clone());
    test::<YamlEncoding<_>, _>(&mut mgr, cfg.clone(), yaml_bucket, tmp.clone());
}
