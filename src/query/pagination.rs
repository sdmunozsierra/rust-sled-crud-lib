// src/query/pagination.rs

pub struct PageRequest {
    pub page: usize,
    pub size: usize,
}

pub struct Page<T> {
    pub content: Vec<T>,
    pub total_elements: usize,
    pub total_pages: usize,
}

impl<T> Page<T> {
    pub fn new(content: Vec<T>, total_elements: usize, page_request: PageRequest) -> Self {
        let total_pages = (total_elements as f64 / page_request.size as f64).ceil() as usize;
        Page {
            content,
            total_elements,
            total_pages,
        }
    }
}
impl PageRequest {
    /// Calculate the offset to start retrieving items for the current page
    pub fn offset(&self) -> usize {
        (self.page - 1) * self.size
    }
}