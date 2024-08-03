// main.rs

use rust_sled_crud_lib::db::sled_db::SledDb;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the database at the specified path
    let db = SledDb::new("my_database")?;

    // Insert a key-value pair into the database
    db.insert(b"key1", b"value1")?;
    println!("Inserted key1 => value1");

    // Retrieve the value by key
    if let Some(value) = db.get(b"key1")? {
        println!(
            "Retrieved key1 => {:?}",
            String::from_utf8(value.to_vec()).unwrap()
        );
    } else {
        println!("key1 not found");
    }

    // Remove the key-value pair
    db.remove(b"key1")?;
    println!("Removed key1");

    // Verify the removal
    if db.get(b"key1")?.is_none() {
        println!("Verified removal of key1");
    } else {
        println!("key1 still exists");
    }

    // Flush changes to disk
    db.flush()?;
    println!("Database changes flushed");

    Ok(())
}
