use anyhow::{bail, Context, Result};
use colored::Colorize;
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::path::Path;

use crate::inject;
use crate::utils;

pub fn run(full: bool) -> Result<()> {
    let target = std::env::current_dir().context("Failed to get current directory")?;

    if !target.join(".devtrail").exists() {
        bail!("DevTrail is not installed in this directory.");
    }

    if full {
        println!(
            "{} This will remove ALL DevTrail files including your documents!",
            "WARNING:".red().bold()
        );
        let confirmed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Are you sure you want to remove everything?")
            .default(false)
            .interact()?;

        if !confirmed {
            println!("Aborted.");
            return Ok(());
        }

        let double_confirmed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("This will delete all your AILOG, AIDEC, ADR, and other documents. Really proceed?")
            .default(false)
            .interact()?;

        if !double_confirmed {
            println!("Aborted.");
            return Ok(());
        }
    }

    println!("{} DevTrail...", "Removing".red().bold());

    // Remove injections from directive files
    utils::info("Cleaning AI agent directives...");
    clean_directives(&target)?;

    // Remove framework files
    utils::info("Removing framework files...");

    if full {
        // Remove everything
        remove_dir_if_exists(&target.join(".devtrail"))?;
    } else {
        // Selective removal: keep user documents, remove framework
        remove_framework_files(&target)?;
    }

    // Remove distributed files
    remove_file_if_exists(&target.join("DEVTRAIL.md"))?;

    // Remove agent skills and workflows
    remove_dir_if_exists(&target.join(".claude/skills"))?;
    remove_dir_if_exists(&target.join(".gemini/skills"))?;
    remove_dir_if_exists(&target.join(".agent/workflows"))?;

    // Clean up empty parent dirs
    remove_empty_dir(&target.join(".claude"))?;
    remove_empty_dir(&target.join(".gemini"))?;
    remove_empty_dir(&target.join(".agent"))?;

    // Remove cursor rules file (if we created it)
    remove_file_if_exists(&target.join(".cursor/rules/devtrail.md"))?;
    remove_empty_dir(&target.join(".cursor/rules"))?;
    remove_empty_dir(&target.join(".cursor"))?;

    // Remove scripts
    let scripts = [
        "scripts/devtrail-new.sh",
        "scripts/devtrail-status.sh",
        "scripts/pre-commit-docs.sh",
        "scripts/validate-docs.ps1",
    ];
    for script in &scripts {
        remove_file_if_exists(&target.join(script))?;
    }
    remove_empty_dir(&target.join("scripts"))?;

    println!();
    utils::success("DevTrail removed successfully.");

    if !full {
        println!();
        println!(
            "  {} User-generated documents in .devtrail/ were preserved.",
            "Note:".bold()
        );
        println!(
            "  Use {} to remove everything.",
            "devtrail remove --full".yellow()
        );
    }

    Ok(())
}

fn clean_directives(target: &Path) -> Result<()> {
    let files = [
        target.join("CLAUDE.md"),
        target.join("GEMINI.md"),
        target.join(".github/copilot-instructions.md"),
        target.join(".cursorrules"),
    ];

    for f in &files {
        if inject::remove_injection(f)? {
            let name = f.strip_prefix(target).unwrap_or(f).display().to_string();
            utils::success(&format!("Cleaned {}", name));
        }
    }

    Ok(())
}

/// Remove framework files but keep user-generated documents
fn remove_framework_files(target: &Path) -> Result<()> {
    let devtrail = target.join(".devtrail");

    // Framework directories to remove entirely
    let framework_dirs = [
        "00-governance",
        "03-implementation",
        "templates",
    ];

    for dir in &framework_dirs {
        remove_dir_if_exists(&devtrail.join(dir))?;
    }

    // Remove framework files but keep user documents in these dirs
    let mixed_dirs = [
        "01-requirements",
        "02-design/decisions",
        "04-testing",
        "05-operations/incidents",
        "05-operations/runbooks",
        "06-evolution/technical-debt",
        "07-ai-audit/agent-logs",
        "07-ai-audit/decisions",
        "07-ai-audit/ethical-reviews",
    ];

    for dir in &mixed_dirs {
        let dir_path = devtrail.join(dir);
        if dir_path.is_dir() {
            // Only remove .gitkeep, keep user documents
            remove_file_if_exists(&dir_path.join(".gitkeep"))?;
        }
    }

    // Remove framework root files
    remove_file_if_exists(&devtrail.join("config.yml"))?;
    remove_file_if_exists(&devtrail.join("QUICK-REFERENCE.md"))?;
    remove_file_if_exists(&devtrail.join(".checksums.json"))?;

    Ok(())
}

fn remove_file_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path).with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

fn remove_dir_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_dir_all(path)
            .with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

fn remove_empty_dir(path: &Path) -> Result<()> {
    if path.is_dir() {
        if let Ok(mut entries) = std::fs::read_dir(path) {
            if entries.next().is_none() {
                std::fs::remove_dir(path).ok();
            }
        }
    }
    Ok(())
}
