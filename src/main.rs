// main.rs

use rust_sled_crud_lib::db::sled_db::SledDb;
use rust_sled_crud_lib::db::models::Parent;
use rust_sled_crud_lib::db::dd_object::DbObject;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the database at the specified path
    let db = SledDb::new("my_database")?;

    // Create a Parent object
    let parent = Parent {
        id: 1,
        name: String::from("Parent1"),
        children: vec![],
    };

    // Insert the Parent object into the database
    db.insert_object(&parent)?;
    println!("Inserted parent with id 1");

    // Retrieve the Parent object by key
    if let Some(retrieved_parent) = db.get_object::<Parent>(parent.key())? {
        println!("Retrieved parent: {:?}", retrieved_parent);
    } else {
        println!("Parent with id 1 not found");
    }

    // Remove the Parent object
    db.remove_object(parent.key())?;
    println!("Removed parent with id 1");

    // Verify the removal
    if db.get_object::<Parent>(parent.key())?.is_none() {
        println!("Verified removal of parent with id 1");
    } else {
        println!("Parent with id 1 still exists");
    }

    // Flush changes to disk
    db.flush()?;
    println!("Database changes flushed");

    Ok(())
}
