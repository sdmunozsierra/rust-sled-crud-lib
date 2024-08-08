// src/transaction/mod.rs

pub struct Transaction<'a> {
    db: &'a sled::Db,
    is_active: bool,
    pending_ops: Vec<(Vec<u8>, Option<Vec<u8>>)>,
}

impl<'a> Transaction<'a> {
    pub fn new(db: &'a sled::Db) -> Self {
        Transaction {
            db,
            is_active: true,
            pending_ops: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.pending_ops.push((key, Some(value)));
    }

    pub fn delete(&mut self, key: Vec<u8>) {
        self.pending_ops.push((key, None));
    }

    pub fn commit(&mut self) -> Result<(), sled::Error> {
        if self.is_active {
            for (key, value) in &self.pending_ops {
                match value {
                    Some(v) => self.db.insert(key.clone(), v.clone())?,
                    None => {
                        self.db.remove(key.clone())?;
                    }
                }
            }
            self.db.flush()?;
            self.is_active = false;
        }
        Ok(())
    }

    pub fn rollback(&mut self) {
        if self.is_active {
            self.pending_ops.clear();
            self.is_active = false;
        }
    }
}
