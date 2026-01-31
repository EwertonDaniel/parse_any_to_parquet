use anyhow::{Context, Result};
use polars::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read(path: &Path) -> Result<DataFrame> {
    let file = File::open(path).context("Failed to open JSON file")?;
    let reader = BufReader::new(file);

    JsonReader::new(reader)
        .finish()
        .context("Failed to read JSON file")
}
