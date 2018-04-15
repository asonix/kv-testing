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

use kv::{Config, Manager, SerdeEncoding, ValueBuf};

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

fn test<C, T>(mgr: &mut Manager, cfg: Config, bucket_name: &str, item: T)
where
    C: SerdeEncoding<T> + Debug,
{
    let handle = mgr.open(cfg).unwrap();

    {
        let store = handle.write().unwrap();
        let bucket = store
            .bucket::<&str, ValueBuf<C>>(Some(bucket_name))
            .unwrap();
        let mut txn = store.write_txn().unwrap();

        txn.set(&bucket, "key", C::from_serde(item)).unwrap();
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

    let messagepack_bucket = "messagepack-encoding";
    let toml_bucket = "toml-encoding";
    let yaml_bucket = "yaml-encoding";

    let tmp = Tmp {
        tmp: "Hello, world!".to_owned(),
    };

    let mut cfg = Config::default("kv-test");
    cfg.bucket(messagepack_bucket, None);
    cfg.bucket(toml_bucket, None);
    cfg.bucket(yaml_bucket, None);

    let mut mgr = Manager::new();

    test::<MessagepackEncoding<_>, _>(&mut mgr, cfg.clone(), messagepack_bucket, tmp.clone());
    test::<TomlEncoding<_>, _>(&mut mgr, cfg.clone(), toml_bucket, tmp.clone());
    test::<YamlEncoding<_>, _>(&mut mgr, cfg.clone(), yaml_bucket, tmp.clone());
}
