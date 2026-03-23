use anyhow::{bail, Result};

use crate::utils;

pub fn run(path: &str) -> Result<()> {
    let resolved = match utils::resolve_project_root(path) {
        Some(r) => r,
        None => {
            utils::warn("DevTrail is not initialized in this directory or repo root.");
            utils::info("Run 'devtrail init' to initialize DevTrail.");
            bail!("No DevTrail installation found");
        }
    };

    crate::tui::run(&resolved.path, resolved.is_fallback)
}
