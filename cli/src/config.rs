use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// DevTrail project configuration from .devtrail/config.yml
#[derive(Debug, Deserialize, Serialize)]
pub struct DevTrailConfig {
    /// Language setting: "en", "es", or "zh-CN"
    #[serde(default = "default_language")]
    pub language: String,
    /// Complexity analysis settings
    #[serde(default)]
    pub complexity: ComplexityConfig,
}

/// Configuration for the `devtrail analyze` command
#[derive(Debug, Deserialize, Serialize)]
pub struct ComplexityConfig {
    /// Cognitive complexity threshold (default: 8)
    #[serde(default = "default_threshold")]
    pub threshold: u32,
}

fn default_threshold() -> u32 {
    8
}

impl Default for ComplexityConfig {
    fn default() -> Self {
        Self {
            threshold: default_threshold(),
        }
    }
}

fn default_language() -> String {
    "en".to_string()
}

impl Default for DevTrailConfig {
    fn default() -> Self {
        Self {
            language: default_language(),
            complexity: ComplexityConfig::default(),
        }
    }
}

impl DevTrailConfig {
    /// Read config from .devtrail/config.yml at the given project root
    pub fn load(project_root: &Path) -> Result<Self> {
        let config_path = project_root.join(".devtrail/config.yml");
        if !config_path.exists() {
            return Ok(Self::default());
        }
        let contents =
            std::fs::read_to_string(&config_path).context("Failed to read config.yml")?;
        let config: Self = serde_yaml::from_str(&contents).context("Failed to parse config.yml")?;
        Ok(config)
    }
}

/// Checksums tracking file for detecting user modifications
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Checksums {
    pub version: String,
    pub files: std::collections::HashMap<String, String>,
}

impl Checksums {
    pub fn load(project_root: &Path) -> Result<Self> {
        let path = project_root.join(".devtrail/.checksums.json");
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents =
            std::fs::read_to_string(&path).context("Failed to read .checksums.json")?;
        let checksums: Self =
            serde_json::from_str(&contents).context("Failed to parse .checksums.json")?;
        Ok(checksums)
    }

    pub fn save(&self, project_root: &Path) -> Result<()> {
        let path = project_root.join(".devtrail/.checksums.json");
        let contents =
            serde_json::to_string_pretty(self).context("Failed to serialize checksums")?;
        std::fs::write(&path, contents).context("Failed to write .checksums.json")?;
        Ok(())
    }
}
