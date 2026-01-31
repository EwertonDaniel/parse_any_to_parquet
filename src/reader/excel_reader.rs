use anyhow::{anyhow, Context, Result};
use calamine::{open_workbook_auto, Data, Reader, Sheets};
use polars::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn read(path: &Path, sheet_name: Option<&str>) -> Result<DataFrame> {
    let mut workbook = open_workbook_auto(path).context("Failed to open Excel/ODS file")?;

    let sheet = get_sheet_name(&workbook, sheet_name)?;
    let range = workbook
        .worksheet_range(&sheet)
        .context(format!("Sheet '{}' not found", sheet))?;

    let (rows, cols) = range.get_size();
    if rows == 0 || cols == 0 {
        return Err(anyhow!("Empty sheet"));
    }

    let headers = extract_headers(&range, cols);
    let columns = build_columns(&range, &headers, rows, cols);

    DataFrame::new(columns).context("Failed to create DataFrame from Excel")
}

fn get_sheet_name(workbook: &Sheets<BufReader<std::fs::File>>, name: Option<&str>) -> Result<String> {
    match name {
        Some(n) => Ok(n.to_string()),
        None => workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| anyhow!("No sheets found")),
    }
}

fn extract_headers(range: &calamine::Range<Data>, cols: usize) -> Vec<String> {
    (0..cols)
        .map(|c| {
            range
                .get((0, c))
                .map(|cell| cell.to_string())
                .unwrap_or_else(|| format!("col_{}", c))
        })
        .collect()
}

fn build_columns(
    range: &calamine::Range<Data>,
    headers: &[String],
    rows: usize,
    cols: usize,
) -> Vec<Column> {
    (0..cols)
        .map(|col_idx| {
            let values: Vec<Option<String>> = (1..rows)
                .map(|row_idx| cell_to_string(range.get((row_idx, col_idx))))
                .collect();

            Column::new(headers[col_idx].as_str().into(), values)
        })
        .collect()
}

fn cell_to_string(cell: Option<&Data>) -> Option<String> {
    cell.and_then(|c| match c {
        Data::Empty => None,
        Data::String(s) => Some(s.clone()),
        Data::Float(f) => Some(f.to_string()),
        Data::Int(i) => Some(i.to_string()),
        Data::Bool(b) => Some(b.to_string()),
        Data::DateTime(dt) => Some(dt.to_string()),
        Data::DateTimeIso(s) => Some(s.clone()),
        Data::DurationIso(s) => Some(s.clone()),
        Data::Error(e) => Some(format!("ERROR: {:?}", e)),
    })
}
