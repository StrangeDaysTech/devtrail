# DevTrail - Recommended Workflows

**Patterns and cadences for using DevTrail day to day.**

[![Strange Days Tech](https://img.shields.io/badge/by-Strange_Days_Tech-purple.svg)](https://strangedays.tech)

**Languages**: English | [Español](../i18n/es/adopters/WORKFLOWS.md) | [简体中文](../i18n/zh-CN/adopters/WORKFLOWS.md)

---

## Table of Contents

1. [After Initial Setup](#after-initial-setup)
2. [Daily Development](#daily-development)
3. [Keeping DevTrail Updated](#keeping-devtrail-updated)
4. [Checking Project Health](#checking-project-health)
5. [Using Skills (Active Documentation)](#using-skills-active-documentation)
6. [Team Patterns](#team-patterns)
7. [Understanding Versions](#understanding-versions)

---

## After Initial Setup

You ran `devtrail init .` and committed the result. Now what?

1. **Open your project** with your AI coding assistant (Claude Code, Cursor, Gemini CLI, etc.)
2. The assistant will **automatically read** the DevTrail directives (`CLAUDE.md`, `GEMINI.md`, etc.)
3. From this point on, the assistant **creates documentation** in `.devtrail/` as part of its normal workflow
4. **No extra configuration needed** — DevTrail works passively through the directive files

---

## Daily Development

### The Passive Loop

1. Work normally with your AI assistant — write features, fix bugs, refactor
2. The AI creates documents in `.devtrail/` according to the governance rules:
   - **AILOG** for significant implementations (>10 lines changed)
   - **AIDEC** when choosing between alternatives
   - **ADR** for architectural decisions
   - **ETH** when ethical concerns arise
3. Review documents flagged with `review_required: true`
4. Commit documentation together with the corresponding code changes

### When to Create Documents Manually

Use the active system (skills) when:

- The AI missed documenting a significant change
- You (a human) made a decision that should be recorded
- You want to create a REQ, TES, TDE, or INC document
- You want to check documentation compliance

---

## Keeping DevTrail Updated

### Recommended Cadence

- **Monthly** or when you see a new release on GitHub
- Check the [releases page](https://github.com/StrangeDaysTech/devtrail/releases) for changelogs

### Update Commands

| Goal | Command |
|------|---------|
| Update both framework and CLI | `devtrail update` |
| Update only templates and governance files | `devtrail update-framework` |
| Update only the CLI binary | `devtrail update-cli` |

Framework and CLI have **independent versions** — you can update one without the other. See [Understanding Versions](#understanding-versions).

### After Updating

1. Review changes to directive files and governance docs
2. Commit the updated files: `git add .devtrail/ && git commit -m "chore: update DevTrail framework"`
3. If you customized any framework files, check for conflicts

---

## Checking Project Health

### CLI Status

```bash
devtrail status
```

Shows: framework version, CLI version, directory structure integrity, and document statistics by type. Use it to verify that the installation is healthy.

### Documentation Compliance (Skill)

```bash
/devtrail-status
```

The `/devtrail-status` skill (available in Claude Code and Gemini CLI) analyzes:

- Which recent code changes lack corresponding documentation
- Document compliance against governance rules
- Overall documentation health

---

## Using Skills (Active Documentation)

DevTrail has two documentation systems:

| System | How it works | When to use |
|--------|-------------|-------------|
| **Passive** | AI auto-documents via directive files | Default — happens automatically |
| **Active** | User invokes skills to create docs | When passive missed something, or for human decisions |

### Available Skills

| Skill | Purpose |
|-------|---------|
| `/devtrail-status` | Check documentation compliance |
| `/devtrail-new` | Create any document type (suggests best fit) |
| `/devtrail-ailog` | Quick AILOG creation |
| `/devtrail-aidec` | Quick AIDEC creation |
| `/devtrail-adr` | Quick ADR creation |

For full skill details, see the [README](../../README.md#skills).

---

## Team Patterns

### PR Reviews

- Check that significant code changes include corresponding `.devtrail/` documents
- Review any documents with `review_required: true`
- Verify that AILOGs accurately describe what the AI did

### Onboarding New Team Members

1. Point them to `.devtrail/QUICK-REFERENCE.md` for a quick overview
2. Have them read recent ADRs to understand architectural context
3. Show them AILOGs from recent features to see how documentation works in practice

### Sprint Retrospectives

- Review AILOGs and AIDECs from the sprint to understand AI contribution patterns
- Identify undocumented decisions that should have been recorded
- Check TDE documents for accumulating technical debt

### Shared AI Assistant Usage

When multiple team members use AI assistants on the same project:

- Each assistant session produces its own documents
- The `agent` field in metadata identifies which assistant created each document
- Review overlapping or contradictory AIDECs during PR review

---

## Understanding Versions

DevTrail uses **independent versioning** for its two components:

| Component | Tag prefix | Contains | Updated via |
|-----------|-----------|----------|-------------|
| **Framework** | `fw-` | Templates, governance docs, directives, scripts | `devtrail update-framework` |
| **CLI** | `cli-` | The `devtrail` binary | `devtrail update-cli` |

### Why Independent Versions?

- Framework changes (new templates, updated rules) are more frequent
- CLI changes (new commands, bug fixes) follow a different cadence
- You can update governance docs without needing a new CLI binary

### Checking Your Versions

```bash
devtrail about     # Quick version check
devtrail status    # Full health report including versions
```

For detailed CLI information, see the [CLI Reference](CLI-REFERENCE.md#versioning).

---

<div align="center">

**DevTrail** — Because every change tells a story.

[Back to docs](../README.md) • [README](../../README.md) • [Strange Days Tech](https://strangedays.tech)

</div>
