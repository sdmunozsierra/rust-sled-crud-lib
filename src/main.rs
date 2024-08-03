// main.rs
//use rust_sled_crud_lib::db::sled_db::SledDb;
//use rust_sled_crud_lib::models::conversation::Conversation;
//use rust_sled_crud_lib::db::dd_object::DbObject;
use rust_sled_crud_lib::models::parser::parse_conversation_data;
//use std::error::Error;
use uuid::Uuid;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_string = r#"
    {
        "id": "ee6a03b5-d17f-42e4-a5b5-65a099c250dc",
        "title": "Escape File Path Rust",
        "create_time": 1720335757.046148,
        "update_time": 1720335895.590312,
        "mapping": {
            "99bbb25d-d020-4bf8-87a3-01c14f3575a5": {
                "id": "99bbb25d-d020-4bf8-87a3-01c14f3575a5",
                "message": {
                    "id": "99bbb25d-d020-4bf8-87a3-01c14f3575a5",
                    "author": {
                        "role": "system",
                        "name": null,
                        "metadata": {}
                    },
                    "create_time": null,
                    "update_time": null,
                    "content": {
                        "content_type": "text",
                        "parts": [""]
                    },
                    "status": "finished_successfully",
                    "end_turn": true,
                    "weight": 0,
                    "metadata": {
                        "is_visually_hidden_from_conversation": true
                    },
                    "recipient": "all",
                    "channel": null
                },
                "parent": "aaa18177-4607-4e8f-ad33-c85e66cdfad7",
                "children": [
                    "aaa2d189-e728-41f5-b670-4faccaa113d9"
                ]
            }
        },
        "current_node": "ff00a280-d6b6-45df-8390-856877f05559",
        "conversation_template_id": null,
        "gizmo_id": null,
        "is_archived": false,
        "workspace_id": null,
        "async_status": null,
        "safe_urls": [],
        "moderation_results": [],
        "plugin_ids": null,
        "conversation_id": "ee6a03b5-d17f-42e4-a5b5-65a099c250dc",
        "default_model_slug": "gpt-4o",
        "conversation_origin": null,
        "voice": null
    }
    "#;

    // Parse the JSON string into a ConversationData object
    let conversation_data = parse_conversation_data(json_string)?;

    println!("Parsed ConversationData: {:?}", conversation_data);

    Ok(())
}
