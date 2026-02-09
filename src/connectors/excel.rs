use std::path::Path;
use calamine::{Data, Reader, open_workbook_auto};
use chrono::{Duration, NaiveDate};

use crate::core::data::{Column, DataType as VizDataType, DataValue};
use crate::core::workbook::{Workbook, Worksheet};

pub fn load_workbook<P: AsRef<Path>>(path: P) -> anyhow::Result<Workbook> {
    let mut workbook = open_workbook_auto(&path)?;
    let mut sheets = Vec::new();

    for sheet_name in workbook.sheet_names().to_owned() {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let mut rows = range.rows();
            let headers = match rows.next() {
                Some(header_row) => header_row
                    .iter()
                    .enumerate()
                    .map(|(idx, cell)| match cell {
                        Data::String(text) => text.to_string(),
                        _ => format!("Column {}", idx + 1),
                    })
                    .collect::<Vec<_>>(),
                None => Vec::new(),
            };

            let mut columns: Vec<Column> = headers
                .iter()
                .map(|name| Column {
                    name: name.clone(),
                    data_type: VizDataType::Empty,
                    values: Vec::new(),
                })
                .collect();

            for row in rows {
                for (idx, cell) in row.iter().enumerate() {
                    if let Some(column) = columns.get_mut(idx) {
                        column.values.push(convert_cell(cell));
                    }
                }
            }

            for column in &mut columns {
                column.data_type = infer_type(&column.values);
            }

            let row_count = columns.first().map(|col| col.len()).unwrap_or(0);

            sheets.push(Worksheet {
                name: sheet_name.to_string(),
                columns,
                row_count,
            });
        }
    }

    Ok(Workbook { sheets })
}

fn convert_cell(cell: &Data) -> DataValue {
    match cell {
        Data::String(value) => DataValue::String(value.clone()),
        Data::Float(value) => DataValue::Number(*value),
        Data::Int(value) => DataValue::Number(*value as f64),
        Data::Bool(value) => DataValue::Boolean(*value),
        Data::DateTime(value) => {
            let serial = value.as_f64();
            DataValue::Date(excel_serial_to_date(serial))
        }
        Data::DateTimeIso(value) => parse_iso_date(value).unwrap_or(DataValue::String(value.clone())),
        Data::Empty => DataValue::Empty,
        _ => DataValue::String(cell.to_string()),
    }
}

fn excel_serial_to_date(value: f64) -> NaiveDate {
    let base = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap();
    base + Duration::days(value.floor() as i64)
}

fn parse_iso_date(value: &str) -> Option<DataValue> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").ok().map(DataValue::Date)
}

fn infer_type(values: &[DataValue]) -> VizDataType {
    let mut has_string = false;
    let mut has_number = false;
    let mut has_date = false;
    let mut has_bool = false;

    for value in values {
        match value {
            DataValue::String(_) => has_string = true,
            DataValue::Number(_) => has_number = true,
            DataValue::Date(_) => has_date = true,
            DataValue::Boolean(_) => has_bool = true,
            DataValue::Empty => {}
        }
    }

    if has_string {
        VizDataType::String
    } else if has_date {
        VizDataType::Date
    } else if has_bool && !has_number {
        VizDataType::Boolean
    } else if has_number {
        VizDataType::Number
    } else {
        VizDataType::Empty
    }
}
