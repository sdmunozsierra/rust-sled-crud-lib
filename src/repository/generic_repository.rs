use crate::repository::repository_error::RepositoryError;

pub trait GenericRepository<T, ID> {
    fn save(&self, entity: T) -> Result<T, RepositoryError>;
    fn find_by_id(&self, id: ID) -> Option<T>;
    fn find_all(&self) -> Vec<T>;
    fn delete(&self, entity: T) -> Result<(), RepositoryError>;
    //fn delete_by_id(&self, id: ID) -> Result<(), RepositoryError>;
}
