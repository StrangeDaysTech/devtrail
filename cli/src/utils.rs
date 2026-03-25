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
        "SEC-", "MCARD-", "SBOM-", "DPIA-",
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

/// Result of resolving the DevTrail project root
pub struct ResolvedPath {
    /// The resolved project root where .devtrail/ exists
    pub path: std::path::PathBuf,
    /// Whether we fell back to the git repo root (not the original path)
    pub is_fallback: bool,
}

/// Resolve the DevTrail project root from a given path.
///
/// 1. If `path` has `.devtrail/`, use it directly
/// 2. If not, try the git repo root
/// 3. If neither has `.devtrail/`, return None
pub fn resolve_project_root(path: &str) -> Option<ResolvedPath> {
    let target = std::path::PathBuf::from(path)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(path));

    // Check the given path first
    if target.join(".devtrail").exists() {
        return Some(ResolvedPath {
            path: target,
            is_fallback: false,
        });
    }

    // Try git repo root
    let git_root = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(&target)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Some(std::path::PathBuf::from(root))
            } else {
                None
            }
        });

    if let Some(root) = git_root {
        // Don't fallback to the same path we already checked
        if root != target && root.join(".devtrail").exists() {
            return Some(ResolvedPath {
                path: root,
                is_fallback: true,
            });
        }
    }

    None
}
