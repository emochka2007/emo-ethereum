use std::fmt::Debug;

pub trait Table: Send + Sync + Debug + 'static {
    //return the table name
    const NAME: &'static str;
    type Key: Key;
    type Value: Value;
}
pub trait Key: Encode + Decode + Ord + Clone + Serialize + for <'a> Deserialize<'a> {}

pub trait Value: Compress + Decompress + Serialize {}