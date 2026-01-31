use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG_NAME: &str = "config.toml";
const DEFAULT_POLL_INTERVAL_SECS: u64 = 2;
const DEFAULT_DEBOUNCE_MS: u64 = 500;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub watch: WatchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_input_dir")]
    pub input_dir: PathBuf,

    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

    #[serde(default)]
    pub recursive: bool,

    #[serde(default)]
    pub delete_source: bool,

    #[serde(default)]
    pub default_sheet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchConfig {
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,

    #[serde(default = "default_debounce_ms")]
    pub debounce_ms: u64,
}

fn default_input_dir() -> PathBuf {
    PathBuf::from("./input")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("./output")
}

fn default_poll_interval() -> u64 {
    DEFAULT_POLL_INTERVAL_SECS
}

fn default_debounce_ms() -> u64 {
    DEFAULT_DEBOUNCE_MS
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            input_dir: default_input_dir(),
            output_dir: default_output_dir(),
            recursive: false,
            delete_source: false,
            default_sheet: None,
        }
    }
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            poll_interval_secs: DEFAULT_POLL_INTERVAL_SECS,
            debounce_ms: DEFAULT_DEBOUNCE_MS,
        }
    }
}

impl GeneralConfig {
    pub fn apply_overrides(
        &mut self,
        input_dir: Option<PathBuf>,
        output_dir: Option<PathBuf>,
        delete_source: bool,
        recursive: bool,
    ) {
        if let Some(dir) = input_dir {
            self.input_dir = dir;
        }
        if let Some(dir) = output_dir {
            self.output_dir = dir;
        }
        if delete_source {
            self.delete_source = true;
        }
        if recursive {
            self.recursive = true;
        }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let config_path = path
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_NAME));

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

            toml::from_str(&content).context("Failed to parse config file")
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content =
            toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to save config to: {:?}", path))
    }

    pub fn generate_default(path: &Path) -> Result<()> {
        Self::default().save(path)
    }
}
