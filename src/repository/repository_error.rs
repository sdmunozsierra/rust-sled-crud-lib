
// Define the RepositoryError enum with appropriate variants
#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    SledError(sled::Error),
    SerializationError(String),
    DeserializationError(String),
    DatabaseError(String),
}

// Implement conversion from sled::Error to RepositoryError
impl From<sled::Error> for RepositoryError {
    fn from(err: sled::Error) -> Self {
        RepositoryError::SledError(err)
    }
}