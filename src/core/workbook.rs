use serde::{Deserialize, Serialize};

use crate::core::data::Column;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worksheet {
    pub name: String,
    pub columns: Vec<Column>,
    pub row_count: usize,
}

impl Worksheet {
    pub fn column_names(&self) -> Vec<String> {
        self.columns.iter().map(|col| col.name.clone()).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workbook {
    pub sheets: Vec<Worksheet>,
}
