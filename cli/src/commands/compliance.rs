use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

use crate::compliance::{self, CheckStatus, ComplianceReport};
use crate::document;
use crate::utils;

pub fn run(path: &str, standard: Option<&str>, all: bool, output: &str) -> Result<()> {
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

    // Discover and parse all documents
    let paths = document::discover_documents(&devtrail_dir);
    let docs: Vec<_> = paths
        .iter()
        .filter_map(|p| document::parse_document(p).ok())
        .collect();

    // Determine which standard(s) to check
    // If neither --standard nor --all, default to --all
    let run_all = all || standard.is_none();

    let mut reports = Vec::new();

    if run_all {
        reports.push(compliance::check_eu_ai_act(&docs, &devtrail_dir));
        reports.push(compliance::check_iso_42001(&docs, &devtrail_dir));
        reports.push(compliance::check_nist_ai_rmf(&docs, &devtrail_dir));
    } else if let Some(std_name) = standard {
        match std_name {
            "eu-ai-act" => reports.push(compliance::check_eu_ai_act(&docs, &devtrail_dir)),
            "iso-42001" => reports.push(compliance::check_iso_42001(&docs, &devtrail_dir)),
            "nist-ai-rmf" => reports.push(compliance::check_nist_ai_rmf(&docs, &devtrail_dir)),
            _ => {
                utils::warn(&format!("Unknown standard: {}", std_name));
                return Ok(());
            }
        }
    }

    // Output
    match output {
        "json" => print_json(&reports),
        "markdown" => print_markdown(&reports, docs.len()),
        _ => print_text(&reports, &target, docs.len()),
    }

    Ok(())
}

fn print_text(reports: &[ComplianceReport], target: &std::path::Path, doc_count: usize) {
    println!();
    println!("  {}", "DevTrail Compliance".bold().cyan());
    println!("  {}", target.display().to_string().dimmed());
    println!(
        "  {}",
        format!("{} document(s) analyzed", doc_count).dimmed()
    );
    println!();

    for report in reports {
        let score_color = if report.score >= 80.0 {
            format!("{:.0}%", report.score).green().bold()
        } else if report.score >= 50.0 {
            format!("{:.0}%", report.score).yellow().bold()
        } else {
            format!("{:.0}%", report.score).red().bold()
        };

        println!(
            "  {} {} {}",
            "■".cyan().bold(),
            report.standard_label.bold(),
            score_color
        );

        for check in &report.checks {
            let status_icon = match check.status {
                CheckStatus::Pass => "✓".green().bold(),
                CheckStatus::Partial => "~".yellow().bold(),
                CheckStatus::Fail => "✗".red().bold(),
            };

            println!("    {} [{}] {}", status_icon, check.id, check.description);

            if !check.evidence.is_empty() && check.status != CheckStatus::Fail {
                let evidence_str = if check.evidence.len() <= 3 {
                    check.evidence.join(", ")
                } else {
                    format!(
                        "{}, ... (+{} more)",
                        check.evidence[..3].join(", "),
                        check.evidence.len() - 3
                    )
                };
                println!("      {}", evidence_str.dimmed());
            }

            if let Some(remediation) = &check.remediation {
                if check.status != CheckStatus::Pass {
                    println!("      {} {}", "fix:".dimmed(), remediation.dimmed());
                }
            }
        }
        println!();
    }

    // Overall summary
    if reports.len() > 1 {
        let avg_score: f64 = reports.iter().map(|r| r.score).sum::<f64>() / reports.len() as f64;
        let summary_color = if avg_score >= 80.0 {
            format!("  Overall compliance: {:.0}%", avg_score)
                .green()
                .bold()
        } else if avg_score >= 50.0 {
            format!("  Overall compliance: {:.0}%", avg_score)
                .yellow()
                .bold()
        } else {
            format!("  Overall compliance: {:.0}%", avg_score)
                .red()
                .bold()
        };
        println!("{}", summary_color);
        println!();
    }
}

fn print_json(reports: &[ComplianceReport]) {
    let json = serde_json::to_string_pretty(reports).unwrap_or_else(|_| "[]".into());
    println!("{}", json);
}

fn print_markdown(reports: &[ComplianceReport], doc_count: usize) {
    println!("# DevTrail Compliance Report");
    println!();
    println!("**Documents analyzed:** {}", doc_count);
    println!();

    for report in reports {
        println!("## {} — {:.0}%", report.standard_label, report.score);
        println!();
        println!("| Check | Status | Description |");
        println!("|-------|--------|-------------|");

        for check in &report.checks {
            let status_emoji = match check.status {
                CheckStatus::Pass => "✅",
                CheckStatus::Partial => "⚠️",
                CheckStatus::Fail => "❌",
            };
            println!(
                "| {} | {} | {} |",
                check.id, status_emoji, check.description
            );
        }
        println!();
    }

    if reports.len() > 1 {
        let avg_score: f64 = reports.iter().map(|r| r.score).sum::<f64>() / reports.len() as f64;
        println!("---");
        println!();
        println!("**Overall compliance: {:.0}%**", avg_score);
    }
}
