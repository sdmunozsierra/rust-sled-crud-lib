use serde::{Serialize, Deserialize};
use crate::models::role::Role;
use crate::query::query_builder::Filterable;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<Role>,  // Add roles to user
}

//impl User {
    ///// Get a field value by name for filtering purposes
    //pub fn get_field_value(&self, field: &str) -> Option<String> {
        //match field {
            //"id" => Some(self.id.clone()),
            //"username" => Some(self.username.clone()),
            //"email" => Some(self.email.clone()),
            //// Add more fields as necessary
            //_ => None,
        //}
    //}
//}

impl Filterable for User {
    fn get_field_value(&self, field: &str) -> Option<&String> {
        let value = match field {
            "id" => Some(&self.id),
            "username" => Some(&self.username),
            "email" => Some(&self.email),
            // Add other fields as necessary
            _ => None,
        };
    println!("Getting field value for {}: {:?}", field, value);
    value
    }
}