use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};

use crate::config::Checksums;
use crate::download;
use crate::inject;
use crate::manifest::DistManifest;
use crate::utils;

pub fn run(path: &str) -> Result<()> {
    let target = PathBuf::from(path)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(path));

    println!(
        "{} DevTrail in {}",
        "Initializing".cyan().bold(),
        target.display()
    );

    // Check if already initialized
    if target.join(".devtrail").exists() {
        bail!(
            ".devtrail/ already exists. Use {} to update.",
            "devtrail update".yellow()
        );
    }

    // Download latest release
    utils::info("Fetching latest release...");
    let release = download::get_latest_release()?;
    println!(
        "  {} {}",
        "Found version:".dimmed(),
        release.tag_name.green()
    );

    // Download ZIP to temp file
    let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
    let zip_path = temp_dir.path().join("devtrail.zip");

    utils::info("Downloading...");
    download::download_zip(&release.zip_url, &zip_path)?;

    // Extract files according to manifest
    utils::info("Extracting files...");
    extract_distribution(&zip_path, &target)?;

    // Create empty directory structure with .gitkeep
    create_empty_dirs(&target)?;

    // Inject into directive files
    utils::info("Configuring AI agent directives...");
    inject_directives(&target)?;

    // Save checksums
    save_initial_checksums(&target, &release.tag_name)?;

    // Print summary
    println!();
    utils::success("DevTrail initialized successfully!");
    println!();
    println!("  {}", "Next steps:".bold());
    println!("    1. Review .devtrail/config.yml for language settings");
    println!("    2. Check DEVTRAIL.md for governance rules");
    println!(
        "    3. Run {} to validate your setup",
        "bash scripts/pre-commit-docs.sh".cyan()
    );
    println!(
        "    4. Commit: {}",
        "git add .devtrail/ DEVTRAIL.md && git commit -m \"chore: adopt DevTrail\"".dimmed()
    );

    Ok(())
}

/// Extract distributable files from the release ZIP
fn extract_distribution(zip_path: &Path, target: &Path) -> Result<()> {
    let file = std::fs::File::open(zip_path).context("Failed to open ZIP file")?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read ZIP archive")?;

    // Find the manifest inside the ZIP (it may be in a subdirectory like devtrail-v2.0.0/)
    let mut manifest_content = None;
    let mut prefix = String::new();

    // First pass: find the manifest entry index
    let mut manifest_index = None;
    for i in 0..archive.len() {
        let entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        if name.ends_with("dist-manifest.yml") {
            if let Some(pos) = name.find("dist-manifest.yml") {
                prefix = name[..pos].to_string();
            }
            manifest_index = Some(i);
            break;
        }
    }

    // Second pass: read manifest content
    if let Some(idx) = manifest_index {
        let mut content = String::new();
        let mut entry = archive.by_index(idx)?;
        std::io::Read::read_to_string(&mut entry, &mut content)?;
        manifest_content = Some(content);
    }

    let manifest_str = manifest_content.context("dist-manifest.yml not found in release ZIP")?;
    let manifest = DistManifest::from_str(&manifest_str)?;

    // Extract each file listed in manifest
    for pattern in &manifest.files {
        extract_matching_files(&mut archive, &prefix, pattern, target)?;
    }

    Ok(())
}

/// Extract files from ZIP matching a manifest pattern
fn extract_matching_files(
    archive: &mut zip::ZipArchive<std::fs::File>,
    prefix: &str,
    pattern: &str,
    target: &Path,
) -> Result<()> {
    let pattern_with_prefix = format!("{}{}", prefix, pattern);

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();

        // Check if this entry matches the pattern
        let matches = if pattern.ends_with('/') {
            // Directory pattern: match anything inside it
            name.starts_with(&pattern_with_prefix)
        } else {
            // Exact file match
            name == pattern_with_prefix
        };

        if matches && !entry.is_dir() {
            // Compute relative path (strip the prefix)
            let relative = &name[prefix.len()..];
            let dest = target.join(relative);

            // Create parent directories
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Write file
            let mut outfile = std::fs::File::create(&dest)?;
            std::io::copy(&mut entry, &mut outfile)?;
        }
    }

    Ok(())
}

/// Create the empty directory structure with .gitkeep files
fn create_empty_dirs(target: &Path) -> Result<()> {
    let dirs = [
        ".devtrail/01-requirements",
        ".devtrail/02-design/decisions",
        ".devtrail/03-implementation",
        ".devtrail/04-testing",
        ".devtrail/05-operations/incidents",
        ".devtrail/05-operations/runbooks",
        ".devtrail/06-evolution/technical-debt",
        ".devtrail/07-ai-audit/agent-logs",
        ".devtrail/07-ai-audit/decisions",
        ".devtrail/07-ai-audit/ethical-reviews",
        ".devtrail/00-governance/exceptions",
    ];

    for dir in &dirs {
        let dir_path = target.join(dir);
        utils::ensure_dir(&dir_path)?;
        let gitkeep = dir_path.join(".gitkeep");
        if !gitkeep.exists() {
            std::fs::write(&gitkeep, "")?;
        }
    }

    Ok(())
}

/// Inject DevTrail references into existing directive files
fn inject_directives(target: &Path) -> Result<()> {
    // Reference-based injection
    let reference_targets = [
        target.join("CLAUDE.md"),
        target.join("GEMINI.md"),
        target.join(".github/copilot-instructions.md"),
    ];

    for t in &reference_targets {
        inject::inject_reference(t)?;
        let name = t
            .strip_prefix(target)
            .unwrap_or(t)
            .display()
            .to_string();
        if t.exists() {
            utils::success(&format!("Configured {}", name));
        }
    }

    // Full content injection for .cursorrules
    let devtrail_md = target.join("DEVTRAIL.md");
    if devtrail_md.exists() {
        let content = std::fs::read_to_string(&devtrail_md)?;
        let cursorrules = target.join(".cursorrules");
        inject::inject_full_content(&cursorrules, &content)?;
        utils::success("Configured .cursorrules (inline)");

        // .cursor/rules/ directory
        let cursor_rules_dir = target.join(".cursor/rules");
        inject::inject_cursor_rules_dir(&cursor_rules_dir, &content)?;
        utils::success("Configured .cursor/rules/devtrail.md");
    }

    Ok(())
}

/// Save initial checksums for all framework files
fn save_initial_checksums(target: &Path, version: &str) -> Result<()> {
    let mut checksums = Checksums {
        version: version.to_string(),
        files: std::collections::HashMap::new(),
    };

    // Walk .devtrail/ and hash all files
    if let Ok(entries) = walkdir(target.join(".devtrail")) {
        for entry in entries {
            if let Some(hash) = utils::file_hash(&entry) {
                let relative = entry
                    .strip_prefix(target)
                    .unwrap_or(&entry)
                    .display()
                    .to_string();
                checksums.files.insert(relative, hash);
            }
        }
    }

    // Also hash DEVTRAIL.md
    let devtrail_path = target.join("DEVTRAIL.md");
    if let Some(hash) = utils::file_hash(&devtrail_path) {
        checksums.files.insert("DEVTRAIL.md".to_string(), hash);
    }

    checksums.save(target)?;
    Ok(())
}

/// Simple recursive directory walker
fn walkdir(dir: PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if !dir.is_dir() {
        return Ok(files);
    }

    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(walkdir(path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}
