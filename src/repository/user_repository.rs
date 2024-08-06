use crate::models::user::User;
use crate::db::sled_mutex::SledDb;
use std::sync::{Arc, Mutex};
use sled::Db;
use serde_json;

pub trait Repository<T, ID> {
    fn save(&self, entity: T) -> Result<(), String>;
    fn find_by_id(&self, id: ID) -> Option<T>;
    fn find_all(&self) -> Vec<T>;
    fn delete(&self, entity: T) -> Result<(), String>;
}

pub struct UserRepository {
    db: Arc<Mutex<Db>>,
}

impl UserRepository {
    pub fn new(sled_db: Arc<Mutex<SledDb>>) -> Self {
        let db = sled_db.lock().unwrap().get_db();
        UserRepository { db }
    }
}

impl Repository<User, String> for UserRepository {
    fn save(&self, user: User) -> Result<(), String> {
        let db = self.db.lock().unwrap();
        let key = user.id.clone();
        let value = serde_json::to_vec(&user).map_err(|e| e.to_string())?;

        db.insert(key.as_bytes(), value).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn find_by_id(&self, id: String) -> Option<User> {
        let db = self.db.lock().unwrap();

        db.get(id.as_bytes()).ok().flatten().and_then(|ivec| {
            serde_json::from_slice(&ivec).ok()
        })
    }

    fn find_all(&self) -> Vec<User> {
        let db = self.db.lock().unwrap();
        let mut users = vec![];

        for item in db.iter() {
            if let Ok((_, value)) = item {
                if let Ok(user) = serde_json::from_slice::<User>(&value) {
                    users.push(user);
                }
            }
        }

        users
    }

    fn delete(&self, user: User) -> Result<(), String> {
        let db = self.db.lock().unwrap();
        db.remove(user.id.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}
