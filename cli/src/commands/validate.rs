use anyhow::Result;
use colored::Colorize;
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::utils;
use crate::validation::{self, Severity, ValidationIssue};

pub fn run(path: &str, fix: bool) -> Result<()> {
    let resolved = match utils::resolve_project_root(path) {
        Some(r) => r,
        None => {
            let target = PathBuf::from(path)
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from(path));
            utils::info(&format!(
                "DevTrail is not installed in {}",
                target.display()
            ));
            utils::info("Run 'devtrail init' to initialize DevTrail in this directory.");
            return Ok(());
        }
    };

    if resolved.is_fallback {
        utils::info(&format!(
            "Using DevTrail installation at repo root: {}",
            resolved.path.display()
        ));
    }

    let target = resolved.path;
    let devtrail_dir = target.join(".devtrail");

    // Header
    println!();
    println!("  {}", "DevTrail Validate".bold().cyan());
    println!("  {}", target.display().to_string().dimmed());
    println!();

    // Run validation
    let (result, doc_count) = validation::validate_all(&devtrail_dir);

    if doc_count == 0 {
        utils::info("No documents found to validate.");
        println!(
            "  {} Create documents with {} or {}",
            "→".blue().bold(),
            "devtrail-new".cyan(),
            "/devtrail-new".cyan()
        );
        println!();
        return Ok(());
    }

    // Apply fixes if requested
    if fix {
        apply_fixes(&devtrail_dir);
        // Re-validate after fixes
        let (result, doc_count) = validation::validate_all(&devtrail_dir);
        print_results(&result, doc_count);
        return exit_with_code(&result);
    }

    print_results(&result, doc_count);
    exit_with_code(&result)
}

fn apply_fixes(devtrail_dir: &std::path::Path) {
    let paths = crate::document::discover_documents(devtrail_dir);
    let mut fixed_count = 0;

    for path in &paths {
        if let Ok(doc) = crate::document::parse_document(path) {
            if let Some(new_content) = validation::apply_fixes(&doc) {
                if std::fs::write(path, new_content).is_ok() {
                    println!(
                        "  {} Fixed: {}",
                        "✓".green().bold(),
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("?")
                    );
                    fixed_count += 1;
                }
            }
        }
    }

    if fixed_count > 0 {
        println!();
        println!(
            "  {} {} file(s) fixed automatically",
            "→".blue().bold(),
            fixed_count
        );
        println!();
    }
}

fn print_results(result: &validation::ValidationResult, doc_count: usize) {
    let all_issues: Vec<&ValidationIssue> = result
        .errors
        .iter()
        .chain(result.warnings.iter())
        .collect();

    if all_issues.is_empty() {
        println!(
            "  {} All {} document(s) passed validation",
            "✓".green().bold(),
            doc_count
        );
        println!();
        return;
    }

    // Group by file
    let mut by_file: BTreeMap<&PathBuf, Vec<&ValidationIssue>> = BTreeMap::new();
    for issue in &all_issues {
        by_file.entry(&issue.file).or_default().push(issue);
    }

    for (file, issues) in &by_file {
        let filename = file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?");

        println!("  {}", filename.bold());

        for issue in issues {
            let severity_label = match issue.severity {
                Severity::Error => "error".red().bold(),
                Severity::Warning => "warn".yellow().bold(),
            };
            println!(
                "    {} [{}] {}",
                severity_label, issue.rule, issue.message
            );
            if let Some(hint) = &issue.fix_hint {
                println!("    {} {}", "hint:".dimmed(), hint.dimmed());
            }
        }
        println!();
    }

    // Summary
    let error_count = result.errors.len();
    let warning_count = result.warnings.len();

    let summary = format!(
        "  {} error(s), {} warning(s) in {} document(s)",
        error_count, warning_count, doc_count
    );

    if error_count > 0 {
        println!("{}", summary.red().bold());
    } else {
        println!("{}", summary.yellow());
    }
    println!();
}

fn exit_with_code(result: &validation::ValidationResult) -> Result<()> {
    if result.errors.is_empty() {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
