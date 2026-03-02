use anyhow::{Context, Result};
use std::path::Path;

const MARKER_BEGIN: &str = "<!-- devtrail:begin -->";
const MARKER_END: &str = "<!-- devtrail:end -->";

/// Inject a lightweight reference into a directive file (CLAUDE.md, GEMINI.md, copilot-instructions.md)
pub fn inject_reference(target: &Path) -> Result<()> {
    let reference_content = format!(
        "{}\n> **Read and follow the rules in [DEVTRAIL.md](DEVTRAIL.md).**\n> That file contains all DevTrail documentation governance rules for this project.\n{}",
        MARKER_BEGIN, MARKER_END
    );

    if target.exists() {
        let content = std::fs::read_to_string(target).context("Failed to read directive file")?;

        if content.contains(MARKER_BEGIN) {
            // Replace existing injection
            let new_content = replace_between_markers(&content, &reference_content);
            std::fs::write(target, new_content).context("Failed to write directive file")?;
        } else {
            // Append injection
            let new_content = format!("{}\n\n{}\n", content.trim_end(), reference_content);
            std::fs::write(target, new_content).context("Failed to write directive file")?;
        }
    } else {
        // Create minimal file with just the injection
        let filename = target
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");
        let header = match filename {
            "CLAUDE.md" => "# DevTrail - Claude Code Configuration",
            "GEMINI.md" => "# DevTrail - Gemini CLI Configuration",
            "copilot-instructions.md" => "# DevTrail - GitHub Copilot Configuration",
            _ => "# DevTrail Configuration",
        };
        let content = format!("{}\n\n{}\n", header, reference_content);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).context("Failed to create directory")?;
        }
        std::fs::write(target, content).context("Failed to create directive file")?;
    }

    Ok(())
}

/// Inject full DEVTRAIL.md content into .cursorrules (which doesn't support includes)
pub fn inject_full_content(target: &Path, devtrail_content: &str) -> Result<()> {
    let inline_content = format!("{}\n{}\n{}", MARKER_BEGIN, devtrail_content.trim(), MARKER_END);

    if target.exists() {
        let content = std::fs::read_to_string(target).context("Failed to read .cursorrules")?;

        if content.contains(MARKER_BEGIN) {
            let new_content = replace_between_markers(&content, &inline_content);
            std::fs::write(target, new_content).context("Failed to write .cursorrules")?;
        } else {
            let new_content = format!("{}\n\n{}\n", content.trim_end(), inline_content);
            std::fs::write(target, new_content).context("Failed to write .cursorrules")?;
        }
    } else {
        let content = format!(
            "# DevTrail - Cursor Configuration\n\n{}\n",
            inline_content
        );
        std::fs::write(target, content).context("Failed to create .cursorrules")?;
    }

    Ok(())
}

/// Create a devtrail.md file inside .cursor/rules/ directory
pub fn inject_cursor_rules_dir(target_dir: &Path, devtrail_content: &str) -> Result<()> {
    std::fs::create_dir_all(target_dir).context("Failed to create .cursor/rules/ directory")?;
    let target = target_dir.join("devtrail.md");
    let content = format!(
        "# DevTrail - Cursor Rules\n\n{}\n{}\n{}\n",
        MARKER_BEGIN,
        devtrail_content.trim(),
        MARKER_END
    );
    std::fs::write(&target, content).context("Failed to write .cursor/rules/devtrail.md")?;
    Ok(())
}

/// Remove DevTrail injection from a directive file
pub fn remove_injection(target: &Path) -> Result<bool> {
    if !target.exists() {
        return Ok(false);
    }

    let content = std::fs::read_to_string(target).context("Failed to read file")?;

    if !content.contains(MARKER_BEGIN) {
        return Ok(false);
    }

    let new_content = remove_between_markers(&content);
    let trimmed = new_content.trim();

    if trimmed.is_empty() {
        // File is empty after removal — delete it
        std::fs::remove_file(target).context("Failed to remove empty directive file")?;
    } else {
        std::fs::write(target, format!("{}\n", trimmed))
            .context("Failed to write updated file")?;
    }

    Ok(true)
}

/// Replace content between markers (inclusive)
fn replace_between_markers(content: &str, replacement: &str) -> String {
    if let (Some(start), Some(end)) = (content.find(MARKER_BEGIN), content.find(MARKER_END)) {
        let end = end + MARKER_END.len();
        format!("{}{}{}", &content[..start], replacement, &content[end..])
    } else {
        content.to_string()
    }
}

/// Remove content between markers (inclusive), including surrounding blank lines
fn remove_between_markers(content: &str) -> String {
    if let (Some(start), Some(end)) = (content.find(MARKER_BEGIN), content.find(MARKER_END)) {
        let end = end + MARKER_END.len();
        let before = content[..start].trim_end();
        let after = content[end..].trim_start();
        if after.is_empty() {
            before.to_string()
        } else {
            format!("{}\n\n{}", before, after)
        }
    } else {
        content.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_between_markers() {
        let content = "before\n<!-- devtrail:begin -->\nold\n<!-- devtrail:end -->\nafter";
        let result = replace_between_markers(
            content,
            "<!-- devtrail:begin -->\nnew\n<!-- devtrail:end -->",
        );
        assert!(result.contains("new"));
        assert!(!result.contains("old"));
        assert!(result.contains("before"));
        assert!(result.contains("after"));
    }

    #[test]
    fn test_remove_between_markers() {
        let content = "header\n\n<!-- devtrail:begin -->\nstuff\n<!-- devtrail:end -->\n\nfooter";
        let result = remove_between_markers(content);
        assert!(result.contains("header"));
        assert!(result.contains("footer"));
        assert!(!result.contains("stuff"));
    }
}
