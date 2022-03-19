use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    page: i64,
    limit: i64,
}

impl Pagination {
    pub fn new(page: i64, limit: i64) -> Self {
        Self { page, limit }
    }

    pub fn to_query(&self) -> [(String, String); 2] {
        [
            ("page".to_string(), self.page.to_string()),
            ("limit".to_string(), self.limit.to_string()),
        ]
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination { page: 1, limit: 20 }
    }
}
