mod db;
mod models;
mod repository;
mod service;
mod query;
mod conf;
mod transaction;

use crate::db::sled_mutex::SledDb;
use crate::repository::{
    user_repository::UserRepository, role_repository::RoleRepository,
    conversation_repository::ConversationRepository,
};
use crate::service::user_service::UserService;
use crate::service::conversation_service::ConversationService;
use crate::models::role::Role;
use crate::query::query_builder::QueryBuilder;
use crate::query::sorting::{Sort, SortDirection};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

fn main() {
    // Initialize the database
    let db_path = Path::new("./my_database");
    let sled_db = Arc::new(
        SledDb::new(db_path).expect("Failed to initialize database"),
    );

    // Create repositories
    let user_repository = UserRepository::new(Arc::clone(&sled_db));
    let role_repository = RoleRepository::new(Arc::clone(&sled_db));
    let conversation_repository = ConversationRepository::new(Arc::clone(&sled_db));

    // Create services
    let user_service = UserService::new(&user_repository, &role_repository);
    let conversation_service = ConversationService::new(conversation_repository);

    // Add roles
    let admin_role = Role {
        id: Uuid::new_v4().to_string(),
        name: "Admin".to_string(),
    };
    let user_role = Role {
        id: Uuid::new_v4().to_string(),
        name: "User".to_string(),
    };

    // Save roles to the database using UserService
    user_service.save_role(&admin_role).unwrap();
    user_service.save_role(&user_role).unwrap();

    // Register a new user
    user_service
        .register_user("johndoe".to_string(), "john@example.com".to_string())
        .unwrap();

    user_service
        .register_user("janedoe".to_string(), "jane@example.com".to_string())
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

    // Create a new conversation using ConversationService
    let conversation = conversation_service
        .register_conversation("Sample Conversation".to_string())
        .unwrap();
    println!("Registered Conversation: {:?}", conversation);

    // Update the conversation title
    conversation_service
        .update_conversation_title(conversation.id, "Updated Conversation Title".to_string())
        .unwrap();

    // Retrieve and display the conversation by ID
    if let Some(retrieved_conversation) = conversation_service.get_conversation(conversation.id) {
        println!("Retrieved Conversation: {:?}", retrieved_conversation);
    } else {
        println!("Conversation not found.");
    }

    // Archive the conversation
    conversation_service
        .archive_conversation(conversation.id)
        .unwrap();

    // Retrieve and display the archived conversation
    if let Some(archived_conversation) = conversation_service.get_conversation(conversation.id) {
        println!("Archived Conversation: {:?}", archived_conversation);
    } else {
        println!("Conversation not found.");
    }

    // Display all conversations
    let all_conversations = conversation_service.find_all_conversations();
    println!("All Conversations: {:?}", all_conversations);

    // Delete the conversation
    conversation_service.delete_conversation(conversation.id).unwrap();

    // Verify deletion
    let all_conversations = conversation_service.find_all_conversations();
    println!("Conversations after deletion: {:?}", all_conversations);

    // Create a new query to find all users named "johndoe"
    let query = QueryBuilder::new()
        .where_eq("username", "johndoe")
        .order_by("email", SortDirection::Ascending)
        .build();

    // Execute the query using the user repository
    let filtered_users = query.execute(&user_repository);
    println!("Filtered Users: {:?}", filtered_users);
}
