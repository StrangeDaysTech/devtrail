use anyhow::{bail, Context, Result};
use colored::Colorize;
use dialoguer::{Select, theme::ColorfulTheme};
use std::path::{Path, PathBuf};

use crate::config::Checksums;
use crate::download;
use crate::inject;
use crate::manifest::DistManifest;
use crate::self_update;
use crate::utils;

pub fn run() -> Result<()> {
    let target = std::env::current_dir().context("Failed to get current directory")?;

    // Verify DevTrail is installed
    if !target.join(".devtrail").exists() {
        bail!(
            ".devtrail/ not found. Use {} to initialize first.",
            "devtrail init".yellow()
        );
    }

    println!("{} DevTrail...", "Updating".cyan().bold());

    // Load current checksums
    let current_checksums = Checksums::load(&target)?;
    if !current_checksums.version.is_empty() {
        println!(
            "  {} {}",
            "Current version:".dimmed(),
            current_checksums.version
        );
    }

    // Fetch latest release
    utils::info("Checking for updates...");
    let release = download::get_latest_release()?;
    println!(
        "  {} {}",
        "Latest version:".dimmed(),
        release.tag_name.green()
    );

    // Download ZIP
    let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
    let zip_path = temp_dir.path().join("devtrail.zip");

    utils::info("Downloading...");
    download::download_zip(&release.zip_url, &zip_path)?;

    // Extract to temp directory for comparison
    let extract_dir = temp_dir.path().join("extracted");
    std::fs::create_dir_all(&extract_dir)?;
    extract_all(&zip_path, &extract_dir)?;

    // Find source root within extracted content
    let source_root = find_source_root(&extract_dir)?;

    // Load manifest from extracted release
    let manifest = DistManifest::load(&source_root.join("dist-manifest.yml"))?;

    // Update framework files
    utils::info("Updating framework files...");
    let stats = update_files(&target, &source_root, &current_checksums)?;

    // Update directive injections
    utils::info("Updating AI agent directives...");
    inject_directives(&target, &source_root, &manifest)?;

    // Save manifest locally for future remove operations
    save_local_manifest(&target, &manifest)?;

    // Save new checksums
    save_checksums(&target, &release.tag_name)?;

    // Print summary
    println!();
    utils::success("DevTrail updated successfully!");
    println!("  Files updated: {}", stats.updated);
    println!("  Files skipped (user-modified): {}", stats.skipped);
    println!("  Files added: {}", stats.added);

    // Notify about CLI updates
    self_update::notify_if_newer(&release.tag_name);

    Ok(())
}

struct UpdateStats {
    updated: usize,
    skipped: usize,
    added: usize,
}

/// Update files, respecting user modifications
fn update_files(
    target: &Path,
    source_root: &Path,
    checksums: &Checksums,
) -> Result<UpdateStats> {
    let mut stats = UpdateStats {
        updated: 0,
        skipped: 0,
        added: 0,
    };

    // Walk extracted files
    let entries = walkdir(source_root.to_path_buf())?;

    for source_path in entries {
        let relative = source_path
            .strip_prefix(source_root)
            .unwrap_or(&source_path)
            .display()
            .to_string();

        // Skip user-generated documents
        if utils::is_user_document(&source_path) {
            continue;
        }

        // Skip checksums file
        if relative == ".devtrail/.checksums.json" {
            continue;
        }

        // Skip dist-manifest.yml (we save it separately)
        if relative == ".devtrail/dist-manifest.yml" {
            continue;
        }

        let target_path = target.join(&relative);

        if !target_path.exists() {
            // New file — just copy it
            if let Some(parent) = target_path.parent() {
                utils::ensure_dir(parent)?;
            }
            std::fs::copy(&source_path, &target_path)?;
            stats.added += 1;
            continue;
        }

        // File exists — check if user modified it
        let current_hash = utils::file_hash(&target_path).unwrap_or_default();
        let original_hash = checksums
            .files
            .get(&relative)
            .cloned()
            .unwrap_or_default();

        if current_hash == original_hash || original_hash.is_empty() {
            // User hasn't modified it (or no previous hash) — safe to overwrite
            std::fs::copy(&source_path, &target_path)?;
            stats.updated += 1;
        } else {
            // User modified it — prompt for action
            let new_hash = utils::file_hash(&source_path).unwrap_or_default();
            if current_hash == new_hash {
                // Same content, no action needed
                continue;
            }

            utils::warn(&format!("User-modified file: {}", relative));
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What would you like to do?")
                .items(&["Keep my version", "Use new version", "Backup mine + use new"])
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    stats.skipped += 1;
                }
                1 => {
                    std::fs::copy(&source_path, &target_path)?;
                    stats.updated += 1;
                }
                2 => {
                    let backup = target_path.with_extension("md.bak");
                    std::fs::copy(&target_path, &backup)?;
                    std::fs::copy(&source_path, &target_path)?;
                    stats.updated += 1;
                    utils::info(&format!("Backup saved: {}", backup.display()));
                }
                _ => {
                    stats.skipped += 1;
                }
            }
        }
    }

    Ok(stats)
}

/// Inject directives based on manifest and templates from the release
fn inject_directives(target: &Path, source_root: &Path, manifest: &DistManifest) -> Result<()> {
    for injection in &manifest.injections {
        let target_path = target.join(&injection.target);

        // In update mode, only update targets that already exist
        if !target_path.exists() {
            continue;
        }

        let template_path = source_root.join(&injection.template);
        let template_content = match std::fs::read_to_string(&template_path) {
            Ok(content) => content,
            Err(_) => {
                utils::warn(&format!(
                    "Template not found: {}",
                    injection.template
                ));
                continue;
            }
        };

        let embed_content = if let Some(embed_file) = &injection.embed {
            // Use the embed file from the release, not the local one
            let embed_path = source_root.join(embed_file);
            if embed_path.exists() {
                Some(std::fs::read_to_string(&embed_path).with_context(|| {
                    format!("Failed to read embed file: {}", embed_path.display())
                })?)
            } else {
                utils::warn(&format!(
                    "Embed file not found in release: {} (skipping {})",
                    embed_file, injection.target
                ));
                continue;
            }
        } else {
            None
        };

        inject::inject_directive(&target_path, &template_content, embed_content.as_deref())?;
    }

    Ok(())
}

/// Save the manifest locally for future remove operations
fn save_local_manifest(target: &Path, manifest: &DistManifest) -> Result<()> {
    let manifest_path = target.join(".devtrail/dist-manifest.yml");
    let content = manifest.to_yaml()?;
    std::fs::write(&manifest_path, content)
        .context("Failed to save local dist-manifest.yml")?;
    Ok(())
}

fn save_checksums(target: &Path, version: &str) -> Result<()> {
    let mut checksums = Checksums {
        version: version.to_string(),
        files: std::collections::HashMap::new(),
    };

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

    let devtrail_path = target.join("DEVTRAIL.md");
    if let Some(hash) = utils::file_hash(&devtrail_path) {
        checksums.files.insert("DEVTRAIL.md".to_string(), hash);
    }

    checksums.save(target)?;
    Ok(())
}

fn extract_all(zip_path: &Path, dest: &Path) -> Result<()> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let path = dest.join(entry.name());

        if entry.is_dir() {
            std::fs::create_dir_all(&path)?;
        } else {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = std::fs::File::create(&path)?;
            std::io::copy(&mut entry, &mut outfile)?;
        }
    }

    Ok(())
}

fn find_source_root(extract_dir: &Path) -> Result<PathBuf> {
    // Check if dist-manifest.yml is directly in extract_dir
    if extract_dir.join("dist-manifest.yml").exists() {
        return Ok(extract_dir.to_path_buf());
    }

    // Check one level deep (GitHub ZIP archives nest in a directory)
    for entry in std::fs::read_dir(extract_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.join("dist-manifest.yml").exists() {
            return Ok(path);
        }
    }

    bail!("Could not find dist-manifest.yml in extracted archive");
}

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
