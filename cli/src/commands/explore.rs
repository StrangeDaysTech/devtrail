use anyhow::{bail, Result};
use std::path::PathBuf;

use crate::utils;

pub fn run(path: &str) -> Result<()> {
    let target = PathBuf::from(path)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(path));

    let devtrail_dir = target.join(".devtrail");

    if !devtrail_dir.exists() {
        utils::warn(&format!(
            "DevTrail is not initialized in {}",
            target.display()
        ));
        utils::info("Run 'devtrail init' to initialize DevTrail in this directory.");
        bail!("No DevTrail installation found");
    }

    crate::tui::run(&target)
}
