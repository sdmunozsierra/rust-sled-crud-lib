// src/query/sorting.rs
use crate::query::pagination::Page;

/// Enum to define sort direction
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Struct to handle sorting requests
pub struct Sort {
    pub field: String,
    pub direction: SortDirection,
}

impl<T> Page<T>
where
    T: Clone + Ord,
{
    /// Sort items in the page based on the sort direction
    pub fn sort(mut self, sort: Sort) -> Self {
        self.content.sort_by(|a, b| match sort.direction {
            SortDirection::Ascending => a.cmp(b),
            SortDirection::Descending => b.cmp(a),
        });
        self
    }
}
