use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    String,
    Number,
    Date,
    Boolean,
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataValue {
    String(String),
    Number(f64),
    Date(NaiveDate),
    Boolean(bool),
    Empty,
}

impl DataValue {
    pub fn data_type(&self) -> DataType {
        match self {
            DataValue::String(_) => DataType::String,
            DataValue::Number(_) => DataType::Number,
            DataValue::Date(_) => DataType::Date,
            DataValue::Boolean(_) => DataType::Boolean,
            DataValue::Empty => DataType::Empty,
        }
    }

    pub fn display_string(&self) -> String {
        match self {
            DataValue::String(value) => value.clone(),
            DataValue::Number(value) => format!("{value}"),
            DataValue::Date(value) => value.format("%Y-%m-%d").to_string(),
            DataValue::Boolean(value) => value.to_string(),
            DataValue::Empty => "".to_string(),
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DataValue::Number(value) => Some(*value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub values: Vec<DataValue>,
}

impl Column {
    pub fn len(&self) -> usize {
        self.values.len()
    }
}
