# DevTrail - Documentation Governance Rules

> **This file is automatically managed by DevTrail CLI.**
> Read and follow these rules when working on this project.
> For complete rules: `.devtrail/00-governance/AGENT-RULES.md`

---

## 1. Fundamental Principle

> **"No significant change without a documented trace."**

---

## 2. Language Configuration

Check `.devtrail/config.yml` for the project's language setting:

```yaml
language: en  # Options: en, es (default: en)
```

**Template paths based on language:**

| Language | Template Path |
|----------|---------------|
| `en` (default) | `.devtrail/templates/TEMPLATE-*.md` |
| `es` | `.devtrail/templates/i18n/es/TEMPLATE-*.md` |

If the config file doesn't exist or `language` is not set, use English (`en`) as default.

---

## 3. Documentation Reporting

At the end of each task, you MUST report your DevTrail documentation status:

**If you created documentation:**
```
DevTrail: Created AILOG-2025-01-27-001-implement-auth.md
```

**If documentation was not needed:**
```
DevTrail: No documentation required (minor change / <10 lines)
```

**If you should have documented but didn't:**
```
DevTrail: Documentation pending - review required
```

This transparency helps users verify compliance with DevTrail rules.

---

## 4. Agent Identity

When working on this project:

- **Identify yourself** with your platform and version (e.g., `claude-code-v1.0`, `gemini-cli-v1.0`, `copilot-cli-v1.0`)
- **Declare** your confidence level in decisions: `high | medium | low`
- **Record** your identification in the `agent:` field of the metadata

---

## 5. Git Operations

> **CRITICAL: Never commit directly to `main` branch.**

All changes must go through feature/fix branches and Pull Requests.

### Branch Prefixes

| Prefix | Purpose |
|--------|---------|
| `feature/` or `feat/` | New features |
| `fix/` | Bug fixes |
| `hotfix/` | Urgent production fixes |
| `docs/` | Documentation only |
| `refactor/` | Code refactoring |
| `test/` | Test changes |

### Conventional Commits

| Prefix | Use Case |
|--------|----------|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `docs:` | Documentation only |
| `refactor:` | No behavior change |
| `chore:` | Maintenance |

### Quick Workflow

```bash
git checkout main && git pull origin main
git checkout -b fix/descriptive-name
# ... make changes and commits ...
git push -u origin fix/descriptive-name
gh pr create --title "fix: description" --body "..."
```

> **Full details:** `.devtrail/00-governance/GIT-BRANCHING-STRATEGY.md`

---

## 6. When to Document

### MANDATORY (create document)

| Situation | Action |
|-----------|--------|
| >10 lines of code in business logic | Create AILOG |
| Decision between technical alternatives | Create AIDEC |
| Changes in security/authentication | Create AILOG + mark `risk_level: high` |
| Personal data (GDPR/PII) | Create AILOG + request ETH |
| Integration with external service | Create AILOG |
| Change in public API or DB schema | Create AILOG |

### DO NOT DOCUMENT

- Trivial changes (whitespace, typos, formatting)
- Sensitive information (credentials, tokens, API keys)

---

## 7. File Naming Convention

```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

**Example**: `AILOG-2025-01-27-001-implement-oauth.md`

---

## 8. Minimum Metadata

```yaml
---
id: AILOG-2025-01-27-001
title: Brief description
status: accepted
created: 2025-01-27
agent: your-agent-id-v1.0
confidence: high | medium | low
review_required: true | false
risk_level: low | medium | high | critical
---
```

---

## 9. Autonomy Limits

| Type | Agent can do | Requires human |
|------|----------|----------------|
| **AILOG** | Create freely | - |
| **AIDEC** | Create freely | - |
| **ETH** | Create draft | Approval |
| **ADR** | Create draft | Review |
| **REQ** | Propose | Validation |
| **INC** | Contribute analysis | Conclusions |
| **TDE** | Identify | Prioritize |

---

## 10. Documentation Map

> **IMPORTANT**: This is the complete project structure.
> Not everything is loaded in this session, but any document can be accessed when needed.

```
.devtrail/
├── 00-governance/              ← POLICIES AND RULES
│   ├── PRINCIPLES.md           # Project guiding principles
│   ├── DOCUMENTATION-POLICY.md # Complete documentation policy
│   ├── AGENT-RULES.md          # Detailed rules for AI agents
│   └── exceptions/             # Documented exceptions
│
├── 01-requirements/            ← REQUIREMENTS (REQ)
│   └── [REQ-*.md]              # System requirements
│
├── 02-design/                  ← DESIGN
│   └── decisions/              # ADRs (Architecture Decision Records)
│       └── [ADR-*.md]
│
├── 03-implementation/          ← IMPLEMENTATION GUIDES
│   └── [technical guides]
│
├── 04-testing/                 ← TESTING (TES)
│   └── [TES-*.md]              # Test strategies and plans
│
├── 05-operations/              ← OPERATIONS
│   ├── [runbooks]
│   └── incidents/              # Post-mortems (INC)
│       └── [INC-*.md]
│
├── 06-evolution/               ← EVOLUTION
│   └── technical-debt/         # Technical debt (TDE)
│       └── [TDE-*.md]
│
├── 07-ai-audit/                ← AI AGENT AUDIT
│   ├── agent-logs/             # Action logs (AILOG)
│   │   └── [AILOG-*.md]
│   ├── decisions/              # Agent decisions (AIDEC)
│   │   └── [AIDEC-*.md]
│   └── ethical-reviews/        # Ethical reviews (ETH)
│       └── [ETH-*.md]
│
├── templates/                  ← TEMPLATES
│   ├── TEMPLATE-AILOG.md
│   ├── TEMPLATE-AIDEC.md
│   ├── TEMPLATE-ADR.md
│   ├── TEMPLATE-ETH.md
│   ├── TEMPLATE-REQ.md
│   ├── TEMPLATE-INC.md
│   ├── TEMPLATE-TDE.md
│   └── TEMPLATE-TES.md
│
└── QUICK-REFERENCE.md          ← 1-page quick reference
```

---

## 11. When to Load Additional Documents

| Situation | Document to load |
|-----------|------------------|
| Going to create an AILOG | `.devtrail/templates/TEMPLATE-AILOG.md` |
| Going to create an AIDEC | `.devtrail/templates/TEMPLATE-AIDEC.md` |
| Going to create an ADR | `.devtrail/templates/TEMPLATE-ADR.md` |
| Going to create a REQ | `.devtrail/templates/TEMPLATE-REQ.md` |
| Questions about naming or metadata | `.devtrail/00-governance/DOCUMENTATION-POLICY.md` |
| Questions about autonomy limits | `.devtrail/00-governance/AGENT-RULES.md` |
| Need to see existing requirements | List `.devtrail/01-requirements/` |
| Need to see existing ADRs | List `.devtrail/02-design/decisions/` |
| Need to see technical debt | List `.devtrail/06-evolution/technical-debt/` |

---

## 12. Workflow

```
1. EVALUATE if the change requires documentation (see section 6)
                            |
                            v
2. LOAD the corresponding template (see section 11)
                            |
                            v
3. CREATE the document with correct naming
   [TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
                            |
                            v
4. If risk_level: high/critical or confidence: low
   -> Mark review_required: true
```

---

## 13. Quick Type Reference

| Prefix | Name | Location |
|--------|------|----------|
| `AILOG` | AI Action Log | `.devtrail/07-ai-audit/agent-logs/` |
| `AIDEC` | AI Decision | `.devtrail/07-ai-audit/decisions/` |
| `ETH` | Ethical Review | `.devtrail/07-ai-audit/ethical-reviews/` |
| `ADR` | Architecture Decision Record | `.devtrail/02-design/decisions/` |
| `REQ` | Requirement | `.devtrail/01-requirements/` |
| `TES` | Test Plan | `.devtrail/04-testing/` |
| `INC` | Incident Post-mortem | `.devtrail/05-operations/incidents/` |
| `TDE` | Technical Debt | `.devtrail/06-evolution/technical-debt/` |

---

*DevTrail v2.0.0 | [GitHub](https://github.com/StrangeDaysTech/devtrail)*
*[Strange Days Tech](https://strangedays.tech) — Because every change tells a story.*
