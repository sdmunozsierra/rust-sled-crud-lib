use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;
use std::collections::HashMap;

/// Root struct for the JSON object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationData {
    pub id: Uuid,
    pub title: String,
    pub create_time: f64,
    pub update_time: f64,
    pub mapping: HashMap<Uuid, Node>,
    pub current_node: Uuid,
    pub conversation_template_id: Option<Uuid>,
    pub gizmo_id: Option<Uuid>,
    pub is_archived: bool,
    pub workspace_id: Option<Uuid>,
    pub async_status: Option<String>,
    pub safe_urls: Vec<String>,
    pub moderation_results: Vec<String>,
    pub plugin_ids: Option<Vec<Uuid>>,
    pub conversation_id: Uuid,
    pub default_model_slug: String,
    pub conversation_origin: Option<String>,
    pub voice: Option<String>,
}

/// Wrapper enum for different types of metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MetadataWrapper {
    Citations(CitationsMetadata),
    Finish(FinishMetadata),
    Source(SourceMetadata),
    Visibility(VisibilityMetadata),
    Common(CommonMetadata),
}

/// Metadata with common fields
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonMetadata {
    pub message_type: Option<String>,
    pub model_slug: Option<String>,
    pub default_model_slug: Option<String>,
    pub parent_id: Option<Uuid>,
    pub request_id: Option<String>,
    pub timestamp_: Option<String>,
    pub model_switcher_deny: Option<Vec<String>>,
}

/// Metadata for messages with citations and finish details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CitationsMetadata {
    pub citations: Vec<String>,
    pub gizmo_id: Option<Uuid>,
    pub finish_details: Option<FinishDetails>,
    pub is_complete: Option<bool>,
    pub pad: Option<String>,
    #[serde(flatten)]
    pub common: CommonMetadata,
}

/// Metadata for messages with finish details only
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinishMetadata {
    pub finish_details: Option<FinishDetails>,
    pub is_complete: Option<bool>,
    pub pad: Option<String>,
    #[serde(flatten)]
    pub common: CommonMetadata,
}

/// Metadata for simple message source
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceMetadata {
    pub message_source: Option<String>,
    #[serde(flatten)]
    pub common: CommonMetadata,
}

/// Simple visibility metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisibilityMetadata {
    pub is_visually_hidden_from_conversation: bool,
}

/// Details about the message finish process
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinishDetails {
    pub r#type: String,
    pub stop_tokens: Vec<i32>,
}

/// The main message structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub author: Author,
    pub create_time: Option<f64>,
    pub update_time: Option<f64>,
    pub content: Content,
    pub status: String,
    pub end_turn: Option<bool>,
    pub weight: i32,
    #[serde(deserialize_with = "deserialize_metadata")]
    pub metadata: MetadataWrapper,
    pub recipient: String,
    pub channel: Option<String>,
}

/// The author of a message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub role: String,
    pub name: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Content structure for message contents
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub content_type: String,
    pub parts: Option<Vec<String>>,
}

/// Node structure containing a message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub message: Option<Message>,
    pub parent: Option<Uuid>,
    pub children: Option<Vec<Uuid>>,
}

/// Custom deserializer for MetadataWrapper
fn deserialize_metadata<'de, D>(deserializer: D) -> Result<MetadataWrapper, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, Value> = HashMap::deserialize(deserializer)?;

    // Convert HashMap to serde_json::Map
    let json_map: Map<String, Value> = map.into_iter().collect();

    // Wrap the map into a Value::Object
    let value = Value::Object(json_map);

    // Determine the appropriate variant by checking for key existence
    if value.get("is_visually_hidden_from_conversation").is_some() {
        serde_json::from_value(value)
            .map(MetadataWrapper::Visibility)
            .map_err(serde::de::Error::custom)
    } else if value.get("citations").is_some() {
        serde_json::from_value(value)
            .map(MetadataWrapper::Citations)
            .map_err(serde::de::Error::custom)
    } else if value.get("finish_details").is_some() {
        serde_json::from_value(value)
            .map(MetadataWrapper::Finish)
            .map_err(serde::de::Error::custom)
    } else if value.get("message_source").is_some() {
        serde_json::from_value(value)
            .map(MetadataWrapper::Source)
            .map_err(serde::de::Error::custom)
    } else {
        serde_json::from_value(value)
            .map(MetadataWrapper::Common)
            .map_err(serde::de::Error::custom)
    }
}
