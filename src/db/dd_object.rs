use sled::IVec;
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};

pub trait DbObject: Serialize + for<'de> Deserialize<'de> {
    fn to_bytes(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        deserialize(bytes).unwrap()
    }

    fn key(&self) -> IVec;
}
