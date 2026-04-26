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
    /// Regional regulatory scope. Values: "global" (NIST + ISO 42001),
    /// "eu" (EU AI Act + GDPR), "china" (TC260, PIPL, GB 45438, CAC, GB/T 45652, CSL).
    /// Default `["global", "eu"]` preserves backward compatibility.
    #[serde(default = "default_regional_scope")]
    pub regional_scope: Vec<String>,
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

fn default_regional_scope() -> Vec<String> {
    vec!["global".to_string(), "eu".to_string()]
}

impl Default for DevTrailConfig {
    fn default() -> Self {
        Self {
            language: default_language(),
            complexity: ComplexityConfig::default(),
            regional_scope: default_regional_scope(),
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

    /// True if `regional_scope` includes the given region (case-insensitive).
    pub fn has_region(&self, region: &str) -> bool {
        self.regional_scope
            .iter()
            .any(|r| r.eq_ignore_ascii_case(region))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_regional_scope() {
        let cfg = DevTrailConfig::default();
        assert!(cfg.has_region("global"));
        assert!(cfg.has_region("eu"));
        assert!(!cfg.has_region("china"));
    }

    #[test]
    fn test_has_region_case_insensitive() {
        let cfg = DevTrailConfig {
            regional_scope: vec!["China".into(), "GLOBAL".into()],
            ..Default::default()
        };
        assert!(cfg.has_region("china"));
        assert!(cfg.has_region("CHINA"));
        assert!(cfg.has_region("global"));
        assert!(!cfg.has_region("eu"));
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
