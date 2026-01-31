# Deployment Guide

This guide explains how to deploy `parse_to_parquet` for production use with MinIO.

## Quick Start

```bash
# Build the release binary
cargo build --release

# Copy to system path
sudo cp target/release/parse_to_parquet /usr/local/bin/
```

## Systemd Service

Create a systemd service for automatic startup and monitoring.

### 1. Create Service File

```bash
sudo nano /etc/systemd/system/parse-to-parquet.service
```

```ini
[Unit]
Description=Parse to Parquet - File Conversion Service
After=network.target minio.service

[Service]
Type=simple
User=minio
Group=minio
ExecStart=/usr/local/bin/parse_to_parquet watch -i /mnt/minio/uploads -o /mnt/minio/parquet -r --delete-source
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

### 2. Enable and Start

```bash
sudo systemctl daemon-reload
sudo systemctl enable parse-to-parquet
sudo systemctl start parse-to-parquet
```

### 3. Check Status

```bash
sudo systemctl status parse-to-parquet
sudo journalctl -u parse-to-parquet -f  # follow logs
```

## Configuration File

For complex setups, use a configuration file:

### 1. Create Config

```bash
sudo mkdir -p /etc/parse-to-parquet
sudo nano /etc/parse-to-parquet/config.toml
```

```toml
[general]
input_dir = "/mnt/minio/uploads"
output_dir = "/mnt/minio/parquet"
recursive = true
delete_source = true

[watch]
poll_interval_secs = 2
debounce_ms = 1000
```

### 2. Update Service

```ini
ExecStart=/usr/local/bin/parse_to_parquet watch -c /etc/parse-to-parquet/config.toml
```

## Docker Deployment

### Dockerfile

```dockerfile
FROM rust:1.75-alpine AS builder
WORKDIR /app
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /app/target/release/parse_to_parquet /usr/local/bin/
ENTRYPOINT ["parse_to_parquet"]
CMD ["watch", "-i", "/data/input", "-o", "/data/output", "-r"]
```

### Docker Compose

```yaml
version: '3.8'
services:
  parse-to-parquet:
    build: .
    volumes:
      - /mnt/minio/uploads:/data/input
      - /mnt/minio/parquet:/data/output
    restart: always
```

## MinIO Bucket Notifications (Alternative)

Instead of polling, you can use MinIO bucket notifications to trigger conversions:

### 1. Create Webhook Handler

```bash
# Create a simple HTTP server that calls parse_to_parquet
```

### 2. Configure MinIO Notification

```bash
mc admin config set myminio notify_webhook:parquet \
    endpoint="http://localhost:8080/convert" \
    queue_dir="/tmp/minio-events"

mc event add myminio/uploads arn:minio:sqs::parquet:webhook \
    --event put \
    --suffix ".csv,.json,.xml,.xls,.xlsx,.ods"
```

## Monitoring

### Health Check Script

```bash
#!/bin/bash
# /usr/local/bin/check-parquet-service.sh

if ! systemctl is-active --quiet parse-to-parquet; then
    echo "CRITICAL: parse-to-parquet is not running"
    exit 2
fi

# Check for recent errors
ERRORS=$(journalctl -u parse-to-parquet --since "5 minutes ago" | grep -c "\[ERROR\]")
if [ "$ERRORS" -gt 10 ]; then
    echo "WARNING: $ERRORS errors in last 5 minutes"
    exit 1
fi

echo "OK: parse-to-parquet is running"
exit 0
```

### Prometheus Metrics (Future)

For future versions, expose Prometheus metrics:
- `parquet_files_converted_total`
- `parquet_conversion_errors_total`
- `parquet_conversion_duration_seconds`

## Troubleshooting

### Common Issues

**Permission Denied**
```bash
sudo chown -R minio:minio /mnt/minio
```

**File Not Converted**
```bash
# Check if extension is supported
./parse_to_parquet --help
```

**High Memory Usage**
- Large Excel files are loaded entirely into memory
- Consider splitting large files before conversion

### Debug Mode

```bash
RUST_BACKTRACE=1 ./parse_to_parquet watch -i ./input -o ./output
```
