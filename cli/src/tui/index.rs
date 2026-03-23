use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::document::DocFrontMatter;

/// A group in the documentation hierarchy (e.g., "02-design")
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DocGroup {
    /// Directory name (e.g., "02-design")
    pub name: String,
    /// Display label (e.g., "Design")
    pub label: String,
    pub path: PathBuf,
    pub subgroups: Vec<DocSubgroup>,
    /// Files directly in this group (not in a subgroup)
    pub files: Vec<DocEntry>,
}

/// A subgroup within a group (e.g., "decisions" under "02-design")
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DocSubgroup {
    /// Directory name (e.g., "technical-debt")
    pub name: String,
    /// Display label (e.g., "Technical debt")
    pub label: String,
    pub path: PathBuf,
    /// Files directly in this subgroup
    pub files: Vec<DocEntry>,
    /// User-created subdirectories within this subgroup
    pub user_dirs: Vec<UserDir>,
}

/// A user-created subdirectory within a subgroup (e.g., "daemon" under "agent-logs")
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UserDir {
    pub name: String,
    pub path: PathBuf,
    pub files: Vec<DocEntry>,
}

/// A documentation file entry
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DocEntry {
    pub filename: String,
    pub path: PathBuf,
    /// Display title (from frontmatter, H1, or humanized filename)
    pub title: String,
    pub id: String,
    /// Short type badge: "AI", "DC", "AD", "ET", "RQ", "TS", "IN", "TD"
    pub doc_type: String,
    pub tags: Vec<String>,
    pub created: String,
    pub has_frontmatter: bool,
}

/// Bidirectional relationship index
#[derive(Debug, Default)]
pub struct RelationIndex {
    /// doc_id -> list of doc_ids it references
    pub references: HashMap<String, Vec<String>>,
    /// doc_id -> list of doc_ids that reference it
    pub referenced_by: HashMap<String, Vec<String>>,
    /// doc_id -> file path
    pub id_to_path: HashMap<String, PathBuf>,
}

/// The full documentation index
pub struct DocIndex {
    pub groups: Vec<DocGroup>,
    pub relations: RelationIndex,
    pub total_docs: usize,
}

/// Subgroup definition: (dir_name, display_label)
type SubgroupDef = (&'static str, &'static str);

/// Known documentation group definitions: (dir_name, display_label, subgroups)
const GROUP_DEFS: &[(&str, &str, &[SubgroupDef])] = &[
    ("00-governance", "Governance", &[("exceptions", "Exceptions")]),
    ("01-requirements", "Requirements", &[]),
    ("02-design", "Design", &[("decisions", "Decisions")]),
    ("03-implementation", "Implementation", &[]),
    ("04-testing", "Testing", &[]),
    (
        "05-operations",
        "Operations",
        &[("incidents", "Incidents"), ("runbooks", "Runbooks")],
    ),
    (
        "06-evolution",
        "Evolution",
        &[("technical-debt", "Technical debt")],
    ),
    (
        "07-ai-audit",
        "AI Audit",
        &[
            ("agent-logs", "Agent logs"),
            ("decisions", "Decisions"),
            ("ethical-reviews", "Ethical reviews"),
        ],
    ),
];

impl DocIndex {
    /// Build the index by scanning the .devtrail directory
    pub fn build(devtrail_dir: &Path) -> Self {
        let mut groups = Vec::new();
        let mut relations = RelationIndex::default();
        let mut total_docs = 0;

        for &(group_name, group_label, subgroup_defs) in GROUP_DEFS {
            let group_path = devtrail_dir.join(group_name);
            if !group_path.exists() {
                groups.push(DocGroup {
                    name: group_name.to_string(),
                    label: group_label.to_string(),
                    path: group_path,
                    subgroups: Vec::new(),
                    files: Vec::new(),
                });
                continue;
            }

            // Scan files directly in the group dir (non-recursive, subgroups scanned separately)
            let files = scan_md_files_flat(&group_path, &mut relations);
            total_docs += files.len();

            // Scan subgroups and their user-created subdirectories
            let mut subgroups = Vec::new();
            for &(sg_name, sg_label) in subgroup_defs {
                let sg_path = group_path.join(sg_name);
                if sg_path.exists() {
                    // Files directly in the subgroup
                    let sg_files = scan_md_files_flat(&sg_path, &mut relations);
                    total_docs += sg_files.len();

                    // Scan user-created subdirectories
                    let mut user_dirs = Vec::new();
                    if let Ok(entries) = std::fs::read_dir(&sg_path) {
                        let mut dirs: Vec<PathBuf> = entries
                            .flatten()
                            .map(|e| e.path())
                            .filter(|p| p.is_dir())
                            .collect();
                        dirs.sort();
                        for dir_path in dirs {
                            let dir_name = dir_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("")
                                .to_string();
                            let dir_files = scan_md_files(&dir_path, &mut relations);
                            total_docs += dir_files.len();
                            user_dirs.push(UserDir {
                                name: dir_name,
                                path: dir_path,
                                files: dir_files,
                            });
                        }
                    }

                    subgroups.push(DocSubgroup {
                        name: sg_name.to_string(),
                        label: sg_label.to_string(),
                        path: sg_path,
                        files: sg_files,
                        user_dirs,
                    });
                } else {
                    subgroups.push(DocSubgroup {
                        name: sg_name.to_string(),
                        label: sg_label.to_string(),
                        path: sg_path,
                        files: Vec::new(),
                        user_dirs: Vec::new(),
                    });
                }
            }

            groups.push(DocGroup {
                name: group_name.to_string(),
                label: group_label.to_string(),
                path: group_path,
                subgroups,
                files,
            });
        }

        Self {
            groups,
            relations,
            total_docs,
        }
    }

    /// Find the file path for a related link.
    /// Tries multiple resolution strategies:
    /// 1. Exact document ID match (e.g., "ADR-2025-06-15-001")
    /// 2. Filename match (e.g., "AGENT-RULES.md")
    /// 3. Path suffix match (e.g., "00-governance/AGENT-RULES.md")
    pub fn find_by_ref(&self, reference: &str) -> Option<PathBuf> {
        // 1. Try as document ID
        if let Some(path) = self.relations.id_to_path.get(reference) {
            return Some(path.clone());
        }

        // Normalize: strip leading ./ or ../ segments for matching
        let clean_ref = reference
            .trim_start_matches("../")
            .trim_start_matches("./");

        // Extract just the filename part
        let ref_filename = std::path::Path::new(clean_ref)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(clean_ref);

        // Search all entries across all groups
        let mut candidates: Vec<&PathBuf> = Vec::new();

        for group in &self.groups {
            for entry in &group.files {
                if entry_matches(&entry.filename, &entry.path, ref_filename, clean_ref) {
                    candidates.push(&entry.path);
                }
            }
            for sg in &group.subgroups {
                for entry in &sg.files {
                    if entry_matches(&entry.filename, &entry.path, ref_filename, clean_ref) {
                        candidates.push(&entry.path);
                    }
                }
                for ud in &sg.user_dirs {
                    for entry in &ud.files {
                        if entry_matches(&entry.filename, &entry.path, ref_filename, clean_ref) {
                            candidates.push(&entry.path);
                        }
                    }
                }
            }
        }

        // If exactly one match, return it. If multiple, prefer the one
        // whose path ends with the clean reference.
        match candidates.len() {
            0 => None,
            1 => Some(candidates[0].clone()),
            _ => {
                // Prefer path suffix match
                let suffix_match = candidates.iter().find(|p| {
                    p.to_str()
                        .map(|s| s.ends_with(clean_ref))
                        .unwrap_or(false)
                });
                Some(suffix_match.unwrap_or(&candidates[0]).to_path_buf())
            }
        }
    }
}

fn entry_matches(filename: &str, path: &Path, ref_filename: &str, clean_ref: &str) -> bool {
    // Exact filename match
    if filename == ref_filename {
        return true;
    }
    // Path suffix match (e.g., "00-governance/AGENT-RULES.md")
    if let Some(path_str) = path.to_str() {
        if path_str.ends_with(clean_ref) {
            return true;
        }
    }
    false
}

/// Scan only direct .md files in a directory (non-recursive, for group root dirs)
fn scan_md_files_flat(dir: &Path, relations: &mut RelationIndex) -> Vec<DocEntry> {
    let mut entries = Vec::new();

    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return entries;
    };

    let mut paths: Vec<PathBuf> = read_dir
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension().and_then(|e| e.to_str()) == Some("md")
                && !p
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with("TEMPLATE-") || n.starts_with('.'))
                    .unwrap_or(true)
        })
        .collect();

    paths.sort_by(|a, b| {
        let name_a = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let name_b = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
        name_a.cmp(name_b)
    });

    for path in paths {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let meta = quick_scan_frontmatter(&path, relations);

        entries.push(DocEntry {
            filename,
            path,
            title: meta.title,
            id: meta.id,
            doc_type: meta.doc_type,
            tags: meta.tags,
            created: meta.created,
            has_frontmatter: meta.has_frontmatter,
        });
    }

    entries
}

/// Scan .md files recursively (for subgroups that may have nested subdirectories)
fn scan_md_files(dir: &Path, relations: &mut RelationIndex) -> Vec<DocEntry> {
    let mut entries = Vec::new();
    let mut paths = Vec::new();
    collect_md_files(dir, &mut paths);
    paths.sort_by(|a, b| {
        let name_a = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let name_b = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
        name_a.cmp(name_b)
    });

    fn collect_md_files(dir: &Path, paths: &mut Vec<PathBuf>) {
        let Ok(read_dir) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_md_files(&path, paths);
            } else if path.is_file()
                && path.extension().and_then(|e| e.to_str()) == Some("md")
                && !path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with("TEMPLATE-") || n.starts_with('.'))
                    .unwrap_or(true)
            {
                paths.push(path);
            }
        }
    }

    for path in paths {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Quick frontmatter scan (just read enough to get id/title/tags/created/related)
        let meta = quick_scan_frontmatter(&path, relations);

        entries.push(DocEntry {
            filename,
            path,
            title: meta.title,
            id: meta.id,
            doc_type: meta.doc_type,
            tags: meta.tags,
            created: meta.created,
            has_frontmatter: meta.has_frontmatter,
        });
    }

    entries
}

struct ScannedMeta {
    title: String,
    id: String,
    doc_type: String,
    tags: Vec<String>,
    created: String,
    has_frontmatter: bool,
}

/// Extract doc type badge from filename prefix
fn doc_type_badge(filename: &str) -> String {
    let badges: &[(&str, &str)] = &[
        ("AILOG-", "AI"),
        ("AIDEC-", "DC"),
        ("ADR-", "AD"),
        ("ETH-", "ET"),
        ("REQ-", "RQ"),
        ("TES-", "TS"),
        ("INC-", "IN"),
        ("TDE-", "TD"),
    ];
    for &(prefix, badge) in badges {
        if filename.starts_with(prefix) {
            return badge.to_string();
        }
    }
    String::new()
}

/// Try to find the first H1 title (# Title) in markdown content
fn find_h1_title(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("# ") {
            let title = title.trim();
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

/// Convert a filename stem to a human-readable title
fn humanize_filename(stem: &str) -> String {
    stem.replace('-', " ").replace('_', " ")
}

fn fallback_meta(path: &Path, content: Option<&str>) -> ScannedMeta {
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown");
    let stem = path
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown");

    // Try H1 from content, then humanize filename
    let title = content
        .and_then(find_h1_title)
        .unwrap_or_else(|| humanize_filename(stem));

    ScannedMeta {
        title,
        id: String::new(),
        doc_type: doc_type_badge(filename),
        tags: Vec::new(),
        created: String::new(),
        has_frontmatter: false,
    }
}

fn quick_scan_frontmatter(path: &Path, relations: &mut RelationIndex) -> ScannedMeta {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return fallback_meta(path, None),
    };

    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return fallback_meta(path, Some(&content));
    }

    let after = &trimmed[3..];
    let Some(end) = after.find("\n---") else {
        return fallback_meta(path, Some(&content));
    };

    let yaml_str = &after[..end];
    let body = &after[end + 4..]; // content after closing ---
    let fm: Option<DocFrontMatter> = serde_yaml::from_str(yaml_str).ok();

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    match fm {
        Some(fm) => {
            let id = fm.id.clone();
            let title = if !fm.title.is_empty() {
                fm.title.clone()
            } else {
                // Try H1 from body, then humanize filename
                find_h1_title(body).unwrap_or_else(|| {
                    humanize_filename(
                        path.file_stem()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown"),
                    )
                })
            };
            let tags = fm.tags.clone();
            let created = fm.created.clone().unwrap_or_default();

            // Index relationships
            if !id.is_empty() {
                relations.id_to_path.insert(id.clone(), path.to_path_buf());

                if !fm.related.is_empty() {
                    for related_id in &fm.related {
                        relations
                            .referenced_by
                            .entry(related_id.clone())
                            .or_default()
                            .push(id.clone());
                    }
                    relations
                        .references
                        .entry(id.clone())
                        .or_default()
                        .extend(fm.related.iter().cloned());
                }
            }

            ScannedMeta {
                title,
                id,
                doc_type: doc_type_badge(filename),
                tags,
                created,
                has_frontmatter: true,
            }
        }
        None => fallback_meta(path, Some(body)),
    }
}
