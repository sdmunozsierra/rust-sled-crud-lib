use crate::repository::conversation_repository::{Repository, ConversationRepository};
use crate::models::conversation::ConversationData;
use uuid::Uuid;
use std::collections::HashMap;

pub struct ConversationService {
    conversation_repository: ConversationRepository,
}

impl ConversationService {
    pub fn new(conversation_repository: ConversationRepository) -> Self {
        ConversationService {
            conversation_repository,
        }
    }

    /// Register a new conversation
    pub fn register_conversation(&self, title: String) -> Result<ConversationData, String> {
        let conversation_id = Uuid::new_v4();
        let conversation = ConversationData {
            id: conversation_id,
            title,
            create_time: Self::current_time(),
            update_time: Self::current_time(),
            mapping: HashMap::new(),
            current_node: Uuid::default(),
            conversation_template_id: None,
            gizmo_id: None,
            is_archived: false,
            workspace_id: None,
            async_status: None,
            safe_urls: vec![],
            moderation_results: vec![],
            plugin_ids: None,
            conversation_id,
            default_model_slug: "default".to_string(),
            conversation_origin: None,
            voice: None,
        };
        
        self.conversation_repository.save(conversation.clone())?;
        Ok(conversation)
    }

    /// Get a conversation by its ID
    pub fn get_conversation(&self, conversation_id: Uuid) -> Option<ConversationData> {
        self.conversation_repository.find_by_id(conversation_id)
    }

    /// Update a conversation title
    pub fn update_conversation_title(&self, conversation_id: Uuid, new_title: String) -> Result<(), String> {
        if let Some(mut conversation) = self.conversation_repository.find_by_id(conversation_id) {
            conversation.title = new_title;
            conversation.update_time = Self::current_time();
            self.conversation_repository.save(conversation)
        } else {
            Err(format!("Conversation with ID {} not found", conversation_id))
        }
    }

    /// Archive a conversation
    pub fn archive_conversation(&self, conversation_id: Uuid) -> Result<(), String> {
        if let Some(mut conversation) = self.conversation_repository.find_by_id(conversation_id) {
            conversation.is_archived = true;
            conversation.update_time = Self::current_time();
            self.conversation_repository.save(conversation)
        } else {
            Err(format!("Conversation with ID {} not found", conversation_id))
        }
    }

    /// Find all conversations
    pub fn find_all_conversations(&self) -> Vec<ConversationData> {
        self.conversation_repository.find_all()
    }

    /// Delete a conversation
    pub fn delete_conversation(&self, conversation_id: Uuid) -> Result<(), String> {
        if let Some(conversation) = self.conversation_repository.find_by_id(conversation_id) {
            self.conversation_repository.delete(conversation)
        } else {
            Err(format!("Conversation with ID {} not found", conversation_id))
        }
    }

    /// Utility function to get the current time as a floating-point number
    fn current_time() -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs_f64()
    }
}
