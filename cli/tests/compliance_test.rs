use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// Helper to create a minimal DevTrail installation
fn setup_devtrail(dir: &std::path::Path) {
    let devtrail = dir.join(".devtrail");
    std::fs::create_dir_all(devtrail.join("00-governance")).unwrap();
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

/// Helper to create AI-GOVERNANCE-POLICY.md
fn create_governance_policy(dir: &std::path::Path) {
    let path = dir.join(".devtrail/00-governance");
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(
        path.join("AI-GOVERNANCE-POLICY.md"),
        "# AI Governance Policy\n\nThis is the governance policy.\n",
    )
    .unwrap();
}

#[test]
fn test_compliance_not_installed() {
    let dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("compliance")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("not installed"));
}

#[test]
fn test_compliance_no_documents() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("compliance")
        .arg("--all")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::contains("EU AI Act")
                .and(predicate::str::contains("ISO/IEC 42001"))
                .and(predicate::str::contains("NIST AI RMF")),
        );
}

#[test]
fn test_compliance_eu_ai_act_only() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("compliance")
        .arg("--standard")
        .arg("eu-ai-act")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::contains("EU AI Act")
                .and(predicate::str::contains("EU-001")),
        );
}

#[test]
fn test_compliance_iso_42001_only() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("compliance")
        .arg("--standard")
        .arg("iso-42001")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::contains("ISO/IEC 42001")
                .and(predicate::str::contains("ISO-001")),
        );
}

#[test]
fn test_compliance_nist_ai_rmf_only() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("compliance")
        .arg("--standard")
        .arg("nist-ai-rmf")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::contains("NIST AI RMF")
                .and(predicate::str::contains("NIST-MAP-001")),
        );
}

#[test]
fn test_compliance_with_governance_policy() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());
    create_governance_policy(dir.path());

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("compliance")
        .arg("--standard")
        .arg("iso-42001")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("✓").and(predicate::str::contains("ISO-001")));
}

#[test]
fn test_compliance_with_documents() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());
    create_governance_policy(dir.path());

    create_doc(
        dir.path(),
        "07-ai-audit/ethical-reviews",
        "ETH-2026-03-20-001-privacy-review.md",
        "id: ETH-2026-03-20-001\ntitle: Privacy Review\nstatus: accepted\ncreated: 2026-03-20\nagent: claude-code\nconfidence: high\nreview_required: true\nrisk_level: high\neu_ai_act_risk: high\nnist_genai_risks:\n  - privacy\n  - bias\ntags: []\nrelated: []",
    );

    create_doc(
        dir.path(),
        "07-ai-audit/agent-logs",
        "AILOG-2026-03-20-001-init.md",
        "id: AILOG-2026-03-20-001\ntitle: Init\nstatus: accepted\ncreated: 2026-03-20\nagent: claude-code\nconfidence: high\nreview_required: false\nrisk_level: low\ntags: []\nrelated: []",
    );

    create_doc(
        dir.path(),
        "07-ai-audit/decisions",
        "AIDEC-2026-03-20-001-choice.md",
        "id: AIDEC-2026-03-20-001\ntitle: Choice\nstatus: accepted\ncreated: 2026-03-20\nagent: claude-code\nconfidence: high\nreview_required: false\nrisk_level: low\ntags: []\nrelated: []",
    );

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("compliance")
        .arg("--all")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::contains("✓")
                .and(predicate::str::contains("Overall compliance")),
        );
}

#[test]
fn test_compliance_output_json() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    let output = cmd
        .arg("compliance")
        .arg("--all")
        .arg("--output")
        .arg("json")
        .arg(dir.path().to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should be valid JSON (array of reports)
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(parsed.is_ok(), "Output is not valid JSON: {}", stdout);
}

#[test]
fn test_compliance_output_markdown() {
    let dir = TempDir::new().unwrap();
    setup_devtrail(dir.path());

    let mut cmd = Command::cargo_bin("devtrail").unwrap();
    cmd.arg("compliance")
        .arg("--all")
        .arg("--output")
        .arg("markdown")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::contains("# DevTrail Compliance Report")
                .and(predicate::str::contains("| Check |")),
        );
}
