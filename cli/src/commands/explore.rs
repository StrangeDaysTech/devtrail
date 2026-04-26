use anyhow::{bail, Result};

use crate::config::DevTrailConfig;
use crate::utils;

pub fn run(path: &str, lang_override: Option<&str>) -> Result<()> {
    let resolved = match utils::resolve_project_root(path) {
        Some(r) => r,
        None => {
            utils::warn("DevTrail is not initialized in this directory or repo root.");
            utils::info("Run 'devtrail init' to initialize DevTrail.");
            bail!("No DevTrail installation found");
        }
    };

    // Effective language: --lang flag > config.language > "en".
    let language = match lang_override {
        Some(l) => l.to_string(),
        None => DevTrailConfig::load(&resolved.path)
            .map(|c| c.language)
            .unwrap_or_else(|_| "en".to_string()),
    };

    crate::tui::run(&resolved.path, resolved.is_fallback, &language)
}
