mod db;
mod models;
mod repository;
mod service;

use crate::db::sled_mutex::SledDb;
use crate::repository::{user_repository::UserRepository, role_repository::RoleRepositoryImpl};
use crate::service::service::UserService;
use crate::models::user::Role;
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

fn main() {
    // Initialize the database
    let db_path = Path::new("./my_database");
    let sled_db = Arc::new(Mutex::new(SledDb::new(db_path).expect("Failed to initialize database")));

    // Create repositories
    let user_repository = UserRepository::new(Arc::clone(&sled_db));
    let role_repository = RoleRepositoryImpl::new(Arc::clone(&sled_db));

    // Create services
    let user_service = UserService::new(user_repository, role_repository);

    // Add roles
    let admin_role = Role {
        id: Uuid::default().to_string(),
        name: "Admin".to_string(),
    };
    let user_role = Role {
        id: Uuid::default().to_string(),
        name: "User".to_string(),
    };

    // Save roles to the database using UserService
    user_service.save_role(&admin_role).unwrap();
    user_service.save_role(&user_role).unwrap();

    // Register a new user
    user_service
        .register_user("johndoe".to_string(), "john@example.com".to_string())
        .unwrap();

    // Retrieve the user ID for further operations
    let users = user_service.find_all_users();
    if users.is_empty() {
        println!("No users found.");
        return;
    }
    let user_id = &users[0].id;

    // Assign a role to the user
    user_service
        .add_role_to_user(user_id.to_string(), admin_role.id.clone())
        .unwrap();

    // Retrieve and display the user's roles
    let roles = user_service.get_user_roles(user_id.to_string()).unwrap();
    println!("User Roles for {}: {:?}", user_id, roles);

    // Display all users
    let users = user_service.find_all_users();
    println!("Current Users: {:?}", users);
}
