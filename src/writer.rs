use anyhow::{Context, Result};
use polars::prelude::*;
use std::fs::{self, File};
use std::path::Path;

pub fn write_parquet(df: &mut DataFrame, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create output directory")?;
    }

    let file = File::create(path).context("Failed to create Parquet file")?;

    ParquetWriter::new(file)
        .finish(df)
        .context("Failed to write Parquet file")?;

    Ok(())
}
