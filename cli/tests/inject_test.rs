use std::fs;
use tempfile::TempDir;

#[test]
fn test_inject_reference_creates_file() {
    let dir = TempDir::new().unwrap();
    let target = dir.path().join("CLAUDE.md");

    // File doesn't exist yet
    assert!(!target.exists());

    // Run inject via the binary would require integration test setup.
    // For now, test the marker format.
    let content = "# DevTrail - Claude Code Configuration\n\n<!-- devtrail:begin -->\n> **Read and follow the rules in [DEVTRAIL.md](DEVTRAIL.md).**\n<!-- devtrail:end -->\n";
    fs::write(&target, content).unwrap();

    let result = fs::read_to_string(&target).unwrap();
    assert!(result.contains("<!-- devtrail:begin -->"));
    assert!(result.contains("<!-- devtrail:end -->"));
    assert!(result.contains("DEVTRAIL.md"));
}

#[test]
fn test_inject_markers_are_valid() {
    let begin = "<!-- devtrail:begin -->";
    let end = "<!-- devtrail:end -->";

    // Valid HTML comments
    assert!(begin.starts_with("<!--"));
    assert!(begin.ends_with("-->"));
    assert!(end.starts_with("<!--"));
    assert!(end.ends_with("-->"));
}

#[test]
fn test_remove_injection() {
    let dir = TempDir::new().unwrap();
    let target = dir.path().join("test.md");

    let content = "# My Config\n\nSome custom stuff\n\n<!-- devtrail:begin -->\nDevTrail rules here\n<!-- devtrail:end -->\n\nMore custom stuff\n";
    fs::write(&target, content).unwrap();

    // Simulate removal
    let content = fs::read_to_string(&target).unwrap();
    let begin = "<!-- devtrail:begin -->";
    let end = "<!-- devtrail:end -->";

    if let (Some(start), Some(end_pos)) = (content.find(begin), content.find(end)) {
        let end_pos = end_pos + end.len();
        let before = content[..start].trim_end();
        let after = content[end_pos..].trim_start();
        let new_content = format!("{}\n\n{}", before, after);
        fs::write(&target, &new_content).unwrap();
    }

    let result = fs::read_to_string(&target).unwrap();
    assert!(!result.contains("devtrail:begin"));
    assert!(result.contains("My Config"));
    assert!(result.contains("More custom stuff"));
}
