use sled::{Config, Db, IVec};
use std::path::Path;
use std::result::Result;
use crate::db::dd_object::DbObject;

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

    /// Insert a generic DbObject into the database
    pub fn insert_object<T: DbObject>(&self, object: &T) -> Result<(), sled::Error> {
        let key = object.key();
        let value = object.to_bytes();
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    /// Retrieve a generic DbObject by key from the database
    pub fn get_object<T: DbObject>(&self, key: IVec) -> Result<Option<T>, sled::Error> {
        if let Some(value) = self.db.get(key)? {
            let object = T::from_bytes(&value);
            Ok(Some(object))
        } else {
            Ok(None)
        }
    }

    /// Remove a generic DbObject from the database
    pub fn remove_object(&self, key: IVec) -> Result<(), sled::Error> {
        self.db.remove(key)?;
        self.db.flush()?;
        Ok(())
    }

    /// Flushes all operations, ensuring durability
    pub fn flush(&self) -> Result<(), sled::Error> {
        self.db.flush().map(|_| ())
    }
}
