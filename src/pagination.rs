use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    page: i32,
    limit: i32,
}

impl Pagination {
    pub fn new(page: i32, limit: i32) -> Self {
        Self { page, limit }
    }

    pub fn to_query(&self) -> [(&'static str, String); 2] {
        [
            ("page", self.page.to_string()),
            ("limit", self.limit.to_string()),
        ]
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination { page: 1, limit: 20 }
    }
}
