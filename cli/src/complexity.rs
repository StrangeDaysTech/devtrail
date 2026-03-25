use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Report of cyclomatic complexity analysis
#[derive(Debug)]
#[allow(dead_code)]
pub struct ComplexityReport {
    pub functions: Vec<FunctionComplexity>,
    pub above_threshold: Vec<FunctionComplexity>,
    pub threshold: u32,
}

/// Complexity data for a single function
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FunctionComplexity {
    pub nloc: u32,
    pub ccn: u32,
    pub token_count: u32,
    pub param_count: u32,
    pub length: u32,
    pub filename: String,
    pub function_name: String,
}

/// Check if lizard is available in PATH
#[allow(dead_code)]
pub fn is_lizard_available() -> bool {
    Command::new("lizard")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Analyze cyclomatic complexity using lizard
#[allow(dead_code)]
pub fn analyze_complexity(paths: &[PathBuf], threshold: u32) -> Result<ComplexityReport> {
    if !is_lizard_available() {
        anyhow::bail!(
            "lizard is not installed. Install with: pip install lizard\n\
             Without lizard, use qualitative criteria for complexity assessment."
        );
    }

    let path_strs: Vec<String> = paths.iter().map(|p| p.display().to_string()).collect();

    let output = Command::new("lizard")
        .arg("--csv")
        .args(&path_strs)
        .output()
        .context("Failed to execute lizard")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("lizard failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let functions = parse_lizard_csv(&stdout)?;

    let above_threshold: Vec<FunctionComplexity> = functions
        .iter()
        .filter(|f| f.ccn > threshold)
        .cloned()
        .collect();

    Ok(ComplexityReport {
        functions,
        above_threshold,
        threshold,
    })
}

/// Parse lizard CSV output
#[allow(dead_code)]
fn parse_lizard_csv(csv: &str) -> Result<Vec<FunctionComplexity>> {
    let mut functions = Vec::new();

    for line in csv.lines() {
        // Skip header and empty lines
        if line.starts_with("NLOC") || line.trim().is_empty() || line.starts_with('-') {
            continue;
        }

        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 7 {
            continue;
        }

        let nloc = fields[0].trim().parse::<u32>().unwrap_or(0);
        let ccn = fields[1].trim().parse::<u32>().unwrap_or(0);
        let token_count = fields[2].trim().parse::<u32>().unwrap_or(0);
        let param_count = fields[3].trim().parse::<u32>().unwrap_or(0);
        let length = fields[4].trim().parse::<u32>().unwrap_or(0);
        let filename = fields[5].trim().trim_matches('"').to_string();
        let function_name = fields[6].trim().trim_matches('"').to_string();

        functions.push(FunctionComplexity {
            nloc,
            ccn,
            token_count,
            param_count,
            length,
            filename,
            function_name,
        });
    }

    Ok(functions)
}

/// Generate JSON output for agent consumption
#[allow(dead_code)]
pub fn report_to_json(report: &ComplexityReport) -> String {
    let mut entries = Vec::new();
    for f in &report.above_threshold {
        entries.push(format!(
            r#"    {{"function":"{}","file":"{}","ccn":{},"nloc":{},"params":{}}}"#,
            f.function_name, f.filename, f.ccn, f.nloc, f.param_count
        ));
    }

    format!(
        r#"{{"threshold":{},"total_functions":{},"above_threshold":{},"functions":[
{}
]}}"#,
        report.threshold,
        report.functions.len(),
        report.above_threshold.len(),
        entries.join(",\n")
    )
}
