use colored::Colorize;
use sha2::{Digest, Sha256};
use std::path::Path;

/// Print a success message
pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

/// Print an info message
pub fn info(msg: &str) {
    println!("{} {}", "→".blue().bold(), msg);
}

/// Print a warning message
pub fn warn(msg: &str) {
    println!("{} {}", "!".yellow().bold(), msg);
}

/// Compute SHA-256 hash of a file's contents
pub fn file_hash(path: &Path) -> Option<String> {
    let content = std::fs::read(path).ok()?;
    let hash = Sha256::digest(&content);
    Some(format!("{:x}", hash))
}

/// Check if a path looks like a user-generated DevTrail document
/// (matches pattern: *-YYYY-MM-DD-NNN-*.md)
pub fn is_user_document(path: &Path) -> bool {
    let name = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return false,
    };

    // Check for patterns like AILOG-2025-01-27-001-description.md
    let prefixes = [
        "AILOG-", "AIDEC-", "ETH-", "ADR-", "REQ-", "TES-", "INC-", "TDE-",
    ];

    prefixes.iter().any(|p| name.starts_with(p))
}

/// Ensure a directory exists, creating it if needed
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
