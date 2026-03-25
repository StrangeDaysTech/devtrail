use std::path::{Path, PathBuf};

use crate::document::{self, DevTrailDocument, DocType};

/// Severity of a validation issue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

/// A single validation issue found in a document
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub file: PathBuf,
    pub rule: String,
    pub message: String,
    pub severity: Severity,
    pub fix_hint: Option<String>,
}

/// Result of validating one or more documents
#[derive(Debug, Default)]
pub struct ValidationResult {
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }

    fn add(&mut self, issue: ValidationIssue) {
        match issue.severity {
            Severity::Error => self.errors.push(issue),
            Severity::Warning => self.warnings.push(issue),
        }
    }
}

/// Valid status values per DOCUMENTATION-POLICY.md
const VALID_STATUSES: &[&str] = &["draft", "review", "accepted", "superseded", "deprecated"];

/// Valid risk levels
const VALID_RISK_LEVELS: &[&str] = &["low", "medium", "high", "critical"];

/// Valid confidence levels
const VALID_CONFIDENCES: &[&str] = &["low", "medium", "high"];

/// Patterns that indicate sensitive information
const SENSITIVE_PATTERNS: &[&str] = &[
    "password:", "api_key:", "secret:", "token:", "private_key:",
    "credentials:", "Bearer ", "AWS_SECRET", "PRIVATE KEY",
];

/// Validate all documents found under a .devtrail/ directory
pub fn validate_all(devtrail_dir: &Path) -> (ValidationResult, usize) {
    let paths = document::discover_documents(devtrail_dir);
    let doc_count = paths.len();
    let mut result = ValidationResult::default();

    for path in &paths {
        match document::parse_document(path) {
            Ok(doc) => {
                result.merge(validate_document(&doc, devtrail_dir));
            }
            Err(e) => {
                result.errors.push(ValidationIssue {
                    file: path.clone(),
                    rule: "PARSE-001".to_string(),
                    message: format!("Failed to parse document: {e}"),
                    severity: Severity::Error,
                    fix_hint: Some("Check that the file has valid YAML frontmatter between --- delimiters".to_string()),
                });
            }
        }
    }

    (result, doc_count)
}

/// Validate a single parsed document
fn validate_document(doc: &DevTrailDocument, devtrail_dir: &Path) -> ValidationResult {
    let mut result = ValidationResult::default();

    check_naming(&mut result, doc);
    check_required_meta(&mut result, doc);
    check_id_matches_filename(&mut result, doc);
    check_valid_status(&mut result, doc);
    check_cross_rules(&mut result, doc);
    check_type_specific(&mut result, doc);
    check_related_exist(&mut result, doc, devtrail_dir);
    check_sensitive_info(&mut result, doc);
    check_observability(&mut result, doc);

    result
}

/// NAMING-001: Verify filename follows TYPE-YYYY-MM-DD-NNN-description.md
fn check_naming(result: &mut ValidationResult, doc: &DevTrailDocument) {
    let name = &doc.filename;
    let prefix = doc.doc_type.prefix();

    // Check: PREFIX-YYYY-MM-DD-NNN-*.md
    let after_prefix = match name.strip_prefix(&format!("{}-", prefix)) {
        Some(rest) => rest,
        None => {
            result.add(ValidationIssue {
                file: doc.path.clone(),
                rule: "NAMING-001".to_string(),
                message: format!("Filename should start with '{}-'", prefix),
                severity: Severity::Error,
                fix_hint: None,
            });
            return;
        }
    };

    // Check date part
    if after_prefix.len() < 10 {
        result.add(ValidationIssue {
            file: doc.path.clone(),
            rule: "NAMING-001".to_string(),
            message: "Filename missing date component (expected YYYY-MM-DD after prefix)".to_string(),
            severity: Severity::Error,
            fix_hint: None,
        });
        return;
    }

    let date_part = &after_prefix[..10];
    let valid_date = date_part.chars().nth(4) == Some('-')
        && date_part.chars().nth(7) == Some('-')
        && date_part[..4].chars().all(|c| c.is_ascii_digit())
        && date_part[5..7].chars().all(|c| c.is_ascii_digit())
        && date_part[8..10].chars().all(|c| c.is_ascii_digit());

    if !valid_date {
        result.add(ValidationIssue {
            file: doc.path.clone(),
            rule: "NAMING-001".to_string(),
            message: format!("Invalid date in filename: '{}'", date_part),
            severity: Severity::Error,
            fix_hint: None,
        });
        return;
    }

    // Check sequence number after date
    let after_date = &after_prefix[10..];
    if !after_date.starts_with('-') {
        result.add(ValidationIssue {
            file: doc.path.clone(),
            rule: "NAMING-001".to_string(),
            message: "Missing sequence number after date (expected -NNN-)".to_string(),
            severity: Severity::Error,
            fix_hint: None,
        });
    }
}

/// META-001: Check presence of required fields
fn check_required_meta(result: &mut ValidationResult, doc: &DevTrailDocument) {
    let fm = &doc.frontmatter;
    let file = &doc.path;

    let required: &[(&str, bool)] = &[
        ("id", fm.id.is_some()),
        ("title", fm.title.is_some()),
        ("status", fm.status.is_some()),
        ("created", fm.created.is_some()),
        ("agent", fm.agent.is_some()),
        ("confidence", fm.confidence.is_some()),
        ("review_required", fm.review_required.is_some()),
        ("risk_level", fm.risk_level.is_some()),
    ];

    for (field, present) in required {
        if !present {
            result.add(ValidationIssue {
                file: file.clone(),
                rule: "META-001".to_string(),
                message: format!("Missing required field: {}", field),
                severity: Severity::Error,
                fix_hint: Some(format!("Add '{}' to the frontmatter", field)),
            });
        }
    }
}

/// META-002: Check that frontmatter id matches filename prefix
fn check_id_matches_filename(result: &mut ValidationResult, doc: &DevTrailDocument) {
    if let Some(id) = &doc.frontmatter.id {
        let expected_prefix = doc.doc_type.prefix();
        if !id.starts_with(expected_prefix) {
            result.add(ValidationIssue {
                file: doc.path.clone(),
                rule: "META-002".to_string(),
                message: format!(
                    "Frontmatter id '{}' does not match filename prefix '{}'",
                    id, expected_prefix
                ),
                severity: Severity::Error,
                fix_hint: Some(format!("Change id to start with '{}-'", expected_prefix)),
            });
        }
    }
}

/// META-003: Check that status has a valid value
fn check_valid_status(result: &mut ValidationResult, doc: &DevTrailDocument) {
    if let Some(status) = &doc.frontmatter.status {
        if !VALID_STATUSES.contains(&status.as_str()) {
            result.add(ValidationIssue {
                file: doc.path.clone(),
                rule: "META-003".to_string(),
                message: format!(
                    "Invalid status '{}'. Valid values: {}",
                    status,
                    VALID_STATUSES.join(", ")
                ),
                severity: Severity::Error,
                fix_hint: None,
            });
        }
    }
}

/// CROSS-001, CROSS-002, CROSS-003: Cross-field validation rules
fn check_cross_rules(result: &mut ValidationResult, doc: &DevTrailDocument) {
    let fm = &doc.frontmatter;

    // CROSS-001: high/critical risk_level requires review_required: true
    if let Some(risk) = &fm.risk_level {
        if (risk == "high" || risk == "critical") && fm.review_required != Some(true) {
            result.add(ValidationIssue {
                file: doc.path.clone(),
                rule: "CROSS-001".to_string(),
                message: format!(
                    "risk_level is '{}' but review_required is not true",
                    risk
                ),
                severity: Severity::Error,
                fix_hint: Some("Set review_required: true".to_string()),
            });
        }
    }

    // Validate risk_level value
    if let Some(risk) = &fm.risk_level {
        if !VALID_RISK_LEVELS.contains(&risk.as_str()) {
            result.add(ValidationIssue {
                file: doc.path.clone(),
                rule: "META-003".to_string(),
                message: format!(
                    "Invalid risk_level '{}'. Valid values: {}",
                    risk,
                    VALID_RISK_LEVELS.join(", ")
                ),
                severity: Severity::Error,
                fix_hint: None,
            });
        }
    }

    // Validate confidence value
    if let Some(conf) = &fm.confidence {
        if !VALID_CONFIDENCES.contains(&conf.as_str()) {
            result.add(ValidationIssue {
                file: doc.path.clone(),
                rule: "META-003".to_string(),
                message: format!(
                    "Invalid confidence '{}'. Valid values: {}",
                    conf,
                    VALID_CONFIDENCES.join(", ")
                ),
                severity: Severity::Error,
                fix_hint: None,
            });
        }
    }

    // CROSS-002: eu_ai_act_risk: high requires review_required: true
    if let Some(eu_risk) = &fm.eu_ai_act_risk {
        if eu_risk == "high" && fm.review_required != Some(true) {
            result.add(ValidationIssue {
                file: doc.path.clone(),
                rule: "CROSS-002".to_string(),
                message: "eu_ai_act_risk is 'high' but review_required is not true".to_string(),
                severity: Severity::Error,
                fix_hint: Some("Set review_required: true".to_string()),
            });
        }
    }

    // CROSS-003: SEC, MCARD, DPIA always require review
    let always_review_types = [DocType::Sec, DocType::Mcard, DocType::Dpia];
    if always_review_types.contains(&doc.doc_type) && fm.review_required != Some(true) {
        result.add(ValidationIssue {
            file: doc.path.clone(),
            rule: "CROSS-003".to_string(),
            message: format!(
                "{} documents must have review_required: true",
                doc.doc_type
            ),
            severity: Severity::Error,
            fix_hint: Some("Set review_required: true".to_string()),
        });
    }
}

/// TYPE-001, TYPE-002: Type-specific field requirements
fn check_type_specific(result: &mut ValidationResult, doc: &DevTrailDocument) {
    let fm = &doc.frontmatter;

    // TYPE-001: INC must have severity
    if doc.doc_type == DocType::Inc && fm.severity.is_none() {
        result.add(ValidationIssue {
            file: doc.path.clone(),
            rule: "TYPE-001".to_string(),
            message: "INC documents must have a 'severity' field (SEV1/SEV2/SEV3/SEV4)".to_string(),
            severity: Severity::Error,
            fix_hint: Some("Add 'severity: SEV3' to the frontmatter".to_string()),
        });
    }

    // TYPE-002: ETH should have gdpr_legal_basis if body contains "Data Privacy"
    if doc.doc_type == DocType::Eth
        && (doc.body.contains("Data Privacy") || doc.body.contains("Privacidad de Datos"))
        && fm.gdpr_legal_basis.is_none()
    {
        result.add(ValidationIssue {
            file: doc.path.clone(),
            rule: "TYPE-002".to_string(),
            message: "ETH document mentions Data Privacy but lacks 'gdpr_legal_basis' field".to_string(),
            severity: Severity::Warning,
            fix_hint: Some("Add 'gdpr_legal_basis: consent' (or appropriate basis) to the frontmatter".to_string()),
        });
    }
}

/// REF-001: Check that documents listed in related: exist
fn check_related_exist(result: &mut ValidationResult, doc: &DevTrailDocument, devtrail_dir: &Path) {
    if let Some(related) = &doc.frontmatter.related {
        for rel_id in related {
            if rel_id.is_empty() {
                continue;
            }
            // Search for a file matching this id
            if !find_document_by_id(devtrail_dir, rel_id) {
                result.add(ValidationIssue {
                    file: doc.path.clone(),
                    rule: "REF-001".to_string(),
                    message: format!("Related document '{}' not found in .devtrail/", rel_id),
                    severity: Severity::Warning,
                    fix_hint: None,
                });
            }
        }
    }
}

/// Search for a document whose filename starts with the given id
fn find_document_by_id(devtrail_dir: &Path, id: &str) -> bool {
    let docs = document::discover_documents(devtrail_dir);
    docs.iter().any(|p| {
        p.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with(id))
            .unwrap_or(false)
    })
}

/// SEC-001: Check for sensitive information patterns
fn check_sensitive_info(result: &mut ValidationResult, doc: &DevTrailDocument) {
    let full_content = doc.body.to_string();
    for pattern in SENSITIVE_PATTERNS {
        if full_content.contains(pattern) {
            result.add(ValidationIssue {
                file: doc.path.clone(),
                rule: "SEC-001".to_string(),
                message: format!("Possible sensitive information detected: '{}'", pattern.trim()),
                severity: Severity::Error,
                fix_hint: Some("Remove or redact sensitive information before committing".to_string()),
            });
        }
    }
}

/// OBS-001: If document has tag 'observabilidad' or 'observability', check for relevant sections
fn check_observability(result: &mut ValidationResult, doc: &DevTrailDocument) {
    let has_obs_tag = doc.frontmatter.tags.as_ref().is_some_and(|tags| {
        tags.iter().any(|t| t == "observabilidad" || t == "observability")
    });

    if has_obs_tag {
        let has_obs_section = doc.body.contains("## Observability")
            || doc.body.contains("## Observabilidad")
            || doc.body.contains("instrumentation")
            || doc.body.contains("instrumentación")
            || doc.body.contains("OpenTelemetry")
            || doc.body.contains("observability_scope");

        if !has_obs_section {
            result.add(ValidationIssue {
                file: doc.path.clone(),
                rule: "OBS-001".to_string(),
                message: "Document tagged with 'observabilidad'/'observability' but lacks observability-related content".to_string(),
                severity: Severity::Warning,
                fix_hint: Some("Add a section describing the instrumentation scope or observability risks".to_string()),
            });
        }
    }
}

/// Apply automatic fixes to a document
pub fn apply_fixes(doc: &DevTrailDocument) -> Option<String> {
    let content = match std::fs::read_to_string(&doc.path) {
        Ok(c) => c,
        Err(_) => return None,
    };

    let mut modified = false;
    let mut new_content = content.clone();

    // Fix: Add review_required: true for high-risk documents
    let needs_review = doc.frontmatter.risk_level.as_deref() == Some("high")
        || doc.frontmatter.risk_level.as_deref() == Some("critical")
        || doc.frontmatter.eu_ai_act_risk.as_deref() == Some("high")
        || matches!(doc.doc_type, DocType::Sec | DocType::Mcard | DocType::Dpia);

    if needs_review && doc.frontmatter.review_required != Some(true) {
        if new_content.contains("review_required: false") {
            new_content = new_content.replacen("review_required: false", "review_required: true", 1);
            modified = true;
        } else if doc.frontmatter.review_required.is_none() {
            // Insert review_required after risk_level
            if let Some(pos) = new_content.find("risk_level:") {
                if let Some(line_end) = new_content[pos..].find('\n') {
                    let insert_pos = pos + line_end + 1;
                    new_content.insert_str(insert_pos, "review_required: true\n");
                    modified = true;
                }
            }
        }
    }

    // Fix: Correct id if it doesn't match filename prefix
    if let Some(id) = &doc.frontmatter.id {
        let expected_prefix = doc.doc_type.prefix();
        if !id.starts_with(expected_prefix) {
            // Extract date-seq from filename
            let name_no_ext = doc.filename.strip_suffix(".md").unwrap_or(&doc.filename);
            if let Some(dash_pos) = name_no_ext.find('-') {
                let after_type = &name_no_ext[dash_pos + 1..];
                if after_type.len() >= 14 {
                    let new_id = format!("{}-{}", expected_prefix, &after_type[..14]);
                    let old_id_line = format!("id: {}", id);
                    let new_id_line = format!("id: {}", new_id);
                    if new_content.contains(&old_id_line) {
                        new_content = new_content.replacen(&old_id_line, &new_id_line, 1);
                        modified = true;
                    }
                }
            }
        }
    }

    if modified {
        Some(new_content)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Frontmatter;

    fn make_doc(filename: &str, doc_type: DocType, fm: Frontmatter, body: &str) -> DevTrailDocument {
        DevTrailDocument {
            path: PathBuf::from(format!(".devtrail/test/{}", filename)),
            filename: filename.to_string(),
            doc_type,
            frontmatter: fm,
            body: body.to_string(),
        }
    }

    #[test]
    fn test_cross_001_high_risk_needs_review() {
        let fm = Frontmatter {
            id: Some("AILOG-2025-01-01-001".into()),
            risk_level: Some("high".into()),
            review_required: Some(false),
            ..Default::default()
        };
        let doc = make_doc("AILOG-2025-01-01-001-test.md", DocType::Ailog, fm, "");
        let mut result = ValidationResult::default();
        check_cross_rules(&mut result, &doc);
        assert!(result.errors.iter().any(|e| e.rule == "CROSS-001"));
    }

    #[test]
    fn test_cross_003_sec_needs_review() {
        let fm = Frontmatter {
            id: Some("SEC-2025-01-01-001".into()),
            review_required: Some(false),
            ..Default::default()
        };
        let doc = make_doc("SEC-2025-01-01-001-test.md", DocType::Sec, fm, "");
        let mut result = ValidationResult::default();
        check_cross_rules(&mut result, &doc);
        assert!(result.errors.iter().any(|e| e.rule == "CROSS-003"));
    }

    #[test]
    fn test_sec_001_sensitive_info() {
        let fm = Frontmatter::default();
        let doc = make_doc("AILOG-2025-01-01-001-test.md", DocType::Ailog, fm, "The api_key: sk-12345 was used");
        let mut result = ValidationResult::default();
        check_sensitive_info(&mut result, &doc);
        assert!(result.errors.iter().any(|e| e.rule == "SEC-001"));
    }

    #[test]
    fn test_type_001_inc_needs_severity() {
        let fm = Frontmatter {
            id: Some("INC-2025-01-01-001".into()),
            ..Default::default()
        };
        let doc = make_doc("INC-2025-01-01-001-test.md", DocType::Inc, fm, "");
        let mut result = ValidationResult::default();
        check_type_specific(&mut result, &doc);
        assert!(result.errors.iter().any(|e| e.rule == "TYPE-001"));
    }
}
