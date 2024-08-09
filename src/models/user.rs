use serde::{Serialize, Deserialize};
use crate::models::role::Role;
use crate::query::query_builder::Filterable;
use std::cmp::Ordering;
use crate::conf::logger::init_logging;
use log::debug;

#[ctor::ctor] 
    fn init(){
    init_logging();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<Role>, // Add roles to user
}

// Implement PartialEq for User
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

// Implement Eq for User
impl Eq for User {}

// Implement PartialOrd for User
impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement Ord for User based on the username field
impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.username.cmp(&other.username)
    }
}

impl Filterable for User {
    fn get_field_value(&self, field: &str) -> Option<&String> {
        let value = match field {
            "id" => Some(&self.id),
            "username" => Some(&self.username),
            "email" => Some(&self.email),
            // Add other fields as necessary
            _ => None,
        };
        debug!("Getting field value for {}: {:?}", field, value);
        value
    }
}
