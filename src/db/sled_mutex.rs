use sled::{Config, Db};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::result::Result;

/// Struct representing a thread-safe database using Sled
pub struct SledDb {
    db: Arc<Mutex<Db>>,
}

impl SledDb {
    /// Initialize a new SledDb instance with thread-safe access
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, sled::Error> {
        // Create a new configuration and set the path
        let config = Config::new().path(path);
        
        // Open the database with the given configuration
        let db = config.open()?;
        
        // Wrap the database in an Arc<Mutex<>> for thread safety
        Ok(SledDb {
            db: Arc::new(Mutex::new(db)),
        })
    }
    
    /// Example method to access the database
    pub fn get_db(&self) -> Arc<Mutex<Db>> {
        Arc::clone(&self.db)
    }
}
