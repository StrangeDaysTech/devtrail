# DevTrail - Adoption Guide

**A comprehensive guide for adopting DevTrail in new or existing projects.**

[![Strange Days Tech](https://img.shields.io/badge/by-Strange_Days_Tech-purple.svg)](https://strangedays.tech)

**Languages**: English | [Español](../i18n/es/adopters/ADOPTION-GUIDE.md)

---

## Table of Contents

1. [What is DevTrail?](#what-is-devtrail-framework)
2. [Who is it for?](#who-is-it-for)
3. [Benefits](#benefits)
4. [Standards Compliance](#standards-compliance)
5. [Adoption Path A: New Projects](#adoption-path-a-new-projects)
6. [Adoption Path B: Existing Projects](#adoption-path-b-existing-projects)
7. [Configuration](#configuration)
8. [Verification](#verification)
9. [FAQ](#faq)

---

## What is DevTrail?

DevTrail is a **documentation governance system** designed for software development projects that utilize AI coding assistants. It provides:

- **Structured documentation** for decisions, actions, and changes
- **AI agent accountability** through mandatory identification and confidence tracking
- **Human oversight** via required review workflows for critical changes
- **Traceability** connecting requirements → design → implementation → testing

### Core Principle

> **"No significant change without a documented trace."**

DevTrail ensures that every meaningful change in your codebase is documented, attributed, and reviewable—whether made by a human developer or an AI assistant.

### What DevTrail is NOT

- ❌ A documentation generator (it provides structure, not content generation)
- ❌ A replacement for code comments or API docs
- ❌ A project management tool
- ❌ A version control system

---

## Who is it for?

### Target Users

| User Type | Use Case |
|-----------|----------|
| **Solo Developers** | Track your own decisions and AI-assisted changes |
| **Small Teams** | Maintain consistency across team members and AI tools |
| **Enterprise Teams** | Audit trails, compliance, governance at scale |
| **Open Source Maintainers** | Document contribution decisions transparently |

### Compatible Development Environments

DevTrail provides configuration files for:

| Platform | Configuration File | Status |
|----------|-------------------|--------|
| **Claude Code** (Anthropic) | `CLAUDE.md` | ✅ Supported |
| **Cursor** | `.cursorrules` | ✅ Supported |
| **GitHub Copilot CLI** | `.github/copilot-instructions.md` | ✅ Supported |
| **Gemini CLI** (Google) | `GEMINI.md` | ✅ Supported |
| **Other AI Tools** | Copy rules from any config file | ✅ Adaptable |

### Compatible Methodologies

DevTrail works with any development methodology:

| Methodology | How DevTrail Fits |
|-------------|-------------------|
| **Agile/Scrum** | REQ documents map to user stories; ADRs capture sprint decisions |
| **Waterfall** | Full traceability from requirements through implementation |
| **DevOps/SRE** | INC documents for post-mortems; TDE for technical debt tracking |
| **Domain-Driven Design** | ADRs document bounded context decisions |
| **Test-Driven Development** | TES documents capture test strategies |

---

## Benefits

### For Development Teams

| Benefit | Description |
|---------|-------------|
| **Institutional Memory** | Decisions survive team member changes |
| **Onboarding Acceleration** | New members understand "why" through ADRs and AIDECs |
| **Reduced Rework** | Context preserved prevents repeated mistakes |
| **Clear Accountability** | Know who (or what) made each change |

### For AI-Assisted Development

| Benefit | Description |
|---------|-------------|
| **AI Transparency** | Every AI action is logged with confidence levels |
| **Human Oversight** | Critical decisions require human approval |
| **Ethical Safeguards** | ETH documents ensure responsible AI use |
| **Audit Trail** | Complete history of AI contributions |

### For Organizations

| Benefit | Description |
|---------|-------------|
| **Compliance Ready** | Documentation structure supports regulatory requirements |
| **Risk Management** | Risk levels flag high-impact changes |
| **Knowledge Retention** | Documentation survives personnel changes |
| **Quality Assurance** | Structured review processes |

---

## Standards Compliance

DevTrail aligns with and supports compliance for:

### Software Engineering Standards

| Standard | How DevTrail Helps |
|----------|-------------------|
| **IEEE 830** (SRS) | REQ documents follow structured requirements format |
| **ISO/IEC 25010** | Quality attributes documented in ADRs |
| **ISO/IEC 12207** | Lifecycle documentation coverage |

### Architecture Documentation

| Standard | How DevTrail Helps |
|----------|-------------------|
| **ADR (Architecture Decision Records)** | Native ADR support with extended metadata |
| **arc42** | ADRs complement arc42 decision documentation |
| **C4 Model** | ADRs document decisions at each C4 level |

### Compliance & Governance

| Regulation | How DevTrail Helps |
|------------|-------------------|
| **GDPR** | ETH documents for privacy impact assessments |
| **SOC 2** | Change documentation and access logging |
| **ISO 27001** | Security decision documentation |
| **HIPAA** | Audit trails for healthcare applications |

### AI Governance

| Framework | How DevTrail Helps |
|-----------|-------------------|
| **EU AI Act** | Transparency through AILOG/AIDEC; human oversight via ETH |
| **NIST AI RMF** | Risk documentation in decision records |
| **IEEE 7000** | Ethical considerations in ETH documents |

---

## Adoption Path A: New Projects

### Option 1: CLI (Recommended)

```bash
# Install the CLI
cargo install devtrail-cli

# Initialize in your project
cd your-project
devtrail init .

# Commit
git add .devtrail/ DEVTRAIL.md scripts/
git commit -m "chore: adopt DevTrail"
```

The CLI automatically:
- Downloads the latest DevTrail release from GitHub
- Sets up the `.devtrail/` directory structure
- Creates `DEVTRAIL.md` with governance rules
- Configures AI agent directives (`CLAUDE.md`, `GEMINI.md`, `.cursorrules`, etc.)
- Copies validation scripts and CI/CD workflows

### Option 2: Manual Setup

1. **Download the latest release**

   Go to [GitHub Releases](https://github.com/StrangeDaysTech/devtrail/releases/latest) and download the distribution ZIP.

2. **Extract to your project**
   ```bash
   unzip devtrail-v*.zip -d your-project/
   ```

3. **Commit the structure**
   ```bash
   git add .devtrail/ DEVTRAIL.md scripts/
   git commit -m "chore: adopt DevTrail for documentation governance"
   ```

---

## Adoption Path B: Existing Projects

### Phase 1: Assessment (Day 1)

1. **Evaluate current documentation**

   Answer these questions:
   - Do you have existing ADRs? Where are they located?
   - Do you have a `docs/` folder? What does it contain?
   - Are there any naming conventions already in place?
   - Do you use any AI coding assistants?

2. **Plan the migration**

   | Current State | Recommended Action |
   |---------------|-------------------|
   | No documentation | Start fresh with DevTrail |
   | Docs in `docs/` folder | Keep `docs/` for user-facing docs, add `.devtrail/` for development docs |
   | Existing ADRs | Migrate to `.devtrail/02-design/decisions/` with new naming |
   | Mixed documentation | Categorize and migrate gradually |

### Phase 2: Installation (Day 1-2)

1. **Add DevTrail structure**
   ```bash
   # Using CLI (recommended)
   devtrail init .

   # Or manually: download from GitHub Releases
   # https://github.com/StrangeDaysTech/devtrail/releases/latest
   ```

2. **Resolve conflicts with existing `docs/`**

   DevTrail uses `.devtrail/` specifically to avoid conflicts:

   ```
   your-project/
   ├── docs/                    ← Keep for API docs, user guides, etc.
   │   ├── api/
   │   └── user-guide/
   ├── .devtrail/              ← Add for development documentation
   │   ├── 00-governance/
   │   ├── 01-requirements/
   │   └── ...
   └── src/
   ```

### Phase 3: Migration (Week 1-2)

1. **Migrate existing ADRs**

   For each existing ADR:
   ```bash
   # Old: docs/adr/001-use-postgresql.md
   # New: .devtrail/02-design/decisions/ADR-2024-01-15-001-use-postgresql.md
   ```

   Add DevTrail metadata to the front-matter:
   ```yaml
   ---
   id: ADR-2024-01-15-001
   title: Use PostgreSQL for primary database
   status: accepted
   created: 2024-01-15
   agent: human
   confidence: high
   review_required: false
   risk_level: high
   # Preserve original metadata
   original_id: "001"
   migrated_from: "docs/adr/001-use-postgresql.md"
   ---
   ```

2. **Document the migration**

   Create an AILOG documenting the migration:
   ```
   .devtrail/07-ai-audit/agent-logs/AILOG-2025-01-27-001-devtrail-adoption.md
   ```

### Phase 4: Team Adoption (Week 2-4)

1. **Update contribution guidelines**

   Add to your `CONTRIBUTING.md`:
   ```markdown
   ## Documentation

   This project uses [DevTrail](https://github.com/StrangeDaysTech/devtrail) for documentation governance.

   - All significant changes must be documented in `.devtrail/`
   - AI-assisted changes require AILOG entries
   - Architectural decisions require ADR documents

   See `.devtrail/QUICK-REFERENCE.md` for document types and naming.
   ```

2. **Enable pre-commit hooks (optional)**
   ```bash
   # Copy the pre-commit hook
   cp scripts/pre-commit-docs.sh .git/hooks/pre-commit
   chmod +x .git/hooks/pre-commit

   # Or with Husky
   npx husky add .husky/pre-commit "bash scripts/pre-commit-docs.sh"
   ```

3. **Enable GitHub Actions (optional)**

   The workflow at `.github/workflows/docs-validation.yml` will automatically validate documentation on PRs.

### Phase 5: Gradual Rollout

| Week | Focus |
|------|-------|
| Week 1 | Core team adopts DevTrail for new decisions |
| Week 2 | Migrate critical existing ADRs |
| Week 3 | Enable validation in CI/CD |
| Week 4 | Full team adoption; document existing technical debt |

---

## Configuration

### Customizing Agent Identifiers

Each AI platform has its own configuration file that:

1. Identifies the agent (e.g., `claude-code-v1.0`)
2. Defines when to document (>10 lines, security changes, etc.)
3. Sets autonomy limits
4. Specifies templates location
5. Requires documentation reporting
6. **Enforces Git workflow** (branch naming, conventional commits, no direct commits to `main`)

Update the agent identifier to match your versioning:

```yaml
# In any agent config file
agent: claude-code-v1.0      # Default
agent: claude-code-v2.1      # Your custom version
agent: acme-corp-claude-v1   # Organization-specific
```

### Customizing Document Types

To add a new document type:

1. **Create the template**
   ```
   .devtrail/templates/TEMPLATE-NEWTYPE.md
   ```

2. **Update governance docs**

   Add the new type to:
   - `.devtrail/00-governance/DOCUMENTATION-POLICY.md`
   - `.devtrail/00-governance/AGENT-RULES.md`
   - `.devtrail/QUICK-REFERENCE.md`

3. **Update agent configs**

   Add the new type to all agent configuration files.

4. **Update validation scripts**

   Add the new type prefix to:
   - `scripts/pre-commit-docs.sh`
   - `scripts/validate-docs.ps1`
   - `.github/workflows/docs-validation.yml`

### Customizing Folder Structure

The numbered folder structure (`00-governance`, `01-requirements`, etc.) is designed for:
- Logical ordering in file explorers
- Clear separation of concerns
- Easy navigation

You can rename folders, but update all references in:
- Agent configuration files
- Governance documents
- Validation scripts

---

## Verification

### Verification with Skills (Claude Code)

If using Claude Code, verify documentation compliance with the built-in skill:

```bash
/devtrail-status
```

This skill shows:
- What DevTrail documents were created recently
- Which modified files may need documentation
- Overall documentation compliance status

### Manual Verification

After adoption, verify your setup:

```bash
# Run the validation script
# On Linux/Mac:
bash scripts/pre-commit-docs.sh

# On Windows PowerShell:
.\scripts\validate-docs.ps1
```

### Checklist

- [ ] `.devtrail/` folder structure exists
- [ ] At least one agent config file exists (`CLAUDE.md`, `GEMINI.md`, etc.)
- [ ] Governance documents are present in `.devtrail/00-governance/`
- [ ] Templates are present in `.devtrail/templates/`
- [ ] Git branching strategy documented in `.devtrail/03-implementation/`
- [ ] `QUICK-REFERENCE.md` is accessible
- [ ] Validation scripts run without errors
- [ ] (Optional) Pre-commit hook is installed
- [ ] (Optional) GitHub Actions workflow is enabled

---

## FAQ

### General Questions

**Q: Does DevTrail replace my existing documentation?**

A: No. DevTrail is for *development process documentation* (decisions, changes, reviews). Keep your existing `docs/` folder for user-facing documentation, API references, and guides.

**Q: Do I need to use AI coding assistants to benefit from DevTrail?**

A: No. DevTrail works for human-only teams too. The AI audit features (AILOG, AIDEC, ETH) become especially valuable when using AI assistants, but ADR, REQ, TDE, and other document types are useful for any team.

**Q: How much overhead does DevTrail add?**

A: DevTrail follows a "minimum viable documentation" principle. Only significant changes require documentation. Trivial changes (typos, formatting) are explicitly excluded.

### Technical Questions

**Q: Why use `.devtrail/` instead of `docs/`?**

A: The `docs/` folder is commonly used for user-facing documentation, GitHub Pages, or generated API docs. Using `.devtrail/` avoids conflicts and clearly separates development documentation from user documentation.

**Q: Can I use DevTrail with monorepos?**

A: Yes. You can either:
- Have one `.devtrail/` at the root for the entire monorepo
- Have separate `.devtrail/` folders in each package/service
- Use a hybrid approach with shared governance at root

**Q: How do I handle sensitive information?**

A: DevTrail explicitly prohibits documenting credentials, tokens, or secrets. The validation scripts check for common sensitive patterns and warn you. For genuinely sensitive decisions, document the *existence* of the decision without the sensitive details.

### Adoption Questions

**Q: My team is resistant to more documentation. How do I convince them?**

A: Start small:
1. Begin with just ADRs for architectural decisions
2. Show value through faster onboarding of new team members
3. Demonstrate time saved when revisiting old decisions
4. Gradually expand to other document types

**Q: How do I handle documents created before adopting DevTrail?**

A: You have three options:
1. **Migrate**: Convert old documents to DevTrail format (recommended for important docs)
2. **Reference**: Keep old docs in place, reference them from DevTrail docs
3. **Archive**: Move old docs to an archive folder, start fresh with DevTrail

**Q: What if my AI assistant doesn't follow the rules?**

A: DevTrail rules are instructions, not enforcement. If an AI assistant creates non-compliant documentation:
1. The pre-commit hook will catch validation errors
2. CI/CD will flag issues in PRs
3. You can manually fix and educate the AI in the next prompt

---

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/StrangeDaysTech/devtrail/issues)
- **Discussions**: [GitHub Discussions](https://github.com/StrangeDaysTech/devtrail/discussions)
- **Contributing**: See [CONTRIBUTING.md](../../CONTRIBUTING.md)

---

---

<div align="center">

**DevTrail** — Because every change tells a story.

[Back to README](../../README.md) • [Strange Days Tech](https://strangedays.tech)

</div>
