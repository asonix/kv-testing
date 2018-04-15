extern crate bincode;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate kv;
extern crate rmp_serde;
extern crate serde;
extern crate serde_cbor;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

use std::{env, fmt::Debug};

use kv::{Config, Encoding, Manager, ValueBuf};

mod bincode_encoding;
mod cbor_encoding;
mod json_encoding;
mod messagepack_encoding;
mod toml_encoding;
mod yaml_encoding;

pub use bincode_encoding::BincodeEncoding;
pub use cbor_encoding::CborEncoding;
pub use json_encoding::JsonEncoding;
pub use messagepack_encoding::MessagepackEncoding;
pub use toml_encoding::TomlEncoding;
pub use yaml_encoding::YamlEncoding;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tmp {
    tmp: String,
}

pub trait CustomEncoding<T>: Encoding {
    fn from_value(value: T) -> Self;
}

fn test<C, T>(mgr: &mut Manager, cfg: Config, bucket_name: &str, item: T)
where
    C: CustomEncoding<T> + Debug,
{
    let handle = mgr.open(cfg).unwrap();

    {
        let store = handle.write().unwrap();
        let bucket = store
            .bucket::<&str, ValueBuf<C>>(Some(bucket_name))
            .unwrap();
        let mut txn = store.write_txn().unwrap();

        txn.set(&bucket, "key", C::from_value(item)).unwrap();
        txn.commit().unwrap();
        info!("Stored in {}", bucket_name);
    }

    {
        let store = handle.read().unwrap();
        let bucket = store
            .bucket::<&str, ValueBuf<C>>(Some(bucket_name))
            .unwrap();

        let txn = store.read_txn().unwrap();

        let item = txn.get(&bucket, "key").unwrap();

        info!("Item from {}: {:?}", bucket_name, item.inner());
    }
}

fn main() {
    env::set_var("RUST_LOG", "kv_test=info");
    env_logger::init();

    let bincode_bucket = "bincode-encoding";
    let cbor_bucket = "cbor-encoding";
    let json_bucket = "json-encoding";
    let messagepack_bucket = "messagepack-encoding";
    let toml_bucket = "toml-encoding";
    let yaml_bucket = "yaml-encoding";

    let tmp = Tmp {
        tmp: "Hello, world!".to_owned(),
    };

    let mut cfg = Config::default("kv-test");
    cfg.bucket(bincode_bucket, None);
    cfg.bucket(cbor_bucket, None);
    cfg.bucket(json_bucket, None);
    cfg.bucket(messagepack_bucket, None);
    cfg.bucket(toml_bucket, None);
    cfg.bucket(yaml_bucket, None);

    let mut mgr = Manager::new();

    test::<BincodeEncoding<_>, _>(&mut mgr, cfg.clone(), bincode_bucket, tmp.clone());
    test::<CborEncoding<_>, _>(&mut mgr, cfg.clone(), cbor_bucket, tmp.clone());
    test::<JsonEncoding<_>, _>(&mut mgr, cfg.clone(), json_bucket, tmp.clone());
    test::<MessagepackEncoding<_>, _>(&mut mgr, cfg.clone(), messagepack_bucket, tmp.clone());
    test::<TomlEncoding<_>, _>(&mut mgr, cfg.clone(), toml_bucket, tmp.clone());
    test::<YamlEncoding<_>, _>(&mut mgr, cfg.clone(), yaml_bucket, tmp.clone());
}
