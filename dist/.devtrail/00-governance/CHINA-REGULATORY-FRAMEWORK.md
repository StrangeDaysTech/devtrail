# China Regulatory Framework — DevTrail

> Overview of how DevTrail maps the six Chinese AI / data regulations covered when `regional_scope: china` is enabled in `.devtrail/config.yml`.

**Languages**: English | [Español](i18n/es/CHINA-REGULATORY-FRAMEWORK.md) | [简体中文](i18n/zh-CN/CHINA-REGULATORY-FRAMEWORK.md)

---

## Activation

Chinese regulatory checks are **opt-in**. Enable them in `.devtrail/config.yml`:

```yaml
regional_scope:
  - global   # NIST + ISO 42001 (always available)
  - eu       # EU AI Act + GDPR
  - china    # adds the 6 frameworks below
```

When `china` is in scope:
- `devtrail new` exposes the four China-specific document types (PIPIA, CACFILE, TC260RA, AILABEL).
- `devtrail compliance --all` includes the six Chinese checkers.
- `devtrail validate` enforces CROSS-004…CROSS-011 and TYPE-003…TYPE-006.

A project without `china` in scope is unaffected.

---

## Coverage Matrix

| # | Regulation | Type | Status | DevTrail Evidence |
|---|------------|------|--------|-------------------|
| 1 | **TC260 AI Safety Governance Framework v2.0** (15 Sep 2025) | Recommended (in drafting as GB) | Active | TC260RA template; `tc260_risk_level`, `tc260_application_scenario`, `tc260_intelligence_level`, `tc260_application_scale` fields on ETH / AILOG / MCARD / SEC |
| 2 | **PIPL** + **PIPIA** (Personal Information Protection Law, Art. 55-56) | Mandatory | Active since 2021-11-01 | PIPIA template; `pipl_*` fields cross-document; PIPIA retention ≥ 3 years |
| 3 | **GB 45438-2025** Cybersecurity Technology — Labeling Method for AI-Generated Content | **Mandatory** | In force 2025-09-01 | AILABEL template; `gb45438_*` fields on MCARD |
| 4 | **CAC Algorithm Filing** (Internet Information Service Algorithm Recommendation Provisions; Interim Measures for GenAI Services) | Mandatory for in-scope services | Active | CACFILE template; `cac_filing_required`, `cac_filing_number`, `cac_filing_status` fields on MCARD |
| 5 | **GB/T 45652-2025** Pre-training & fine-tuning data security | Recommended | In force 2025-11-01 | `gb45652_training_data_compliance` field on SBOM / MCARD; SBOM "GB/T 45652 Training Data Compliance" section |
| 6 | **CSL 2026** Cybersecurity Law amendments + Administrative Measures for National Cybersecurity Incident Reporting | Mandatory | In force 2026-01-01 | INC "CSL 2026 Incident Reporting" section; `csl_severity_level`, `csl_report_deadline_hours` fields on INC |

---

## Document Type → Framework Mapping

| Framework | Primary template | Cross-references | Optional fields elsewhere |
|-----------|-----------------|------------------|---------------------------|
| TC260 v2.0 | TC260RA | ETH, MCARD | `tc260_risk_level` on AILOG / SEC |
| PIPL / PIPIA | PIPIA | DPIA (cross-reference) | `pipl_applicable` on ETH / MCARD |
| GB 45438 | AILABEL | MCARD (generative models) | `gb45438_applicable` on MCARD |
| CAC Algorithm Filing | CACFILE | MCARD, SBOM | `cac_filing_number` on MCARD |
| GB/T 45652 | (Sections in SBOM and MCARD) | TC260RA | `gb45652_training_data_compliance` |
| CSL 2026 | INC (extended) | (none) | `csl_severity_level` on INC |

---

## Implementation Guides

| Framework | Guide |
|-----------|-------|
| TC260 v2.0 risk grading | [TC260-IMPLEMENTATION-GUIDE.md](TC260-IMPLEMENTATION-GUIDE.md) |
| PIPL Art. 55 trigger → PIPIA | [PIPL-PIPIA-GUIDE.md](PIPL-PIPIA-GUIDE.md) |
| Dual filing process | [CAC-FILING-GUIDE.md](CAC-FILING-GUIDE.md) |
| Explicit + implicit labeling | [GB-45438-LABELING-GUIDE.md](GB-45438-LABELING-GUIDE.md) |

---

## Compliance Checks

When `china` is in `regional_scope`, the following compliance checks are exposed via `devtrail compliance --standard <name>`:

| `--standard` | Check IDs | Validates |
|--------------|-----------|-----------|
| `china-tc260` | TC260-001 / 002 / 003 | At least one TC260RA exists; high-risk levels require human review; the three grading criteria are populated |
| `china-pipl` | PIPL-001 / 002 / 003 / 004 | PIPIA exists when `pipl_applicable` or sensitive_data; cross-border transfer documented; retention ≥ 3 years |
| `china-gb45438` | GB45438-001 / 002 / 003 | AILABEL exists when an MCARD declares generative content; explicit + implicit labeling strategies declared; metadata fields populated |
| `china-cac` | CAC-001 / 002 / 003 | CACFILE exists when `cac_filing_required: true`; status not stuck in `pending` for > 90 days; filing number populated when status is `*_approved` |
| `china-gb45652` | GB45652-001 / 002 | SBOM declares per-row training data compliance; MCARD describes data security controls |
| `china-csl` | CSL-001 / 002 / 003 | INC has `csl_severity_level`; deadline hours coherent with severity (1h ↔ particularly_serious, 4h ↔ relatively_major); 30-day post-mortem documented for ≥ relatively_major |

`devtrail compliance --region china` runs all six.

---

## Sources

- [TC260 AI Safety Governance Framework v2.0 — Geopolitechs analysis](https://www.geopolitechs.org/p/china-releases-upgraded-ai-safety)
- [GB 45438-2025 — Code of China](https://www.codeofchina.com/standard/GB45438-2025.html)
- [Measures for the Identification of AI-Generated (Synthetic) Content (Regulations.AI)](https://regulations.ai/regulations/RAI-CN-NA-MIASCXX-2025)
- [China's PIPIA under PIPL — Securiti](https://securiti.ai/personal-information-protection-impact-assessment-pipia-under-china-pipl/)
- [China's Algorithm Filing Regime — Lexology](https://www.lexology.com/library/detail.aspx?g=3c7273cf-8f85-4702-af70-6edf394ff1c3)
- [China Cybersecurity Law amendments effective 2026-01-01 — Mayer Brown](https://www.mayerbrown.com/en/insights/publications/2025/12/china-finalises-amendments-to-the-cybersecurity-law-what-businesses-need-to-know-before-1-january-2026)

<!-- DevTrail | https://strangedays.tech -->
