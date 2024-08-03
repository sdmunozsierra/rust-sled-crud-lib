// main.rs

use rust_sled_crud_lib::db::sled_db::SledDb;
use rust_sled_crud_lib::models::conversation::Conversation;
use rust_sled_crud_lib::db::dd_object::DbObject;
use std::error::Error;
use uuid::Uuid;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the database at the specified path
    let db = SledDb::new("my_database")?;

    // Create a Conversation object
    let conversation = Conversation {
        id: Uuid::default(),
        title: String::from("Escape File Path Rust"),
        create_time: 1720335757.046148,
        update_time: 1720335895.590312,
        mapping: HashMap::new(), // Populate as needed
        current_node: Uuid::default(),
        conversation_template_id: None,
        gizmo_id: None,
        is_archived: false,
        workspace_id: None,
        async_status: None,
        safe_urls: vec![],
        moderation_results: vec![],
        plugin_ids: None,
        conversation_id: Uuid::default(),
        default_model_slug: String::from("gpt-4o"),
        conversation_origin: None,
        voice: None,
    };

    // Insert the Conversation object into the database
    db.insert_object(&conversation)?;
    println!("Inserted conversation with id {:?}", conversation.id);

    // Retrieve the Conversation object by key
    if let Some(retrieved_conversation) = db.get_object::<Conversation>(conversation.key())? {
        println!("Retrieved conversation: {:?}", retrieved_conversation);
    } else {
        println!("Conversation not found");
    }

    // Remove the Conversation object
    db.remove_object(conversation.key())?;
    println!("Removed conversation with id {:?}", conversation.id);

    // Verify the removal
    if db.get_object::<Conversation>(conversation.key())?.is_none() {
        println!("Verified removal of conversation");
    } else {
        println!("Conversation still exists");
    }

    // Flush changes to disk
    db.flush()?;
    println!("Database changes flushed");

    Ok(())
}
