use crate::transaction::transaction::Transaction;
use sled::Db;
use std::sync::Arc;

/// Trait to manage transactions in the database
pub trait TransactionManager {
    fn begin_transaction(&self) -> Transaction;
}

impl TransactionManager for Arc<Db> {
    fn begin_transaction(&self) -> Transaction {
        Transaction::new(self.clone()) // Clone the Arc to pass it into the transaction
    }
}
