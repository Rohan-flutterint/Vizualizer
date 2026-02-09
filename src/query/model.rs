use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Aggregation {
    Sum,
    Avg,
    Count,
    Min,
    Max,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub column: String,
    pub equals: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySpec {
    pub rows: Vec<String>,
    pub columns: Vec<String>,
    pub aggregations: Vec<Aggregation>,
    pub filters: Vec<Filter>,
}

impl QuerySpec {
    pub fn empty() -> Self {
        Self {
            rows: Vec::new(),
            columns: Vec::new(),
            aggregations: vec![Aggregation::Sum],
            filters: Vec::new(),
        }
    }
}
