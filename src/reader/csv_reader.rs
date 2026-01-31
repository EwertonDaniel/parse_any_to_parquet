use anyhow::{Context, Result};
use polars::prelude::*;
use std::path::Path;

pub fn read(path: &Path) -> Result<DataFrame> {
    CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(Some(1000))
        .try_into_reader_with_file_path(Some(path.into()))
        .context("Failed to open CSV file")?
        .finish()
        .context("Failed to read CSV file")
}
