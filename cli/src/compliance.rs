use serde::Serialize;
use std::path::Path;

use crate::document::{DevTrailDocument, DocType};

/// The 12 NIST AI 600-1 GenAI risk categories (canonical identifiers)
pub const NIST_GENAI_CATEGORIES: &[&str] = &[
    "cbrn",
    "confabulation",
    "dangerous_content",
    "privacy",
    "environmental",
    "bias",
    "human_ai_config",
    "information_integrity",
    "information_security",
    "intellectual_property",
    "obscene_content",
    "value_chain",
];

/// Which regulatory standard to check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Standard {
    EuAiAct,
    Iso42001,
    NistAiRmf,
}

impl Standard {
    pub fn label(&self) -> &'static str {
        match self {
            Standard::EuAiAct => "EU AI Act",
            Standard::Iso42001 => "ISO/IEC 42001",
            Standard::NistAiRmf => "NIST AI RMF",
        }
    }
}

/// Status of a single compliance check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CheckStatus {
    Pass,
    Partial,
    Fail,
}

/// A single compliance check result
#[derive(Debug, Clone, Serialize)]
pub struct ComplianceCheck {
    pub id: String,
    pub description: String,
    pub status: CheckStatus,
    pub evidence: Vec<String>,
    pub remediation: Option<String>,
}

/// Result of running one standard's checker
#[derive(Debug, Serialize)]
pub struct ComplianceReport {
    pub standard: Standard,
    pub standard_label: String,
    pub checks: Vec<ComplianceCheck>,
    pub score: f64,
}

/// Check if a governance file exists in the devtrail directory
fn governance_file_exists(devtrail_dir: &Path, filename: &str) -> bool {
    devtrail_dir.join("00-governance").join(filename).exists()
}

/// Count documents of a specific type
fn count_type(docs: &[DevTrailDocument], doc_type: DocType) -> usize {
    docs.iter().filter(|d| d.doc_type == doc_type).count()
}

/// Collect document IDs of a specific type
fn ids_of_type(docs: &[DevTrailDocument], doc_type: DocType) -> Vec<String> {
    docs.iter()
        .filter(|d| d.doc_type == doc_type)
        .filter_map(|d| d.frontmatter.id.clone())
        .collect()
}

/// Calculate score from checks
fn calculate_score(checks: &[ComplianceCheck]) -> f64 {
    if checks.is_empty() {
        return 0.0;
    }
    let total = checks.len() as f64;
    let passed = checks.iter().filter(|c| c.status == CheckStatus::Pass).count() as f64;
    let partial = checks
        .iter()
        .filter(|c| c.status == CheckStatus::Partial)
        .count() as f64;
    ((passed + partial * 0.5) / total) * 100.0
}

// ---------------------------------------------------------------------------
// EU AI Act checker
// ---------------------------------------------------------------------------

pub fn check_eu_ai_act(docs: &[DevTrailDocument], _governance_dir: &Path) -> ComplianceReport {
    let mut checks = Vec::new();

    // EU-001: Risk classification — at least one doc classifies EU AI Act risk
    {
        let classified: Vec<String> = docs
            .iter()
            .filter(|d| {
                d.frontmatter
                    .eu_ai_act_risk
                    .as_deref()
                    .is_some_and(|r| r != "not_applicable")
            })
            .filter_map(|d| d.frontmatter.id.clone())
            .collect();

        let status = if classified.is_empty() {
            CheckStatus::Fail
        } else {
            CheckStatus::Pass
        };
        checks.push(ComplianceCheck {
            id: "EU-001".into(),
            description: "AI systems have EU AI Act risk classification".into(),
            status,
            evidence: classified,
            remediation: Some(
                "Set eu_ai_act_risk field in documents involving AI systems".into(),
            ),
        });
    }

    // EU-002: High-risk docs have ETH linked in related
    {
        let high_risk_docs: Vec<&DevTrailDocument> = docs
            .iter()
            .filter(|d| d.frontmatter.eu_ai_act_risk.as_deref() == Some("high"))
            .collect();

        if high_risk_docs.is_empty() {
            checks.push(ComplianceCheck {
                id: "EU-002".into(),
                description: "High-risk AI systems have ethical review (ETH) linked".into(),
                status: CheckStatus::Pass,
                evidence: vec!["No high-risk systems — check not applicable".into()],
                remediation: None,
            });
        } else {
            let eth_ids: Vec<String> = ids_of_type(docs, DocType::Eth);
            let mut linked = Vec::new();
            let mut unlinked = Vec::new();

            for doc in &high_risk_docs {
                let has_eth_related = doc
                    .frontmatter
                    .related
                    .as_ref()
                    .is_some_and(|rels| rels.iter().any(|r| r.starts_with("ETH-")));
                let doc_id = doc.frontmatter.id.clone().unwrap_or_default();
                if has_eth_related {
                    linked.push(doc_id);
                } else {
                    unlinked.push(doc_id);
                }
            }

            let status = if unlinked.is_empty() {
                CheckStatus::Pass
            } else if linked.is_empty() && !eth_ids.is_empty() {
                CheckStatus::Partial
            } else if eth_ids.is_empty() {
                CheckStatus::Fail
            } else {
                CheckStatus::Partial
            };

            checks.push(ComplianceCheck {
                id: "EU-002".into(),
                description: "High-risk AI systems have ethical review (ETH) linked".into(),
                status,
                evidence: linked,
                remediation: if !unlinked.is_empty() {
                    Some(format!(
                        "Link ETH documents in 'related' for: {}",
                        unlinked.join(", ")
                    ))
                } else {
                    None
                },
            });
        }
    }

    // EU-003: DPIA exists if needed
    {
        let needs_dpia = docs.iter().any(|d| d.frontmatter.gdpr_article_35 == Some(true));
        let has_dpia = count_type(docs, DocType::Dpia) > 0;

        if !needs_dpia {
            checks.push(ComplianceCheck {
                id: "EU-003".into(),
                description: "Data Protection Impact Assessment (DPIA) exists where required"
                    .into(),
                status: CheckStatus::Pass,
                evidence: vec!["No GDPR Art. 35 triggers found — check not applicable".into()],
                remediation: None,
            });
        } else {
            checks.push(ComplianceCheck {
                id: "EU-003".into(),
                description: "Data Protection Impact Assessment (DPIA) exists where required"
                    .into(),
                status: if has_dpia {
                    CheckStatus::Pass
                } else {
                    CheckStatus::Fail
                },
                evidence: ids_of_type(docs, DocType::Dpia),
                remediation: if !has_dpia {
                    Some("Create a DPIA document for processing with gdpr_article_35: true".into())
                } else {
                    None
                },
            });
        }
    }

    // EU-004: Incident reporting compliance
    {
        let inc_docs: Vec<&DevTrailDocument> = docs
            .iter()
            .filter(|d| d.doc_type == DocType::Inc)
            .collect();

        if inc_docs.is_empty() {
            checks.push(ComplianceCheck {
                id: "EU-004".into(),
                description: "Incident reporting compliant with EU AI Act Art. 73".into(),
                status: CheckStatus::Pass,
                evidence: vec!["No incidents recorded — check not applicable".into()],
                remediation: None,
            });
        } else {
            let with_severity: Vec<String> = inc_docs
                .iter()
                .filter(|d| d.frontmatter.severity.is_some())
                .filter_map(|d| d.frontmatter.id.clone())
                .collect();

            let status = if with_severity.len() == inc_docs.len() {
                CheckStatus::Pass
            } else if !with_severity.is_empty() {
                CheckStatus::Partial
            } else {
                CheckStatus::Fail
            };

            checks.push(ComplianceCheck {
                id: "EU-004".into(),
                description: "Incident reporting compliant with EU AI Act Art. 73".into(),
                status,
                evidence: with_severity,
                remediation: Some(
                    "Ensure all INC documents have severity and report deadlines".into(),
                ),
            });
        }
    }

    let score = calculate_score(&checks);
    ComplianceReport {
        standard: Standard::EuAiAct,
        standard_label: Standard::EuAiAct.label().into(),
        checks,
        score,
    }
}

// ---------------------------------------------------------------------------
// ISO/IEC 42001 checker
// ---------------------------------------------------------------------------

pub fn check_iso_42001(docs: &[DevTrailDocument], governance_dir: &Path) -> ComplianceReport {
    let mut checks = Vec::new();

    // ISO-001: AI Governance Policy exists
    {
        let exists = governance_file_exists(governance_dir, "AI-GOVERNANCE-POLICY.md");
        checks.push(ComplianceCheck {
            id: "ISO-001".into(),
            description: "AI Governance Policy exists (Clauses 4-5)".into(),
            status: if exists {
                CheckStatus::Pass
            } else {
                CheckStatus::Fail
            },
            evidence: if exists {
                vec!["AI-GOVERNANCE-POLICY.md".into()]
            } else {
                vec![]
            },
            remediation: if !exists {
                Some("Run 'devtrail init' or 'devtrail repair' to restore governance files".into())
            } else {
                None
            },
        });
    }

    // ISO-002: Risk planning — at least one ETH (Clause 6)
    {
        let eth_ids = ids_of_type(docs, DocType::Eth);
        checks.push(ComplianceCheck {
            id: "ISO-002".into(),
            description: "Risk planning documented — ETH reviews exist (Clause 6)".into(),
            status: if eth_ids.is_empty() {
                CheckStatus::Fail
            } else {
                CheckStatus::Pass
            },
            evidence: eth_ids.clone(),
            remediation: if eth_ids.is_empty() {
                Some("Create at least one ETH (Ethical Review) document for risk planning".into())
            } else {
                None
            },
        });
    }

    // ISO-003: Operations documented — AILOG + AIDEC exist (Clause 8)
    {
        let has_ailog = count_type(docs, DocType::Ailog) > 0;
        let has_aidec = count_type(docs, DocType::Aidec) > 0;

        let status = if has_ailog && has_aidec {
            CheckStatus::Pass
        } else if has_ailog || has_aidec {
            CheckStatus::Partial
        } else {
            CheckStatus::Fail
        };

        let mut evidence = ids_of_type(docs, DocType::Ailog);
        evidence.extend(ids_of_type(docs, DocType::Aidec));

        checks.push(ComplianceCheck {
            id: "ISO-003".into(),
            description: "AI lifecycle operations documented — AILOG + AIDEC (Clause 8)".into(),
            status,
            evidence,
            remediation: if status != CheckStatus::Pass {
                Some("Document AI actions (AILOG) and decisions (AIDEC) during development".into())
            } else {
                None
            },
        });
    }

    // ISO-004: Annex A coverage via document types
    {
        // Annex A groups and the doc types that cover them
        let annex_a_groups: &[(&str, &[DocType])] = &[
            ("A.5 Impact Assessment", &[DocType::Eth, DocType::Dpia]),
            (
                "A.6 AI Lifecycle",
                &[DocType::Ailog, DocType::Aidec, DocType::Adr, DocType::Mcard],
            ),
            ("A.7 Data for AI", &[DocType::Sbom, DocType::Mcard]),
            ("A.8 Information", &[DocType::Adr, DocType::Req]),
            ("A.9 Use of AI", &[DocType::Ailog]),
            ("A.10 Third-Party", &[DocType::Sbom]),
        ];

        let mut covered = Vec::new();
        let mut missing = Vec::new();

        for (group_name, types) in annex_a_groups {
            let has_any = types.iter().any(|t| count_type(docs, *t) > 0);
            if has_any {
                covered.push((*group_name).to_string());
            } else {
                missing.push((*group_name).to_string());
            }
        }

        let total = annex_a_groups.len();
        let status = if covered.len() == total {
            CheckStatus::Pass
        } else if !covered.is_empty() {
            CheckStatus::Partial
        } else {
            CheckStatus::Fail
        };

        checks.push(ComplianceCheck {
            id: "ISO-004".into(),
            description: format!(
                "Annex A control coverage ({}/{} groups)",
                covered.len(),
                total
            ),
            status,
            evidence: covered,
            remediation: if !missing.is_empty() {
                Some(format!("Missing coverage for: {}", missing.join(", ")))
            } else {
                None
            },
        });
    }

    let score = calculate_score(&checks);
    ComplianceReport {
        standard: Standard::Iso42001,
        standard_label: Standard::Iso42001.label().into(),
        checks,
        score,
    }
}

// ---------------------------------------------------------------------------
// NIST AI RMF checker
// ---------------------------------------------------------------------------

pub fn check_nist_ai_rmf(docs: &[DevTrailDocument], governance_dir: &Path) -> ComplianceReport {
    let mut checks = Vec::new();

    // NIST-MAP-001: MAP function — AILOG documents exist (context)
    {
        let ids = ids_of_type(docs, DocType::Ailog);
        checks.push(ComplianceCheck {
            id: "NIST-MAP-001".into(),
            description: "MAP function — AI actions documented (AILOG)".into(),
            status: if ids.is_empty() {
                CheckStatus::Fail
            } else {
                CheckStatus::Pass
            },
            evidence: ids.clone(),
            remediation: if ids.is_empty() {
                Some("Create AILOG documents to map AI system context".into())
            } else {
                None
            },
        });
    }

    // NIST-MEASURE-001: MEASURE function — TES documents exist
    {
        let ids = ids_of_type(docs, DocType::Tes);
        checks.push(ComplianceCheck {
            id: "NIST-MEASURE-001".into(),
            description: "MEASURE function — Test plans exist (TES)".into(),
            status: if ids.is_empty() {
                CheckStatus::Fail
            } else {
                CheckStatus::Pass
            },
            evidence: ids.clone(),
            remediation: if ids.is_empty() {
                Some("Create TES documents to measure AI system trustworthiness".into())
            } else {
                None
            },
        });
    }

    // NIST-MANAGE-001: MANAGE function — ETH + INC exist
    {
        let has_eth = count_type(docs, DocType::Eth) > 0;
        let has_inc = count_type(docs, DocType::Inc) > 0;

        let status = if has_eth && has_inc {
            CheckStatus::Pass
        } else if has_eth || has_inc {
            CheckStatus::Partial
        } else {
            CheckStatus::Fail
        };

        let mut evidence = ids_of_type(docs, DocType::Eth);
        evidence.extend(ids_of_type(docs, DocType::Inc));

        checks.push(ComplianceCheck {
            id: "NIST-MANAGE-001".into(),
            description: "MANAGE function — Risk management documented (ETH + INC)".into(),
            status,
            evidence,
            remediation: if status != CheckStatus::Pass {
                Some("Create ETH and INC documents for risk management".into())
            } else {
                None
            },
        });
    }

    // NIST-GOVERN-001: GOVERN function — governance policy + ADR exist
    {
        let has_policy = governance_file_exists(governance_dir, "AI-GOVERNANCE-POLICY.md");
        let has_adr = count_type(docs, DocType::Adr) > 0;

        let status = if has_policy && has_adr {
            CheckStatus::Pass
        } else if has_policy || has_adr {
            CheckStatus::Partial
        } else {
            CheckStatus::Fail
        };

        let mut evidence: Vec<String> = Vec::new();
        if has_policy {
            evidence.push("AI-GOVERNANCE-POLICY.md".into());
        }
        evidence.extend(ids_of_type(docs, DocType::Adr));

        checks.push(ComplianceCheck {
            id: "NIST-GOVERN-001".into(),
            description: "GOVERN function — Governance policy and decisions documented".into(),
            status,
            evidence,
            remediation: if status != CheckStatus::Pass {
                Some("Ensure AI-GOVERNANCE-POLICY.md exists and create ADR documents".into())
            } else {
                None
            },
        });
    }

    // NIST-GENAI-001: GenAI risk coverage (12 NIST 600-1 categories)
    {
        let mut covered_categories: Vec<String> = Vec::new();
        for doc in docs {
            if let Some(risks) = &doc.frontmatter.nist_genai_risks {
                for risk in risks {
                    let r = risk.to_lowercase();
                    if NIST_GENAI_CATEGORIES.contains(&r.as_str())
                        && !covered_categories.contains(&r)
                    {
                        covered_categories.push(r);
                    }
                }
            }
        }

        let coverage = covered_categories.len();
        let total = NIST_GENAI_CATEGORIES.len();

        let status = if coverage == total {
            CheckStatus::Pass
        } else if coverage > 0 {
            CheckStatus::Partial
        } else {
            CheckStatus::Fail
        };

        let missing: Vec<&str> = NIST_GENAI_CATEGORIES
            .iter()
            .filter(|c| !covered_categories.contains(&c.to_string()))
            .copied()
            .collect();

        checks.push(ComplianceCheck {
            id: "NIST-GENAI-001".into(),
            description: format!(
                "GenAI risk coverage — NIST AI 600-1 ({}/{} categories)",
                coverage, total
            ),
            status,
            evidence: covered_categories,
            remediation: if !missing.is_empty() {
                Some(format!(
                    "Add nist_genai_risks entries for: {}",
                    missing.join(", ")
                ))
            } else {
                None
            },
        });
    }

    let score = calculate_score(&checks);
    ComplianceReport {
        standard: Standard::NistAiRmf,
        standard_label: Standard::NistAiRmf.label().into(),
        checks,
        score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Frontmatter;
    use std::path::PathBuf;

    fn make_doc(
        filename: &str,
        doc_type: DocType,
        fm: Frontmatter,
        body: &str,
    ) -> DevTrailDocument {
        DevTrailDocument {
            path: PathBuf::from(format!(".devtrail/test/{}", filename)),
            filename: filename.to_string(),
            doc_type,
            frontmatter: fm,
            body: body.to_string(),
        }
    }

    #[test]
    fn test_eu_ai_act_empty_docs() {
        let report = check_eu_ai_act(&[], &PathBuf::from("/tmp"));
        assert_eq!(report.standard, Standard::EuAiAct);
        // EU-001 should fail with no docs
        assert_eq!(report.checks[0].status, CheckStatus::Fail);
    }

    #[test]
    fn test_eu_ai_act_with_classified_doc() {
        let docs = vec![make_doc(
            "AILOG-2026-01-01-001-test.md",
            DocType::Ailog,
            Frontmatter {
                id: Some("AILOG-2026-01-01-001".into()),
                eu_ai_act_risk: Some("high".into()),
                ..Default::default()
            },
            "",
        )];
        let report = check_eu_ai_act(&docs, &PathBuf::from("/tmp"));
        assert_eq!(report.checks[0].status, CheckStatus::Pass);
    }

    #[test]
    fn test_iso_42001_partial() {
        let docs = vec![make_doc(
            "ETH-2026-01-01-001-review.md",
            DocType::Eth,
            Frontmatter {
                id: Some("ETH-2026-01-01-001".into()),
                ..Default::default()
            },
            "",
        )];
        let report = check_iso_42001(&docs, &PathBuf::from("/tmp"));
        // ISO-001 should fail (no governance dir)
        assert_eq!(report.checks[0].status, CheckStatus::Fail);
        // ISO-002 should pass (ETH exists)
        assert_eq!(report.checks[1].status, CheckStatus::Pass);
    }

    #[test]
    fn test_nist_genai_coverage() {
        let docs = vec![make_doc(
            "ETH-2026-01-01-001-review.md",
            DocType::Eth,
            Frontmatter {
                nist_genai_risks: Some(vec!["bias".into(), "privacy".into()]),
                ..Default::default()
            },
            "",
        )];
        let report = check_nist_ai_rmf(&docs, &PathBuf::from("/tmp"));
        // NIST-GENAI-001 should be partial (2/12)
        let genai_check = report.checks.iter().find(|c| c.id == "NIST-GENAI-001").unwrap();
        assert_eq!(genai_check.status, CheckStatus::Partial);
        assert_eq!(genai_check.evidence.len(), 2);
    }

    #[test]
    fn test_calculate_score() {
        let checks = vec![
            ComplianceCheck {
                id: "T-001".into(),
                description: "test".into(),
                status: CheckStatus::Pass,
                evidence: vec![],
                remediation: None,
            },
            ComplianceCheck {
                id: "T-002".into(),
                description: "test".into(),
                status: CheckStatus::Fail,
                evidence: vec![],
                remediation: None,
            },
        ];
        let score = calculate_score(&checks);
        assert!((score - 50.0).abs() < 0.01);
    }
}
