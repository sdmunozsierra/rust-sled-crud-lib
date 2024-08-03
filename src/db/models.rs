use serde_derive::{Deserialize, Serialize};
use sled::IVec;
use crate::db::dd_object::DbObject;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parent {
    pub id: u32,
    pub name: String,
    pub children: Vec<Child>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Child {
    pub id: u32,
    pub name: String,
}

impl DbObject for Parent {
    fn key(&self) -> IVec {
        IVec::from(format!("parent:{}", self.id).as_bytes())
    }
}

impl DbObject for Child {
    fn key(&self) -> IVec {
        IVec::from(format!("child:{}", self.id).as_bytes())
    }
}