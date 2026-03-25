use anyhow::{Context, Result};
use serde::Deserialize;
use std::fmt;
use std::path::{Path, PathBuf};

/// All supported DevTrail document types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocType {
    Ailog,
    Aidec,
    Adr,
    Eth,
    Req,
    Tes,
    Inc,
    Tde,
    Sec,
    Mcard,
    Sbom,
    Dpia,
}

impl DocType {
    /// Prefix used in filenames (e.g., "AILOG", "SEC")
    pub fn prefix(&self) -> &'static str {
        match self {
            DocType::Ailog => "AILOG",
            DocType::Aidec => "AIDEC",
            DocType::Adr => "ADR",
            DocType::Eth => "ETH",
            DocType::Req => "REQ",
            DocType::Tes => "TES",
            DocType::Inc => "INC",
            DocType::Tde => "TDE",
            DocType::Sec => "SEC",
            DocType::Mcard => "MCARD",
            DocType::Sbom => "SBOM",
            DocType::Dpia => "DPIA",
        }
    }

    /// Parse a DocType from a filename prefix
    pub fn from_prefix(prefix: &str) -> Option<DocType> {
        match prefix {
            "AILOG" => Some(DocType::Ailog),
            "AIDEC" => Some(DocType::Aidec),
            "ADR" => Some(DocType::Adr),
            "ETH" => Some(DocType::Eth),
            "REQ" => Some(DocType::Req),
            "TES" => Some(DocType::Tes),
            "INC" => Some(DocType::Inc),
            "TDE" => Some(DocType::Tde),
            "SEC" => Some(DocType::Sec),
            "MCARD" => Some(DocType::Mcard),
            "SBOM" => Some(DocType::Sbom),
            "DPIA" => Some(DocType::Dpia),
            _ => None,
        }
    }

    /// All valid prefixes
    pub const ALL_PREFIXES: &'static [&'static str] = &[
        "AILOG", "AIDEC", "ADR", "ETH", "REQ", "TES", "INC", "TDE",
        "SEC", "MCARD", "SBOM", "DPIA",
    ];
}

impl fmt::Display for DocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.prefix())
    }
}

/// Frontmatter fields extracted from a DevTrail document.
/// All fields are optional so the validator can report which are missing.
#[derive(Debug, Clone, Deserialize, Default)]
#[allow(dead_code)]
pub struct Frontmatter {
    pub id: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub created: Option<String>,
    pub agent: Option<String>,
    pub confidence: Option<String>,
    pub review_required: Option<bool>,
    pub risk_level: Option<String>,
    pub eu_ai_act_risk: Option<String>,
    pub nist_genai_risks: Option<Vec<String>>,
    pub iso_42001_clause: Option<Vec<u8>>,
    pub tags: Option<Vec<String>>,
    pub related: Option<Vec<String>>,
    // INC-specific
    pub severity: Option<String>,
    // ETH-specific
    pub gdpr_legal_basis: Option<String>,
    // SEC-specific
    pub threat_model_methodology: Option<String>,
    pub owasp_asvs_level: Option<serde_yaml::Value>,
    // MCARD-specific
    pub model_name: Option<String>,
    pub model_type: Option<String>,
    pub model_version: Option<String>,
    pub provider: Option<String>,
    pub license: Option<String>,
    // SBOM-specific
    pub sbom_format_reference: Option<String>,
    pub system_name: Option<String>,
    // DPIA-specific
    pub gdpr_article_35: Option<bool>,
    pub dpo_consulted: Option<bool>,
    pub supervisory_authority_consulted: Option<bool>,
}

/// A parsed DevTrail document
#[derive(Debug)]
pub struct DevTrailDocument {
    pub path: PathBuf,
    pub filename: String,
    pub doc_type: DocType,
    pub frontmatter: Frontmatter,
    pub body: String,
}

/// Parse a DevTrail document from a file path
pub fn parse_document(path: &Path) -> Result<DevTrailDocument> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    // Determine doc type from filename prefix
    let doc_type = detect_doc_type(&filename)
        .with_context(|| format!("Cannot determine document type for {}", filename))?;

    // Extract frontmatter
    let (frontmatter, body) = extract_frontmatter(&content)
        .with_context(|| format!("Failed to parse frontmatter in {}", path.display()))?;

    Ok(DevTrailDocument {
        path: path.to_path_buf(),
        filename,
        doc_type,
        frontmatter,
        body,
    })
}

/// Detect document type from filename prefix
fn detect_doc_type(filename: &str) -> Option<DocType> {
    for prefix in DocType::ALL_PREFIXES {
        if filename.starts_with(&format!("{}-", prefix)) {
            return DocType::from_prefix(prefix);
        }
    }
    None
}

/// Extract YAML frontmatter (between --- delimiters) and body
fn extract_frontmatter(content: &str) -> Result<(Frontmatter, String)> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        anyhow::bail!("No frontmatter found (missing opening ---)");
    }

    let after_first = &trimmed[3..];
    let end_pos = after_first
        .find("\n---")
        .ok_or_else(|| anyhow::anyhow!("No closing --- found for frontmatter"))?;

    let yaml_str = &after_first[..end_pos];
    let body_start = end_pos + 4; // skip "\n---"
    let body = if body_start < after_first.len() {
        after_first[body_start..].to_string()
    } else {
        String::new()
    };

    let frontmatter: Frontmatter = serde_yaml::from_str(yaml_str)
        .with_context(|| "Failed to deserialize frontmatter YAML")?;

    Ok((frontmatter, body))
}

/// Discover all user-generated DevTrail documents under a .devtrail/ directory
pub fn discover_documents(devtrail_dir: &Path) -> Vec<PathBuf> {
    let mut results = Vec::new();
    walk_for_documents(devtrail_dir, &mut results);
    results.sort();
    results
}

fn walk_for_documents(dir: &Path, results: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Skip templates directory
            if path.ends_with("templates") {
                continue;
            }
            walk_for_documents(&path, results);
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            // Match pattern: TYPE-YYYY-MM-DD-NNN-*.md
            if detect_doc_type(filename).is_some() && is_dated_document(filename) {
                results.push(path);
            }
        }
    }
}

/// Check if filename follows the dated pattern TYPE-YYYY-MM-DD-NNN-*.md
fn is_dated_document(filename: &str) -> bool {
    // Find the first '-' (after the type prefix)
    let after_prefix = match filename.find('-') {
        Some(pos) => &filename[pos + 1..],
        None => return false,
    };
    // Should start with a date pattern YYYY-MM-DD
    if after_prefix.len() < 10 {
        return false;
    }
    let date_part = &after_prefix[..10];
    // Basic date pattern check: NNNN-NN-NN
    date_part.len() == 10
        && date_part.chars().nth(4) == Some('-')
        && date_part.chars().nth(7) == Some('-')
        && date_part[..4].chars().all(|c| c.is_ascii_digit())
        && date_part[5..7].chars().all(|c| c.is_ascii_digit())
        && date_part[8..10].chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_doc_type() {
        assert_eq!(detect_doc_type("AILOG-2025-01-01-001-test.md"), Some(DocType::Ailog));
        assert_eq!(detect_doc_type("SEC-2025-01-01-001-auth.md"), Some(DocType::Sec));
        assert_eq!(detect_doc_type("MCARD-2025-01-01-001-gpt.md"), Some(DocType::Mcard));
        assert_eq!(detect_doc_type("SBOM-2025-01-01-001-deps.md"), Some(DocType::Sbom));
        assert_eq!(detect_doc_type("DPIA-2025-01-01-001-gdpr.md"), Some(DocType::Dpia));
        assert_eq!(detect_doc_type("README.md"), None);
        assert_eq!(detect_doc_type("TEMPLATE-SEC.md"), None);
    }

    #[test]
    fn test_is_dated_document() {
        assert!(is_dated_document("AILOG-2025-01-27-001-implement-auth.md"));
        assert!(is_dated_document("SEC-2026-03-24-001-api-review.md"));
        assert!(!is_dated_document("TEMPLATE-SEC.md"));
        assert!(!is_dated_document("README.md"));
    }

    #[test]
    fn test_extract_frontmatter() {
        let content = "---\nid: AILOG-2025-01-01-001\ntitle: Test\nstatus: draft\n---\n\n# Body";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert_eq!(fm.id.as_deref(), Some("AILOG-2025-01-01-001"));
        assert_eq!(fm.title.as_deref(), Some("Test"));
        assert!(body.contains("# Body"));
    }
}
