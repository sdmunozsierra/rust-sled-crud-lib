use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::db::dd_object::DbObject;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub title: String,
    pub create_time: f64,
    pub update_time: f64,
    pub mapping: HashMap<String, String>, // Adjust type based on actual nested structure
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

impl DbObject for Conversation {
    fn key(&self) -> sled::IVec {
        sled::IVec::from(self.id.to_string().as_bytes())
    }
}
