use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocStatus {
    Draft,
    Accepted,
    Deprecated,
    Superseded,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for DocStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "DRAFT"),
            Self::Accepted => write!(f, "ACCEPTED"),
            Self::Deprecated => write!(f, "DEPRECATED"),
            Self::Superseded => write!(f, "SUPERSEDED"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct DocFrontMatter {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub status: Option<DocStatus>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub updated: Option<String>,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub confidence: Option<ConfidenceLevel>,
    #[serde(default)]
    pub review_required: Option<bool>,
    #[serde(default)]
    pub risk_level: Option<RiskLevel>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub related: Vec<String>,
    #[serde(default)]
    pub supersedes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub frontmatter: Option<DocFrontMatter>,
    pub body: String,
    pub filename: String,
}

impl Document {
    pub fn load(path: &Path) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let (frontmatter, body) = parse_frontmatter(&content);

        Some(Self {
            frontmatter,
            body,
            filename,
        })
    }
}

fn parse_frontmatter(content: &str) -> (Option<DocFrontMatter>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content.to_string());
    }

    // Find the closing ---
    let after_first = &trimmed[3..];
    let closing = after_first.find("\n---");
    match closing {
        Some(pos) => {
            let yaml_str = &after_first[..pos];
            let body_start = pos + 4; // skip \n---
            let body = after_first[body_start..].trim_start_matches('\n').to_string();
            let fm: Option<DocFrontMatter> = serde_yaml::from_str(yaml_str).ok();
            (fm, body)
        }
        None => (None, content.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
id: ADR-2025-06-15-001
title: Test Document
status: accepted
created: 2025-06-15
risk_level: low
tags: [rust, tui]
related: [REQ-2025-06-10-003]
---

# Test Document

Some content here.
"#;
        let (fm, body) = parse_frontmatter(content);
        let fm = fm.unwrap();
        assert_eq!(fm.id, "ADR-2025-06-15-001");
        assert_eq!(fm.status, Some(DocStatus::Accepted));
        assert!(body.starts_with("# Test Document"));
    }
}
