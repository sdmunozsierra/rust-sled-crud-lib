use crate::db::sled_mutex::SledDb;
use crate::models::user::Role;
use std::sync::{Arc, Mutex};
use sled::Db;

pub trait RoleRepository {
    fn save(&self, role: &Role) -> Result<(), String>;
    fn find_by_id(&self, id: &str) -> Option<Role>;
    fn find_all(&self) -> Vec<Role>;
    fn delete(&self, id: &str) -> Result<(), String>;
}

pub struct RoleRepositoryImpl {
    db: Arc<Mutex<Db>>,
}

impl RoleRepositoryImpl {
    pub fn new(sled_db: Arc<Mutex<SledDb>>) -> Self {
        let db = sled_db.lock().unwrap().get_db();
        RoleRepositoryImpl { db }
    }

    fn role_key(id: &str) -> String {
        format!("role:{}", id)
    }
}

impl RoleRepository for RoleRepositoryImpl {
    fn save(&self, role: &Role) -> Result<(), String> {
        let db = self.db.lock().unwrap();
        let key = RoleRepositoryImpl::role_key(&role.id);
        let value = serde_json::to_vec(role).map_err(|e| e.to_string())?;

        db.insert(key.as_bytes(), value).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Option<Role> {
        let db = self.db.lock().unwrap();
        let key = RoleRepositoryImpl::role_key(id);

        db.get(key.as_bytes()).ok().flatten().and_then(|ivec| {
            serde_json::from_slice(&ivec).ok()
        })
    }

    fn find_all(&self) -> Vec<Role> {
        let db = self.db.lock().unwrap();
        let mut roles = vec![];

        for item in db.scan_prefix("role:") {
            if let Ok((_, value)) = item {
                if let Ok(role) = serde_json::from_slice::<Role>(&value) {
                    roles.push(role);
                }
            }
        }

        roles
    }

    fn delete(&self, id: &str) -> Result<(), String> {
        let db = self.db.lock().unwrap();
        let key = RoleRepositoryImpl::role_key(id);
        db.remove(key.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}
