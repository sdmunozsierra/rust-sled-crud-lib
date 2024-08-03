use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Wrapper enum for different types of metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]  // Helps distinguish between different variants during serialization
pub enum MetadataWrapper {
    Common(CommonMetadata),
    Citations(CitationsMetadata),
    Finish(FinishMetadata),
    Source(SourceMetadata),
    Visibility(VisibilityMetadata),
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
    pub model_switcher_deny: Vec<String>,
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
    pub end_turn: bool,
    pub weight: i32,
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
    pub parts: Vec<String>,
}

/// Node structure containing a message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub message: Message,
    pub parent: Uuid,
    pub children: Vec<Uuid>,
}
