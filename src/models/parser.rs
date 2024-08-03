use serde_json;
use std::error::Error;
use crate::models::message::ConversationData;

/// Function to parse a JSON string into a ConversationData object
pub fn parse_conversation_data(json_data: &str) -> Result<ConversationData, Box<dyn std::error::Error>> {
    let conversation_data: ConversationData = serde_json::from_str(json_data)?;
    Ok(conversation_data)
}