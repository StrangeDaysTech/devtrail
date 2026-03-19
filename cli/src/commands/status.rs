use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

use crate::config::DevTrailConfig;
use crate::manifest::DistManifest;
use crate::utils;

/// Expected directories inside .devtrail/
const EXPECTED_DIRS: &[&str] = &[
    "00-governance",
    "01-requirements",
    "02-design/decisions",
    "03-implementation",
    "04-testing",
    "05-operations/incidents",
    "05-operations/runbooks",
    "06-evolution/technical-debt",
    "07-ai-audit/agent-logs",
    "07-ai-audit/decisions",
    "07-ai-audit/ethical-reviews",
    "templates",
];

/// Expected files (relative to project root)
const EXPECTED_FILES: &[(&str, &str)] = &[
    (".devtrail/config.yml", "config.yml"),
    (".devtrail/dist-manifest.yml", "dist-manifest.yml"),
    ("DEVTRAIL.md", "DEVTRAIL.md"),
];

/// Document type prefixes for counting
const DOC_TYPES: &[&str] = &["ADR", "AIDEC", "AILOG", "ETH", "INC", "REQ", "TDE", "TES"];

pub fn run(path: &str) -> Result<()> {
    let target = PathBuf::from(path)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(path));

    let devtrail_dir = target.join(".devtrail");

    // Phase 1: Check installation
    if !devtrail_dir.exists() {
        utils::info(&format!(
            "DevTrail is not installed in {}",
            target.display()
        ));
        utils::info("Run 'devtrail init' to initialize DevTrail in this directory.");
        return Ok(());
    }

    // Phase 2: Header
    let version = load_version(&target);
    let language = load_language(&target);

    let cli_version = env!("CARGO_PKG_VERSION");

    println!();
    println!("{}", "DevTrail Status".bold());
    println!("  {}      {}", "Path:".dimmed(), target.display());
    println!("  {} fw-{}", "Framework:".dimmed(), version);
    println!("  {}       cli-{}", "CLI:".dimmed(), cli_version);
    println!("  {}  {}", "Language:".dimmed(), language);

    // Phase 2: Structure check
    println!();
    println!("{}", "Structure".bold());

    for dir in EXPECTED_DIRS {
        let dir_path = devtrail_dir.join(dir);
        if dir_path.exists() {
            utils::success(&format!("{dir}/"));
        } else {
            println!(
                "{} {}  {}",
                "!".yellow().bold(),
                format!("{dir}/").yellow(),
                "(missing)".dimmed()
            );
        }
    }

    for &(rel_path, label) in EXPECTED_FILES {
        let file_path = target.join(rel_path);
        if file_path.exists() {
            utils::success(label);
        } else {
            println!(
                "{} {}  {}",
                "!".yellow().bold(),
                label.yellow(),
                "(missing)".dimmed()
            );
        }
    }

    // Phase 3: Documentation statistics
    let counts = count_documents(&devtrail_dir);
    let total: usize = counts.iter().map(|(_, c)| c).sum();

    println!();
    println!("{}", "Documentation".bold());
    println!("  {:<7}  {:>5}", "Type", "Count");
    println!("  {:<7}  {:>5}", "───────", "─────");
    for (doc_type, count) in &counts {
        println!("  {:<7}  {:>5}", doc_type, count);
    }
    println!("  {:<7}  {:>5}", "───────", "─────");
    println!("  {:<7}  {:>5}", "Total", total);
    println!();

    Ok(())
}

fn load_version(project_root: &std::path::Path) -> String {
    let manifest_path = project_root.join(".devtrail/dist-manifest.yml");
    match DistManifest::load(&manifest_path) {
        Ok(m) => m.version,
        Err(_) => {
            utils::warn("Could not read dist-manifest.yml");
            "unknown".to_string()
        }
    }
}

fn load_language(project_root: &std::path::Path) -> String {
    match DevTrailConfig::load(project_root) {
        Ok(c) => c.language,
        Err(_) => {
            utils::warn("Could not read config.yml");
            "unknown".to_string()
        }
    }
}

fn count_documents(devtrail_dir: &std::path::Path) -> Vec<(&'static str, usize)> {
    let files = walk_files(devtrail_dir);
    DOC_TYPES
        .iter()
        .map(|&doc_type| {
            let prefix = format!("{}-", doc_type);
            let count = files
                .iter()
                .filter(|p| {
                    utils::is_user_document(p)
                        && p.file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n.starts_with(&prefix))
                            .unwrap_or(false)
                })
                .count();
            (doc_type, count)
        })
        .collect()
}

fn walk_files(dir: &std::path::Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walk_files(&path));
            } else {
                files.push(path);
            }
        }
    }
    files
}
