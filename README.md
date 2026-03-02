<div align="center">

# DevTrail

**Documentation Governance for AI-Assisted Software Development**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Handbook](https://img.shields.io/badge/docs-Handbook-orange.svg)](.devtrail/QUICK-REFERENCE.md)
[![Strange Days Tech](https://img.shields.io/badge/by-Strange_Days_Tech-purple.svg)](https://strangedays.tech)

[Getting Started](#getting-started) •
[Features](#features) •
[Documentation](#documentation) •
[Contributing](#contributing)

**Languages**: English | [Español](docs/i18n/es/README.md)

</div>

---

## The Problem

As AI coding assistants become integral to software development, a critical gap emerges:

- **Who made this change?** Was it a developer or an AI assistant?
- **Why was this decision made?** What alternatives were considered?
- **Should this have been reviewed?** Was human oversight appropriate?
- **What's the impact?** How risky is this change?

Without structured documentation, AI-assisted development becomes a black box.

## The Solution

DevTrail provides a **documentation governance system** that ensures:

> **"No significant change without a documented trace."**

Every meaningful change—whether by human or AI—is documented, attributed, and reviewable.

---

## Features

### 📋 Structured Documentation

Eight document types covering the full development lifecycle:

| Type | Purpose | Example |
|------|---------|---------|
| **REQ** | Requirements | System requirements, user stories |
| **ADR** | Architecture Decisions | Technology choices, design patterns |
| **TES** | Test Plans | Test strategies, coverage goals |
| **INC** | Incident Post-mortems | Root cause analysis, lessons learned |
| **TDE** | Technical Debt | Identified debt, remediation plans |
| **AILOG** | AI Action Logs | What AI assistants did and why |
| **AIDEC** | AI Decisions | Choices made by AI with alternatives |
| **ETH** | Ethical Reviews | Privacy, bias, responsible AI |

### 🤖 AI Agent Support

Pre-configured for popular AI coding assistants:

- **Claude Code** (Anthropic) → `CLAUDE.md`
- **Cursor** → `.cursorrules`
- **GitHub Copilot CLI** → `.github/copilot-instructions.md`
- **Gemini CLI** (Google) → `GEMINI.md`

Each configuration instructs the AI to:
- Identify itself in every document
- Declare confidence levels
- Request human review when appropriate
- Follow naming conventions
- **Follow Git branching strategy** (never commit to `main` directly)

### 👁️ Human Oversight

Built-in safeguards ensure humans stay in control:

- **Autonomy levels**: Some document types require human approval
- **Review triggers**: Low confidence or high risk → mandatory review
- **Ethical reviews**: Privacy and bias concerns flagged for human decision

### ✅ Validation & CI/CD

Automated validation tools:

- **Pre-commit hooks** (Bash) - Validate before commit
- **PowerShell script** - Windows-friendly validation
- **GitHub Actions** - PR validation workflow

---

## Getting Started

### Option 1: CLI (Recommended)

```bash
# Install the CLI
cargo install devtrail-cli

# Initialize in your project
cd your-project
devtrail init .
```

The CLI downloads the latest DevTrail release, sets up the framework, and configures your AI agent directive files automatically.

```bash
# Update to latest version
devtrail update

# Remove DevTrail
devtrail remove
```

### Option 2: Manual Setup

```bash
# Download the latest release ZIP from GitHub
# https://github.com/StrangeDaysTech/devtrail/releases/latest

# Extract and copy to your project
cp -r .devtrail your-project/
cp DEVTRAIL.md your-project/
cp -r scripts your-project/

# Commit
git add .devtrail/ DEVTRAIL.md scripts/
git commit -m "chore: adopt DevTrail"
```

📖 **See [ADOPTION-GUIDE.md](docs/adopters/ADOPTION-GUIDE.md) for detailed instructions, migration strategies, and team rollout plans.**

---

## Documentation

DevTrail documentation is organized by audience:

| Track | For | Start here |
|-------|-----|------------|
| [**Adopters**](docs/adopters/) | Teams adopting DevTrail in their projects | [ADOPTION-GUIDE.md](docs/adopters/ADOPTION-GUIDE.md) |
| [**Contributors**](docs/contributors/) | Developers contributing to DevTrail | [TRANSLATION-GUIDE.md](docs/contributors/TRANSLATION-GUIDE.md) |

**Adopters**: Follow the [Adoption Guide](docs/adopters/ADOPTION-GUIDE.md) for step-by-step instructions, migration strategies for existing projects, and team rollout plans.

**Contributors**: See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines, and the [Translation Guide](docs/contributors/TRANSLATION-GUIDE.md) for adding new languages.

### Key References

| Document | Description |
|----------|-------------|
| [**📘 Quick Reference**](.devtrail/QUICK-REFERENCE.md) | One-page overview of document types and naming |
| [DEVTRAIL.md](DEVTRAIL.md) | Unified governance rules (source of truth) |
| [ADOPTION-GUIDE.md](docs/adopters/ADOPTION-GUIDE.md) | Adoption guide for new/existing projects |

### Internal Structure

Once adopted, DevTrail creates a `.devtrail/` directory in your project for development governance:

```
.devtrail/
├── 00-governance/           # Policies and rules
├── 01-requirements/         # REQ documents
├── 02-design/decisions/     # ADR documents
├── 03-implementation/       # Implementation guides (incl. Git strategy)
├── 04-testing/              # TES documents
├── 05-operations/incidents/ # INC documents
├── 06-evolution/technical-debt/ # TDE documents
├── 07-ai-audit/
│   ├── agent-logs/          # AILOG documents
│   ├── decisions/           # AIDEC documents
│   └── ethical-reviews/     # ETH documents
└── templates/               # Document templates
```

### Naming Convention

```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

Example: `ADR-2025-01-27-001-use-postgresql-for-persistence.md`

---

## How It Works

### 1. AI Makes a Change

An AI assistant working on your code automatically:

```yaml
# Creates: .devtrail/07-ai-audit/agent-logs/AILOG-2025-01-27-001-implement-auth.md
---
id: AILOG-2025-01-27-001
title: Implement JWT authentication
agent: claude-code-v1.0
confidence: high
risk_level: high
review_required: true
---
```

### 2. Human Reviews (When Needed)

High-risk or low-confidence changes are flagged:

```
📋 AILOG-2025-01-27-001-implement-auth.md
   Agent: claude-code-v1.0
   Confidence: high
   Risk Level: high ⚠️
   Review Required: YES
```

### 3. Decisions Are Preserved

When choosing between alternatives, decisions are documented:

```yaml
# Creates: .devtrail/07-ai-audit/decisions/AIDEC-2025-01-27-001-auth-strategy.md
---
id: AIDEC-2025-01-27-001
title: Choose JWT over session-based auth
alternatives_considered:
  - JWT tokens (chosen)
  - Session cookies
  - OAuth only
justification: "Stateless architecture requirement..."
---
```

### 4. Ethical Concerns Are Flagged

When AI encounters ethical considerations:

```yaml
# Creates: .devtrail/07-ai-audit/ethical-reviews/ETH-2025-01-27-001-user-data.md
---
id: ETH-2025-01-27-001
title: User data collection scope
status: draft  # Requires human approval
review_required: true
concerns:
  - GDPR compliance
  - Data minimization
---
```

---

## Validation

### Pre-commit Hook

```bash
# Install the pre-commit hook
cp scripts/pre-commit-docs.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

### Manual Validation

```bash
# Linux/Mac
bash scripts/pre-commit-docs.sh

# Windows PowerShell
.\scripts\validate-docs.ps1
```

### GitHub Actions

The included workflow (`.github/workflows/docs-validation.yml`) automatically validates:
- File naming conventions
- Required metadata fields
- Sensitive information detection
- Markdown formatting
- Internal link integrity

---

## Skills

DevTrail includes skills for AI agents that enable **active documentation creation**.

> **Binary System**: DevTrail uses a passive system (agents auto-document via context instructions) and an active system (users invoke skills to create documentation manually or when the agent missed something).

### Available Skills

| Skill | Purpose | Claude | Gemini |
|-------|---------|--------|--------|
| `/devtrail-status` | Check documentation compliance | ✅ | ✅ |
| `/devtrail-new` | Create any document type (unified) | ✅ | ✅ |
| `/devtrail-ailog` | Quick AILOG creation | ✅ | ✅ |
| `/devtrail-aidec` | Quick AIDEC creation | ✅ | ✅ |
| `/devtrail-adr` | Quick ADR creation | ✅ | ✅ |

### Usage Examples

```bash
# Check documentation status
/devtrail-status

# Create documentation (agent suggests type)
/devtrail-new

# Force specific document type
/devtrail-new ailog

# Direct shortcuts
/devtrail-ailog
/devtrail-aidec
/devtrail-adr
```

### Shell Scripts (Manual Use)

For users who prefer command-line or use agents without skill support:

```bash
# Interactive document creation
./scripts/devtrail-new.sh

# Create specific type directly
./scripts/devtrail-new.sh ailog

# Check documentation status
./scripts/devtrail-status.sh
```


### Agent Reporting

AI agents report documentation status at the end of each task:

| Status | Meaning |
|--------|---------|
| `DevTrail: Created AILOG-...` | Documentation was created |
| `DevTrail: No documentation required` | Change was minor (<10 lines) |
| `DevTrail: Documentation pending` | May need manual review |

### Multi-Agent Architecture

DevTrail provides native skill support for multiple AI agents through a layered architecture:

```
your-project/
├── .agent/workflows/       # 🌐 Agnostic (Antigravity, future agents)
│   ├── devtrail-new.md
│   ├── devtrail-status.md
│   └── ...
├── .gemini/skills/         # 🔵 Gemini CLI (Google)
│   ├── devtrail-new/SKILL.md
│   └── ...
└── .claude/skills/         # 🟣 Claude Code (Anthropic)
    ├── devtrail-new/SKILL.md
    └── ...
```

| Directory | Agent | Product | Format |
|-----------|-------|---------|--------|
| `.agent/workflows/` | Antigravity, generic | VS Code/Cursor extensions | `skill-name.md` with YAML frontmatter |
| `.gemini/skills/` | Gemini CLI | Google's terminal CLI | `skill-name/SKILL.md` |
| `.claude/skills/` | Claude Code | Anthropic's coding agent | `skill-name/SKILL.md` |

> **Note**: `.agent/` is the **vendor-agnostic** standard. Agent-specific directories (`.gemini/`, `.claude/`) provide compatibility for those platforms while following their native conventions.

All skill implementations are **functionally identical**—only the format differs to match each agent's requirements.

---

## Supported Platforms

### AI Coding Assistants

| Platform | Config File | Status |
|----------|-------------|--------|
| Claude Code | `CLAUDE.md` | ✅ Full support |
| Cursor | `.cursorrules` | ✅ Full support |
| GitHub Copilot CLI | `.github/copilot-instructions.md` | ✅ Full support |
| Gemini CLI | `GEMINI.md` | ✅ Full support |

### Operating Systems

| OS | Validation Script |
|----|-------------------|
| Linux | `scripts/pre-commit-docs.sh` |
| macOS | `scripts/pre-commit-docs.sh` |
| Windows | `scripts/validate-docs.ps1` |

### CI/CD Platforms

| Platform | Support |
|----------|---------|
| GitHub Actions | ✅ Included workflow |
| GitLab CI | 🔧 Adaptable from GitHub Actions |
| Azure DevOps | 🔧 Adaptable from GitHub Actions |

---

## Standards Alignment

DevTrail aligns with:

- **ADR** (Architecture Decision Records) - Native support
- **IEEE 830** - Requirements documentation structure
- **ISO/IEC 25010** - Quality attributes in ADRs
- **GDPR** - Privacy impact documentation (ETH)
- **EU AI Act** - AI transparency and human oversight
- **NIST AI RMF** - Risk documentation

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Ways to Contribute

- 🐛 Report bugs
- 💡 Suggest features
- 📖 Improve documentation
- 🔧 Submit pull requests
- 🌍 Add translations

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## About Strange Days Tech, S.A.S.

<div align="center">

**[Strange Days Tech](https://strangedays.tech)** builds tools for responsible AI-assisted software development. DevTrail is one of our open-source projects.

[Website](https://strangedays.tech) • [GitHub](https://github.com/StrangeDaysTech)

</div>

---

<div align="center">

**DevTrail** — Because every change tells a story.

[⬆ Back to top](#devtrail)

</div>
