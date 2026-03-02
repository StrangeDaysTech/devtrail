use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

/// Parsed dist-manifest.yml
#[derive(Debug, Deserialize)]
pub struct DistManifest {
    pub version: String,
    pub description: String,
    pub files: Vec<String>,
    #[serde(default)]
    pub injections: Option<Injections>,
}

#[derive(Debug, Deserialize)]
pub struct Injections {
    #[serde(default)]
    pub reference: Vec<String>,
    #[serde(default)]
    pub inline: Vec<String>,
    #[serde(default)]
    pub directory: Vec<String>,
}

impl DistManifest {
    /// Load manifest from within a ZIP archive's extracted directory
    pub fn from_str(content: &str) -> Result<Self> {
        serde_yaml::from_str(content).context("Failed to parse dist-manifest.yml")
    }

    /// Load manifest from a file path
    pub fn load(path: &Path) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).context("Failed to read dist-manifest.yml")?;
        Self::from_str(&content)
    }
}
