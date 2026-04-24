# Changelog

All notable changes to DevTrail will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [independent versioning](README.md#versioning) for Framework (`fw-`) and CLI (`cli-`).

---

## CLI 3.2.3 — UTF-8 Crash Fix in `explore` Tables

### Fixed (CLI)
- Fix panic in `devtrail explore` when rendering Markdown tables whose cells contain multi-byte UTF-8 characters (em-dash `—`, CJK ideograms, accented characters, emoji). Cell wrapping now uses `char_indices()` for safe slicing and measures text in visual columns via `unicode-width`, so table borders also stay aligned with Chinese and double-wide content.

---

## CLI 3.2.2 — crates.io README Broken Links Fix

### Fixed (CLI)
- Convert all relative links in README to absolute GitHub URLs so badges, documentation links, and language switcher all resolve correctly on crates.io

---

## CLI 3.2.1 — crates.io README Language Links Fix

### Fixed (CLI)
- Use absolute GitHub URLs for Español and 简体中文 language links in README so they resolve correctly on crates.io

---

## Framework 4.2.0 / CLI 3.2.0 — Simplified Chinese (zh-CN) Localization

### Added (Framework)
- **Simplified Chinese (zh-CN)**: Complete localization as the third supported language alongside English and Spanish
  - 12 document templates (AILOG, ADR, AIDEC, DPIA, ETH, INC, MCARD, REQ, SBOM, SEC, TDE, TES)
  - 12 governance documents (AGENT-RULES, AI-GOVERNANCE-POLICY, AI-KPIS, AI-LIFECYCLE-TRACKER, AI-RISK-CATALOG, C4-DIAGRAM-GUIDE, DOCUMENTATION-POLICY, GIT-BRANCHING-STRATEGY, MANAGEMENT-REVIEW-TEMPLATE, OBSERVABILITY-GUIDE, PRINCIPLES, QUICK-REFERENCE)
  - 5 NIST implementation guides (AI RMF Govern/Map/Measure/Manage + GenAI Risks)
  - 6 user-facing docs (README, CONTRIBUTING, CODE_OF_CONDUCT, ADOPTION-GUIDE, CLI-REFERENCE, WORKFLOWS)

### Added (CLI)
- **Generic language support**: Template resolution now supports any configured language, not just hardcoded `es`

### Changed (Framework)
- Language navigation links updated across all three languages (EN, ES, zh-CN) in governance and documentation files
- Language navigation links added to English governance files (PRINCIPLES, DOCUMENTATION-POLICY, AGENT-RULES) that previously lacked them

---

## CLI 3.1.1 — crates.io README Fix

### Fixed (CLI)
- Include project README in crates.io package (copy from repo root during CI publish)
- Restore `readme` field in `Cargo.toml` pointing to local copy

---

## CLI 3.1.0 — crates.io Distribution & Smart Self-Update

### Added (CLI)
- **crates.io distribution**: `cargo install devtrail-cli` now available as an installation method
- **Install method detection**: `devtrail update-cli` auto-detects whether the CLI was installed via cargo or prebuilt binary and uses the appropriate update mechanism
- **`--method` flag**: Override auto-detection on `update-cli` and `update` commands (`auto`, `github`, `cargo`)
- **`devtrail about`**: Now displays the detected installation method
- **CI**: `release-cli.yml` workflow publishes to crates.io after GitHub Release upload

### Changed (CLI)
- `Cargo.toml`: Added `include` field for crate packaging, removed `readme` path (outside crate boundary)

---

## Framework 4.1.1 — Status Skill Complexity Fix

### Fixed (Framework)
- **devtrail-status skill**: Replace outdated ">10 lines of changes" heuristic with `devtrail analyze --output json` (cognitive complexity, threshold 8) as the primary method for AILOG triggers, with >20 lines fallback when CLI is unavailable
- Updated across all 3 platform variants: Claude Code, Gemini, and generic agent workflow

---

## CLI 3.0.1 — Validate False Positive Fix

### Fixed (CLI)
- **REF-001**: Only validate `related:` references that match DevTrail document ID patterns (AILOG-*, AIDEC-*, ADR-*, etc.). Skips task IDs, requirement IDs, risk IDs, external paths, and other non-document references
- **SEC-001**: `Bearer` and `token:` moved from errors to warnings — common in documentation describing auth flows. Actual secret patterns remain as errors

---

## Framework 4.1.0 / CLI 3.0.0 — Complexity-Based Documentation & Ecosystem

### Added (CLI)
- **`devtrail analyze`** command — Code complexity analysis (cognitive + cyclomatic) powered by [arborist-metrics](https://github.com/StrangeDaysTech/arborist)
  - Output formats: text (colored), json, markdown
  - 12 languages: Rust, Python, JavaScript, TypeScript, Java, Go, C, C++, C#, PHP, Kotlin, Swift
  - Configurable threshold: CLI flag → `.devtrail/config.yml` → default (8)
- **`devtrail new`** command — Interactive document creation with type suggestion based on context
  - Supports all 12 document types
  - Analyzes git diff to suggest appropriate type
- **`--staged` flag** for `devtrail validate` — Validate only staged documents (pre-commit hooks)

### Changed (Framework)
- **Documentation triggers redesigned**: `devtrail analyze --output json` is now the primary method for determining when to create AILOGs. The >20 lines heuristic is preserved as fallback when the CLI is unavailable
  - Updated across all governance docs, agent directives, skills/workflows (18 files, EN + ES)
- Agent directives (Claude, Gemini, Copilot, Cursor) updated with complexity-based pre-commit checklist

### Changed (CLI)
- All 12 arborist-metrics languages enabled (was limited subset)
- Legacy scripts removed, replaced with CLI commands in all docs

### Added (Docs)
- arborist-metrics promotion in README (EN + ES) — Open Source Ecosystem table
- Documentation trigger notes in CLI-REFERENCE (EN + ES)

### Changed (CI/CD)
- Release workflows unified: both trigger on tag push with automatic release creation
- Idempotent releases: create if missing, upload with `--clobber` if exists
- Auto-delete previous releases on new version (keeps only latest per component)
- GitHub Actions updated to Node.js 24 compatible versions (checkout v6, upload-artifact v7, download-artifact v8)
- Version verification: workflows check Cargo.toml / dist-manifest.yml matches tag
- `workflow_dispatch` added to both workflows for manual re-runs

### Removed
- Legacy `auditoria/` directory (one-time audit reviews, findings already addressed)
- Legacy `docs/archive/` (obsolete planning documents)
- Legacy shell scripts (replaced by CLI commands)

---

## Framework 4.0.0 / CLI 2.1.0 — Advanced Automation & Ecosystem

### Added (CLI)
- **`devtrail audit`** command — Generate audit trail reports with timeline, traceability map, risk distribution, and compliance summary
  - Output formats: text (colored terminal), markdown, json, html (with SVG pie chart)
  - Filters: `--from`/`--to` date range, `--system` component filter
  - Traceability graph built from document `related:` fields using BFS

### Added (Framework)
- **C4-DIAGRAM-GUIDE.md** — Complete guide for C4 Model diagrams with Mermaid syntax (EN + ES)
  - Examples for all 4 levels: Context, Container, Component, Code
  - PlantUML alternative syntax
  - Integration guidance for ADR and REQ documents
- `api_changes` field in TEMPLATE-ADR.md frontmatter for tracking API endpoint changes
- `api_spec_path` field in TEMPLATE-REQ.md frontmatter for OpenAPI/AsyncAPI spec references
- Architecture Diagram section in TEMPLATE-ADR.md with Mermaid C4 placeholder
- Sections 10 (C4 Model) and 11 (API Specification Tracking) in AGENT-RULES.md
- Terminal compatibility notes in skill files for box-drawing character fallback

### Changed
- QUICK-REFERENCE.md: Added C4 Model reference to regulatory alignment table
- Updated CLI-REFERENCE.md, README.md with 13 commands (EN + ES)

---

## Framework 3.2.0 / CLI 2.0.0 — Compliance Automation & Metrics

### Added (CLI)
- **`devtrail compliance`** command — Check regulatory compliance (EU AI Act, ISO 42001, NIST AI RMF)
  - Output formats: text, markdown, json
  - Per-standard or `--all` mode with percentage scores
- **`devtrail metrics`** command — Governance metrics and documentation statistics
  - Period filtering, review compliance rate, risk distribution, agent activity, trends

### Added (Framework)
- AI-RISK-CATALOG.md — Risk catalog mapped to 12 NIST AI 600-1 categories + ISO 42001 Annex C
- AI-LIFECYCLE-TRACKER.md — AI system lifecycle tracking mapped to ISO 42001 Annex A.6
- AI-KPIS.md — Governance KPI tracking template
- MANAGEMENT-REVIEW-TEMPLATE.md — ISO 42001 Clause 9.3 review agenda
- OBSERVABILITY-GUIDE.md — OpenTelemetry integration guide with 10 sections (EN + ES)
- NIST AI RMF implementation guides: MAP, MEASURE, MANAGE, GOVERN
- NIST-AI-600-1-GENAI-RISKS.md — Detailed 12 GenAI risk categories

---

## Framework 3.1.0 / CLI 1.4.0 — New Document Types & Validation

### Added (CLI)
- **`devtrail validate`** command — Validate documents with 13 rules (NAMING, META, CROSS, TYPE, REF, SEC, OBS)
  - `--fix` flag for automatic corrections
  - Exit code 1 on errors, 0 on warnings-only
- Document parsing engine (`document.rs`) — Shared by validate, compliance, metrics, audit
- Validation engine (`validation.rs`) — Extensible rule-based validation

### Added (Framework)
- **TEMPLATE-SEC.md** — Security Assessment (STRIDE threat model, OWASP ASVS)
- **TEMPLATE-MCARD.md** — Model/System Card (Mitchell et al. 2019)
- **TEMPLATE-SBOM.md** — Software Bill of Materials (SPDX/CycloneDX aligned)
- **TEMPLATE-DPIA.md** — Data Protection Impact Assessment (GDPR Art. 35)
- Skills: `/devtrail-sec`, `/devtrail-mcard` (Claude, Gemini, generic agent)
- Updated `/devtrail-new` and `/devtrail-status` for 12 document types
- Compliance CI jobs in docs-validation.yml

---

## Framework 3.0.0 / CLI 1.3.0 — Regulatory Base & Standards Update

### Changed (Framework)
- **IEEE 830 → ISO/IEC/IEEE 29148:2018** in TEMPLATE-REQ.md (External Interfaces, V&V, Traceability)
- **ISO/IEC 25010:2011 → 2023** in TEMPLATE-ADR.md and TEMPLATE-REQ.md (9 quality characteristics)
- **ISO/IEC/IEEE 29119-3:2021** alignment in TEMPLATE-TES.md (3-level hierarchy, 29119 terminology)
- Regulatory fields added to all templates: `eu_ai_act_risk`, `nist_genai_risks`, `iso_42001_clause`
- OpenTelemetry optional sections in TEMPLATE-REQ, TEMPLATE-TES, TEMPLATE-INC, TEMPLATE-AILOG

### Added (Framework)
- **AI-GOVERNANCE-POLICY.md** — ISO 42001 Clauses 4-10 governance template
- **ISO-25010-2023-REFERENCE.md** — Quality characteristics reference
- EU AI Act, NIST GenAI, GDPR sections in ETH, INC, and AILOG templates
- Observability rules in AGENT-RULES.md (Section 9)
- Expanded agent directives with pre-commit checklists
- New folders: `08-security/`, `09-ai-models/`

### Added (CLI)
- Support for 12 document types (was 8): SEC, MCARD, SBOM, DPIA
- New directories in `init`, `status`, `repair`, `explore`

### Changed (CLI)
- Cross-validation rules in pre-commit hooks and CI

---

*DevTrail is maintained by [Strange Days Tech](https://strangedays.tech).*
