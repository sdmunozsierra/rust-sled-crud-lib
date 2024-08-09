// src/query/query_builder.rs

use crate::query::sorting::{Sort,SortDirection};
use crate::query::pagination::{Page, PageRequest};
use crate::repository::generic_repository::GenericRepository;
use crate::conf::logger::init_logging;
use log::debug;

#[ctor::ctor] 
    fn init(){
    init_logging();
}

/// Struct to build complex queries with filtering options
pub struct QueryBuilder {
    filters: Vec<Filter>,
    sort: Option<Sort>,
    page_request: Option<PageRequest>,
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
        QueryBuilder { filters: Vec::new(), sort: None, page_request: None }
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

    /// Add pagination
    pub fn paginate(mut self, page: usize, size: usize) -> Self {
        self.page_request = Some(PageRequest { page, size });
        self
    }

    /// Build the query with the specified filters
    pub fn build(self) -> Query {
        Query { filters: self.filters, sort: self.sort, page_request: self.page_request }
    }
}

/// Struct to represent a built query
pub struct Query {
    filters: Vec<Filter>,
    sort: Option<Sort>,
    page_request: Option<PageRequest>,
}

impl Query {
    /// Execute the query against a generic repository
    pub fn execute<T, ID>(&self, repository: &impl GenericRepository<T, ID>) -> Page<T>
    where
        T: Filterable + Clone + Ord, // Ensure T implements Ord for sorting
    {
        let mut results: Vec<T> = repository
        .find_all()
        .into_iter()
        .filter(|entity| {
            self.filters.iter().all(|filter| {
                let entity_value = entity.get_field_value(&filter.field);
                debug!("Filtering: {:?} with {:?}", entity_value, filter);
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
            debug!("Sorting: {:?} vs {:?}", a_value, b_value);
            let order = a_value.cmp(&b_value);
            match sort.direction {
                SortDirection::Ascending => order,
                SortDirection::Descending => order.reverse(),
            }
        });
    }
      // Apply pagination if specified
    if let Some(page_request) = &self.page_request {
        let offset = page_request.offset();
        let paginated_content = results
            .into_iter()
            .skip(offset)
            .take(page_request.size)
            .collect::<Vec<T>>();
        let total_elements = paginated_content.len();

        Page::new(paginated_content, total_elements, page_request.clone())
    } else {
        // Return the entire list as one page if no pagination is specified
        let total_elements = results.len();
        Page::new(results, total_elements, PageRequest { page: 1, size: total_elements })
    }
}
}

/// Trait to enable filtering entities by their fields
pub trait Filterable {
    fn get_field_value(&self, field: &str) -> Option<&String>;
}
