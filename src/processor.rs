use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;
use crate::converter::convert;
use crate::format::FileFormat;

pub struct ProcessStats {
    pub processed: usize,
    pub errors: usize,
}

pub fn process_batch(config: &Config) -> Result<ProcessStats> {
    print_header(config);

    let mut stats = ProcessStats { processed: 0, errors: 0 };

    for entry in create_walker(config).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && FileFormat::is_supported(path) {
            let output = get_output_path(path, &config.general.input_dir, &config.general.output_dir);

            match process_file(path, &output, config) {
                Ok(_) => stats.processed += 1,
                Err(e) => {
                    eprintln!("[ERROR] {}: {}", path.display(), e);
                    stats.errors += 1;
                }
            }
        }
    }

    println!("---");
    println!("Processed: {} | Errors: {}", stats.processed, stats.errors);

    Ok(stats)
}

pub fn process_file(input: &Path, output: &Path, config: &Config) -> Result<()> {
    println!("[CONVERTING] {}", input.display());

    let result = convert(input, output, config.general.default_sheet.as_deref())?;

    println!(
        "  Format: {:?} | Rows: {} | Columns: {}",
        result.format, result.rows, result.columns
    );
    println!("[OK] {}", output.display());

    if config.general.delete_source {
        delete_source_file(input);
    }

    Ok(())
}

pub fn get_output_path(input: &Path, input_dir: &Path, output_dir: &Path) -> PathBuf {
    let relative = input.strip_prefix(input_dir).unwrap_or(input);
    let mut output = output_dir.join(relative);
    output.set_extension("parquet");
    output
}

fn print_header(config: &Config) {
    println!("Processing directory: {}", config.general.input_dir.display());
    println!("Output: {}", config.general.output_dir.display());
    println!("Supported extensions: {:?}", FileFormat::supported_extensions());
    println!("---");
}

fn create_walker(config: &Config) -> WalkDir {
    let walker = WalkDir::new(&config.general.input_dir);
    if config.general.recursive {
        walker
    } else {
        walker.max_depth(1)
    }
}

fn delete_source_file(path: &Path) {
    match fs::remove_file(path) {
        Ok(_) => println!("[DELETED] {}", path.display()),
        Err(e) => eprintln!("[WARN] Could not delete {}: {}", path.display(), e),
    }
}
