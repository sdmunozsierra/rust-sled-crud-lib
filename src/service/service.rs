use crate::repository::{user_repository::{Repository, UserRepository}, role_repository::{RoleRepository, RoleRepositoryImpl}};
use crate::models::user::{User,Role};

pub struct UserService {
    user_repository: UserRepository,
    role_repository: RoleRepositoryImpl,
}

impl UserService {
    pub fn new(user_repository: UserRepository, role_repository: RoleRepositoryImpl) -> Self {
        UserService {
            user_repository,
            role_repository,
        }
    }

    pub fn register_user(&self, username: String, email: String) -> Result<(), String> {
        let user = User {
            id: uuid::Uuid::default().to_string(),
            username,
            email,
            roles: vec![],  // Initialize with no roles
        };
        self.user_repository.save(user)
    }

    pub fn get_user(&self, user_id: String) -> Option<User> {
        self.user_repository.find_by_id(user_id)
    }

    pub fn add_role_to_user(&self, user_id: String, role_id: String) -> Result<(), String> {
        if let Some(mut user) = self.user_repository.find_by_id(user_id.clone()) {
            if let Some(role) = self.role_repository.find_by_id(&role_id.clone()) {
                user.roles.push(role);
                return self.user_repository.save(user);
            }
            return Err(format!("Role with ID {} not found", role_id));
        }
        Err(format!("User with ID {} not found", user_id))
    }

    pub fn get_user_roles(&self, user_id: String) -> Result<Vec<Role>, String> {
        if let Some(user) = self.user_repository.find_by_id(user_id) {
            return Ok(user.roles);
        }
        Err("User not found".to_string())
    }

    pub fn save_role(&self, role: &Role) -> Result<(), String> {
        self.role_repository.save(role)
    }

    pub fn find_all_users(&self) -> Vec<User> {
        self.user_repository.find_all()
    }
}
