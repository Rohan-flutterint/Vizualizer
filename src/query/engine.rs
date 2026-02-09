use std::collections::HashMap;

use crate::core::data::{Column, DataValue};
use crate::core::workbook::Worksheet;
use crate::query::model::{Aggregation, QuerySpec};

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub fn run_query(sheet: &Worksheet, spec: &QuerySpec) -> QueryResult {
    if spec.rows.is_empty() && spec.columns.is_empty() {
        return preview_table(sheet);
    }

    let row_columns = select_columns(sheet, &spec.rows);
    let measure_columns = select_columns(sheet, &spec.columns);

    let mut groups: HashMap<Vec<String>, Vec<Vec<f64>>> = HashMap::new();
    let row_count = sheet.row_count;

    for row_index in 0..row_count {
        let mut row_key = Vec::new();
        for column in &row_columns {
            row_key.push(value_at(column, row_index).display_string());
        }

        let entry = groups.entry(row_key).or_insert_with(|| {
            vec![Vec::new(); measure_columns.len()]
        });

        for (idx, column) in measure_columns.iter().enumerate() {
            if let Some(value) = value_at(column, row_index).as_f64() {
                entry[idx].push(value);
            }
        }
    }

    let mut headers = spec.rows.clone();
    for measure in &spec.columns {
        for agg in &spec.aggregations {
            headers.push(format!("{measure} ({:?})", agg));
        }
    }

    let mut rows = Vec::new();
    for (key, values) in groups {
        let mut row = key;
        for series in values {
            for agg in &spec.aggregations {
                row.push(match agg {
                    Aggregation::Sum => series.iter().sum::<f64>().to_string(),
                    Aggregation::Avg => {
                        if series.is_empty() {
                            "0".to_string()
                        } else {
                            (series.iter().sum::<f64>() / series.len() as f64).to_string()
                        }
                    }
                    Aggregation::Count => series.len().to_string(),
                    Aggregation::Min => series
                        .iter()
                        .cloned()
                        .reduce(f64::min)
                        .unwrap_or(0.0)
                        .to_string(),
                    Aggregation::Max => series
                        .iter()
                        .cloned()
                        .reduce(f64::max)
                        .unwrap_or(0.0)
                        .to_string(),
                });
            }
        }
        rows.push(row);
    }

    QueryResult { headers, rows }
}

fn preview_table(sheet: &Worksheet) -> QueryResult {
    let headers = sheet.column_names();
    let mut rows = Vec::new();
    let row_count = sheet.row_count.min(25);
    for row_index in 0..row_count {
        let mut row = Vec::new();
        for column in &sheet.columns {
            row.push(value_at(column, row_index).display_string());
        }
        rows.push(row);
    }
    QueryResult { headers, rows }
}

fn select_columns<'a>(sheet: &'a Worksheet, names: &[String]) -> Vec<&'a Column> {
    sheet
        .columns
        .iter()
        .filter(|column| names.contains(&column.name))
        .collect()
}

fn value_at(column: &Column, index: usize) -> DataValue {
    column.values.get(index).cloned().unwrap_or(DataValue::Empty)
}
