# Changelog

All notable changes to DevTrail will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [independent versioning](README.md#versioning) for Framework (`fw-`) and CLI (`cli-`).

---

## Framework 4.0.0 / CLI 2.1.0 — Phase 4: Advanced Automation & Ecosystem

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
- Canonical source comment in docs-validation.yml for document type list

### Changed
- QUICK-REFERENCE.md: Added C4 Model reference to regulatory alignment table
- Version bumps: Framework 3.2.0 → 4.0.0, CLI 2.0.0 → 2.1.0
- Updated CLI-REFERENCE.md, README.md with 13 commands (EN + ES)

---

## Framework 3.2.0 / CLI 2.0.0 — Phase 3: Compliance Automation & Metrics

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

## Framework 3.1.0 / CLI 1.4.0 — Phase 2: New Document Types & Validation

### Added (CLI)
- **`devtrail validate`** command — Validate documents with 13 rules (NAMING, META, CROSS, TYPE, REF, SEC, OBS)
  - `--fix` flag for automatic corrections
  - Exit code 1 on errors, 0 on warnings-only
- Document parsing engine (`document.rs`) — Shared by validate, compliance, metrics, audit
- Validation engine (`validation.rs`) — Extensible rule-based validation
- Lizard integration (`complexity.rs`) — Cyclomatic complexity analysis

### Added (Framework)
- **TEMPLATE-SEC.md** — Security Assessment (STRIDE threat model, OWASP ASVS)
- **TEMPLATE-MCARD.md** — Model/System Card (Mitchell et al. 2019)
- **TEMPLATE-SBOM.md** — Software Bill of Materials (SPDX/CycloneDX aligned)
- **TEMPLATE-DPIA.md** — Data Protection Impact Assessment (GDPR Art. 35)
- Skills: `/devtrail-sec`, `/devtrail-mcard` (Claude, Gemini, generic agent)
- Updated `/devtrail-new` and `/devtrail-status` for 12 document types
- Compliance CI jobs in docs-validation.yml

---

## Framework 3.0.0 / CLI 1.3.0 — Phase 1: Regulatory Base & Standards Update

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
