// controller/conversation.rs

use serde::Serialize;
use serde_json::to_string_pretty;
use std::fmt;
use uuid::Uuid;
use crate::models::conversation::ConversationData;

// Updated SimplifiedConversation to include a chronological list of messages
#[derive(Debug, Serialize)]
pub struct SimplifiedConversation {
    pub id: Uuid,
    pub title: String,
    pub messages: Vec<ConversationEntry>,
}

// New struct to represent each message in the conversation timeline
#[derive(Debug, Serialize)]
pub struct ConversationEntry {
    pub author: String,
    pub content: String,
}

// Implement the Display trait for SimplifiedConversation
impl fmt::Display for SimplifiedConversation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match to_string_pretty(self) {
            Ok(json_string) => write!(f, "{}", json_string),
            Err(e) => write!(f, "Error serializing to JSON: {}", e),
        }
    }
}

// Controller functions for ConversationData
pub struct ConversationController;

impl ConversationController {
    /// Collect all content parts from the conversation in chronological order
    pub fn collect_content(conversation: &ConversationData) -> SimplifiedConversation {
        let mut messages = Vec::new();

        // Iterate over nodes in the conversation mapping and collect messages
        for node in conversation.mapping.values() {
            if let Some(message) = &node.message {
                if let Some(parts) = &message.content.parts {
                    // Combine all parts into a single string per message
                    let content = parts.join(" ");
                    
                    // Determine the author
                    let author = match message.author.role.as_str() {
                        "user" => "User",
                        "assistant" => "Assistant",
                        _ => "Unknown",
                    };
                    
                    // Use message's create_time for sorting
                    if let Some(create_time) = message.create_time {
                        messages.push((create_time, ConversationEntry {
                            author: author.to_string(),
                            content,
                        }));
                    }
                }
            }
        }

        // Sort messages based on their creation time
        messages.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let messages: Vec<ConversationEntry> = messages.into_iter().map(|(_, entry)| entry).collect();

        SimplifiedConversation {
            id: conversation.id,
            title: conversation.title.clone(),
            messages,
        }
    }
}

