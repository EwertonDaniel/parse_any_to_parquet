use anyhow::Result;
use std::path::Path;

use crate::format::FileFormat;
use crate::reader::read_file;
use crate::writer::write_parquet;

pub struct ConversionResult {
    pub rows: usize,
    pub columns: usize,
    pub format: FileFormat,
}

pub fn convert(input: &Path, output: &Path, sheet: Option<&str>) -> Result<ConversionResult> {
    let format = FileFormat::from_path(input)
        .ok_or_else(|| anyhow::anyhow!("Unsupported format: {:?}", input.extension()))?;

    let mut df = read_file(input, sheet)?;

    let result = ConversionResult {
        rows: df.height(),
        columns: df.width(),
        format,
    };

    write_parquet(&mut df, output)?;

    Ok(result)
}
