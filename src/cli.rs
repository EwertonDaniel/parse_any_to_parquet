use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "parse_to_parquet")]
#[command(author = "Ewerton")]
#[command(version = "1.0")]
#[command(about = "Convert CSV, JSON, XML, XLS, XLSX, ODS to Parquet - MinIO Integration")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the configuration file
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert a single file
    Convert {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Sheet name for Excel/ODS files
        #[arg(short, long)]
        sheet: Option<String>,
    },

    /// Process all files in a directory
    Batch {
        /// Input directory (overrides config)
        #[arg(short, long)]
        input_dir: Option<PathBuf>,

        /// Output directory (overrides config)
        #[arg(short, long)]
        output_dir: Option<PathBuf>,

        /// Delete source files after conversion
        #[arg(long)]
        delete_source: bool,

        /// Process subdirectories recursively
        #[arg(short, long)]
        recursive: bool,
    },

    /// Watch a directory and convert files automatically
    Watch {
        /// Directory to watch (overrides config)
        #[arg(short, long)]
        input_dir: Option<PathBuf>,

        /// Output directory (overrides config)
        #[arg(short, long)]
        output_dir: Option<PathBuf>,

        /// Delete source files after conversion
        #[arg(long)]
        delete_source: bool,

        /// Watch subdirectories recursively
        #[arg(short, long)]
        recursive: bool,
    },

    /// Generate a default configuration file
    InitConfig {
        /// Path for the configuration file
        #[arg(short, long, default_value = "config.toml")]
        output: PathBuf,
    },
}
