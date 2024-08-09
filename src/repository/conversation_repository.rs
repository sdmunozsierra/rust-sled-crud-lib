use crate::models::conversation::ConversationData;
use crate::db::sled_mutex::SledDb;
use std::sync::Arc;
use serde_json;
use uuid::Uuid;
use sled::Db;

/// The repository trait defines basic CRUD operations
pub trait Repository<T, ID> {
    fn save(&self, entity: T) -> Result<(), String>;
    fn find_by_id(&self, id: ID) -> Option<T>;
    fn find_all(&self) -> Vec<T>;
    fn delete(&self, entity: T) -> Result<(), String>;
}

/// The ConversationRepository struct holds a reference to the database
pub struct ConversationRepository {
    db: Arc<Db>,
}

impl ConversationRepository {
    /// Create a new ConversationRepository instance
    pub fn new(sled_db: Arc<SledDb>) -> Self {
        let db = sled_db.get_db();
        ConversationRepository { db }
    }
}

impl Repository<ConversationData, Uuid> for ConversationRepository {
    /// Save a conversation to the database
    fn save(&self, conversation: ConversationData) -> Result<(), String> {
        let key = conversation.id.to_string();
        let value = serde_json::to_vec(&conversation).map_err(|e| e.to_string())?;

        self.db.insert(key.as_bytes(), value).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Find a conversation by its ID
    fn find_by_id(&self, id: Uuid) -> Option<ConversationData> {
        let key = id.to_string();

        self.db.get(key.as_bytes()).ok().flatten().and_then(|ivec| {
            serde_json::from_slice(&ivec).ok()
        })
    }

    /// Retrieve all conversations from the database
    fn find_all(&self) -> Vec<ConversationData> {
        let mut conversations = vec![];

        for item in self.db.iter() {
            if let Ok((_, value)) = item {
                if let Ok(conversation) = serde_json::from_slice::<ConversationData>(&value) {
                    conversations.push(conversation);
                }
            }
        }

        conversations
    }

    /// Delete a conversation from the database
    fn delete(&self, conversation: ConversationData) -> Result<(), String> {
        let key = conversation.id.to_string();

        self.db.remove(key.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}
