use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Table,
    Scatter,
}

impl ChartType {
    pub const ALL: [ChartType; 5] = [
        ChartType::Bar,
        ChartType::Line,
        ChartType::Pie,
        ChartType::Table,
        ChartType::Scatter,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ChartType::Bar => "Bar",
            ChartType::Line => "Line",
            ChartType::Pie => "Pie",
            ChartType::Table => "Table",
            ChartType::Scatter => "Scatter",
        }
    }
}
