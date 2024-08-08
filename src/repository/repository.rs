pub trait Repository<T, ID> {
    fn save(&self, entity: T) -> Result<(), String>;
    fn find_by_id(&self, id: ID) -> Option<T>;
    fn find_all(&self) -> Vec<T>;
    fn delete(&self, entity: T) -> Result<(), String>;
}

