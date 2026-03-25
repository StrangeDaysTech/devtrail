use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// Helper to create a minimal DevTrail installation
fn setup_devtrail(dir: &std::path::Path) {
    let devtrail = dir.join(".devtrail");
    std::fs::create_dir_all(devtrail.join("07-ai-audit/agent-logs")).unwrap();
    std::fs::create_dir_all(devtrail.join("07-ai-audit/decisions")).unwrap();
    std::fs::create_dir_all(devtrail.join("07-ai-audit/ethical-reviews")).unwrap();
    std::fs::create_dir_all(devtrail.join("08-security")).unwrap();
    std::fs::create_dir_all(devtrail.join("09-ai-models")).unwrap();
    std::fs::create_dir_all(devtrail.join("05-operations/incidents")).unwrap();
    std::fs::create_dir_all(devtrail.join("templates")).unwrap();
    std::fs::write(devtrail.join("config.yml"), "language: en\n").unwrap();
    std::fs::write(
        devtrail.join("dist-manifest.yml"),
        "version: \"3.0.0\"\ndescription: test\n",
    )
    .unwrap();
}

/// Helper to create a document file with frontmatter
fn create_doc(dir: &std::path::Path, subpath: &str, filename: &str, frontmatter: &str) {
    let path = dir.join(".devtrail").join(subpath);
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(
        path.join(filename),
        format!("---\n{}\n---\n\n# Document\n", frontmatter),
    )
    .unwrap();
}

#[test]
fn test_validate_not_installed() {
    let dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("not installed"));
}

#[test]
fn test_validate_no_documents() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("No documents found"));
}

#[test]
fn test_validate_valid_document() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit/agent-logs",
        "AILOG-2025-01-27-001-implement-auth.md",
        "id: AILOG-2025-01-27-001\ntitle: Implement auth\nstatus: draft\ncreated: 2025-01-27\nagent: claude-code-v1.0\nconfidence: high\nreview_required: false\nrisk_level: low\ntags: []\nrelated: []",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("passed validation"));
}

#[test]
fn test_validate_missing_frontmatter_fields() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit/agent-logs",
        "AILOG-2025-01-27-001-test.md",
        "id: AILOG-2025-01-27-001\ntitle: Test",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains("META-001"));
}

#[test]
fn test_validate_invalid_status() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit/agent-logs",
        "AILOG-2025-01-27-001-test.md",
        "id: AILOG-2025-01-27-001\ntitle: Test\nstatus: invalid_status\ncreated: 2025-01-27\nagent: test\nconfidence: high\nreview_required: false\nrisk_level: low",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains("META-003"));
}

#[test]
fn test_validate_cross_001_high_risk_no_review() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit/agent-logs",
        "AILOG-2025-01-27-001-test.md",
        "id: AILOG-2025-01-27-001\ntitle: Test\nstatus: draft\ncreated: 2025-01-27\nagent: test\nconfidence: high\nreview_required: false\nrisk_level: high",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains("CROSS-001"));
}

#[test]
fn test_validate_sensitive_info_detected() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    let path = dir.path().join(".devtrail/07-ai-audit/agent-logs");
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(
        path.join("AILOG-2025-01-27-001-secrets.md"),
        "---\nid: AILOG-2025-01-27-001\ntitle: Test\nstatus: draft\ncreated: 2025-01-27\nagent: test\nconfidence: high\nreview_required: false\nrisk_level: low\n---\n\nThe api_key: sk-12345 was leaked\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains("SEC-001"));
}

#[test]
fn test_validate_related_not_found() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit/agent-logs",
        "AILOG-2025-01-27-001-test.md",
        "id: AILOG-2025-01-27-001\ntitle: Test\nstatus: draft\ncreated: 2025-01-27\nagent: test\nconfidence: high\nreview_required: false\nrisk_level: low\nrelated:\n  - AIDEC-2025-01-27-001",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success() // Warnings don't cause failure
        .stdout(predicate::str::contains("REF-001"));
}

#[test]
fn test_validate_sec_requires_review() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "08-security",
        "SEC-2025-01-27-001-api-review.md",
        "id: SEC-2025-01-27-001\ntitle: API Review\nstatus: draft\ncreated: 2025-01-27\nagent: test\nconfidence: medium\nreview_required: false\nrisk_level: high",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains("CROSS-003"));
}

#[test]
fn test_validate_fix_review_required() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    let doc_path = dir
        .path()
        .join(".devtrail/08-security/SEC-2025-01-27-001-fix-test.md");
    std::fs::write(
        &doc_path,
        "---\nid: SEC-2025-01-27-001\ntitle: Fix Test\nstatus: draft\ncreated: 2025-01-27\nagent: test\nconfidence: medium\nreview_required: false\nrisk_level: high\n---\n\n# Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg("--fix")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .stdout(predicate::str::contains("Fixed"));

    // Verify the file was modified
    let content = std::fs::read_to_string(&doc_path).unwrap();
    assert!(content.contains("review_required: true"));
}

#[test]
fn test_validate_obs_001_tag_without_content() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit/agent-logs",
        "AILOG-2025-01-27-001-obs.md",
        "id: AILOG-2025-01-27-001\ntitle: Obs Test\nstatus: draft\ncreated: 2025-01-27\nagent: test\nconfidence: high\nreview_required: false\nrisk_level: low\ntags:\n  - observability",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success() // Warnings don't cause failure
        .stdout(predicate::str::contains("OBS-001"));
}

#[test]
fn test_validate_inc_needs_severity() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "05-operations/incidents",
        "INC-2025-01-27-001-outage.md",
        "id: INC-2025-01-27-001\ntitle: Outage\nstatus: draft\ncreated: 2025-01-27\nagent: test\nconfidence: high\nreview_required: true\nrisk_level: high",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains("TYPE-001"));
}

// =============================================================================
// F2.QA.02 — Verification of new templates
// =============================================================================

/// F2.QA.02.01 — Create a test document for each new type (SEC, MCARD, SBOM, DPIA)
/// and validate with `devtrail validate`
#[test]
fn test_validate_sec_document_valid() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "08-security",
        "SEC-2026-03-24-001-api-assessment.md",
        "id: SEC-2026-03-24-001\ntitle: API Security Assessment\nstatus: draft\ncreated: 2026-03-24\nagent: claude-code-v1.0\nconfidence: medium\nreview_required: true\nrisk_level: high\nthreat_model_methodology: STRIDE\nowasp_asvs_level: 1\ntags:\n  - security\nrelated: []",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("passed validation"));
}

#[test]
fn test_validate_mcard_document_valid() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "09-ai-models",
        "MCARD-2026-03-24-001-gpt4-turbo.md",
        "id: MCARD-2026-03-24-001\ntitle: GPT-4 Turbo Card\nstatus: draft\ncreated: 2026-03-24\nagent: claude-code-v1.0\nconfidence: medium\nreview_required: true\nrisk_level: medium\nmodel_name: gpt-4-turbo\nmodel_type: LLM\ntags:\n  - ai-model\nrelated: []",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("passed validation"));
}

#[test]
fn test_validate_sbom_document_valid() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit",
        "SBOM-2026-03-24-001-platform-deps.md",
        "id: SBOM-2026-03-24-001\ntitle: Platform AI SBOM\nstatus: accepted\ncreated: 2026-03-24\nagent: claude-code-v1.0\nconfidence: high\nreview_required: false\nrisk_level: low\ntags:\n  - sbom\nrelated: []",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("passed validation"));
}

#[test]
fn test_validate_dpia_document_valid() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit/ethical-reviews",
        "DPIA-2026-03-24-001-user-profiling.md",
        "id: DPIA-2026-03-24-001\ntitle: User Profiling DPIA\nstatus: draft\ncreated: 2026-03-24\nagent: claude-code-v1.0\nconfidence: low\nreview_required: true\nrisk_level: high\ntags:\n  - privacy\n  - gdpr\nrelated: []",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("passed validation"));
}

/// F2.QA.02.01 — Also verify that MCARD and DPIA fail without review_required: true
#[test]
fn test_validate_mcard_requires_review() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "09-ai-models",
        "MCARD-2026-03-24-001-no-review.md",
        "id: MCARD-2026-03-24-001\ntitle: Test\nstatus: draft\ncreated: 2026-03-24\nagent: test\nconfidence: medium\nreview_required: false\nrisk_level: medium\ntags: []\nrelated: []",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains("CROSS-003"));
}

#[test]
fn test_validate_dpia_requires_review() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit/ethical-reviews",
        "DPIA-2026-03-24-001-no-review.md",
        "id: DPIA-2026-03-24-001\ntitle: Test\nstatus: draft\ncreated: 2026-03-24\nagent: test\nconfidence: low\nreview_required: false\nrisk_level: high\ntags: []\nrelated: []",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("validate")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains("CROSS-003"));
}

/// F2.QA.02.03 — Verify that new templates and directories exist in dist/
#[test]
fn test_new_templates_exist_in_dist() {
    let dist_templates = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("dist/.devtrail/templates");

    // EN templates
    assert!(dist_templates.join("TEMPLATE-SEC.md").exists(), "TEMPLATE-SEC.md (EN) missing");
    assert!(dist_templates.join("TEMPLATE-MCARD.md").exists(), "TEMPLATE-MCARD.md (EN) missing");
    assert!(dist_templates.join("TEMPLATE-SBOM.md").exists(), "TEMPLATE-SBOM.md (EN) missing");
    assert!(dist_templates.join("TEMPLATE-DPIA.md").exists(), "TEMPLATE-DPIA.md (EN) missing");

    // ES templates
    let es = dist_templates.join("i18n/es");
    assert!(es.join("TEMPLATE-SEC.md").exists(), "TEMPLATE-SEC.md (ES) missing");
    assert!(es.join("TEMPLATE-MCARD.md").exists(), "TEMPLATE-MCARD.md (ES) missing");
    assert!(es.join("TEMPLATE-SBOM.md").exists(), "TEMPLATE-SBOM.md (ES) missing");
    assert!(es.join("TEMPLATE-DPIA.md").exists(), "TEMPLATE-DPIA.md (ES) missing");

    // New directories
    let devtrail = dist_templates.parent().unwrap();
    assert!(devtrail.join("08-security").exists(), "08-security/ directory missing");
    assert!(devtrail.join("09-ai-models").exists(), "09-ai-models/ directory missing");
}

/// F2.QA.02.02 — Verify devtrail-new.sh has all 12 types configured
#[test]
fn test_devtrail_new_script_has_all_types() {
    let script_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("dist/scripts/devtrail-new.sh");

    let content = std::fs::read_to_string(&script_path).expect("Cannot read devtrail-new.sh");

    // Verify all 12 types are in DOC_PATHS
    for doc_type in &["ailog", "aidec", "adr", "eth", "req", "tes", "inc", "tde", "sec", "mcard", "sbom", "dpia"] {
        assert!(
            content.contains(&format!("[\"{}\"]", doc_type)),
            "devtrail-new.sh missing DOC_PATHS entry for '{}'", doc_type
        );
    }

    // Verify the menu has 12 options
    assert!(content.contains("1-12 or name"), "devtrail-new.sh menu does not show 1-12 range");
}
