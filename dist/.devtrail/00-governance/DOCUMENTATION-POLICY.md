# Documentation Policy - DevTrail

---

## 1. File Naming Convention

### Standard Format

```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

| Component | Description | Example |
|-----------|-------------|---------|
| `TYPE` | Document type prefix | `AILOG`, `AIDEC`, `ADR` |
| `YYYY-MM-DD` | Creation date | `2025-01-27` |
| `NNN` | Sequential number for the day | `001`, `002` |
| `description` | Brief description in kebab-case | `implement-oauth` |

### Examples

```
AILOG-2025-01-27-001-implement-oauth.md
AIDEC-2025-01-27-001-testing-framework-selection.md
ADR-2025-01-27-001-microservices-architecture.md
REQ-2025-01-27-001-user-authentication.md
```

---

## 2. Required Metadata (Frontmatter)

All documents must include YAML metadata at the beginning:

```yaml
---
id: AILOG-2025-01-27-001
title: OAuth Authentication Implementation
status: draft | accepted | deprecated | superseded
created: 2025-01-27
updated: 2025-01-27
agent: claude-code-v1.0
confidence: high | medium | low
review_required: true | false
risk_level: low | medium | high | critical
tags:
  - auth
  - security
related:
  - ADR-2025-01-20-001
  - REQ-2025-01-15-003
---
```

### Required Fields

| Field | Description |
|-------|-------------|
| `id` | Unique identifier (same as filename without .md) |
| `title` | Descriptive title |
| `status` | Current document status |
| `created` | Creation date |
| `agent` | Identifier of the agent that created the document |
| `confidence` | Agent's confidence level |
| `review_required` | Whether human review is required |
| `risk_level` | Change risk level |

### Optional Fields

| Field | Description |
|-------|-------------|
| `updated` | Last update date |
| `tags` | Tags for categorization (see conventions below) |
| `related` | References to related documents (see conventions below) |
| `supersedes` | ID of the document this one replaces |
| `superseded_by` | ID of the document that replaces this one |

### Tags Convention

Tags are **free-form keywords** used for categorization and search. They help discover related documents across the project.

**Format rules:**
- Use **kebab-case** (lowercase, hyphens): `gnome-integration`, `sqlite`, `api-design`
- One concept per tag — avoid compound tags like `auth-and-security`
- Recommended range: **3 to 8 tags** per document
- Tags should describe the **topic**, **technology**, **component**, or **concern** of the document

**Example:**
```yaml
tags: [sqlite, persistence, hexagonal-architecture, repository-pattern]
```

### Related Convention

Related references link documents to other **DevTrail documents** within the same project. They enable cross-navigation in tools like `devtrail explore`.

**Format rules:**
- Use the **document filename** (with `.md` extension): `AILOG-2026-02-03-001-implement-sync-item.md`
- For governance or non-typed documents, use the filename as-is: `AGENT-RULES.md`, `DOCUMENTATION-POLICY.md`
- Paths are resolved relative to `.devtrail/` — if the document is in a subdirectory, include the path from `.devtrail/`: `07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implement-sync-item.md`
- When the file is in the same directory as the referencing document, the filename alone is sufficient
- **Do not use** external task IDs (`T001`, `US3`), issue numbers, or URLs — those belong in the document body, not in frontmatter
- **Do not use** partial IDs without description (prefer `AILOG-2026-02-03-001-implement-sync-item.md` over `AILOG-2026-02-03-001`)

**Examples:**
```yaml
# Same directory or well-known location — filename is enough
related:
  - AIDEC-2026-02-02-001-sqlite-bundled-vs-system.md
  - AGENT-RULES.md

# Documents in specific subdirectories — include path from .devtrail/
related:
  - 07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implement-sync-item.md
  - 02-design/decisions/ADR-2026-01-15-001-use-hexagonal-architecture.md
```

**Resolution:** The CLI resolves references by searching: (1) exact ID match, (2) filename match anywhere in `.devtrail/`, (3) path suffix match. Using the full filename provides the most reliable resolution.

---

## 3. Document Statuses

```
draft ──────► accepted ──────► deprecated
                │                   │
                │                   ▼
                └──────► superseded
```

| Status | Description |
|--------|-------------|
| `draft` | In draft, pending review |
| `accepted` | Approved and current |
| `deprecated` | Obsolete, but kept as reference |
| `superseded` | Replaced by another document |

---

## 4. Risk Levels

| Level | When to use | Requires review |
|-------|-------------|-----------------|
| `low` | Cosmetic changes, documentation | No |
| `medium` | New functionality, refactoring | Recommended |
| `high` | Security, sensitive data, public APIs | Yes |
| `critical` | Irreversible changes, production | Mandatory |

---

## 5. Confidence Levels

| Level | Meaning | Action |
|-------|---------|--------|
| `high` | The agent is certain about the decision | Proceed |
| `medium` | The agent has minor doubts | Document alternatives |
| `low` | The agent needs validation | Mark `review_required: true` |

---

## 6. Folder Structure

```
.devtrail/
├── 00-governance/          # Policies and rules
├── 01-requirements/        # System requirements
├── 02-design/              # Design and architecture
│   └── decisions/          # ADRs
├── 03-implementation/      # Implementation guides
├── 04-testing/             # Test strategies
├── 05-operations/          # Operations
│   └── incidents/          # Post-mortems
├── 06-evolution/           # System evolution
│   └── technical-debt/     # Technical debt
├── 07-ai-audit/            # AI agent audit
│   ├── agent-logs/         # AILOG
│   ├── decisions/          # AIDEC
│   └── ethical-reviews/    # ETH
└── templates/              # Templates
```

---

## 7. Cross-References

Use the `[TYPE-ID]` format for references:

```markdown
This decision is based on the requirements defined in [REQ-2025-01-15-003].
See also [ADR-2025-01-20-001] for architectural context.
```

---

*DevTrail v1.0.0 | [Strange Days Tech](https://strangedays.tech)*
