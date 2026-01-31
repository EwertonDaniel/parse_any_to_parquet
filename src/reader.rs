use anyhow::{anyhow, Result};
use polars::prelude::*;
use std::path::Path;

use crate::format::FileFormat;

mod csv_reader;
mod excel_reader;
mod json_reader;
mod xml_reader;

pub fn read_file(path: &Path, sheet: Option<&str>) -> Result<DataFrame> {
    let format = FileFormat::from_path(path)
        .ok_or_else(|| anyhow!("Unsupported format: {:?}", path.extension()))?;

    match format {
        FileFormat::Csv => csv_reader::read(path),
        FileFormat::Json => json_reader::read(path),
        FileFormat::Xml => xml_reader::read(path),
        FileFormat::Xls | FileFormat::Xlsx | FileFormat::Ods => excel_reader::read(path, sheet),
    }
}
