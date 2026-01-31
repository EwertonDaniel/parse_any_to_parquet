mod cli;
mod config;
mod converter;
mod format;
mod processor;
mod reader;
mod watcher;
mod writer;

use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

use cli::{Cli, Commands};
use config::Config;
use converter::convert;
use processor::process_batch;
use watcher::watch;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { input, output, sheet } => {
            run_convert(input, output, sheet)
        }

        Commands::Batch { input_dir, output_dir, delete_source, recursive } => {
            let config = load_config_with_overrides(
                cli.config.as_deref(),
                input_dir,
                output_dir,
                delete_source,
                recursive,
            )?;
            process_batch(&config)?;
            Ok(())
        }

        Commands::Watch { input_dir, output_dir, delete_source, recursive } => {
            let config = load_config_with_overrides(
                cli.config.as_deref(),
                input_dir,
                output_dir,
                delete_source,
                recursive,
            )?;
            watch(&config)
        }

        Commands::InitConfig { output } => {
            Config::generate_default(&output)?;
            println!("Configuration file created: {}", output.display());
            Ok(())
        }
    }
}

fn run_convert(input: PathBuf, output: Option<PathBuf>, sheet: Option<String>) -> Result<()> {
    if !input.exists() {
        return Err(anyhow!("File not found: {}", input.display()));
    }

    let output = output.unwrap_or_else(|| {
        let mut out = input.clone();
        out.set_extension("parquet");
        out
    });

    println!("[CONVERTING] {}", input.display());

    let result = convert(&input, &output, sheet.as_deref())?;

    println!(
        "  Format: {:?} | Rows: {} | Columns: {}",
        result.format, result.rows, result.columns
    );
    println!("[OK] {}", output.display());

    Ok(())
}

fn load_config_with_overrides(
    config_path: Option<&std::path::Path>,
    input_dir: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    delete_source: bool,
    recursive: bool,
) -> Result<Config> {
    let mut config = Config::load(config_path)?;

    config.general.apply_overrides(input_dir, output_dir, delete_source, recursive);

    if !config.general.input_dir.exists() {
        return Err(anyhow!(
            "Directory not found: {}",
            config.general.input_dir.display()
        ));
    }

    Ok(config)
}
