use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure for gitnav.
///
/// Contains all configuration options organized into nested structures
/// for search behavior, caching, UI, and preview settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub search: SearchConfig,
    pub cache: CacheConfig,
    pub ui: UiConfig,
    pub preview: PreviewConfig,
}

/// Configuration for repository search behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Base path to start searching from (supports ~ expansion)
    pub base_path: String,
    /// Maximum directory depth to traverse
    pub max_depth: usize,
}

/// Configuration for caching behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Whether caching is enabled
    pub enabled: bool,
    /// Time-to-live for cached data in seconds
    pub ttl_seconds: u64,
}

/// Configuration for the fzf UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// The prompt displayed to the user
    pub prompt: String,
    /// Header text shown in the fzf interface
    pub header: String,
    /// Width of the preview pane as a percentage (0-100)
    pub preview_width_percent: u8,
    /// Layout style for fzf (e.g., "reverse", "default")
    pub layout: String,
    /// Height of the fzf window as a percentage (1-100)
    pub height_percent: u8,
    /// Whether to show a border around the fzf window
    pub show_border: bool,
}

/// Configuration for repository preview display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewConfig {
    /// Whether to show the current branch
    pub show_branch: bool,
    /// Whether to show the last activity (most recent commit)
    pub show_last_activity: bool,
    /// Whether to show the working tree status
    pub show_status: bool,
    /// Number of recent commits to display
    pub recent_commits: usize,
    /// Date format string for timestamps (strftime format)
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
    /// Load configuration from a file path, using defaults if file doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file
    ///
    /// # Returns
    ///
    /// Loaded configuration or defaults if file doesn't exist
    ///
    /// # Errors
    ///
    /// Returns an error if the file exists but cannot be read or parsed
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

    /// Get the default configuration file paths in priority order.
    ///
    /// Returns a list of potential config paths to check in order:
    /// 1. Home directory `.config/gitnav/config.toml` (cross-platform compatibility)
    /// 2. Platform-specific config directory (XDG_CONFIG_HOME on Linux, ~/Library/Application Support on macOS, %APPDATA% on Windows)
    ///
    /// # Returns
    ///
    /// A vector of config paths to check, in priority order
    fn default_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // Add cross-platform ~/.config/gitnav/config.toml first (highest priority)
        if let Some(home_dir) = dirs::home_dir() {
            paths.push(home_dir.join(".config").join("gitnav").join("config.toml"));
        }

        // Add platform-specific config directory as fallback
        if let Some(config_dir) = dirs::config_dir() {
            paths.push(config_dir.join("gitnav").join("config.toml"));
        }

        paths
    }

    /// Get the default configuration file path.
    ///
    /// Returns the first valid path from the default paths list.
    ///
    /// # Returns
    ///
    /// The default config path or `None` if no config directory can be determined
    pub fn default_path() -> Option<PathBuf> {
        Self::default_paths().into_iter().next()
    }

    /// Load configuration with priority: env > custom > default > built-in defaults.
    ///
    /// Configuration is loaded in the following order:
    /// 1. Built-in defaults
    /// 2. Default paths in priority order (`~/.config/gitnav/config.toml`, then platform-specific)
    /// 3. Custom path (if provided)
    /// 4. Environment variables (override everything)
    ///
    /// # Arguments
    ///
    /// * `custom_path` - Optional custom configuration file path
    ///
    /// # Returns
    ///
    /// The loaded configuration
    ///
    /// # Errors
    ///
    /// Returns an error if a specified config file cannot be read or parsed
    pub fn load(custom_path: Option<PathBuf>) -> Result<Self> {
        let mut config = Self::default();

        // Load from first available default path
        for default_path in Self::default_paths() {
            if let Ok(loaded) = Self::load_from_file(&default_path) {
                config = loaded;
                break;
            }
        }

        // Load from custom path if provided
        if let Some(path) = custom_path {
            config = Self::load_from_file(&path)?;
        }

        // Override with environment variables
        config.apply_env_vars();

        Ok(config)
    }

    /// Apply environment variable overrides to configuration.
    ///
    /// Supports the following environment variables:
    /// - GITNAV_BASE_PATH: Base search path
    /// - GITNAV_MAX_DEPTH: Maximum search depth
    /// - GITNAV_CACHE_ENABLED: Cache enabled (true/false)
    /// - GITNAV_CACHE_TTL: Cache TTL in seconds
    /// - GITNAV_UI_PROMPT: FZF prompt text
    /// - GITNAV_UI_HEADER: FZF header text
    /// - GITNAV_UI_PREVIEW_WIDTH: Preview pane width (0-100)
    /// - GITNAV_UI_LAYOUT: FZF layout style
    /// - GITNAV_UI_HEIGHT: FZF window height (1-100)
    /// - GITNAV_UI_BORDER: Show border (true/false)
    /// - GITNAV_PREVIEW_SHOW_BRANCH: Show branch info (true/false)
    /// - GITNAV_PREVIEW_SHOW_ACTIVITY: Show last activity (true/false)
    /// - GITNAV_PREVIEW_SHOW_STATUS: Show status (true/false)
    /// - GITNAV_PREVIEW_RECENT_COMMITS: Number of recent commits to show
    /// - GITNAV_PREVIEW_DATE_FORMAT: Date format string (strftime format)
    fn apply_env_vars(&mut self) {
        // Search configuration
        if let Ok(val) = std::env::var("GITNAV_BASE_PATH") {
            self.search.base_path = val;
        }
        if let Ok(val) = std::env::var("GITNAV_MAX_DEPTH") {
            if let Ok(depth) = val.parse::<usize>() {
                self.search.max_depth = depth;
            }
        }

        // Cache configuration
        if let Ok(val) = std::env::var("GITNAV_CACHE_ENABLED") {
            self.cache.enabled = val.to_lowercase() == "true" || val == "1" || val == "yes";
        }
        if let Ok(val) = std::env::var("GITNAV_CACHE_TTL") {
            if let Ok(ttl) = val.parse::<u64>() {
                self.cache.ttl_seconds = ttl;
            }
        }

        // UI configuration
        if let Ok(val) = std::env::var("GITNAV_UI_PROMPT") {
            self.ui.prompt = val;
        }
        if let Ok(val) = std::env::var("GITNAV_UI_HEADER") {
            self.ui.header = val;
        }
        if let Ok(val) = std::env::var("GITNAV_UI_PREVIEW_WIDTH") {
            if let Ok(width) = val.parse::<u8>() {
                if width <= 100 {
                    self.ui.preview_width_percent = width;
                }
            }
        }
        if let Ok(val) = std::env::var("GITNAV_UI_LAYOUT") {
            self.ui.layout = val;
        }
        if let Ok(val) = std::env::var("GITNAV_UI_HEIGHT") {
            if let Ok(height) = val.parse::<u8>() {
                if height > 0 && height <= 100 {
                    self.ui.height_percent = height;
                }
            }
        }
        if let Ok(val) = std::env::var("GITNAV_UI_BORDER") {
            self.ui.show_border = val.to_lowercase() == "true" || val == "1" || val == "yes";
        }

        // Preview configuration
        if let Ok(val) = std::env::var("GITNAV_PREVIEW_SHOW_BRANCH") {
            self.preview.show_branch = val.to_lowercase() == "true" || val == "1" || val == "yes";
        }
        if let Ok(val) = std::env::var("GITNAV_PREVIEW_SHOW_ACTIVITY") {
            self.preview.show_last_activity = val.to_lowercase() == "true" || val == "1" || val == "yes";
        }
        if let Ok(val) = std::env::var("GITNAV_PREVIEW_SHOW_STATUS") {
            self.preview.show_status = val.to_lowercase() == "true" || val == "1" || val == "yes";
        }
        if let Ok(val) = std::env::var("GITNAV_PREVIEW_RECENT_COMMITS") {
            if let Ok(commits) = val.parse::<usize>() {
                self.preview.recent_commits = commits;
            }
        }
        if let Ok(val) = std::env::var("GITNAV_PREVIEW_DATE_FORMAT") {
            self.preview.date_format = val;
        }
    }

    /// Validate configuration values for correctness
    ///
    /// # Errors
    /// Returns an error if any configuration values are invalid
    pub fn validate(&self) -> Result<()> {
        if self.search.max_depth == 0 {
            anyhow::bail!("search.max_depth must be at least 1, got 0");
        }

        if self.ui.preview_width_percent > 100 {
            anyhow::bail!(
                "ui.preview_width_percent cannot exceed 100, got {}",
                self.ui.preview_width_percent
            );
        }

        if self.ui.height_percent > 100 {
            anyhow::bail!(
                "ui.height_percent cannot exceed 100, got {}",
                self.ui.height_percent
            );
        }

        if self.ui.height_percent == 0 {
            anyhow::bail!("ui.height_percent must be at least 1, got 0");
        }

        Ok(())
    }

    /// Generate an example configuration file as a TOML string.
    ///
    /// Used by `gitnav config` command to show users an example configuration.
    ///
    /// # Returns
    ///
    /// A TOML string representation of the default configuration
    pub fn example_toml() -> String {
        let default = Self::default();
        toml::to_string_pretty(&default).unwrap_or_else(|_| String::from("# Error generating example config"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_sensible_values() {
        let config = Config::default();
        assert_eq!(config.search.max_depth, 5);
        assert!(config.cache.enabled);
        assert_eq!(config.cache.ttl_seconds, 300);
        assert_eq!(config.ui.preview_width_percent, 60);
        assert_eq!(config.ui.height_percent, 90);
        assert_eq!(config.preview.recent_commits, 5);
    }

    #[test]
    fn test_default_config_is_valid() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_rejects_zero_max_depth() {
        let mut config = Config::default();
        config.search.max_depth = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_depth must be at least 1"));
    }

    #[test]
    fn test_validation_rejects_preview_width_over_100() {
        let mut config = Config::default();
        config.ui.preview_width_percent = 101;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("preview_width_percent cannot exceed 100"));
    }

    #[test]
    fn test_validation_rejects_height_percent_over_100() {
        let mut config = Config::default();
        config.ui.height_percent = 101;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("height_percent cannot exceed 100"));
    }

    #[test]
    fn test_validation_rejects_zero_height_percent() {
        let mut config = Config::default();
        config.ui.height_percent = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("height_percent must be at least 1"));
    }

    #[test]
    fn test_validation_accepts_boundary_values() {
        let mut config = Config::default();
        config.search.max_depth = 1;
        config.ui.preview_width_percent = 100;
        config.ui.height_percent = 100;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_search_config_serialize_deserialize() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).expect("Failed to serialize");
        let parsed: Config = toml::from_str(&toml_str).expect("Failed to parse");

        assert_eq!(parsed.search.max_depth, config.search.max_depth);
        assert_eq!(parsed.cache.ttl_seconds, config.cache.ttl_seconds);
    }

    #[test]
    fn test_cache_config_defaults() {
        let config = Config::default();
        assert!(config.cache.enabled);
        assert_eq!(config.cache.ttl_seconds, 300);
    }

    #[test]
    fn test_ui_config_defaults() {
        let config = Config::default();
        assert!(!config.ui.prompt.is_empty());
        assert!(!config.ui.header.is_empty());
        assert!(config.ui.show_border);
        assert_eq!(config.ui.layout, "reverse");
    }

    #[test]
    fn test_preview_config_defaults() {
        let config = Config::default();
        assert!(config.preview.show_branch);
        assert!(config.preview.show_last_activity);
        assert!(config.preview.show_status);
        assert_eq!(config.preview.recent_commits, 5);
        assert!(!config.preview.date_format.is_empty());
    }

    #[test]
    fn test_example_toml_generates_valid_config() {
        let example_toml = Config::example_toml();
        assert!(!example_toml.is_empty());
        assert!(example_toml.contains("[search]"));
        assert!(example_toml.contains("[cache]"));
        assert!(example_toml.contains("[ui]"));
        assert!(example_toml.contains("[preview]"));

        // Verify it can be parsed back
        let parsed: Result<Config, _> = toml::from_str(&example_toml);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_load_from_nonexistent_file_returns_defaults() {
        let nonexistent_path = PathBuf::from("/nonexistent/path/config.toml");
        let config = Config::load_from_file(&nonexistent_path).expect("Should return defaults");
        assert_eq!(config.search.max_depth, 5);
        assert!(config.cache.enabled);
    }

    #[test]
    fn test_config_is_cloneable() {
        let config1 = Config::default();
        let config2 = config1.clone();

        assert_eq!(config1.search.max_depth, config2.search.max_depth);
        assert_eq!(config1.cache.ttl_seconds, config2.cache.ttl_seconds);
    }

    #[test]
    fn test_config_is_debuggable() {
        let config = Config::default();
        let debug_str = format!("{:?}", config);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("max_depth"));
    }

    #[test]
    fn test_nested_config_defaults() {
        let config = Config::default();

        // Search config
        assert!(!config.search.base_path.is_empty());
        assert!(config.search.max_depth > 0);

        // Cache config
        assert!(config.cache.enabled);
        assert!(config.cache.ttl_seconds > 0);

        // UI config
        assert!(!config.ui.prompt.is_empty());
        assert!(!config.ui.header.is_empty());
        assert!(config.ui.preview_width_percent > 0);
        assert!(config.ui.preview_width_percent <= 100);
        assert!(config.ui.height_percent > 0);
        assert!(config.ui.height_percent <= 100);

        // Preview config
        assert!(config.preview.recent_commits > 0);
        assert!(!config.preview.date_format.is_empty());
    }

    #[test]
    fn test_all_config_sections_present() {
        let config = Config::default();

        // Verify all major sections exist
        let _ = &config.search;
        let _ = &config.cache;
        let _ = &config.ui;
        let _ = &config.preview;
    }

    #[test]
    fn test_validation_with_minimum_valid_values() {
        let mut config = Config::default();
        config.search.max_depth = 1;
        config.ui.preview_width_percent = 0;
        config.ui.height_percent = 1;

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_with_maximum_valid_values() {
        let mut config = Config::default();
        config.search.max_depth = 1000;
        config.ui.preview_width_percent = 100;
        config.ui.height_percent = 100;

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_multiple_violations() {
        let mut config = Config::default();
        config.search.max_depth = 0;
        config.ui.preview_width_percent = 101;
        config.ui.height_percent = 101;

        // First violation should be caught
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_example_toml_contains_all_sections() {
        let example = Config::example_toml();

        assert!(example.contains("[search]"));
        assert!(example.contains("[cache]"));
        assert!(example.contains("[ui]"));
        assert!(example.contains("[preview]"));
    }

    #[test]
    fn test_example_toml_contains_all_fields() {
        let example = Config::example_toml();

        // Search fields
        assert!(example.contains("base_path"));
        assert!(example.contains("max_depth"));

        // Cache fields
        assert!(example.contains("enabled"));
        assert!(example.contains("ttl_seconds"));

        // UI fields
        assert!(example.contains("prompt"));
        assert!(example.contains("header"));
        assert!(example.contains("preview_width_percent"));
        assert!(example.contains("layout"));
        assert!(example.contains("height_percent"));
        assert!(example.contains("show_border"));

        // Preview fields
        assert!(example.contains("show_branch"));
        assert!(example.contains("show_last_activity"));
        assert!(example.contains("show_status"));
        assert!(example.contains("recent_commits"));
        assert!(example.contains("date_format"));
    }

    #[test]
    fn test_search_config_base_path_not_empty() {
        let config = Config::default();
        assert!(!config.search.base_path.is_empty());
        assert!(!config.search.base_path.trim().is_empty());
    }

    #[test]
    fn test_ui_config_show_border_default() {
        let config = Config::default();
        assert!(config.ui.show_border);
    }

    #[test]
    fn test_preview_config_all_features_enabled_by_default() {
        let config = Config::default();
        assert!(config.preview.show_branch);
        assert!(config.preview.show_last_activity);
        assert!(config.preview.show_status);
    }

    #[test]
    fn test_config_validation_returns_correct_error_messages() {
        let mut config = Config::default();

        // Test max_depth error message
        config.search.max_depth = 0;
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("max_depth"));
        assert!(err.to_string().contains("at least 1"));

        // Test preview_width error message
        config.search.max_depth = 5;
        config.ui.preview_width_percent = 150;
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("preview_width_percent"));
        assert!(err.to_string().contains("exceed"));

        // Test height_percent error message
        config.ui.preview_width_percent = 60;
        config.ui.height_percent = 0;
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("height_percent"));
        assert!(err.to_string().contains("at least 1"));
    }
}
