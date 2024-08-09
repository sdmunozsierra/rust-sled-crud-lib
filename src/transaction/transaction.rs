use sled;
use std::sync::Arc;

/// Struct representing a database transaction
pub struct Transaction {
    db: Arc<sled::Db>,
    is_active: bool,
    pending_ops: Vec<(Vec<u8>, Option<Vec<u8>>)>,
}

impl Transaction {
    pub fn new(db: Arc<sled::Db>) -> Self {
        Transaction {
            db,
            is_active: true,
            pending_ops: Vec::new(),
        }
    }

    /// Queue an insert or update operation
    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.pending_ops.push((key, Some(value)));
    }

    /// Queue a delete operation
    pub fn delete(&mut self, key: Vec<u8>) {
        self.pending_ops.push((key, None));
    }

    /// Commit all queued operations to the database
    pub fn commit(&mut self) -> Result<(), sled::Error> {
        if self.is_active {
            for (key, value) in &self.pending_ops {
                match value {
                    Some(v) => {
                        self.db.insert(key.clone(), v.clone())?;
                    },
                    None => {
                        self.db.remove(key.clone())?;
                    }
                }
            }
            self.db.flush()?; // Ensure all operations are written to disk
            self.is_active = false;
            self.pending_ops.clear(); // Clear pending ops after commit
        }
        Ok(())
    }

    /// Rollback by clearing all queued operations
    pub fn rollback(&mut self) {
        if self.is_active {
            self.pending_ops.clear(); // Discard all uncommitted changes
            self.is_active = false; // Mark transaction as inactive
        }
    }
}
