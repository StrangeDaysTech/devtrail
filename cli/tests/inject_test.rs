use std::fs;
use tempfile::TempDir;

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
fn test_inject_reference_creates_file() {
    let dir = TempDir::new().unwrap();
    let target = dir.path().join("CLAUDE.md");

    // Simulate what inject_directive does with a reference template
    let template = "# DevTrail - Claude Code Configuration\n\n<!-- devtrail:begin -->\n> **Read and follow the rules in [DEVTRAIL.md](DEVTRAIL.md).**\n> That file contains all DevTrail documentation governance rules for this project.\n<!-- devtrail:end -->\n";
    fs::write(&target, template).unwrap();

    let result = fs::read_to_string(&target).unwrap();
    assert!(result.contains("<!-- devtrail:begin -->"));
    assert!(result.contains("<!-- devtrail:end -->"));
    assert!(result.contains("DEVTRAIL.md"));
    assert!(result.contains("# DevTrail - Claude Code Configuration"));
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

#[test]
fn test_template_with_embed_markers() {
    let dir = TempDir::new().unwrap();
    let target = dir.path().join(".cursorrules");

    // Template with empty markers (embed-style)
    let template = "# DevTrail - Cursor Configuration\n\n<!-- devtrail:begin -->\n<!-- devtrail:end -->\n";
    let embed_content = "# DEVTRAIL Rules\nRule 1: Document everything\nRule 2: Follow conventions";

    // Simulate inject_directive with embed: build marker block with embedded content
    let marker_block = format!(
        "<!-- devtrail:begin -->\n{}\n<!-- devtrail:end -->",
        embed_content.trim()
    );
    let full_content = template.replace(
        "<!-- devtrail:begin -->\n<!-- devtrail:end -->",
        &marker_block,
    );
    fs::write(&target, &full_content).unwrap();

    let result = fs::read_to_string(&target).unwrap();
    assert!(result.contains("# DevTrail - Cursor Configuration"));
    assert!(result.contains("Rule 1: Document everything"));
    assert!(result.contains("Rule 2: Follow conventions"));
    assert!(result.contains("<!-- devtrail:begin -->"));
    assert!(result.contains("<!-- devtrail:end -->"));
}
