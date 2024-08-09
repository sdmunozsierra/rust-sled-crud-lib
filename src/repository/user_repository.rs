// src/repository/user_repository.rs

use crate::models::user::User;
use crate::db::sled_mutex::SledDb;
use crate::repository::generic_repository::GenericRepository;
use crate::repository::repository_error::RepositoryError;
use crate::repository::repository::Repository;
use std::sync::Arc;
use serde_json;
use crate::transaction::transaction_manager::TransactionManager;
use sled::Db;

pub struct UserRepository {
    db: Arc<Db>,
}

impl UserRepository {
    pub fn new(sled_db: Arc<SledDb>) -> Self {
        let db = sled_db.get_db();
        UserRepository { db }
    }

    pub fn save_with_transaction(&self, user: User) -> Result<(), String> {
        let mut transaction = self.db.begin_transaction();

        let key = user.id.clone();
        let value = serde_json::to_vec(&user).map_err(|e| e.to_string())?;

        transaction.insert(key.into_bytes(), value);

        match transaction.commit() {
            Ok(_) => Ok(()),
            Err(err) => {
                transaction.rollback();
                Err(err.to_string())
            }
        }
    }
}

impl Repository<User, String> for UserRepository {
    fn save(&self, user: User) -> Result<(), String> {
        let key = user.id.clone();
        let value = serde_json::to_vec(&user).map_err(|e| e.to_string())?;

        self.db.insert(key.as_bytes(), value).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn find_by_id(&self, id: String) -> Option<User> {
        self.db.get(id.as_bytes()).ok().flatten().and_then(|ivec| {
            serde_json::from_slice(&ivec).ok()
        })
    }

    fn find_all(&self) -> Vec<User> {
        let mut users = vec![];

        for item in self.db.iter() {
            if let Ok((_, value)) = item {
                if let Ok(user) = serde_json::from_slice::<User>(&value) {
                    users.push(user);
                }
            }
        }

        users
    }

    fn delete(&self, user: User) -> Result<(), String> {
        self.db.remove(user.id.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl GenericRepository<User, String> for UserRepository {
    fn save(&self, user: User) -> Result<User, RepositoryError> {
        // Save the user using the existing Repository implementation
        Repository::save(self, user.clone())
            .map_err(RepositoryError::DatabaseError)?;
        Ok(user)
    }

    fn find_by_id(&self, id: String) -> Option<User> {
        // Explicitly use the Repository's find_by_id
        Repository::find_by_id(self, id)
    }

    fn find_all(&self) -> Vec<User> {
        // Use the existing Repository method
        Repository::find_all(self)
    }

    fn delete(&self, user: User) -> Result<(), RepositoryError> {
        // Delete the user using the existing Repository implementation
        Repository::delete(self, user).map_err(RepositoryError::DatabaseError)
    }
}
