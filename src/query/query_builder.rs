// src/query/query_builder.rs

use crate::query::sorting::{Sort,SortDirection};
use crate::repository::generic_repository::GenericRepository;

/// Struct to build complex queries with filtering options
pub struct QueryBuilder {
    filters: Vec<Filter>,
    sort: Option<Sort>,
}

/// Struct to define a filter condition
#[derive(Debug)]
pub struct Filter {
    field: String,
    value: String,
    operation: FilterOperation,
}

/// Enum to define filter operations
#[derive(Debug)]
pub enum FilterOperation {
    Eq,
    Gt,
    Lt,
    // Additional operations
}


impl QueryBuilder {
    /// Create a new QueryBuilder instance
    pub fn new() -> Self {
        QueryBuilder { filters: Vec::new(), sort: None }
    }

    /// Add an equality filter
    pub fn where_eq(mut self, field: &str, value: &str) -> Self {
        self.filters.push(Filter {
            field: field.to_string(),
            value: value.to_string(),
            operation: FilterOperation::Eq,
        });
        self
    }

    /// Add a greater-than filter
    pub fn where_gt(mut self, field: &str, value: &str) -> Self {
        self.filters.push(Filter {
            field: field.to_string(),
            value: value.to_string(),
            operation: FilterOperation::Gt,
        });
        self
    }

    /// Add a less-than filter
    pub fn where_lt(mut self, field: &str, value: &str) -> Self {
        self.filters.push(Filter {
            field: field.to_string(),
            value: value.to_string(),
            operation: FilterOperation::Lt,
        });
        self
    }

    /// Add sorting
    pub fn order_by(mut self, field: &str, direction: SortDirection) -> Self {
        self.sort = Some(Sort {
            field: field.to_string(),
            direction,
        });
        self
    }

    /// Build the query with the specified filters
    pub fn build(self) -> Query {
        Query { filters: self.filters, sort: self.sort }
    }
}

/// Struct to represent a built query
pub struct Query {
    filters: Vec<Filter>,
    sort: Option<Sort>,
}

impl Query {
    /// Execute the query against a generic repository
    pub fn execute<T, ID>(&self, repository: &impl GenericRepository<T, ID>) -> Vec<T>
    where
        T: Filterable, // Assume T implements a trait Filterable for field access
    {
        let mut results: Vec<T> = repository
        .find_all()
        .into_iter()
        .filter(|entity| {
            self.filters.iter().all(|filter| {
                let entity_value = entity.get_field_value(&filter.field);
                println!("Filtering: {:?} with {:?}", entity_value, filter);
                match filter.operation {
                    FilterOperation::Eq => entity_value == Some(&filter.value),
                    FilterOperation::Gt => entity_value.map_or(false, |v| v > &filter.value),
                    FilterOperation::Lt => entity_value.map_or(false, |v| v < &filter.value),
                }
            })
        })
        .collect();

    if let Some(sort) = &self.sort {
        results.sort_by(|a, b| {
            let a_value = a.get_field_value(&sort.field);
            let b_value = b.get_field_value(&sort.field);
            println!("Sorting: {:?} vs {:?}", a_value, b_value);
            let order = a_value.cmp(&b_value);
            match sort.direction {
                SortDirection::Ascending => order,
                SortDirection::Descending => order.reverse(),
            }
        });
    }

    results
}
}

/// Trait to enable filtering entities by their fields
pub trait Filterable {
    fn get_field_value(&self, field: &str) -> Option<&String>;
}
