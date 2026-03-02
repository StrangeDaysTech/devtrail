# Rules for AI Agents - DevTrail

> This document defines the rules that all AI agents must follow when working on projects under DevTrail.

---

## 1. Mandatory Identification

### When Starting a Session

Every agent must identify themselves with:
- Agent name (e.g.: `claude-code-v1.0`, `cursor-v1.0`, `gemini-cli-v1.0`)
- Agent version if available

### In Every Document

Include in the frontmatter:
```yaml
agent: agent-name-v1.0
confidence: high | medium | low
```

---

## 2. When to Document

### MANDATORY - Create document

| Situation | Type | Notes |
|-----------|------|-------|
| >10 lines of code in business logic | AILOG | - |
| Decision between 2+ technical alternatives | AIDEC | Document alternatives |
| Changes in security/authentication | AILOG | `risk_level: high` |
| Handling personal data (PII/GDPR) | AILOG + ETH | ETH requires approval |
| Integration with external service | AILOG | - |
| Change in public API | AILOG | `risk_level: medium+` |
| Change in database schema | AILOG | `risk_level: medium+` |

### PROHIBITED - Do not document

- Credentials, tokens, API keys
- Personally identifiable information
- Secrets of any kind

### OPTIONAL - Does not require document

- Formatting changes (whitespace, indentation)
- Typo corrections
- Code comments
- Minor style changes

---

## 3. Autonomy Limits

### Create Freely

| Type | Description |
|------|-------------|
| AILOG | Logs of actions performed |
| AIDEC | Technical decisions made |

### Create Draft → Requires Human Approval

| Type | Description |
|------|-------------|
| ETH | Ethical reviews |
| ADR | Architectural decisions |

### Propose → Requires Human Validation

| Type | Description |
|------|-------------|
| REQ | System requirements |
| TES | Test plans |

### Identify Only → Human Prioritizes

| Type | Description |
|------|-------------|
| TDE | Technical debt |
| INC | Incident conclusions |

---

## 4. When to Request Human Review

Mark `review_required: true` when:

1. **Low confidence**: `confidence: low`
2. **High risk**: `risk_level: high | critical`
3. **Security decisions**: Any change in auth/authz
4. **Irreversible changes**: Migrations, deletions
5. **User impact**: Changes affecting UX
6. **Ethical concerns**: Privacy, bias, accessibility

---

## 5. Document Format

### Use Templates

Before creating a document, load the corresponding template:

```
.devtrail/templates/TEMPLATE-[TYPE].md
```

### Naming Convention

```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

### Location

| Type | Folder |
|------|--------|
| AILOG | `.devtrail/07-ai-audit/agent-logs/` |
| AIDEC | `.devtrail/07-ai-audit/decisions/` |
| ETH | `.devtrail/07-ai-audit/ethical-reviews/` |
| ADR | `.devtrail/02-design/decisions/` |
| REQ | `.devtrail/01-requirements/` |
| TES | `.devtrail/04-testing/` |
| INC | `.devtrail/05-operations/incidents/` |
| TDE | `.devtrail/06-evolution/technical-debt/` |

---

## 6. Communication with Humans

### Be Transparent

- Explain the reasoning behind decisions
- Document considered alternatives
- Admit uncertainty when it exists

### Be Concise

- Get to the point
- Avoid unnecessary jargon
- Use lists and tables when appropriate

### Be Proactive

- Identify potential risks
- Suggest improvements when evident
- Alert about technical debt

---

## 7. Error Handling

If the agent makes an error:

1. **Document** the error in an AILOG
2. **Explain** what went wrong
3. **Propose** correction
4. **Mark** `review_required: true`

---

## 8. Document Updates

### Create New vs Update

| Situation | Action |
|-----------|--------|
| Minor correction | Update existing document |
| Significant change | Create new document |
| Obsolete document | Mark as `deprecated` |
| Complete replacement | Create new + mark previous as `superseded` |

### When Updating

- Update the `updated` field in frontmatter
- Add a note in the history section if it exists
- Maintain consistency with related documents

---

*DevTrail v1.0.0 | [Strange Days Tech](https://strangedays.tech)*
