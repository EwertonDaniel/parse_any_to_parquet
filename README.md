# Parse to Parquet

A high-performance CLI tool to convert various file formats (CSV, JSON, XML, XLS, XLSX, ODS) to Apache Parquet format. Built with Rust and Polars for maximum efficiency.

## Features

- **Multiple Input Formats**: CSV, JSON, XML, Excel (XLS, XLSX), LibreOffice Calc (ODS)
- **High Compression**: Parquet format typically achieves 80-90% compression
- **Directory Watching**: Monitor directories and auto-convert new files (ideal for MinIO integration)
- **Batch Processing**: Convert entire directories at once
- **Configurable**: TOML-based configuration file

## Installation

```bash
cargo build --release
```

The binary will be available at `./target/release/parse_to_parquet`.

## Usage

### Convert a Single File

```bash
./parse_to_parquet convert -i input.csv
./parse_to_parquet convert -i input.xlsx -o output.parquet
./parse_to_parquet convert -i spreadsheet.xlsx -s "Sheet2"  # specific sheet
```

### Batch Process a Directory

```bash
./parse_to_parquet batch -i ./input -o ./output
./parse_to_parquet batch -i ./input -o ./output -r              # recursive
./parse_to_parquet batch -i ./input -o ./output --delete-source # delete after convert
```

### Watch Mode (MinIO Integration)

```bash
./parse_to_parquet watch -i /minio/uploads -o /minio/parquet -r
```

This mode:
1. Processes all existing files in the directory
2. Monitors for new files and converts them automatically
3. Runs until stopped with Ctrl+C

### Generate Configuration File

```bash
./parse_to_parquet init-config -o config.toml
```

## Configuration

Create a `config.toml` file:

```toml
[general]
input_dir = "./input"
output_dir = "./output"
recursive = false
delete_source = false

[watch]
poll_interval_secs = 2
debounce_ms = 500
```

Use with:

```bash
./parse_to_parquet batch -c config.toml
./parse_to_parquet watch -c config.toml
```

CLI arguments override config file values.

## Supported Formats

| Extension | Format |
|-----------|--------|
| `.csv`    | Comma-Separated Values |
| `.json`   | JSON (array of objects) |
| `.xml`    | XML (auto-detects record structure) |
| `.xls`    | Excel 97-2003 |
| `.xlsx`   | Excel 2007+ |
| `.ods`    | LibreOffice Calc |

## MinIO Integration

Deploy the binary on your MinIO server and configure it to watch your upload bucket:

```bash
# Watch a mounted MinIO bucket
./parse_to_parquet watch \
    -i /mnt/minio/raw-data \
    -o /mnt/minio/parquet-data \
    -r --delete-source
```

For systemd service deployment, see [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md).

## Performance

Example conversion:
- **Input**: 14MB CSV file (112,916 rows)
- **Output**: 1.2MB Parquet file
- **Compression**: ~91%

## License

MIT
