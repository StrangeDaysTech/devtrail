# DevTrail CLI Reference

**Complete reference for the `devtrail` command-line tool.**

[![Strange Days Tech](https://img.shields.io/badge/by-Strange_Days_Tech-purple.svg)](https://strangedays.tech)

**Languages**: English

---

## Table of Contents

1. [Installation](#installation)
2. [Versioning](#versioning)
3. [Commands](#commands)
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
| Framework | `fw-` | `fw-2.1.0` | Templates, governance docs, directives, scripts |
| CLI | `cli-` | `cli-1.0.0` | The `devtrail` binary |

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
5. Copies validation scripts and CI/CD workflows

**Example:**

```bash
$ devtrail init .
✔ Downloaded DevTrail fw-2.1.0
✔ Created .devtrail/ directory structure
✔ Created DEVTRAIL.md
✔ Configured AI agent directives
✔ Copied validation scripts

DevTrail initialized successfully!
Next: git add .devtrail/ DEVTRAIL.md scripts/ && git commit -m "chore: adopt DevTrail"
```

---

### `devtrail update`

Update **both** framework and CLI to their latest versions. Equivalent to running `update-framework` followed by `update-cli`.

If `.devtrail/` does not exist in the current directory, the framework update is skipped with a warning.

**Example:**

```bash
$ devtrail update
Updating framework...
✔ Framework updated to fw-2.1.0
Updating CLI...
✔ CLI updated to cli-1.0.0
```

---

### `devtrail update-framework`

Update only the framework files. Looks for the latest `fw-*` release on GitHub.

**Conflict handling:** If you have modified framework files (e.g., governance docs or templates), the update preserves your changes and reports conflicts for manual resolution.

**Example:**

```bash
$ devtrail update-framework
✔ Framework updated to fw-2.1.0
```

---

### `devtrail update-cli`

Auto-update the `devtrail` binary itself. Looks for the latest `cli-*` release on GitHub and replaces the current binary.

**Example:**

```bash
$ devtrail update-cli
✔ CLI updated to cli-1.0.0
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

```bash
$ devtrail status
DevTrail Status
───────────────
Path:              /home/user/my-project
Framework version: fw-2.1.0
CLI version:       cli-1.0.0
Language:          en
Structure:         ✔ Complete

Documents:
  AILOG:  12
  AIDEC:   4
  ADR:     7
  REQ:     3
  TES:     2
  TDE:     1
  INC:     0
  ETH:     1
  Total:  30
```

---

### `devtrail about`

Show version, authorship, and license information.

**Example:**

```bash
$ devtrail about
DevTrail CLI
  CLI version:       cli-1.0.0
  Framework version: fw-2.1.0
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
