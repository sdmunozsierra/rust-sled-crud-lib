// src/db/sled_db.rs

use serde_derive::{Deserialize, Serialize};
use sled::{Config, Db, IVec};
use std::path::Path;
use std::result::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parent {
    pub id: u32,
    pub name: String,
    pub children: Vec<Child>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Child {
    pub id: u32,
    pub name: String,
}

pub struct SledDb {
    db: Db,
}

impl SledDb {
    /// Initialize a new SledDb instance
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, sled::Error> {
        let config = Config::new().path(path);
        let db = config.open()?;
        Ok(SledDb { db })
    }

    /// Insert a key-value pair into the database
    pub fn insert(&self, key: &[u8], value: &[u8]) -> Result<Option<IVec>, sled::Error> {
        self.db.insert(key, value)
    }

    /// Retrieve a value by key from the database
    pub fn get(&self, key: &[u8]) -> Result<Option<IVec>, sled::Error> {
        self.db.get(key)
    }

    /// Remove a key-value pair from the database
    pub fn remove(&self, key: &[u8]) -> Result<Option<IVec>, sled::Error> {
        self.db.remove(key)
    }

    /// Insert a parent object into the database
    pub fn insert_parent(&self, parent: &Parent) -> Result<(), sled::Error> {
        let key = format!("parent:{}", parent.id);
        let value = bincode::serialize(parent).unwrap();
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    /// Retrieve a parent object by ID from the database
    pub fn get_parent(&self, parent_id: u32) -> Result<Option<Parent>, sled::Error> {
        let key = format!("parent:{}", parent_id);
        if let Some(value) = self.db.get(key)? {
            let parent: Parent = bincode::deserialize(&value).unwrap();
            Ok(Some(parent))
        } else {
            Ok(None)
        }
    }

    /// Remove a parent object from the database
    pub fn remove_parent(&self, parent_id: u32) -> Result<(), sled::Error> {
        let key = format!("parent:{}", parent_id);
        self.db.remove(key)?;
        self.db.flush()?;
        Ok(())
    }

    /// Flushes all operations, ensuring durability
    pub fn flush(&self) -> Result<(), sled::Error> {
        self.db.flush().map(|_| ())
    }
}
