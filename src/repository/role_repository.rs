use crate::db::sled_mutex::SledDb;
use crate::models::role::Role;
use std::sync::Arc;
use crate::repository::generic_repository::GenericRepository;
use crate::repository::repository_error::RepositoryError;
use crate::repository::repository::Repository;
use sled::Db;

pub struct RoleRepository {
    db: Arc<Db>,
}

impl RoleRepository {
    pub fn new(sled_db: Arc<SledDb>) -> Self {
        let db = sled_db.get_db();
        RoleRepository { db }
    }

    fn role_key(id: &str) -> String {
        format!("role:{}", id)
    }
}

impl Repository<Role, String> for RoleRepository {
    fn save(&self, role: Role) -> Result<(), String> {
        let key = RoleRepository::role_key(&role.id);
        let value = serde_json::to_vec(&role).map_err(|e| e.to_string())?;

        self.db.insert(key.as_bytes(), value).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn find_by_id(&self, id: String) -> Option<Role> {
        let key = RoleRepository::role_key(&id);

        self.db.get(key.as_bytes()).ok().flatten().and_then(|ivec| {
            serde_json::from_slice(&ivec).ok()
        })
    }

    fn find_all(&self) -> Vec<Role> {
        let mut roles = vec![];

        for item in self.db.scan_prefix("role:") {
            if let Ok((_, value)) = item {
                if let Ok(role) = serde_json::from_slice::<Role>(&value) {
                    roles.push(role);
                }
            }
        }

        roles
    }

    fn delete(&self, role: Role) -> Result<(), String> {
        let key = RoleRepository::role_key(&role.id);
        self.db.remove(key.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl GenericRepository<Role, String> for RoleRepository {
    fn save(&self, role: Role) -> Result<Role, RepositoryError> {
        // Save the role using the existing Repository implementation
        Repository::save(self, role.clone())
            .map_err(RepositoryError::DatabaseError)?;
        Ok(role)
    }

    fn find_by_id(&self, id: String) -> Option<Role> {
        // Explicitly use the Repository's find_by_id
        Repository::find_by_id(self, id)
    }

    fn find_all(&self) -> Vec<Role> {
        // Use the existing Repository method
        Repository::find_all(self)
    }

    fn delete(&self, role: Role) -> Result<(), RepositoryError> {
        // Delete the role using the existing Repository implementation
        Repository::delete(self, role).map_err(RepositoryError::DatabaseError)
    }
}
