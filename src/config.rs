use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub search: SearchConfig,
    pub cache: CacheConfig,
    pub ui: UiConfig,
    pub preview: PreviewConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub base_path: String,
    pub max_depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub prompt: String,
    pub header: String,
    pub preview_width_percent: u8,
    pub layout: String,
    pub height_percent: u8,
    pub show_border: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewConfig {
    pub show_branch: bool,
    pub show_last_activity: bool,
    pub show_status: bool,
    pub recent_commits: usize,
    pub date_format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            search: SearchConfig {
                base_path: dirs::home_dir()
                    .and_then(|p| p.to_str().map(String::from))
                    .unwrap_or_else(|| String::from("~")),
                max_depth: 5,
            },
            cache: CacheConfig {
                enabled: true,
                ttl_seconds: 300,
            },
            ui: UiConfig {
                prompt: String::from("Select repo > "),
                header: String::from("Repository (↑/↓, ⏎, Esc)"),
                preview_width_percent: 60,
                layout: String::from("reverse"),
                height_percent: 90,
                show_border: true,
            },
            preview: PreviewConfig {
                show_branch: true,
                show_last_activity: true,
                show_status: true,
                recent_commits: 5,
                date_format: String::from("%Y-%m-%d %H:%M"),
            },
        }
    }
}

impl Config {
    /// Load config from a file path, falling back to default if file doesn't exist
    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Get the default config file path (~/.config/gitnav/config.toml)
    pub fn default_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("gitnav").join("config.toml"))
    }

    /// Load config with priority: custom_path > default_path > built-in defaults
    pub fn load(custom_path: Option<PathBuf>) -> Result<Self> {
        if let Some(path) = custom_path {
            return Self::load_from_file(&path);
        }

        if let Some(default_path) = Self::default_path() {
            return Self::load_from_file(&default_path);
        }

        Ok(Self::default())
    }

    /// Generate an example config file as a string
    pub fn example_toml() -> String {
        let default = Self::default();
        toml::to_string_pretty(&default).unwrap_or_else(|_| String::from("# Error generating example config"))
    }
}
