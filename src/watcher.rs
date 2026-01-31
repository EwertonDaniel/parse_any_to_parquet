use anyhow::{Context, Result};
use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::config::Config;
use crate::format::FileFormat;
use crate::processor::{get_output_path, process_batch, process_file};

pub fn watch(config: &Config) -> Result<()> {
    let input_dir = &config.general.input_dir;
    let output_dir = &config.general.output_dir;

    println!("Watching directory: {}", input_dir.display());
    println!("Output: {}", output_dir.display());
    println!(
        "Supported extensions: {:?}",
        FileFormat::supported_extensions()
    );
    println!("Press Ctrl+C to stop.");
    println!("---");

    // Process existing files first
    process_batch(config)?;

    println!("---");
    println!("Waiting for new files...");

    let (tx, rx) = channel();

    let poll_interval = Duration::from_secs(config.watch.poll_interval_secs);
    let debounce = Duration::from_millis(config.watch.debounce_ms);

    let mut watcher =
        RecommendedWatcher::new(tx, NotifyConfig::default().with_poll_interval(poll_interval))
            .context("Failed to create watcher")?;

    let mode = if config.general.recursive {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    };

    watcher
        .watch(input_dir, mode)
        .context("Failed to start watching")?;

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                for path in event.paths {
                    if path.is_file() && FileFormat::is_supported(&path) {
                        // Wait to ensure file is completely written
                        std::thread::sleep(debounce);

                        let output = get_output_path(&path, input_dir, output_dir);

                        if let Err(e) = process_file(&path, &output, config) {
                            eprintln!("[ERROR] {}: {}", path.display(), e);
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("[WATCH ERROR] {:?}", e);
            }
            Err(e) => {
                eprintln!("[CHANNEL ERROR] {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
