use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};

use crate::download;
use crate::platform;
use crate::utils;

/// Perform the CLI self-update
pub fn perform_update() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    utils::info(&format!("Current version: cli-{}", current_version));

    // Fetch latest release
    utils::info("Checking for updates...");
    let release = download::get_latest_release_full()?;
    let tag_version = download::strip_tag_prefix(&release.tag_name);

    println!(
        "  {} {}",
        "Latest version:".dimmed(),
        release.tag_name.green()
    );

    // Compare versions
    let current =
        semver::Version::parse(current_version).context("Failed to parse current version")?;
    let latest =
        semver::Version::parse(tag_version).context("Failed to parse release version")?;

    if latest <= current {
        utils::success(&format!(
            "CLI is already at the latest version (cli-{})",
            current_version
        ));
        return Ok(());
    }

    // Detect platform
    let target = platform::current_target()?;
    println!("  {} {}", "Detected platform:".dimmed(), target);

    // Find matching asset
    let expected_name = platform::asset_name(tag_version)?;
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == expected_name)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No CLI binary found for {} in release cli-{}",
                target,
                tag_version
            )
        })?;

    // Download to temp directory
    let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
    let archive_path = temp_dir.path().join(&asset.name);

    utils::info(&format!("Downloading {}...", asset.name));
    download::download_file(&asset.download_url, &archive_path, &asset.name)?;

    // Extract binary
    utils::info("Extracting binary...");
    let binary_name = if cfg!(windows) {
        "devtrail.exe"
    } else {
        "devtrail"
    };

    let extracted_binary = if cfg!(windows) {
        extract_from_zip(&archive_path, temp_dir.path(), binary_name)?
    } else {
        extract_from_tar_gz(&archive_path, temp_dir.path(), binary_name)?
    };

    // Replace binary
    utils::info("Replacing binary...");
    let current_exe =
        std::env::current_exe().context("Failed to determine current executable path")?;

    replace_binary(&extracted_binary, &current_exe)?;

    utils::success(&format!("CLI updated to cli-{}!", tag_version));

    #[cfg(windows)]
    println!(
        "  {}",
        "Note: The old binary will be cleaned up on next run.".dimmed()
    );

    Ok(())
}

/// Extract a binary from a tar.gz archive
fn extract_from_tar_gz(archive: &Path, dest: &Path, binary_name: &str) -> Result<PathBuf> {
    let file = std::fs::File::open(archive).context("Failed to open archive")?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut tar = tar::Archive::new(decoder);

    for entry in tar.entries().context("Failed to read archive entries")? {
        let mut entry = entry.context("Failed to read archive entry")?;
        let path = entry.path().context("Failed to read entry path")?;

        if let Some(name) = path.file_name() {
            if name == binary_name {
                let dest_path = dest.join(binary_name);
                entry
                    .unpack(&dest_path)
                    .context("Failed to extract binary")?;
                return Ok(dest_path);
            }
        }
    }

    bail!("Binary '{}' not found in archive", binary_name)
}

/// Extract a binary from a zip archive
#[allow(dead_code)]
fn extract_from_zip(archive: &Path, dest: &Path, binary_name: &str) -> Result<PathBuf> {
    let file = std::fs::File::open(archive).context("Failed to open archive")?;
    let mut zip = zip::ZipArchive::new(file).context("Failed to read ZIP archive")?;

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).context("Failed to read ZIP entry")?;
        let entry_path = entry.name().to_string();

        let entry_file_name = Path::new(&entry_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if entry_file_name == binary_name {
            let dest_path = dest.join(binary_name);
            let mut outfile =
                std::fs::File::create(&dest_path).context("Failed to create output file")?;
            std::io::copy(&mut entry, &mut outfile).context("Failed to extract binary")?;
            return Ok(dest_path);
        }
    }

    bail!("Binary '{}' not found in archive", binary_name)
}

/// Replace the current binary with a new one
#[cfg(unix)]
fn replace_binary(new_binary: &Path, current_exe: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let old_path = current_exe.with_extension("old");

    // Rename current to .old
    std::fs::rename(current_exe, &old_path).with_context(|| {
        format!(
            "Cannot update binary: permission denied at {}",
            current_exe.display()
        )
    })?;

    // Copy new binary (copy, not rename, to handle cross-filesystem)
    if let Err(e) = std::fs::copy(new_binary, current_exe) {
        // Rollback: restore old binary
        let _ = std::fs::rename(&old_path, current_exe);
        bail!(
            "Failed to install new binary: {}. Rolled back to previous version.",
            e
        );
    }

    // Set executable permissions
    let perms = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(current_exe, perms)
        .context("Failed to set executable permissions")?;

    // Clean up old binary
    let _ = std::fs::remove_file(&old_path);

    Ok(())
}

/// Replace the current binary with a new one (Windows)
#[cfg(windows)]
fn replace_binary(new_binary: &Path, current_exe: &Path) -> Result<()> {
    let old_path = current_exe.with_extension("old.exe");

    // Rename current to .old.exe (Windows allows rename of running exe)
    std::fs::rename(current_exe, &old_path).with_context(|| {
        format!(
            "Cannot update binary: permission denied at {}",
            current_exe.display()
        )
    })?;

    // Copy new binary
    if let Err(e) = std::fs::copy(new_binary, current_exe) {
        // Rollback: restore old binary
        let _ = std::fs::rename(&old_path, current_exe);
        bail!(
            "Failed to install new binary: {}. Rolled back to previous version.",
            e
        );
    }

    Ok(())
}

/// Clean up leftover old binary from a previous update (mainly for Windows)
pub fn cleanup_old_binary() {
    if let Ok(current_exe) = std::env::current_exe() {
        let old_exe = if cfg!(windows) {
            current_exe.with_extension("old.exe")
        } else {
            current_exe.with_extension("old")
        };

        if old_exe.exists() {
            let _ = std::fs::remove_file(&old_exe);
        }
    }
}
