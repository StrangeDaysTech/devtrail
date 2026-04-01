# DevTrail CLI Reference

**Complete reference for the `devtrail` command-line tool.**

[![Strange Days Tech](https://img.shields.io/badge/by-Strange_Days_Tech-purple.svg)](https://strangedays.tech)

**Languages**: English

---

## Table of Contents

1. [Installation](#installation)
2. [Versioning](#versioning)
3. [Commands](#commands) — init, update, remove, status, repair, validate, new, compliance, metrics, analyze, audit, explore, about
4. [Environment Variables](#environment-variables)
5. [Exit Codes](#exit-codes)

---

## Installation

Install the DevTrail CLI using one of the methods below. For full setup instructions, see the [README](../../README.md#getting-started).

**Quick install (prebuilt binary):**

```bash
# Linux / macOS
curl -fsSL https://raw.githubusercontent.com/StrangeDaysTech/devtrail/main/install.sh | sh
```

```powershell
# Windows (PowerShell)
irm https://raw.githubusercontent.com/StrangeDaysTech/devtrail/main/install.ps1 | iex
```

**From source:**

```bash
cargo install devtrail-cli
```

---

## Versioning

DevTrail uses **independent version tags** for each component:

| Component | Tag prefix | Example | What it includes |
|-----------|-----------|---------|------------------|
| Framework | `fw-` | `fw-4.0.0` | Templates (12 types), governance docs, directives |
| CLI | `cli-` | `cli-2.1.0` | The `devtrail` binary |

Framework and CLI are released independently. A framework update does not require a CLI update, and vice versa.

**Check installed versions:**

```bash
devtrail about    # Shows CLI version + framework version (if installed)
devtrail status   # Shows full installation health including versions
```

---

## Commands

### `devtrail init [path]`

Initialize DevTrail in a project directory.

**Arguments:**

| Argument | Default | Description |
|----------|---------|-------------|
| `path` | `.` (current directory) | Target project directory |

**What it does:**

1. Downloads the latest framework release (`fw-*`) from GitHub
2. Creates the `.devtrail/` directory structure
3. Creates `DEVTRAIL.md` with governance rules
4. Configures AI agent directive files (`CLAUDE.md`, `GEMINI.md`, `.cursorrules`, etc.)
5. Copies CI/CD workflows

**Example:**

```bash
$ devtrail init .
✔ Downloaded DevTrail fw-4.0.0
✔ Created .devtrail/ directory structure
✔ Created DEVTRAIL.md
✔ Configured AI agent directives

DevTrail initialized successfully!
Next: git add .devtrail/ DEVTRAIL.md && git commit -m "chore: adopt DevTrail"
```

---

### `devtrail update`

Update **both** framework and CLI to their latest versions. Equivalent to running `update-framework` followed by `update-cli`.

If `.devtrail/` does not exist in the current directory, the framework update is skipped with a warning.

**Example:**

```bash
$ devtrail update
Updating framework...
✔ Framework updated to fw-4.0.0
Updating CLI...
✔ CLI updated to cli-2.1.0
```

---

### `devtrail update-framework`

Update only the framework files. Looks for the latest `fw-*` release on GitHub.

**Conflict handling:** If you have modified framework files (e.g., governance docs or templates), the update preserves your changes and reports conflicts for manual resolution.

**Example:**

```bash
$ devtrail update-framework
✔ Framework updated to fw-4.0.0
```

---

### `devtrail update-cli`

Auto-update the `devtrail` binary itself. Looks for the latest `cli-*` release on GitHub and replaces the current binary.

**Example:**

```bash
$ devtrail update-cli
✔ CLI updated to cli-2.1.0
```

---

### `devtrail remove [--full]`

Remove DevTrail from the current project.

**Flags:**

| Flag | Description |
|------|-------------|
| `--full` | Remove everything, including user-created documents in `.devtrail/`. Asks for confirmation. |

**Default behavior** (without `--full`): removes the framework structure but preserves documents you created inside `.devtrail/`.

**Example:**

```bash
$ devtrail remove
✔ DevTrail framework removed. User documents preserved in .devtrail/.

$ devtrail remove --full
⚠ This will delete all DevTrail files including your documents.
Continue? [y/N]: y
✔ DevTrail completely removed.
```

---

### `devtrail status [path]`

Show installation health and documentation statistics.

**Arguments:**

| Argument | Default | Description |
|----------|---------|-------------|
| `path` | `.` (current directory) | Target project directory |

**Output includes:**

- Project path
- Framework version
- CLI version
- Configured language
- Directory structure integrity
- Document statistics (count by type)

**Example:**

```
$ devtrail status

  ╔════════════════════════════════════════════════╗
  ║ DevTrail Status                                ║
  ╚════════════════════════════════════════════════╝

  Project
  ┌───────────┬──────────────────────────┐
  │ Path      │ /home/user/my-project    │
  │ Framework │ fw-4.0.0                 │
  │ CLI       │ cli-2.1.0                │
  │ Language  │ en                       │
  └───────────┴──────────────────────────┘

  Structure
  ✓ All 15 items present
  ┌──────────────────────────────┬────────┐
  │ Directory / File             │ Status │
  ├──────────────────────────────┼────────┤
  │ 00-governance/               │ ✓ OK   │
  │ ...                          │ ...    │
  └──────────────────────────────┴────────┘

  Documentation
  ┌──────────────────────────────┬───────┐
  │ Type                         │ Count │
  ├──────────────────────────────┼───────┤
  │ AILOG AI Action Logs         │    12 │
  │ ADR   Architecture Decisions │     7 │
  │ ...                          │   ... │
  ├──────────────────────────────┼───────┤
  │ Total                        │    30 │
  └──────────────────────────────┴───────┘

  → Run devtrail explore to browse documentation interactively
```

---

### `devtrail repair [path]`

Repair a broken DevTrail installation by restoring missing directories and framework files.

**Arguments:**

| Argument | Default | Description |
|----------|---------|-------------|
| `path` | `.` (current directory) | Target project directory |

**What it does:**

1. Checks for missing directories and restores them with `.gitkeep`
2. Downloads the framework release **once** if files need restoration (templates, governance docs, config)
3. Re-injects directives if `DEVTRAIL.md` is missing
4. Recalculates checksums after repair
5. Never modifies or deletes user-generated documents

**Example:**

```bash
$ devtrail repair
Repairing DevTrail in /home/user/my-project
  → Found 1 issue(s) to repair
→ Restoring 1 missing directory...
✓ Restored .devtrail/templates/
→ Downloading framework to restore missing files...
  Using version: fw-4.0.0
✓ Restored 16 file(s) from framework
→ Updating checksums...

✓ DevTrail repaired successfully!
```

---

### `devtrail validate [path] [--fix] [--staged]`

Validate DevTrail documents for compliance and correctness.

**Arguments and flags:**

| Argument/Flag | Default | Description |
|---------------|---------|-------------|
| `path` | `.` (current directory) | Target project directory |
| `--fix` | — | Automatically fix simple issues (e.g., missing `review_required: true` for high-risk docs) |
| `--staged` | — | Validate only staged (git-added) files. Ideal for pre-commit hooks. |

**What it checks:**

- Naming conventions (`TYPE-YYYY-MM-DD-NNN-description.md`)
- Required metadata fields (id, title, status, created, agent, confidence, review_required, risk_level, tags, related)
- Cross-field consistency (e.g., high risk must have review_required)
- Type-specific fields (e.g., INC needs severity, SEC needs threat_model_methodology)
- Sensitive information detection (API keys, passwords)
- Related document existence

**Example:**

```bash
$ devtrail validate
  DevTrail Validate
  All 15 document(s) passed validation
  0 error(s), 0 warning(s) in 15 document(s)

$ devtrail validate --fix
  DevTrail Validate
  Auto-fixing 2 issue(s)...
  ✓ Fixed 2 issue(s)
```

---

### `devtrail new [path] [-t <type>] [--title <title>]`

Create a new DevTrail document from a template.

**Arguments and flags:**

| Argument/Flag | Default | Description |
|---------------|---------|-------------|
| `path` | `.` (current directory) | Target project directory |
| `--doc-type`, `-t` | — | Document type: `ailog`, `aidec`, `adr`, `eth`, `req`, `tes`, `inc`, `tde`, `sec`, `mcard`, `sbom`, `dpia` |
| `--title` | — | Title for the new document |

If `--doc-type` or `--title` are omitted, the command prompts interactively.

**Examples:**

```bash
# Interactive — prompts for type and title
$ devtrail new

# Create an AILOG with a title (non-interactive)
$ devtrail new -t ailog --title "Implement JWT authentication"

# Create an ADR
$ devtrail new --doc-type adr --title "Use PostgreSQL for persistence"
```

**Example output:**

```
$ devtrail new -t ailog --title "Implement JWT authentication"

  ✔ Created: .devtrail/07-ai-audit/agent-logs/AILOG-2026-04-01-001-implement-jwt-authentication.md

  Next steps:
    1. Edit the document to fill in details
    2. Commit: git add .devtrail/07-ai-audit/agent-logs/AILOG-2026-04-01-001-implement-jwt-authentication.md
```

---

### `devtrail compliance [path] [--standard <name>] [--all] [--output <format>]`

Check regulatory compliance against EU AI Act, ISO/IEC 42001, and NIST AI RMF.

**Arguments and flags:**

| Argument/Flag | Default | Description |
|---------------|---------|-------------|
| `path` | `.` (current directory) | Target project directory |
| `--standard` | — | Check a specific standard: `eu-ai-act`, `iso-42001`, or `nist-ai-rmf` |
| `--all` | — | Check all three standards |
| `--output` | `text` | Output format: `text`, `markdown`, or `json` |

If neither `--standard` nor `--all` is specified, defaults to `--all`.

**What it checks:**

- **EU AI Act**: Risk classification, ethical review linkage, DPIA existence, incident reporting
- **ISO/IEC 42001**: Governance policy, risk planning (ETH), operations documentation (AILOG/AIDEC), Annex A coverage
- **NIST AI RMF**: MAP (AILOG), MEASURE (TES), MANAGE (ETH/INC), GOVERN (policy + ADR), GenAI risk coverage (12 NIST 600-1 categories)

**Example:**

```bash
$ devtrail compliance --all
  DevTrail Compliance
  /home/user/my-project
  12 document(s) analyzed

  ■ EU AI Act 75%
    ✓ [EU-001] AI systems have EU AI Act risk classification
    ~ [EU-002] High-risk AI systems have ethical review (ETH) linked
    ✓ [EU-003] Data Protection Impact Assessment (DPIA) exists where required
    ✓ [EU-004] Incident reporting compliant with EU AI Act Art. 73

  ■ ISO/IEC 42001 100%
    ✓ [ISO-001] AI Governance Policy exists (Clauses 4-5)
    ✓ [ISO-002] Risk planning documented — ETH reviews exist (Clause 6)
    ✓ [ISO-003] AI lifecycle operations documented — AILOG + AIDEC (Clause 8)
    ✓ [ISO-004] Annex A control coverage (6/6 groups)

  ■ NIST AI RMF 60%
    ✓ [NIST-MAP-001] MAP function — AI actions documented (AILOG)
    ✓ [NIST-MEASURE-001] MEASURE function — Test plans exist (TES)
    ✓ [NIST-MANAGE-001] MANAGE function — Risk management documented (ETH + INC)
    ✓ [NIST-GOVERN-001] GOVERN function — Governance policy and decisions documented
    ~ [NIST-GENAI-001] GenAI risk coverage — NIST AI 600-1 (4/12 categories)

  Overall compliance: 78%

$ devtrail compliance --standard eu-ai-act --output json
[{"standard":"EuAiAct","checks":[...],"score":75.0}]
```

---

### `devtrail metrics [path] [--period <period>] [--output <format>]`

Show governance metrics and documentation statistics.

**Arguments and flags:**

| Argument/Flag | Default | Description |
|---------------|---------|-------------|
| `path` | `.` (current directory) | Target project directory |
| `--period` | `last-30-days` | Time period: `last-7-days`, `last-30-days`, `last-90-days`, or `all` |
| `--output` | `text` | Output format: `text`, `markdown`, or `json` |

**Metrics included:**

- Document count by type within the period
- Review compliance rate (% of review_required docs that reached accepted/superseded status)
- Risk distribution (low/medium/high/critical)
- Agent activity (documents per agent)
- Trends vs previous period (↑/↓/→)

**Example:**

```bash
$ devtrail metrics --period last-30-days
  DevTrail Metrics
  /home/user/my-project
  Period: Last 30 days — 2026-02-25 to 2026-03-27

  Documents by Type
     AILOG   8 ████████
       ETH   3 ███
       ADR   2 ██
       INC   1 █

  Summary
    → Total documents: 14
    → Review compliance: 80% (4/5 reviewed)

  Risk Distribution
          low 8
       medium 4
         high 2

  Agent Activity
    claude-code 10
    gemini-cli 4

  Trends
    ↑ Total documents 14 (was 9)
    ↑ Reviews completed 4 (was 2)
    → High/critical risk 2 (was 2)
```

---

### `devtrail analyze [path] [--threshold <N>] [--output <format>] [--top <N>]`

Analyze code complexity using cognitive and cyclomatic metrics powered by [arborist-metrics](https://crates.io/crates/arborist-metrics).

**Arguments and flags:**

| Argument/Flag | Default | Description |
|---------------|---------|-------------|
| `path` | `.` (current directory) | Target directory to analyze |
| `--threshold` | `8` (or from config) | Cognitive complexity threshold |
| `--output` | `text` | Output format: `text`, `json`, or `markdown` |
| `--top` | — | Show only top N most complex functions |

**Supported languages:** Rust, Python, JavaScript, TypeScript, Java, Go, C, C++, C#, PHP, Kotlin, Swift

**Threshold resolution:** CLI flag → `.devtrail/config.yml` → default (8)

**Configuration** (optional, in `.devtrail/config.yml`):

```yaml
complexity:
  threshold: 8
```

**Examples:**

```bash
# Analyze current directory
$ devtrail analyze

# Custom threshold and top 10
$ devtrail analyze --threshold 5 --top 10

# JSON output for CI integration
$ devtrail analyze --output json

# Analyze a specific project
$ devtrail analyze /path/to/project
```

**Example output:**

```
  DevTrail Analyze
  /home/user/project
  Threshold: cognitive complexity > 8

  Functions exceeding threshold (3 of 42 total)

    FILE                                     FUNCTION                  LINE  COGN  CYCL  SLOC
    src/parser.rs                            parse_expression            42    18    12    45
    src/compiler.rs                          Compiler::emit             128    15     9    38
    src/eval.rs                              evaluate                    67    12     8    29

  Summary
    → Files analyzed: 12
    → Total functions: 42
    → Above threshold: 3 (7.1%)
    → Max cognitive complexity: 18 (src/parser.rs:parse_expression)
    → Average cognitive complexity: 3.8
```

> **Note:** This command works without `devtrail init`. It operates on source files, not DevTrail documents. The `analyze` feature can be disabled at compile time with `--no-default-features`.

---

### `devtrail audit [path] [--from <date>] [--to <date>] [--system <name>] [--output <format>]`

Generate audit trail reports with timeline, traceability map, and compliance summary.

**Arguments and flags:**

| Argument/Flag | Default | Description |
|---------------|---------|-------------|
| `path` | `.` (current directory) | Target project directory |
| `--from` | — | Start date for audit period (YYYY-MM-DD) |
| `--to` | — | End date for audit period (YYYY-MM-DD) |
| `--system` | — | Filter by system/component name (matches tags and title) |
| `--output` | `text` | Output format: `text`, `markdown`, `json`, or `html` |

**Report includes:**

- Chronological timeline of all documents with type, title, agent, and risk level
- Traceability map showing document relationship chains (e.g., REQ → ADR → AILOG → TES)
- Risk distribution (low/medium/high/critical)
- Compliance summary (EU AI Act, ISO 42001, NIST AI RMF scores)

**Output formats:**

| Format | Use case |
|--------|----------|
| `text` | Terminal review (colored, formatted) |
| `markdown` | Include in PRs, wikis, or reports |
| `json` | Integration with external tools |
| `html` | Standalone reports with styled tables and SVG risk chart |

**Examples:**

```bash
# Full audit report
$ devtrail audit

# Audit for Q1 2026
$ devtrail audit --from 2026-01-01 --to 2026-03-31

# Audit filtered by system
$ devtrail audit --system auth-service

# Generate HTML report
$ devtrail audit --from 2026-01-01 --to 2026-03-31 --output html > audit-q1.html

# Generate Markdown for a PR
$ devtrail audit --output markdown
```

---

### `devtrail explore [path]`

Browse and read DevTrail documentation interactively in a terminal UI.

**Arguments:**

| Argument | Default | Description |
|----------|---------|-------------|
| `path` | `.` (current directory) | Target project directory |

**Features:**

- Two-panel layout: navigation tree + document viewer
- Metadata panel showing status, confidence, risk, tags, and related links
- Markdown rendering with colors, tables, code blocks, and heading indentation
- Navigate between related documents via hyperlinks
- Search by filename, title, tags, or date
- Fullscreen document mode, vim-style keybindings

**Key bindings:**

| Key | Action |
|-----|--------|
| `↑↓` / `j/k` | Navigate / Scroll |
| `Enter` | Expand group / Open document |
| `Tab` | Cycle panels: Navigation → Metadata → Document |
| `f` | Toggle fullscreen document |
| `/` | Search |
| `Esc` | Back / Collapse / Clear search |
| `?` | Help popup with all shortcuts |
| `q` | Quit |

**Example:**

```bash
$ devtrail explore
```

> **Note:** The `explore` command requires the `tui` feature (enabled by default). To compile without it: `cargo build --no-default-features`.

---

### `devtrail about`

Show version, authorship, and license information.

**Example:**

```bash
$ devtrail about
DevTrail CLI
  CLI version:       cli-2.1.0
  Framework version: fw-4.0.0
  Author:            Strange Days Tech, S.A.S.
  License:           MIT
  Repository:        https://github.com/StrangeDaysTech/devtrail
  Website:           https://strangedays.tech
```

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `GITHUB_TOKEN` | GitHub personal access token for authenticated API requests. Useful to avoid rate limits when downloading releases. |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Error (details printed to stderr) |

---

<div align="center">

**DevTrail** — Because every change tells a story.

[Back to docs](../README.md) • [README](../../README.md) • [Strange Days Tech](https://strangedays.tech)

</div>
