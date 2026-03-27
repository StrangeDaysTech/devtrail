# DevTrail — Plan de Implementación Detallado

> **Documento derivado de**: `evolution/CLAUDE-PLAN-MAESTRO.md`
> **Fecha de creación**: 23 de marzo de 2026
> **Última actualización**: 25 de marzo de 2026 — Revisión de congruencia post-integración OpenTelemetry
> **Autor**: Claude Opus 4.6
> **Estado**: Borrador para revisión y aprobación

---

## Convenciones de este documento

### Identificación de tareas

Cada tarea usa un ID jerárquico con el formato `F{fase}.{área}.{tarea}[.{subtarea}]`:

- **F1, F2, F3, F4** — Fases del plan maestro
- **Áreas**: `FW` (Framework), `CLI` (CLI Rust), `CI` (CI/CD y scripts), `DOC` (Documentación), `SK` (Skills/Workflows), `QA` (Testing/Verificación)
- **Ejemplo**: `F1.FW.03.02` = Fase 1, Framework, tarea 3, subtarea 2

### Estados

- `[ ]` — Pendiente
- `[~]` — En progreso
- `[x]` — Completada
- `[!]` — Bloqueada (ver notas)

### Prioridad

- **P0** — Crítica (bloqueante para el resto)
- **P1** — Alta (necesaria para el release de fase)
- **P2** — Media (mejora significativa)
- **P3** — Baja (nice-to-have, puede posponerse)

### Dependencias

Se indican como `→ depende de F1.FW.01` tras la subtarea.

---

## Resumen de Fases y Releases

| Fase | Período | Release FW | Release CLI | Entregable principal |
| --- | --- | --- | --- | --- |
| **1** | Abril — Mayo 2026 | fw-3.0.0 | cli-1.3.0 | Base regulatoria + estándares actualizados |
| **2** | Mayo — Julio 2026 | fw-3.1.0 | cli-1.4.0 | Nuevos tipos documentales + `devtrail validate` |
| **3** | Agosto — Octubre 2026 | fw-3.2.0 | cli-2.0.0 | `devtrail compliance` + `devtrail metrics` + ISO 42001 |
| **4** | Noviembre — Diciembre 2026 | fw-4.0.0 | cli-2.1.0 | `devtrail audit` + C4 Model + ecosistema |

---

# FASE 1: Correcciones Críticas y Base Regulatoria

**Objetivo**: Actualizar estándares obsoletos, establecer campos regulatorios en templates, fortalecer directivas y definir la base de gobernanza ISO 42001.

**Releases**: fw-3.0.0 / cli-1.3.0

---

## F1.FW — Framework: Templates y Gobernanza

### F1.FW.01 — Actualizar referencia de requerimientos de IEEE 830 a ISO/IEC/IEEE 29148:2018 `P0`

- [x] **F1.FW.01.01** — En `TEMPLATE-REQ.md` (EN): reemplazar toda mención de "IEEE 830" por "ISO/IEC/IEEE 29148:2018" en comentarios, encabezados y secciones de referencia
- [x] **F1.FW.01.02** — En `TEMPLATE-REQ.md` (EN): agregar nueva sección `## External Interfaces` con subsecciones: `### User Interfaces`, `### Hardware Interfaces`, `### Software Interfaces`, `### Communications Interfaces` — cada una con tabla de campos: Interface ID, Description, Source, Protocol/Format, Data Items, Constraints
- [x] **F1.FW.01.03** — En `TEMPLATE-REQ.md` (EN): agregar sección `## Verification and Validation` con tabla: Requirement ID, Verification Method (inspection/analysis/demonstration/test), Acceptance Criteria, Responsible
- [x] **F1.FW.01.04** — En `TEMPLATE-REQ.md` (EN): agregar campo `stakeholder_type` al frontmatter con opciones: `end_user | operator | acquirer | regulator | maintainer | developer` para distinguir stakeholder vs. system requirements según 29148
- [x] **F1.FW.01.05** — En `TEMPLATE-REQ.md` (EN): agregar sección `## Traceability` con tabla: Stakeholder Need → System Requirement → Acceptance Test — para trazabilidad conforme 29148 §6.3
- [x] **F1.FW.01.06** — Replicar cambios F1.FW.01.01 a F1.FW.01.05 en `templates/i18n/es/TEMPLATE-REQ.md` con textos en español

### F1.FW.02 — Actualizar modelo de calidad de ISO/IEC 25010:2011 a ISO/IEC 25010:2023 `P0`

- [x] **F1.FW.02.01** — Crear archivo de referencia `dist/.devtrail/00-governance/ISO-25010-2023-REFERENCE.md` que documente las 9 características de calidad y sus subcaracterísticas, incluyendo los cambios respecto a 2011: Safety (nueva), Interaction Capability (renombrada de Usability), Flexibility (renombrada de Portability), Faultlessness (reemplaza Maturity), User Engagement (reemplaza User Interface Aesthetics), Inclusivity y User Assistance (divididas de Accessibility), Self-descriptiveness (nueva), Resistance (nueva en Security), Scalability (nueva en Flexibility)
- [x] **F1.FW.02.02** — En `TEMPLATE-ADR.md` (EN), sección "Consequences": actualizar la referencia a ISO 25010 para usar la taxonomía 2023. Donde se mencione "Usability" cambiar a "Interaction Capability", donde se mencione "Portability" cambiar a "Flexibility". Agregar "Safety" como dimensión de consecuencias a evaluar con sus 5 subcaracterísticas: Operational Constraint, Risk Identification, Fail Safe, Hazard Warning, Safe Integration
- [x] **F1.FW.02.03** — En `TEMPLATE-REQ.md` (EN), sección "Non-Functional Requirements": actualizar las categorías para reflejar las 9 características de ISO 25010:2023. Renombrar headers existentes y agregar Safety como nueva categoría
- [x] **F1.FW.02.04** — Replicar cambios F1.FW.02.02 y F1.FW.02.03 en templates ES

### F1.FW.03 — Alinear pruebas con ISO/IEC/IEEE 29119-3:2021 `P1`

- [x] **F1.FW.03.01** — En `TEMPLATE-TES.md` (EN): reestructurar para reflejar la jerarquía de 29119-3. Agregar sección introductoria que explique los tres niveles: Organizational Test Policy (nivel org) → Test Strategy (nivel proyecto) → Test Plan (este documento). Aclarar que este template corresponde al nivel "Test Plan"
- [x] **F1.FW.03.02** — En `TEMPLATE-TES.md` (EN): renombrar sección "Test Strategy" actual a "Test Approach" (para evitar confusión con el nivel organizacional de 29119-3). Agregar subsecciones: Test Design Techniques, Test Completion Criteria, Test Suspension/Resumption Criteria
- [x] **F1.FW.03.03** — En `TEMPLATE-TES.md` (EN): agregar sección `## Test Data Requirements` con campos: Data Set ID, Source, Preparation Steps, Sensitivity Classification, Retention Policy. Agregar sección `## Test Environment Requirements` con campos: Component, Version, Configuration, Dependencies
- [x] **F1.FW.03.04** — En `TEMPLATE-TES.md` (EN): en la sección de resultados, adoptar terminología 29119-3: "Test Execution Log" (no solo "Results"), "Test Incident Report" (para fallos), "Test Status Report" (para progreso), "Test Completion Report" (para cierre)
- [x] **F1.FW.03.05** — Replicar cambios F1.FW.03.01 a F1.FW.03.04 en template ES

### F1.FW.04 — Enriquecer TEMPLATE-AILOG.md con campos regulatorios `P0`

- [x] **F1.FW.04.01** — En frontmatter de `TEMPLATE-AILOG.md` (EN): agregar campos opcionales:
```yaml
  eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
  nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
  iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
  lines_changed: 0               # Auto-calculable
  files_modified: []              # Auto-calculable
```
- [x] **F1.FW.04.02** — Agregar sección condicional `## EU AI Act Considerations` (después de Impact) con campos: Risk Classification, Annex III Category (si aplica), Conformity Assessment Required (Yes/No), Transparency Obligations. Incluir nota: "Complete this section only if `eu_ai_act_risk` is `high` or `limited`"
- [x] **F1.FW.04.03** — Agregar sección condicional `## NIST GenAI Risk Assessment` (después de EU AI Act) con tabla de las 12 categorías NIST AI 600-1: Category, Applicable (Yes/No), Description, Mitigation. Incluir nota: "Complete this section when the change involves generative AI components"
- [x] **F1.FW.04.04** — En sección "Impact" existente: agregar campos `Privacy` y `Environmental` a la lista existente (Functionality, Performance, Security)
- [x] **F1.FW.04.05** — En sección "Verification" existente: agregar checkboxes: `- [ ] Security scan passed (if risk_level: high/critical)` y `- [ ] Privacy review completed (if handling PII)`
- [x] **F1.FW.04.06** — En sección "Modified Files": cambiar formato a tabla con columnas: File, Lines Changed (+N/-M), Change Description
- [x] **F1.FW.04.07** — Replicar todos los cambios F1.FW.04.01 a F1.FW.04.06 en template ES

### F1.FW.05 — Enriquecer TEMPLATE-ETH.md con compliance regulatorio `P0`

- [x] **F1.FW.05.01** — En frontmatter de `TEMPLATE-ETH.md` (EN): agregar campos:
```yaml
  eu_ai_act_risk: not_applicable
  nist_genai_risks: []
  iso_42001_clause: []
  gdpr_legal_basis: none  # consent | contract | legal_obligation | vital_interests | public_task | legitimate_interests | none
  fria_required: false     # Fundamental Rights Impact Assessment
```
- [x] **F1.FW.05.02** — Agregar sección `## EU AI Act Risk Classification` con: tabla de clasificación (Unacceptable → Prohibited, High → Full compliance required, Limited → Transparency obligations, Minimal → No specific obligations), campo para categoría Annex III específica, y checklist de obligaciones según nivel de riesgo
- [x] **F1.FW.05.03** — En sección "Data Privacy" existente: agregar subsección `### GDPR Legal Basis` con tabla: Processing Activity, Legal Basis (Art. 6 GDPR), Justification, Data Retention Period. Agregar subsección `### Data Protection Impact Assessment Reference` con enlace a DPIA correspondiente si existe
- [x] **F1.FW.05.04** — En sección "Bias and Fairness" existente: agregar subsección `### Protected Characteristics` con tabla: Characteristic (age, disability, gender, race, religion, sexual orientation, etc.), Potentially Affected (Yes/No), Assessment, Mitigation
- [x] **F1.FW.05.05** — Agregar nueva sección `## Environmental Impact` con campos: Training Energy Estimate (kWh), CO2 Equivalent (tons), Hardware Used, Inference Cost per Request, Mitigation Measures
- [x] **F1.FW.05.06** — Agregar nueva sección `## Dual-Use Potential` con campos: Beneficial Uses (lista), Potential Misuses (lista), Safeguards Implemented, Residual Risk Assessment
- [x] **F1.FW.05.07** — Agregar nueva sección `## Fundamental Rights Impact Assessment (FRIA)` (requerida por Art. 27 EU AI Act para deployers de alto riesgo) con campos: Categories of Affected Persons, Specific Risks to Fundamental Rights, Period and Frequency of Use, Governance Measures, Human Oversight Processes
- [x] **F1.FW.05.08** — Replicar todos los cambios en template ES

### F1.FW.06 — Enriquecer TEMPLATE-ADR.md `P1`

- [x] **F1.FW.06.01** — En frontmatter: agregar campos opcionales `iso_42001_clause: []` y `eu_ai_act_risk: not_applicable`
- [x] **F1.FW.06.02** — Agregar nota visible al inicio del cuerpo: "**Immutability Rule**: Once an ADR reaches `accepted` status, it MUST NOT be modified. If the decision changes, create a new ADR with `supersedes: ADR-YYYY-MM-DD-NNN` in its frontmatter. The original ADR's status changes to `superseded`."
- [x] **F1.FW.06.03** — Agregar sección `## Validation Criteria` después de "Consequences" con campos: Metric, Target Value, Measurement Method, Timeline. Ejemplo pre-llenado: "Response time < 200ms measured via load test at 30 days post-deployment"
- [x] **F1.FW.06.04** — En sección "Alternatives": agregar campo `alternatives_documented: []` en frontmatter para linkear a AIDEC-* relacionados
- [x] **F1.FW.06.05** — Replicar cambios en template ES

### F1.FW.07 — Enriquecer templates restantes con campos regulatorios `P1`

- [x] **F1.FW.07.01** — `TEMPLATE-AIDEC.md` (EN): agregar campos opcionales `eu_ai_act_risk`, `nist_genai_risks`, `iso_42001_clause` al frontmatter. Sin nuevas secciones en cuerpo (AIDEC es conciso por diseño)
- [x] **F1.FW.07.02** — `TEMPLATE-INC.md` (EN): agregar al frontmatter `eu_ai_act_applicable: false` y `incident_report_deadline: null`. Agregar sección `## EU AI Act Incident Reporting` con nota: "For high-risk AI systems under EU AI Act, incidents must be reported to the market surveillance authority within: 15 days (standard), 10 days (death), 2 days (widespread/very serious). Reference: Article 73, EU AI Act." Incluir campos: Report Deadline, Authority Notified (Yes/No/NA), Report Reference
- [x] **F1.FW.07.03** — `TEMPLATE-TDE.md` (EN): agregar `iso_42001_clause: []` al frontmatter. Sin nuevas secciones
- [x] **F1.FW.07.04** — `TEMPLATE-TES.md` (EN): agregar `iso_42001_clause: []` al frontmatter (se complementa con F1.FW.03)
- [x] **F1.FW.07.05** — Replicar todos los cambios en templates ES correspondientes

### F1.FW.08 — Crear AI-GOVERNANCE-POLICY.md (ISO 42001 base) `P0`

- [x] **F1.FW.08.01** — Crear `dist/.devtrail/00-governance/AI-GOVERNANCE-POLICY.md` con estructura alineada a las cláusulas 4-10 de ISO 42001:
  - Sección 1: "Scope and Context" (Cláusula 4) — Template para definir alcance del sistema de gestión de IA, partes interesadas, requisitos legales
  - Sección 2: "Leadership and Commitment" (Cláusula 5) — Template para política de IA, roles y responsabilidades, compromiso de dirección
  - Sección 3: "Risk Planning" (Cláusula 6) — Template para identificación de riesgos, objetivos de IA, planificación de cambios. Incluir referencia a los 9 topic areas de Annex A de ISO 42001 (A.2–A.10) con sus 39 controles
  - Sección 4: "Support and Resources" (Cláusula 7) — Template para recursos, competencias, concienciación, comunicación, información documentada
  - Sección 5: "AI Lifecycle Operations" (Cláusula 8) — Template para planificación operativa, gestión del ciclo de vida (diseño → despliegue → monitoreo → retiro), gestión continua de riesgos
  - Sección 6: "Performance Evaluation" (Cláusula 9) — Template para monitoreo, auditoría interna, revisión por dirección
  - Sección 7: "Continual Improvement" (Cláusula 10) — Template para no conformidades, acciones correctivas, mejora continua
- [x] **F1.FW.08.02** — Cada sección debe incluir: un ejemplo pre-llenado con datos ficticios realistas, una tabla de mapeo "ISO 42001 Control → DevTrail Document Type" (ej: A.5.2 Risk Assessment → ETH, A.6.2.9 Documentation → AILOG), y notas guía para el usuario
- [x] **F1.FW.08.03** — Agregar tabla resumen de mapeo cruzado de Annex A controls a documentos DevTrail al final del documento:
  - A.2 (Policies for AI) → AI-GOVERNANCE-POLICY.md
  - A.3 (Internal Organization) → AI-GOVERNANCE-POLICY.md §2
  - A.4 (Resources) → AI-GOVERNANCE-POLICY.md §4
  - A.5 (Assessing Impacts) → ETH, DPIA (Fase 2)
  - A.6 (AI System Lifecycle) → AILOG, AIDEC, ADR, MCARD (Fase 2)
  - A.7 (Data for AI) → ETH (Data Privacy), SBOM (Fase 2)
  - A.8 (Information for Interested Parties) → ADR, REQ
  - A.9 (Use of AI Systems) → AGENT-RULES.md, AILOG
  - A.10 (Third-Party) → SBOM (Fase 2), ETH
- [x] **F1.FW.08.04** — Crear versión ES: `dist/.devtrail/00-governance/i18n/es/AI-GOVERNANCE-POLICY.md`

### F1.FW.09 — Actualizar documentos de gobernanza existentes `P1`

- [x] **F1.FW.09.01** — `DOCUMENTATION-POLICY.md`: agregar los 4 nuevos tipos documentales (SEC, MCARD, SBOM, DPIA) a la tabla de tipos, con sus carpetas destino, convención de naming, y estatus por defecto
- [x] **F1.FW.09.02** — `DOCUMENTATION-POLICY.md`: agregar los nuevos campos opcionales de frontmatter (eu_ai_act_risk, nist_genai_risks, iso_42001_clause, lines_changed, files_modified, observability_scope) a la sección de metadatos, marcados como opcionales con descripción de cuándo usarlos
- [x] **F1.FW.09.03** — `DOCUMENTATION-POLICY.md`: actualizar tabla de estándares referenciados: IEEE 830 → ISO/IEC/IEEE 29148:2018, ISO 25010 → ISO/IEC 25010:2023, agregar ISO/IEC 42001:2023, EU AI Act, NIST AI RMF 1.0, NIST AI 600-1, ISO/IEC 23894:2023
- [x] **F1.FW.09.04** — `AGENT-RULES.md`: reemplazar umbral de "10 líneas de código" por criterios híbridos:
  - Primario: Si el CLI está disponible y `lizard` está instalado, el agente puede invocar `devtrail analyze-complexity` para obtener delta de complejidad ciclomática. Documentar si delta > 5
  - Fallback: documentar obligatoriamente si: cambios en auth/authorization/PII, cambios en API pública, cambios en schema de BD, cambios en modelos de ML o prompts de IA, o >20 líneas de lógica de negocio
  - Referencia simplificada: mantener "~20 líneas de lógica de negocio" en QUICK-REFERENCE.md como regla rápida
- [x] **F1.FW.09.05** — `AGENT-RULES.md`: agregar triggers de review humana adicionales: "Changes to ML model parameters or architecture", "Changes to AI prompts or agent instructions", "Addition/removal/upgrade of security-critical dependencies", "Changes affecting AI system lifecycle status (deployment, retirement)"
- [x] **F1.FW.09.06** — `AGENT-RULES.md`: agregar reglas de observabilidad OpenTelemetry: "Do not capture PII, tokens, or secrets in OTel attributes or logs. Record instrumentation pipeline changes (new spans, changed attributes, Collector configuration) in AILOG with tag `observabilidad`. When adopting OTel in distributed projects, create AIDEC or ADR documenting the adoption decision and backend selection"
- [x] **F1.FW.09.07** — `AGENT-RULES.md`: agregar tabla de autonomía para los 4 nuevos tipos:
  - SEC → Draft → aprobación humana (review_required: true obligatorio)
  - MCARD → Draft → aprobación humana (review_required: true obligatorio)
  - SBOM → Crear libremente (es inventario factual)
  - DPIA → Draft → aprobación humana (review_required: true obligatorio)
- [x] **F1.FW.09.08** — `QUICK-REFERENCE.md`: actualizar con los 12 tipos documentales, nuevos campos, nuevos criterios de documentación, referencia a ISO 42001 y EU AI Act
- [x] **F1.FW.09.09** — Replicar cambios de F1.FW.09.01 a F1.FW.09.08 en versiones ES

### F1.FW.10 — Actualizar DEVTRAIL.md (documento maestro unificado) `P1`

> → depende de F1.FW.01 a F1.FW.09

- [x] **F1.FW.10.01** — Actualizar sección de tipos documentales: agregar SEC, MCARD, SBOM, DPIA con descripciones y autonomía
- [x] **F1.FW.10.02** — Actualizar sección de metadatos mínimos: incluir campos regulatorios opcionales con comentarios explicativos
- [x] **F1.FW.10.03** — Actualizar sección "When to Document": reemplazar criterios de 10 líneas por criterios híbridos (lizard + fallback)
- [x] **F1.FW.10.04** — Actualizar mapa de documentación con nuevas carpetas (08-security, 09-ai-models)
- [x] **F1.FW.10.05** — Agregar sección breve "Regulatory Alignment": explicar que DevTrail está alineado con ISO 42001, EU AI Act y NIST AI RMF, con referencia a AI-GOVERNANCE-POLICY.md
- [x] **F1.FW.10.06** — Actualizar tabla de estándares referenciados
- [x] **F1.FW.10.07** — Agregar OpenTelemetry a la lista de estándares complementarios en DEVTRAIL.md

### F1.FW.11 — Agregar secciones opcionales de observabilidad (OpenTelemetry) `P2`

> Basado en la propuesta evaluada en `evolution/recomendaciones/propuesta-opentelemetry.md`. Todas las secciones son opcionales y se activan con el tag `observabilidad`.

- [x] **F1.FW.11.01** — En `TEMPLATE-REQ.md` (EN): agregar sección condicional `## Observability Requirements` (después de Non-Functional Requirements) con campos: Coverage (endpoints que deben generar trazas), Trace Quality (% mínimo de spans con atributos clave), Max Trace Latency (tiempo máximo aceptable para disponibilidad de trazas), Retention Policy (periodo por entorno prod/dev), SLOs Linked to Observable Metrics. Incluir nota: "Complete this section when the project uses OpenTelemetry or has observability requirements. Activate with tag `observabilidad`"
- [x] **F1.FW.11.02** — En `TEMPLATE-TES.md` (EN): agregar sección condicional `## Observability Tests` (después de Test Environment Requirements) con pruebas sugeridas:
  - Verificar propagación de trace context en llamadas internas y externas (W3C Trace Context)
  - Validar correlación log-trace (`trace_id` y `span_id` presentes en logs)
  - Test de muestreo bajo carga (head sampling y tail sampling si aplica)
  - Test de redacción de datos sensibles en Collector (verificar que PII no llega al backend)
- [x] **F1.FW.11.03** — En `TEMPLATE-INC.md` (EN): en sección de timeline, agregar campos opcionales `trace_id` y `span_id` para evidencia basada en trazas OTel. Agregar campo `dashboard_links: []` para referencias a dashboards de observabilidad. Incluir nota: "If your system uses OpenTelemetry, include trace-id for correlated evidence"
- [x] **F1.FW.11.04** — En `TEMPLATE-AILOG.md` (EN): en sección "Change Description", agregar nota de instrumentación: "If this change modifies observability instrumentation (new spans, changed attributes, pipeline configuration), describe the observability impact and include tag `observabilidad`"
- [x] **F1.FW.11.05** — En frontmatter de `TEMPLATE-AILOG.md`, `TEMPLATE-REQ.md`, `TEMPLATE-TES.md` y `TEMPLATE-INC.md` (EN): agregar campo opcional `observability_scope: none  # none | basic | full` para indicar el nivel de instrumentación OTel relevante al documento. Incluir comentario: "Set to 'basic' or 'full' when the change involves OTel instrumentation"
- [x] **F1.FW.11.06** — Replicar cambios F1.FW.11.01 a F1.FW.11.05 en templates ES correspondientes

---

## F1.FW.DIR — Framework: Directivas de Agentes

### F1.FW.DIR.01 — Expandir directivas con checklists y reglas autónomas `P0`

- [x] **F1.FW.DIR.01.01** — En `dist/dist-templates/directives/CLAUDE.md`: agregar después del bloque existente:
  - Bloque de reglas mínimas autónomas (para que funcione sin cargar DEVTRAIL.md): identidad obligatoria del agente, review_required en ETH/ADR/SEC/MCARD/DPIA, prohibición de documentar secretos/PII
  - Checklist pre-commit:
```
    Before committing, check:
    - [ ] Changed auth/PII/security code? → Create AILOG (risk_level: high) + ETH draft
    - [ ] Changed >20 lines of business logic? → Create AILOG
    - [ ] Chose between 2+ alternatives? → Create AIDEC
    - [ ] Changed public API or DB schema? → Create AILOG + consider ADR
    - [ ] Changed ML model/prompts? → Create AILOG + human review
```
  - Snippet de frontmatter con campos regulatorios pre-llenados
  - Lista de las 12 categorías de riesgo NIST AI 600-1 para referencia rápida
  - Regla de observabilidad: "Do not capture PII, tokens, or secrets in OTel attributes or logs. Record instrumentation pipeline changes (new spans, changed attributes, Collector configuration) in AILOG with tag `observabilidad`"
- [x] **F1.FW.DIR.01.02** — Replicar la misma estructura expandida en `dist/dist-templates/directives/GEMINI.md`
- [x] **F1.FW.DIR.01.03** — Replicar en `dist/dist-templates/directives/copilot-instructions.md` (adaptando al formato Copilot)
- [x] **F1.FW.DIR.01.04** — Verificar que `dist/dist-templates/directives/cursorrules` existe y contiene contenido funcional. Si solo es stub, expandir con las mismas reglas mínimas
- [x] **F1.FW.DIR.01.05** — Actualizar `dist/dist-templates/directives/cursor-rules-devtrail.md` con las mismas reglas expandidas

---

## F1.CLI — CLI: Ajustes para fw-3.0.0

### F1.CLI.01 — Actualizar `devtrail init` para nuevas carpetas `P1`

- [x] **F1.CLI.01.01** — En `cli/src/commands/init.rs`: agregar `08-security` y `09-ai-models` a la constante `EXPECTED_DIRS` (o equivalente) que define las carpetas a crear. Agregar `.gitkeep` en cada una
- [x] **F1.CLI.01.02** — En `cli/src/commands/init.rs`: asegurar que los 4 nuevos templates (SEC, MCARD, SBOM, DPIA) se copien correctamente durante init
- [x] **F1.CLI.01.03** — En `cli/src/commands/init.rs`: asegurar que `AI-GOVERNANCE-POLICY.md` se copie a `00-governance/` durante init

### F1.CLI.02 — Actualizar `devtrail status` para 12 tipos `P1`

- [x] **F1.CLI.02.01** — En `cli/src/commands/status.rs`: agregar SEC, MCARD, SBOM, DPIA a la lista de prefijos que se cuentan en la función de estadísticas documentales
- [x] **F1.CLI.02.02** — En `cli/src/commands/status.rs`: agregar `08-security` y `09-ai-models` a `EXPECTED_DIRS` para verificación de estructura
- [x] **F1.CLI.02.03** — Verificar que el output formateado no se rompa con 12 tipos (posible ajuste de ancho de columnas)

### F1.CLI.03 — Actualizar `devtrail repair` `P2`

- [x] **F1.CLI.03.01** — En `cli/src/commands/repair.rs`: agregar las nuevas carpetas y archivos a la lógica de restauración → depende de F1.CLI.01

### F1.CLI.04 — Actualizar `devtrail explore` para nuevos tipos `P2`

- [x] **F1.CLI.04.01** — En `cli/src/tui/`: verificar que el browser de documentación descubra y muestre archivos en `08-security/` y `09-ai-models/`. Si el TUI usa EXPECTED_DIRS para listar, actualizar la constante

### F1.CLI.05 — Bump de versiones `P0`

> → depende de todos los F1.CLI anteriores

- [x] **F1.CLI.05.01** — Editar `cli/Cargo.toml`: cambiar version a `"1.3.0"`
- [x] **F1.CLI.05.02** — Ejecutar `cargo check` en `cli/` para actualizar `Cargo.lock`
- [x] **F1.CLI.05.03** — Editar `dist/dist-manifest.yml`: cambiar version a `"3.0.0"`
- [x] **F1.CLI.05.04** — Actualizar `dist-manifest.yml`: agregar nuevos archivos a la sección `files` (AI-GOVERNANCE-POLICY.md, ISO-25010-2023-REFERENCE.md, templates nuevos de Fase 2 si se incluyen como stubs)
- [x] **F1.CLI.05.05** — Actualizar referencias de versión en `docs/adopters/CLI-REFERENCE.md`

---

## F1.CI — CI/CD y Scripts: Mejoras inmediatas

### F1.CI.01 — Actualizar `pre-commit-docs.sh` con reglas cruzadas (mejora inmediata) `P1`

- [x] **F1.CI.01.01** — Agregar validación: si `risk_level` es `high` o `critical`, verificar que `review_required` sea `true`. Si no, emitir ERROR (no warning)
- [x] **F1.CI.01.02** — Agregar validación: verificar que el campo `id` del frontmatter coincida con el nombre del archivo (sin extensión `.md` ni la parte de description). Ej: archivo `AILOG-2026-03-23-001-fix-login.md` debe tener `id: AILOG-2026-03-23-001`
- [x] **F1.CI.01.03** — Actualizar la regex de tipos válidos para incluir SEC, MCARD, SBOM, DPIA: `ADR|REQ|TES|OPS|INC|TDE|AILOG|AIDEC|ETH|DOC|SEC|MCARD|SBOM|DPIA`
- [x] **F1.CI.01.04** — Agregar validación de campos obligatorios por tipo: INC debe tener `severity`, ETH/ADR/SEC/MCARD/DPIA deben tener `review_required: true`
- [x] **F1.CI.01.05** — Agregar validación opcional de observabilidad: si el documento tiene tag `observabilidad`, verificar que incluya al menos una sección de alcance o riesgos de instrumentación (warning, no error)

### F1.CI.02 — Actualizar `validate-docs.ps1` con las mismas reglas `P1`

- [x] **F1.CI.02.01** — Replicar regla de risk_level ↔ review_required (F1.CI.01.01)
- [x] **F1.CI.02.02** — Replicar validación de id vs filename (F1.CI.01.02)
- [x] **F1.CI.02.03** — Actualizar regex de tipos (F1.CI.01.03)
- [x] **F1.CI.02.04** — Replicar validaciones por tipo (F1.CI.01.04)
- [x] **F1.CI.02.05** — Replicar validación opcional de observabilidad (F1.CI.01.05)

### F1.CI.03 — Actualizar `docs-validation.yml` `P2`

- [x] **F1.CI.03.01** — Actualizar regex de tipos en el workflow de GitHub Actions para incluir los 12 tipos
- [x] **F1.CI.03.02** — Agregar paso de validación de reglas cruzadas (risk_level ↔ review_required) al job `validate-docs`

---

## F1.DOC — Documentación del Proyecto

### F1.DOC.01 — Actualizar README.md `P1`

- [x] **F1.DOC.01.01** — Actualizar tabla de estándares: IEEE 830 → ISO/IEC/IEEE 29148:2018, ISO 25010 → 2023, agregar ISO 42001, EU AI Act, NIST AI RMF
- [x] **F1.DOC.01.02** — Actualizar número de tipos documentales de 8 a 12
- [x] **F1.DOC.01.03** — Agregar mención a AI-GOVERNANCE-POLICY.md como nuevo documento de gobernanza

### F1.DOC.02 — Actualizar documentación en docs/ `P2`

- [x] **F1.DOC.02.01** — `docs/adopters/ADOPTION-GUIDE.md`: actualizar referencias de estándares y agregar sección sobre alineación ISO 42001
- [x] **F1.DOC.02.02** — `docs/adopters/CLI-REFERENCE.md`: actualizar output de ejemplo de `devtrail status` para mostrar 12 tipos
- [x] **F1.DOC.02.03** — `docs/adopters/DOCUMENTATION-STANDARDS-REVIEW.md`: N/A — archivo no existe en el repositorio, tarea eliminada

---

## F1.QA — Testing y Verificación de Fase 1

### F1.QA.01 — Tests de integración CLI `P1`

> → depende de F1.CLI.01 a F1.CLI.04

- [x] **F1.QA.01.01** — Actualizar `cli/tests/init_test.rs`: verificar que init crea las carpetas `08-security` y `09-ai-models`
- [x] **F1.QA.01.02** — Actualizar `cli/tests/status_test.rs`: verificar que status reporta los 12 tipos correctamente
- [x] **F1.QA.01.03** — Ejecutar `cargo test` completo y verificar que los 26+ tests pasan
- [x] **F1.QA.01.04** — Ejecutar `cargo build --release` para verificar compilación limpia

### F1.QA.02 — Verificación manual de templates `P1`

- [x] **F1.QA.02.01** — Verificar que todos los templates EN tienen frontmatter YAML válido (parseable sin errores)
- [x] **F1.QA.02.02** — Verificar que todos los templates ES tienen frontmatter YAML válido
- [x] **F1.QA.02.03** — Verificar que los campos nuevos opcionales no causan errores en scripts de validación existentes (pre-commit, PS1, CI)
- [x] **F1.QA.02.04** — Verificar que `devtrail init` en un directorio vacío genera la estructura completa correctamente, incluyendo nuevas carpetas y AI-GOVERNANCE-POLICY.md

---

# FASE 2: Nuevos Tipos Documentales y Validación

**Objetivo**: Crear los 4 nuevos templates, fortalecer validación, implementar `devtrail validate`, crear skills interactivas.

**Releases**: fw-3.1.0 / cli-1.4.0

**Prerequisito**: Fase 1 completada y mergeada a main.

---

## F2.FW — Framework: Nuevos Templates

### F2.FW.01 — Crear TEMPLATE-SEC.md (Security Assessment) `P0`

- [x] **F2.FW.01.01** — Diseñar frontmatter:
```yaml
  id: SEC-YYYY-MM-DD-NNN
  title: "[System/Component] Security Assessment"
  status: draft
  created: YYYY-MM-DD
  agent: [agent-name]
  confidence: medium
  review_required: true  # Siempre obligatorio
  risk_level: high       # Mínimo: high
  eu_ai_act_risk: not_applicable
  iso_42001_clause: [6, 8]
  threat_model_methodology: STRIDE | PASTA | LINDDUN | custom
  owasp_asvs_level: 1 | 2 | 3
  tags: [security]
  related: []
```
- [x] **F2.FW.01.02** — Crear sección `## Scope and Objectives` con: System Under Assessment, Assessment Type (design review/code review/penetration test/threat model), Assessment Date, Assessor
- [x] **F2.FW.01.03** — Crear sección `## Threat Model` con subsecciones según metodología STRIDE: Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service, Elevation of Privilege. Cada una con tabla: Threat ID, Description, Likelihood (1-5), Impact (1-5), Risk Score, Mitigation
- [x] **F2.FW.01.04** — Crear sección `## OWASP ASVS Compliance` con tabla de verificación de controles ASVS organizados por los 17 capítulos de ASVS 5.0. Campos: Control ID, Description, Level (L1/L2/L3), Status (Pass/Fail/NA), Evidence, Notes
- [x] **F2.FW.01.05** — Crear sección `## Vulnerabilities Found` con tabla: Vuln ID, CWE, Severity (CVSS), Description, Affected Component, Remediation, Status (open/mitigated/accepted)
- [x] **F2.FW.01.06** — Crear sección `## Security Controls` con tabla basada en OWASP SAMM: Business Function (Governance/Design/Implementation/Verification/Operations), Practice, Maturity Level (1-3), Current Status, Gaps
- [x] **F2.FW.01.07** — Crear sección `## Recommendations` con: Priority, Description, Effort, Impact
- [x] **F2.FW.01.08** — Crear versión ES del template

### F2.FW.02 — Crear TEMPLATE-MCARD.md (Model/System Card) `P0`

- [x] **F2.FW.02.01** — Diseñar frontmatter:
```yaml
  id: MCARD-YYYY-MM-DD-NNN
  title: "[Model Name] Card"
  status: draft
  created: YYYY-MM-DD
  agent: [agent-name]
  confidence: medium
  review_required: true
  risk_level: medium
  eu_ai_act_risk: not_applicable
  nist_genai_risks: []
  iso_42001_clause: [8]
  model_name: ""
  model_type: LLM | classifier | regressor | generator | recommender | other
  model_version: ""
  provider: ""
  license: ""
  tags: [ai-model]
  related: []
```
- [x] **F2.FW.02.02** — Crear sección `## Model Details` (basada en Mitchell et al. 2019): Developer, Model Date, Model Version, Model Type, Training Algorithms, Base Model (if fine-tuned), Paper/Resource, Citation, License
- [x] **F2.FW.02.03** — Crear sección `## Intended Use` con: Primary Intended Uses, Primary Intended Users, Out-of-Scope Uses
- [x] **F2.FW.02.04** — Crear sección `## Training Data` con: Dataset Name, Source, Size, Collection Methodology, Preprocessing, Known Limitations, PII Assessment, License. Incluir nota sobre CycloneDX modelCard.modelParameters para interoperabilidad con SBOM
- [x] **F2.FW.02.05** — Crear sección `## Performance Metrics` con tabla: Metric, Value, Test Dataset, Confidence Interval, Conditions. Separar por disaggregated evaluation si aplica
- [x] **F2.FW.02.06** — Crear sección `## Bias and Fairness Evaluation` con tabla: Demographic Group, Metric, Performance, Disparity vs Baseline, Mitigation Applied
- [x] **F2.FW.02.07** — Crear sección `## Environmental Impact` con: Training Energy (kWh), CO2 Equivalent, Hardware Used, Training Duration, Inference Cost, Region/Grid Carbon Intensity
- [x] **F2.FW.02.08** — Crear sección `## Security Considerations` con: Known Vulnerabilities, Adversarial Robustness Assessment, Prompt Injection Risk (Low/Med/High), Data Poisoning Risk, Model Extraction Risk
- [x] **F2.FW.02.09** — Crear sección `## Ethical Considerations` con: Sensitive Data Used, Human Subjects in Training, Dual-Use Potential, Societal Impact Assessment
- [x] **F2.FW.02.10** — Crear sección `## Limitations and Recommendations` con: Known Limitations, Failure Modes, Recommendations for Deployers
- [x] **F2.FW.02.11** — Crear versión ES del template

### F2.FW.03 — Crear TEMPLATE-SBOM.md (Software Bill of Materials para IA) `P0`

- [x] **F2.FW.03.01** — Diseñar frontmatter:
```yaml
  id: SBOM-YYYY-MM-DD-NNN
  title: "[System/Component] AI SBOM"
  status: accepted
  created: YYYY-MM-DD
  agent: [agent-name]
  confidence: high
  review_required: false  # Inventario factual
  risk_level: low
  iso_42001_clause: [8]
  sbom_format_reference: SPDX-3.0 | CycloneDX-1.6 | custom
  system_name: ""
  tags: [sbom, supply-chain]
  related: []
```
- [x] **F2.FW.03.02** — Crear sección `## AI/ML Components` con tabla: Component Name, Version, Provider, Type (model/library/service/dataset), License, Risk Level, Vulnerability Status, Last Audit Date. Nota: "This section maps to CycloneDX `component` with `type: machine-learning-model`"
- [x] **F2.FW.03.03** — Crear sección `## Training Data Sources` con tabla: Dataset, Source, License, PII Included (Yes/No), Bias Assessment Summary, Data Provenance, Retention Policy. Nota: "Aligns with ISO 42001 Annex A.7 (Data for AI Systems)"
- [x] **F2.FW.03.04** — Crear sección `## Third-Party AI Services` con tabla: Service, Provider, Purpose, Data Shared, DPA in Place (Yes/No), SLA, Region, Compliance Certifications
- [x] **F2.FW.03.05** — Crear sección `## Software Dependencies` con tabla: Package, Version, License, Known Vulnerabilities (CVE list), Last Updated. Nota: "Consider generating this section automatically with tools like `syft` or `trivy`"
- [x] **F2.FW.03.06** — Crear sección `## Supply Chain Risk Assessment` con: Overall Risk Level, Key Risks Identified, Mitigations, Monitoring Plan. Nota: "Aligns with NIST AI 600-1 Category 12: Value Chain and Component Integration"
- [x] **F2.FW.03.07** — Crear versión ES del template

### F2.FW.04 — Crear TEMPLATE-DPIA.md (Data Protection Impact Assessment) `P0`

- [x] **F2.FW.04.01** — Diseñar frontmatter:
```yaml
  id: DPIA-YYYY-MM-DD-NNN
  title: "[System/Process] Data Protection Impact Assessment"
  status: draft
  created: YYYY-MM-DD
  agent: [agent-name]
  confidence: low  # DPIA requiere juicio humano extenso
  review_required: true  # Siempre obligatorio
  risk_level: high
  eu_ai_act_risk: not_applicable
  gdpr_article_35: true  # Art. 35 GDPR triggers DPIA
  iso_42001_clause: [6, 8]
  dpo_consulted: false
  supervisory_authority_consulted: false
  tags: [privacy, gdpr, dpia]
  related: []
```
- [x] **F2.FW.04.02** — Crear sección `## Processing Description` con: Nature of Processing, Scope, Context, Purpose, Legal Basis (Art. 6 GDPR), Categories of Data Subjects, Categories of Personal Data, Recipients, International Transfers, Retention Period
- [x] **F2.FW.04.03** — Crear sección `## Necessity and Proportionality` con: Necessity Assessment, Purpose Limitation, Data Minimization, Storage Limitation, Data Quality
- [x] **F2.FW.04.04** — Crear sección `## Risk Assessment` con tabla: Risk, Likelihood (Low/Med/High), Severity (Low/Med/High), Risk Level, Source of Risk, Nature of Impact (physical/material/non-material). Separar por categorías: Risks to Data Subjects, Risks to Rights and Freedoms
- [x] **F2.FW.04.05** — Crear sección `## Measures to Mitigate Risks` con tabla: Risk, Measure, Type (technical/organizational/legal), Residual Risk, Responsible
- [x] **F2.FW.04.06** — Crear sección `## Consultation` con: Data Protection Officer Opinion, Data Subjects Consulted (Yes/No, methodology), Supervisory Authority Consulted (Yes/No, reference). Nota: "If residual risk remains high after mitigation, consultation with the supervisory authority is mandatory per Art. 36 GDPR"
- [x] **F2.FW.04.07** — Crear sección `## AI-Specific Considerations` (cuando aplique EU AI Act): Automated Decision-Making (Art. 22 GDPR), Right to Explanation, Human Oversight Measures, Transparency to Data Subjects
- [x] **F2.FW.04.08** — Crear sección `## Review Schedule` con: Next Review Date, Review Trigger Events, Review Responsible
- [x] **F2.FW.04.09** — Crear versión ES del template

### F2.FW.05 — Actualizar estructura de carpetas en dist/ `P1`

> → depende de F2.FW.01 a F2.FW.04

- [x] **F2.FW.05.01** — Crear `dist/.devtrail/08-security/.gitkeep`
- [x] **F2.FW.05.02** — Crear `dist/.devtrail/09-ai-models/.gitkeep`
- [x] **F2.FW.05.03** — Agregar los 4 nuevos templates EN a `dist/.devtrail/templates/`
- [x] **F2.FW.05.04** — Agregar los 4 nuevos templates ES a `dist/.devtrail/templates/i18n/es/`
- [x] **F2.FW.05.05** — Actualizar `dist/dist-manifest.yml` con los nuevos archivos en sección `files`

---

## F2.SK — Skills y Workflows Interactivos

### F2.SK.01 — Crear skill `devtrail-sec` (Security Assessment) `P1`

- [x] **F2.SK.01.01** — Crear `dist/.claude/skills/devtrail-sec/SKILL.md` con:
  - Invocación: `/devtrail-sec [component-name]`
  - Flujo interactivo paso a paso: (1) Preguntar alcance del assessment, (2) Preguntar metodología de threat model (STRIDE recomendado), (3) Preguntar nivel OWASP ASVS objetivo (L1/L2/L3), (4) Generar documento con campos pre-llenados basándose en el contexto del código, (5) Marcar como draft con review_required: true
  - Allowed tools: Read, Write, Glob, Bash (git, date)
  - Ejemplo completo de output esperado
- [x] **F2.SK.01.02** — Crear `dist/.gemini/skills/devtrail-sec/SKILL.md` adaptado al formato Gemini
- [x] **F2.SK.01.03** — Crear `dist/.agent/workflows/devtrail-sec.md` como workflow genérico

### F2.SK.02 — Crear skill `devtrail-mcard` (Model Card) `P1`

- [x] **F2.SK.02.01** — Crear `dist/.claude/skills/devtrail-mcard/SKILL.md` con:
  - Invocación: `/devtrail-mcard [model-name]`
  - Flujo interactivo: (1) Preguntar nombre y tipo del modelo, (2) Preguntar proveedor y versión, (3) Preguntar usos previstos, (4) Guiar al usuario por cada sección con preguntas específicas y ejemplos de respuesta, (5) Generar documento con review_required: true
  - Incluir ejemplos de respuestas bien formadas para cada sección
- [x] **F2.SK.02.02** — Crear versión Gemini
- [x] **F2.SK.02.03** — Crear workflow genérico

### F2.SK.03 — Actualizar skill `devtrail-new` para 12 tipos `P1`

- [x] **F2.SK.03.01** — En `dist/.claude/skills/devtrail-new/SKILL.md`: agregar SEC, MCARD, SBOM, DPIA al listado de tipos disponibles con descripciones y carpetas destino
- [x] **F2.SK.03.02** — Replicar en `dist/.gemini/skills/devtrail-new/SKILL.md`
- [x] **F2.SK.03.03** — Replicar en `dist/.agent/workflows/devtrail-new.md`

### F2.SK.04 — Actualizar skill `devtrail-status` para 12 tipos `P1`

- [x] **F2.SK.04.01** — Actualizar skills de status en las 3 plataformas para buscar y contar los 12 tipos

### F2.SK.05 — Actualizar script `devtrail-new.sh` `P2`

- [x] **F2.SK.05.01** — Agregar los 4 nuevos tipos al menú interactivo del script bash
- [x] **F2.SK.05.02** — Agregar las carpetas destino correctas para cada nuevo tipo

---

## F2.CLI — CLI: Comando `devtrail validate`

### F2.CLI.01 — Implementar módulo de parseo de documentos `P0`

> Este módulo será compartido por validate, compliance, metrics y audit.

- [x] **F2.CLI.01.01** — Crear `cli/src/document.rs` con struct `DevTrailDocument`:
```rust
  pub struct DevTrailDocument {
      pub path: PathBuf,
      pub filename: String,
      pub doc_type: DocType,
      pub frontmatter: Frontmatter,
      pub body: String,
  }
```
- [x] **F2.CLI.01.02** — Crear enum `DocType` con los 12 tipos: AILOG, AIDEC, ADR, ETH, REQ, TES, INC, TDE, SEC, MCARD, SBOM, DPIA
- [x] **F2.CLI.01.03** — Crear struct `Frontmatter` con todos los campos (obligatorios como `Option<T>` para que el validador reporte cuáles faltan):
```rust
  pub struct Frontmatter {
      pub id: Option<String>,
      pub title: Option<String>,
      pub status: Option<String>,
      pub created: Option<String>,
      pub agent: Option<String>,
      pub confidence: Option<String>,
      pub review_required: Option<bool>,
      pub risk_level: Option<String>,
      pub eu_ai_act_risk: Option<String>,
      pub nist_genai_risks: Option<Vec<String>>,
      pub iso_42001_clause: Option<Vec<u8>>,
      pub tags: Option<Vec<String>>,
      pub related: Option<Vec<String>>,
      // ... campos adicionales por tipo
  }
```
- [x] **F2.CLI.01.04** — Implementar función `parse_document(path: &Path) -> Result<DevTrailDocument>` que: lea el archivo, extraiga frontmatter YAML entre `---`, parsee con serde_yaml, determine DocType por prefijo del filename
- [x] **F2.CLI.01.05** — Implementar función `discover_documents(root: &Path) -> Vec<PathBuf>` que recorra `.devtrail/` recursivamente y retorne archivos que matcheen el patrón de naming `TYPE-YYYY-MM-DD-NNN-*.md`, excluyendo templates y archivos de gobernanza
- [x] **F2.CLI.01.06** — Agregar `pub mod document;` a `cli/src/main.rs`

### F2.CLI.02 — Implementar motor de validación `P0`

> → depende de F2.CLI.01

- [x] **F2.CLI.02.01** — Crear `cli/src/validation.rs` con struct `ValidationResult`:
```rust
  pub struct ValidationResult {
      pub errors: Vec<ValidationIssue>,
      pub warnings: Vec<ValidationIssue>,
  }
  pub struct ValidationIssue {
      pub file: PathBuf,
      pub rule: String,      // e.g., "NAMING-001"
      pub message: String,
      pub severity: Severity, // Error | Warning
      pub fix_hint: Option<String>,
  }
```
- [x] **F2.CLI.02.02** — Implementar regla `NAMING-001`: verificar que el nombre del archivo sigue el patrón `TYPE-YYYY-MM-DD-NNN-description.md` con TYPE en los 12 tipos válidos
- [x] **F2.CLI.02.03** — Implementar regla `META-001`: verificar presencia de campos obligatorios: id, title, status, created, agent, confidence, review_required, risk_level
- [x] **F2.CLI.02.04** — Implementar regla `META-002`: verificar que el campo `id` del frontmatter coincida con el prefijo del nombre de archivo
- [x] **F2.CLI.02.05** — Implementar regla `META-003`: verificar que `status` tenga un valor válido según DOCUMENTATION-POLICY.md
- [x] **F2.CLI.02.06** — Implementar regla `CROSS-001`: si `risk_level` es `high` o `critical`, `review_required` DEBE ser `true`
- [x] **F2.CLI.02.07** — Implementar regla `CROSS-002`: si `eu_ai_act_risk` es `high`, `review_required` DEBE ser `true`
- [x] **F2.CLI.02.08** — Implementar regla `CROSS-003`: para tipos SEC, MCARD, DPIA, `review_required` DEBE ser `true`
- [x] **F2.CLI.02.09** — Implementar regla `TYPE-001`: INC debe tener campo `severity`
- [x] **F2.CLI.02.10** — Implementar regla `TYPE-002`: ETH debe tener campo `gdpr_legal_basis` si contiene la sección "Data Privacy"
- [x] **F2.CLI.02.11** — Implementar regla `REF-001`: verificar que todos los documentos listados en `related:` existan en `.devtrail/`. Emitir warning si no se encuentran
- [x] **F2.CLI.02.12** — Implementar regla `SEC-001`: buscar patrones de información sensible (password, api_key, secret, token, private_key, credentials, Bearer) en todo el documento. Emitir error si se encuentran
- [x] **F2.CLI.02.13** — Implementar regla `OBS-001` (warning): si el documento tiene tag `observabilidad`, verificar que contenga al menos una sección de alcance de instrumentación o riesgos de observabilidad. Emitir warning si no se encuentra
- [x] **F2.CLI.02.14** — Implementar función `validate_all(root: &Path) -> ValidationResult` que aplique todas las reglas a todos los documentos descubiertos

### F2.CLI.03 — Implementar comando `devtrail validate` `P0`

> → depende de F2.CLI.02

- [x] **F2.CLI.03.01** — Crear `cli/src/commands/validate.rs` con función `pub fn run(path: &str, fix: bool) -> Result<()>`
- [x] **F2.CLI.03.02** — Agregar variante `Validate` al enum `Commands` en main.rs con args: `path` (default "."), `--fix` (flag booleano)
- [x] **F2.CLI.03.03** — Agregar `pub mod validate;` en `commands/mod.rs`
- [x] **F2.CLI.03.04** — Implementar output formateado: header con ruta, luego errores agrupados por archivo (rojo), luego warnings (amarillo), luego resumen final: "X errors, Y warnings in Z documents"
- [x] **F2.CLI.03.05** — Implementar `--fix` para correcciones automáticas simples: agregar `review_required: true` a documentos de alto riesgo que no lo tengan, corregir `id` si no coincide con filename
- [x] **F2.CLI.03.06** — Exit code: 0 si no hay errores (warnings OK), 1 si hay errores
- [x] **F2.CLI.03.07** — Agregar al routing en main.rs match statement

### F2.CLI.04 — Integración de Lizard para complejidad ciclomática `P2`

- [x] **F2.CLI.04.01** — Crear `cli/src/complexity.rs` con función `pub fn analyze_complexity(paths: &[PathBuf]) -> Result<ComplexityReport>`
- [x] **F2.CLI.04.02** — Implementar detección de `lizard` en el PATH del sistema
- [x] **F2.CLI.04.03** — Implementar ejecución de `lizard --csv <paths>` y parseo del output CSV (lizard no soporta JSON nativo; CSV es el formato más portable). Campos a parsear: NLOC, CCN (cyclomatic complexity number), token, param, length, filename, function name
- [x] **F2.CLI.04.04** — Crear struct `ComplexityReport` con: functions analizadas, delta CCN vs baseline, funciones que exceden umbral (CCN > 5 delta)
- [x] **F2.CLI.04.05** — Implementar output JSON del reporte para que los skills de agentes puedan consumirlo: `devtrail analyze-complexity --json <paths>`
- [x] **F2.CLI.04.06** — Implementar fallback cuando lizard no está disponible: emitir warning con instrucciones de instalación (`pip install lizard`) y sugerir usar criterios cualitativos

### F2.CLI.05 — Bump de versiones Fase 2 `P0`

> → depende de F2.CLI.01 a F2.CLI.04

- [x] **F2.CLI.05.01** — Editar `cli/Cargo.toml`: cambiar version a `"1.4.0"`
- [x] **F2.CLI.05.02** — Ejecutar `cargo check`
- [x] **F2.CLI.05.03** — Editar `dist/dist-manifest.yml`: cambiar version a `"3.1.0"`
- [x] **F2.CLI.05.04** — Actualizar `dist-manifest.yml` sección files con todos los archivos nuevos

---

## F2.CI — CI/CD: Validación extendida

### F2.CI.01 — Agregar jobs de compliance a `docs-validation.yml` `P1`

- [x] **F2.CI.01.01** — Agregar job `compliance-check` que verifique: documentos de alto riesgo (`risk_level: high|critical`) tienen ETH asociado en `related:`, documentos con `eu_ai_act_risk: high` tienen sección "EU AI Act Considerations"
- [x] **F2.CI.01.02** — Agregar job `governance-metrics` (solo en push a main) que genere reporte en `$GITHUB_STEP_SUMMARY`: conteo de documentos por tipo, documentos de la semana, distribución de risk_level, tasa de review compliance

### F2.CI.02 — Agregar validación de referencias cruzadas `P2`

- [x] **F2.CI.02.01** — En `pre-commit-docs.sh`: agregar validación de que documentos en `related:` existen en `.devtrail/`
- [x] **F2.CI.02.02** — En `pre-commit-docs.sh`: agregar detección de cambios de código sin AILOG del mismo día (warning, no error)
- [x] **F2.CI.02.03** — Replicar ambas validaciones en `validate-docs.ps1`

---

## F2.QA — Testing y Verificación de Fase 2

### F2.QA.01 — Tests para `devtrail validate` `P0`

- [x] **F2.QA.01.01** — Crear `cli/tests/validate_test.rs` con tests:
  - Test con directorio sin DevTrail → mensaje de error apropiado
  - Test con documentos válidos → exit code 0, "0 errors"
  - Test con naming incorrecto → error NAMING-001
  - Test con frontmatter incompleto → error META-001
  - Test con id ≠ filename → error META-002
  - Test con risk_level: high + review_required: false → error CROSS-001
  - Test con información sensible → error SEC-001
  - Test con related inexistente → warning REF-001
  - Test con --fix corrigiendo review_required → archivo modificado correctamente
  - Test con tag `observabilidad` sin sección de alcance → warning OBS-001
- [x] **F2.QA.01.02** — Ejecutar `cargo test` completo
- [x] **F2.QA.01.03** — Verificar `cargo clippy` sin warnings

### F2.QA.02 — Verificación de templates nuevos `P1`

- [x] **F2.QA.02.01** — Crear un documento de prueba para cada nuevo tipo (SEC, MCARD, SBOM, DPIA) y validar con `devtrail validate`
- [x] **F2.QA.02.02** — Verificar que `devtrail-new.sh` crea correctamente documentos de los 4 nuevos tipos
- [x] **F2.QA.02.03** — Verificar que `devtrail init` en directorio vacío incluye los 4 nuevos templates y las nuevas carpetas

---

# FASE 3: Compliance Automatizado y Métricas

**Objetivo**: Implementar `devtrail compliance`, `devtrail metrics`, documentos de gobernanza ISO 42001, mapeo NIST AI RMF.

**Releases**: fw-3.2.0 / cli-2.0.0

**Prerequisito**: Fase 2 completada.

---

## F3.CLI — CLI: Nuevos Comandos

### F3.CLI.01 — Implementar `devtrail compliance` `P0`

> → depende de F2.CLI.01 (document.rs) y F2.CLI.02 (validation.rs)

- [x] **F3.CLI.01.01** — Crear `cli/src/commands/compliance.rs` con función `pub fn run(path: &str, standard: Option<&str>, all: bool, output: &str) -> Result<()>`
- [x] **F3.CLI.01.02** — Agregar variante `Compliance` al enum `Commands` con args: `path` (default "."), `--standard` (eu-ai-act | iso-42001 | nist-ai-rmf), `--all`, `--output` (text | markdown | json, default text)
- [x] **F3.CLI.01.03** — Implementar checker `eu-ai-act`:
  - Verificar que al menos 1 documento con `eu_ai_act_risk: high` existe SI hay documentos con `risk_level: high|critical` → "Risk Classification: X systems classified"
  - Verificar que documentos con `eu_ai_act_risk: high` tienen ETH asociado en `related:` → "Technical Documentation (Annex IV): X complete"
  - Verificar existencia de al menos 1 DPIA si hay ETH con `gdpr_article_35: true` → "FRIA/DPIA: X assessments"
  - Verificar que INC tiene campo de deadline regulatorio si `eu_ai_act_applicable: true` → "Incident Reporting: X compliant"
  - Calcular porcentaje de compliance general
- [x] **F3.CLI.01.04** — Implementar checker `iso-42001`:
  - Verificar existencia de `AI-GOVERNANCE-POLICY.md` en `00-governance/` → "Clause 4-5: Policy exists"
  - Verificar que hay al menos 1 ETH → "Clause 6: Risk planning exists"
  - Verificar que hay documentación de lifecycle (AILOG, AIDEC) → "Clause 8: Operations documented"
  - Verificar cobertura de Annex A controls a través de documentos existentes: A.5 → ETH/DPIA, A.6 → AILOG/MCARD, A.7 → SBOM (Data), A.9 → AGENT-RULES exists
  - Calcular porcentaje de cobertura
- [x] **F3.CLI.01.05** — Implementar checker `nist-ai-rmf`:
  - MAP: verificar AILOG con contexto → "MAP coverage: X documents"
  - MEASURE: verificar TES existentes → "MEASURE coverage: X test plans"
  - MANAGE: verificar ETH/INC existentes → "MANAGE coverage: X risk documents"
  - GOVERN: verificar AI-GOVERNANCE-POLICY y ADR → "GOVERN coverage: X governance documents"
  - Verificar mapeo de 12 categorías GenAI (NIST 600-1) en ETH → "GenAI Risk Coverage: X/12 categories"
- [x] **F3.CLI.01.06** — Implementar `--all`: ejecutar los 3 checkers y mostrar resumen consolidado
- [x] **F3.CLI.01.07** — Implementar output JSON con struct serializable para integración con herramientas externas
- [x] **F3.CLI.01.08** — Implementar output Markdown para uso en PRs y reportes

### F3.CLI.02 — Implementar `devtrail metrics` `P1`

> → depende de F2.CLI.01 (document.rs)

- [x] **F3.CLI.02.01** — Crear `cli/src/commands/metrics.rs` con función `pub fn run(path: &str, period: &str, output: &str) -> Result<()>`
- [x] **F3.CLI.02.02** — Agregar variante `Metrics` al enum `Commands` con args: `path`, `--period` (last-7-days | last-30-days | last-90-days | all, default last-30-days), `--output` (text | markdown | json)
- [x] **F3.CLI.02.03** — Agregar dependencia `chrono` a Cargo.toml para parseo y cálculo de fechas
- [x] **F3.CLI.02.04** — Implementar conteo de documentos por tipo y periodo: parsear campo `created` de cada documento, filtrar por rango de fechas, agrupar por DocType
- [x] **F3.CLI.02.05** — Implementar tasa de cumplimiento de reviews: contar documentos con `review_required: true`, de esos cuántos tienen `status: accepted|approved` (completed) vs `status: draft` (pending)
- [x] **F3.CLI.02.06** — Implementar distribución de niveles de riesgo: contar por risk_level (low/medium/high/critical), calcular porcentajes
- [x] **F3.CLI.02.07** — Implementar actividad por agente: agrupar por campo `agent`, contar documentos por agente
- [x] **F3.CLI.02.08** — Implementar tendencias: comparar período actual vs período anterior del mismo tamaño, calcular delta porcentual, mostrar ↑/↓/→
- [x] **F3.CLI.02.09** — Implementar output formateado con tablas coloreadas (reutilizar patrones de status.rs)
- [x] **F3.CLI.02.10** — Implementar output JSON y Markdown

### F3.CLI.03 — Bump de versión CLI a 2.0.0 `P0`

> → depende de F3.CLI.01 y F3.CLI.02

- [x] **F3.CLI.03.01** — Editar `cli/Cargo.toml`: cambiar version a `"2.0.0"`. Justificación: 2 nuevos comandos significativos con nuevas dependencias
- [x] **F3.CLI.03.02** — Agregar `chrono = "0.4"` a Cargo.toml si no existe
- [x] **F3.CLI.03.03** — Ejecutar `cargo check` y `cargo build --release`
- [x] **F3.CLI.03.04** — Actualizar CLI-REFERENCE.md con documentación de compliance y metrics

---

## F3.FW — Framework: Documentos ISO 42001 y NIST

### F3.FW.01 — Crear documentos de gobernanza ISO 42001 `P0`

- [x] **F3.FW.01.01** — Crear `dist/.devtrail/00-governance/AI-RISK-CATALOG.md` como template con:
  - Estructura de catálogo mapeada a las 12 categorías NIST AI 600-1 + riesgos adicionales de Annex C de ISO 42001
  - Tabla por categoría: Risk ID, Category, Description, Likelihood, Impact, Current Controls, Residual Risk, Owner, Review Date
  - Ejemplo pre-llenado para 3-4 riesgos comunes (bias en clasificador, leak de datos de entrenamiento, hallucination en generador, supply chain dependency)
  - Notas de mapeo a ISO 42001 Annex A.5 (Assessing Impacts) y proceso de ISO 23894 (4 fases: identificación → evaluación → tratamiento → monitoreo)
- [x] **F3.FW.01.02** — Crear `dist/.devtrail/00-governance/AI-LIFECYCLE-TRACKER.md` como template con:
  - Tabla de sistemas IA: System Name, Type, Current Phase (design/development/testing/deployment/monitoring/retirement), Version, Owner, Last Review
  - Mapeo a ISO 42001 Annex A.6 (AI System Lifecycle) con sus 10 controles
  - Fase de lifecycle con checkboxes: Design (A.6.2.2), Training/Testing (A.6.2.3), Verification/Validation (A.6.2.4), Deployment (A.6.2.5), Operation/Monitoring (A.6.2.6), Retirement (A.6.2.7)
- [x] **F3.FW.01.03** — Crear `dist/.devtrail/00-governance/AI-KPIS.md` como template con:
  - KPIs de gobernanza propuestos: Documentation Coverage (% de cambios documentados), Review Compliance Rate (% reviews completadas), Mean Time to Document, Risk Distribution, Agent Activity Distribution, Incident Response Time
  - Tabla: KPI, Target, Current Value, Measurement Method, Frequency, Owner
  - Notas de mapeo a ISO 42001 Clause 9 (Performance Evaluation)
- [x] **F3.FW.01.04** — Crear `dist/.devtrail/00-governance/MANAGEMENT-REVIEW-TEMPLATE.md` con:
  - Agenda de revisión alineada con ISO 42001 Clause 9.3: status of actions from previous reviews, changes in external/internal issues, audit results, objectives achievement, nonconformities, improvement opportunities
  - Decisiones requeridas: changes to AI policy, resource allocation, improvement actions
  - Tabla de acciones: Action, Owner, Deadline, Status

### F3.FW.02 — Crear guías de implementación NIST AI RMF `P1`

- [x] **F3.FW.02.01** — Crear `dist/.devtrail/03-implementation/NIST-AI-RMF-MAP-GUIDE.md` con:
  - Explicación de la función MAP y sus 5 categorías (MP-1 a MP-5)
  - Mapeo a documentos DevTrail: MP-1 (Context) → AILOG context section, MP-2 (Categorization) → ETH risk classification, MP-3 (Capabilities) → MCARD intended use, MP-4 (Risk Mapping) → AI-RISK-CATALOG, MP-5 (Impact) → DPIA/ETH
  - Checklist de implementación por subcategoría
- [x] **F3.FW.02.02** — Crear `dist/.devtrail/03-implementation/NIST-AI-RMF-MEASURE-GUIDE.md` con:
  - Explicación de MEASURE y sus 4 categorías (MS-1 a MS-4)
  - Mapeo: MS-1 (Metrics) → AI-KPIS, TES, MS-2 (Trustworthiness) → ETH bias section + MCARD performance, MS-3 (Risk Tracking) → AI-RISK-CATALOG review, MS-4 (Feedback) → INC + management review
- [x] **F3.FW.02.03** — Crear `dist/.devtrail/03-implementation/NIST-AI-RMF-MANAGE-GUIDE.md` con:
  - Explicación de MANAGE y sus 4 categorías (MG-1 a MG-4)
  - Mapeo: MG-1 (Response) → ETH recommendations + ADR, MG-2 (Mitigation) → SEC mitigations + DPIA measures, MG-3 (Third-party) → SBOM, MG-4 (Post-deployment) → INC + AI-LIFECYCLE-TRACKER
- [x] **F3.FW.02.04** — Crear `dist/.devtrail/03-implementation/NIST-AI-RMF-GOVERN-GUIDE.md` con:
  - Explicación de GOVERN y sus 6 categorías (GV-1 a GV-6)
  - Mapeo: GV-1 (Policies) → AI-GOVERNANCE-POLICY + DOCUMENTATION-POLICY, GV-2 (Accountability) → AGENT-RULES + AI-GOVERNANCE-POLICY §2, GV-3 (Workforce Diversity) → AI-GOVERNANCE-POLICY §4, GV-4 (Culture) → PRINCIPLES.md, GV-5 (Stakeholder Engagement) → MANAGEMENT-REVIEW-TEMPLATE, GV-6 (Supply Chain) → SBOM
- [x] **F3.FW.02.05** — Crear `dist/.devtrail/03-implementation/NIST-AI-600-1-GENAI-RISKS.md` con: las 12 categorías detalladas, cada una con: descripción, ejemplos, mapeo a templates DevTrail, subcategorías NIST específicas, mitigaciones recomendadas

### F3.FW.03 — Crear OBSERVABILITY-GUIDE.md (OpenTelemetry) `P2`

> Basado en la propuesta evaluada en `evolution/recomendaciones/propuesta-opentelemetry.md`. Documento opcional.

- [x] **F3.FW.03.01** — Crear `dist/.devtrail/00-governance/OBSERVABILITY-GUIDE.md` con:
  - Sección 1: "Scope and Purpose" — Explicar que esta guía es para proyectos que adoptan OpenTelemetry. No es obligatoria. Se activa cuando el equipo decide instrumentar su sistema
  - Sección 2: "Signals and Correlation" — Trazas (spans con nombres consistentes y atributos de negocio), Métricas (latencia, tasa de error, saturación, throughput), Logs (asociar `trace_id` y `span_id`). Tabla de correlación entre señales
  - Sección 3: "Minimum Resource Attributes" — Tabla con atributos recomendados: `service.name`, `service.version`, `deployment.environment`, `service.instance.id` (cuando aplique). Alineados con OTel Semantic Conventions
  - Sección 4: "Context Propagation" — W3C Trace Context (`traceparent`, `tracestate`). Propagación a través de HTTP, colas y procesos asíncronos. Notas sobre instrumentación automática vs manual
  - Sección 5: "Collector Pipeline Architecture" — Receivers (OTLP), Processors (batching, redacción de datos sensibles), Exporters (hacia backend). Separar pipelines de traces, metrics y logs. Diagrama Mermaid de la arquitectura recomendada
  - Sección 6: "Sampling and Retention" — Head sampling (decisión temprana) y tail sampling (para incidentes). Límites de retención por criticidad y costo. Documentar excepciones para flujos críticos
  - Sección 7: "Data Policies and Security" — Prohibir PII, tokens y secretos en atributos. Allowlist de atributos permitidos. Cifrado en tránsito y en reposo. Control de acceso por entorno y rol. Redacción automática en Collector
  - Sección 8: "DevTrail Integration" — Tabla de mapeo: cambios de instrumentación → AILOG, decisión de backend → AIDEC/ADR, requisitos de observabilidad → REQ, pruebas de propagación → TES, evidencia en incidentes → INC, deuda de instrumentación → TDE, privacidad de telemetría → ETH
  - Sección 9: "Adoption Roadmap" — Fase 0: AIDEC/ADR de adopción y decisión de backend. Fase 1: Instrumentación mínima en endpoints críticos. Fase 2: Cobertura completa y pruebas de correlación. Fase 3: Integración en INC, REQ y TES con automatización
  - Sección 10: "Checklist" — Lista de verificación para proyectos: AIDEC/ADR documentando adopción, REQ con requisitos de observabilidad, TES validando propagación, INC usando trazas como evidencia, ETH cubriendo privacidad, AILOG registrando cambios de instrumentación
- [x] **F3.FW.03.02** — Agregar mapeos a controles ISO 42001: A.6.2.6 (Operation and Monitoring — evidencia continua), A.5.2 (Risk Assessment — datos operativos como insumo), A.9 (Performance Evaluation — métricas OTel como KPIs operativos)
- [x] **F3.FW.03.03** — Agregar tabla de alineación con estándares regulatorios: EU AI Act Art. 72 (post-market monitoring), NIST AI RMF MEASURE (métricas operativas), GDPR (minimización de datos en telemetría), ISO 25010:2023 (fiabilidad, eficiencia)
- [x] **F3.FW.03.04** — Crear versión ES: `dist/.devtrail/00-governance/i18n/es/OBSERVABILITY-GUIDE.md`

### F3.FW.04 — Bump de versión Framework `P0`

> → depende de F3.FW.01, F3.FW.02 y F3.FW.03

- [x] **F3.FW.04.01** — Editar `dist/dist-manifest.yml`: cambiar version a `"3.2.0"`
- [x] **F3.FW.04.02** — Agregar todos los archivos nuevos de F3.FW.01, F3.FW.02 y F3.FW.03 a la sección files del manifest (incluir OBSERVABILITY-GUIDE.md)
- [x] **F3.FW.04.03** — Actualizar QUICK-REFERENCE.md si es necesario (incluir referencia a guía de observabilidad)

---

## F3.QA — Testing y Verificación de Fase 3

### F3.QA.01 — Tests para `devtrail compliance` `P0`

- [x] **F3.QA.01.01** — Crear `cli/tests/compliance_test.rs` con tests:
  - Test sin documentos → 0% compliance
  - Test con documentos completos → porcentaje correcto
  - Test con --standard eu-ai-act → solo checker EU AI Act
  - Test con --standard iso-42001 → solo checker ISO 42001
  - Test con --all → todos los checkers
  - Test con --output json → JSON válido
  - Test con --output markdown → Markdown válido
- [x] **F3.QA.01.02** — Crear `cli/tests/metrics_test.rs` con tests similares

### F3.QA.02 — Verificación de documentos de gobernanza `P1`

- [x] **F3.QA.02.01** — Verificar que AI-RISK-CATALOG.md tiene mapeo correcto de las 12 categorías NIST 600-1
- [x] **F3.QA.02.02** — Verificar que AI-LIFECYCLE-TRACKER.md cubre los 10 controles de Annex A.6
- [x] **F3.QA.02.03** — Verificar que las guías NIST mapean correctamente a documentos DevTrail
- [x] **F3.QA.02.04** — Ejecutar `devtrail compliance --all` en un proyecto de prueba con documentos de ejemplo y verificar que el reporte es coherente
- [x] **F3.QA.02.05** — Verificar que OBSERVABILITY-GUIDE.md incluye: mapeo completo a documentos DevTrail (8 tipos), baseline técnico con atributos mínimos, políticas de no-PII, checklist de adopción, y roadmap de fases
- [x] **F3.QA.02.06** — Verificar que la versión ES de OBSERVABILITY-GUIDE.md es completa y coherente con la versión EN

---

# FASE 4: Automatización Avanzada y Ecosistema

**Objetivo**: Implementar `devtrail audit`, guía C4 Model, integración OpenAPI/AsyncAPI, mejoras de ecosistema.

**Releases**: fw-4.0.0 / cli-2.1.0

**Prerequisito**: Fase 3 completada.

---

## F4.CLI — CLI: Comando `devtrail audit`

### F4.CLI.01 — Implementar `devtrail audit` `P0`

> → depende de F2.CLI.01 (document.rs)

- [x] **F4.CLI.01.01** — Crear `cli/src/commands/audit.rs` con función `pub fn run(path: &str, from: Option<&str>, to: Option<&str>, system: Option<&str>, output: &str) -> Result<()>`
- [x] **F4.CLI.01.02** — Agregar variante `Audit` al enum `Commands` con args: `path`, `--from` (YYYY-MM-DD), `--to` (YYYY-MM-DD), `--system` (nombre de sistema), `--output` (text | markdown | html, default text)
- [x] **F4.CLI.01.03** — Implementar timeline cronológica: ordenar todos los documentos del período por campo `created`, mostrar en formato timeline con tipo, título, agente, risk_level
- [x] **F4.CLI.01.04** — Implementar filtrado por sistema/componente: filtrar documentos cuyo campo `tags` o `title` contenga el nombre del sistema
- [x] **F4.CLI.01.05** — Implementar mapa de trazabilidad: construir grafo de relaciones usando campo `related:` de cada documento. Mostrar cadenas como: REQ → ADR → AILOG → TES → INC
- [x] **F4.CLI.01.06** — Implementar output Markdown con secciones: Executive Summary (período, totales), Timeline, Traceability Map, Risk Distribution, Compliance Summary (reutilizar compliance checkers)
- [x] **F4.CLI.01.07** — Implementar output HTML básico: header con logo/título, tablas estilizadas, timeline visual, pie chart de risk distribution (SVG inline simple). Para uso en presentaciones y auditorías externas
- [x] **F4.CLI.01.08** — Agregar al routing en main.rs

### F4.CLI.02 — Bump de versión CLI `P0`

- [x] **F4.CLI.02.01** — Editar `cli/Cargo.toml`: cambiar version a `"2.1.0"`
- [x] **F4.CLI.02.02** — Ejecutar `cargo check` y `cargo build --release`

---

## F4.FW — Framework: C4 Model, OpenAPI, Ecosistema

### F4.FW.01 — Crear guía de C4 Model con Mermaid `P1`

- [x] **F4.FW.01.01** — Crear `dist/.devtrail/00-governance/C4-DIAGRAM-GUIDE.md` con:
  - Explicación de los 4 niveles C4: Context (quién usa el sistema), Container (aplicaciones y stores), Component (componentes dentro de un container), Code (clases/funciones)
  - Sintaxis Mermaid para cada nivel usando `C4Context`, `C4Container`, `C4Component` (nota: soporte experimental en Mermaid)
  - Ejemplo completo para cada nivel con código Mermaid renderizable
  - Cuándo usar cada nivel: Context en REQ/ADR de alto nivel, Container en ADR de arquitectura, Component en ADR/AILOG de módulo, Code solo si es necesario
  - Alternativa PlantUML: sintaxis básica para cada nivel para equipos que prefieran PlantUML
- [x] **F4.FW.01.02** — Actualizar AGENT-RULES.md: agregar regla "When creating ADR documents that involve architectural changes, include a Mermaid C4 diagram at the appropriate level. Use C4Context for system-level decisions, C4Container for service-level decisions"
- [x] **F4.FW.01.03** — Actualizar TEMPLATE-ADR.md: agregar sección opcional `## Architecture Diagram` con placeholder de Mermaid C4 y nota "Include a C4 diagram at the appropriate level. See C4-DIAGRAM-GUIDE.md for syntax reference"

### F4.FW.02 — Integración OpenAPI/AsyncAPI `P1`

- [x] **F4.FW.02.01** — Actualizar TEMPLATE-REQ.md: en sección "External Interfaces > Software Interfaces", agregar campo `api_spec_path: ""` al frontmatter para referenciar la ubicación del archivo OpenAPI/AsyncAPI spec
- [x] **F4.FW.02.02** — Actualizar TEMPLATE-ADR.md: agregar campo `api_changes: []` al frontmatter para documentar endpoints afectados por la decisión
- [x] **F4.FW.02.03** — Agregar nota en AGENT-RULES.md: "When a change modifies API endpoints, verify that the corresponding OpenAPI/AsyncAPI specification is updated. Reference the spec path in the AILOG or ADR using `api_spec_path` or `api_changes` fields"

### F4.FW.03 — Mejoras de ecosistema `P2`

- [x] **F4.FW.03.01** — Actualizar regex de tipos en todas las validaciones (scripts, CI, CLI) para usar una única fuente de verdad. Crear constante o archivo de configuración con los 12 tipos oficiales
- [x] **F4.FW.03.02** — Agregar detección de terminal en skills: si el terminal no soporta box-drawing, usar formato texto plano alternativo. Implementar en los SKILL.md como nota condicional
- [x] **F4.FW.03.03** — Documentar relación entre QUICK-REFERENCE.md y DOCUMENTATION-POLICY.md: agregar nota al inicio de QUICK-REFERENCE indicando que es un derivado de DOCUMENTATION-POLICY y que esta última es la fuente autoritativa

### F4.FW.04 — Bump de versión Framework `P0`

- [x] **F4.FW.04.01** — Editar `dist/dist-manifest.yml`: cambiar version a `"4.0.0"`. Justificación: C4 Model, OpenAPI integration, ecosistema consolidado, 4 nuevos comandos CLI
- [x] **F4.FW.04.02** — Agregar todos los archivos nuevos de F4 al manifest
- [x] **F4.FW.04.03** — Actualizar QUICK-REFERENCE.md con referencia a C4-DIAGRAM-GUIDE.md

---

## F4.QA — Testing y Verificación de Fase 4

### F4.QA.01 — Tests para `devtrail audit` `P0`

- [x] **F4.QA.01.01** — Crear `cli/tests/audit_test.rs` con tests:
  - Test con rango de fechas → solo documentos en rango
  - Test con --system → solo documentos del sistema
  - Test de trazabilidad → cadena REQ → ADR → AILOG correcta
  - Test con --output markdown → Markdown válido
  - Test con --output html → HTML válido con estructura esperada
- [x] **F4.QA.01.02** — Ejecutar suite completa: `cargo test`
- [x] **F4.QA.01.03** — Ejecutar `cargo clippy` sin warnings

### F4.QA.02 — Verificación integral del ecosistema `P1`

- [x] **F4.QA.02.01** — Test end-to-end: `devtrail init` → crear documentos de cada tipo → `devtrail validate` → `devtrail compliance --all` → `devtrail metrics` → `devtrail audit --output markdown`
- [x] **F4.QA.02.02** — Verificar que `devtrail status` muestra los 12 tipos y 13 comandos
- [x] **F4.QA.02.03** — Verificar que `devtrail explore` navega correctamente todas las carpetas incluyendo 08-security y 09-ai-models
- [x] **F4.QA.02.04** — Verificar que el proceso `devtrail update-framework` de fw-2.2.0 a fw-4.0.0 funciona sin romper documentos existentes del usuario

---

## F4.DOC — Documentación Final

### F4.DOC.01 — Actualización completa de docs/ `P1`

- [x] **F4.DOC.01.01** — `CLI-REFERENCE.md`: documentar los 4 nuevos comandos (validate, compliance, metrics, audit) con ejemplos de uso y output esperado
- [x] **F4.DOC.01.02** — `ADOPTION-GUIDE.md`: actualizar con la nueva estructura de 12 tipos, gobernanza ISO 42001, y nuevos comandos
- [x] **F4.DOC.01.03** — `DOCUMENTATION-STANDARDS-REVIEW.md`: actualizar con las decisiones tomadas y los estándares adoptados
- [x] **F4.DOC.01.04** — README.md: actualización final con todos los cambios, 12 tipos, 13 comandos, estándares actualizados

### F4.DOC.02 — CHANGELOG `P0`

- [x] **F4.DOC.02.01** — Crear o actualizar CHANGELOG.md con entradas para fw-3.0.0, fw-3.1.0, fw-3.2.0, fw-4.0.0 y cli-1.3.0, cli-1.4.0, cli-2.0.0, cli-2.1.0

---

# Apéndice A: Mapeo de Controles ISO 42001 Annex A → Documentos DevTrail

Esta tabla sirve como referencia maestra para las implementaciones de Fase 1-3.

| Control ISO 42001 | ID | Documento(s) DevTrail |
| --- | --- | --- |
| AI Policy | A.2.2 | AI-GOVERNANCE-POLICY.md §2 |
| Responsible AI Topics | A.2.3 | AI-GOVERNANCE-POLICY.md §2, PRINCIPLES.md |
| Roles and Responsibilities | A.3.2 | AI-GOVERNANCE-POLICY.md §2, AGENT-RULES.md |
| Reporting of AI Concerns | A.3.3 | INC, ETH |
| Impact of Organizational Changes | A.3.4 | ADR |
| Resources | A.4.2 | AI-GOVERNANCE-POLICY.md §4 |
| Competencies | A.4.3 | AI-GOVERNANCE-POLICY.md §4 |
| Awareness of Responsible Use | A.4.4 | PRINCIPLES.md, directivas de agentes |
| Consultation | A.4.5 | MANAGEMENT-REVIEW-TEMPLATE.md |
| Communication About AI System | A.4.6 | ADR, REQ |
| Risk Assessment | A.5.2 | ETH, AI-RISK-CATALOG.md |
| Impact Assessment | A.5.3 | ETH, DPIA |
| Impact Documentation | A.5.4 | ETH, DPIA |
| Design and Development | A.6.2.2 | ADR, AIDEC |
| Training and Testing AI Model | A.6.2.3 | MCARD (training data), TES |
| Verification and Validation | A.6.2.4 | TES |
| Deployment | A.6.2.5 | AILOG, AI-LIFECYCLE-TRACKER.md |
| Operation and Monitoring | A.6.2.6 | AILOG, AI-LIFECYCLE-TRACKER.md, OBSERVABILITY-GUIDE.md (Fase 3) |
| Retirement | A.6.2.7 | AI-LIFECYCLE-TRACKER.md, ADR |
| Responsible Integration | A.6.2.8 | ADR, AIDEC |
| Documentation | A.6.2.9 | AILOG (todo cambio documentado) |
| Defined Use and Misuse | A.6.2.10 | MCARD (intended use, out-of-scope) |
| Management of Third-Party Components | A.6.2.11 | SBOM |
| Data for Development/Enhancement | A.7.2 | MCARD (training data) |
| Data Quality for ML | A.7.3 | MCARD (training data quality) |
| Data Preparation | A.7.4 | MCARD (preprocessing) |
| Data Acquisition/Collection | A.7.5 | SBOM (data sources), DPIA |
| Data Provenance | A.7.6 | SBOM (data provenance) |
| Informing About AI Interaction | A.8.2 | ETH (transparency section) |
| Informing About AI Outcomes | A.8.3 | ETH (explainability) |
| Access to Information | A.8.4 | REQ, ADR |
| Enabling Appropriate Human Actions | A.8.5 | AGENT-RULES.md (human review triggers) |
| Objectives for Responsible Use | A.9.2 | AI-GOVERNANCE-POLICY.md §5, PRINCIPLES.md |
| Intended Use | A.9.3 | MCARD, REQ |
| Processes for Responsible Use | A.9.4 | DOCUMENTATION-POLICY.md, AGENT-RULES.md |
| Human Oversight Aspects | A.9.5 | AGENT-RULES.md (autonomy limits table) |
| Suppliers of AI Components | A.10.2 | SBOM (third-party services) |
| Shared ML Models | A.10.3 | SBOM (AI components) |
| Provision of AI System to Third Parties | A.10.4 | ETH, MCARD |

---

# Apéndice B: Mapeo EU AI Act Annex IV → Documentos DevTrail

| Sección Annex IV | Contenido Requerido | Documento(s) DevTrail |
| --- | --- | --- |
| §1 General Description | Intended purpose, provider, version, hardware, distribution | MCARD, REQ |
| §2 Development Process | Methodology, algorithm logic, design rationale, training data | MCARD, ADR, AIDEC |
| §3 Monitoring & Control | Performance, limitations, risk sources, human oversight | MCARD (limitations), ETH (risks), AGENT-RULES |
| §4 Performance Metrics | Appropriateness demonstration | MCARD (performance metrics), TES |
| §5 Risk Management System | Per Article 9 | AI-RISK-CATALOG, ETH, AI-GOVERNANCE-POLICY |
| §6 Lifecycle Changes | Record of modifications | AILOG (all changes), AI-LIFECYCLE-TRACKER |
| §7 Harmonized Standards | Applied standards list | AI-GOVERNANCE-POLICY (standards table) |
| §8 EU Declaration of Conformity | Per Article 47 | (fuera de alcance DevTrail — documento legal) |
| §9 Post-Market Monitoring | Per Article 72 | AI-LIFECYCLE-TRACKER (monitoring phase), INC, OBSERVABILITY-GUIDE.md (Fase 3) |

---

# Apéndice C: Las 12 Categorías de Riesgo NIST AI 600-1 y Documentos DevTrail

| # | Categoría | Descripción | Template DevTrail | Campo frontmatter |
| --- | --- | --- | --- | --- |
| 1 | CBRN Information | Acceso a info de armas químicas/biológicas/radiológicas/nucleares | ETH (Security) | `nist_genai_risks: [cbrn]` |
| 2 | Confabulation | Outputs falsos presentados con confianza (alucinaciones) | ETH (Transparency), MCARD (Limitations) | `nist_genai_risks: [confabulation]` |
| 3 | Dangerous/Violent/Hateful Content | Generación de contenido peligroso o de odio | ETH (Social Impact) | `nist_genai_risks: [dangerous_content]` |
| 4 | Data Privacy | Memorización de datos de entrenamiento, ataques de inferencia | ETH (Data Privacy), DPIA | `nist_genai_risks: [privacy]` |
| 5 | Environmental Impacts | Huella de carbono computacional | ETH (Environmental Impact), MCARD (Environmental) | `nist_genai_risks: [environmental]` |
| 6 | Harmful Bias / Homogenization | Amplificación de sesgos, reducción de diversidad | ETH (Bias and Fairness), MCARD (Bias Evaluation) | `nist_genai_risks: [bias]` |
| 7 | Human-AI Configuration | Antropomorfismo, automation bias, confianza inapropiada | ETH (Dual-Use), MCARD (Ethical Considerations) | `nist_genai_risks: [human_ai_config]` |
| 8 | Information Integrity | Desinformación a escala, deepfakes | ETH (Transparency, Social Impact) | `nist_genai_risks: [information_integrity]` |
| 9 | Information Security | Prompt injection, robo de modelo, phishing automatizado | SEC, ETH (Security) | `nist_genai_risks: [information_security]` |
| 10 | Intellectual Property | Reproducción no autorizada de contenido con copyright | ETH (Legal), MCARD (License) | `nist_genai_risks: [intellectual_property]` |
| 11 | Obscene/Abusive Content | Generación de material abusivo o degradante | ETH (Social Impact) | `nist_genai_risks: [obscene_content]` |
| 12 | Value Chain / Component Integration | Riesgos de componentes de terceros opacos | SBOM, ETH (Third-Party) | `nist_genai_risks: [value_chain]` |

---

# Apéndice D: Reglas de Validación del CLI

Referencia completa de las reglas implementadas en `devtrail validate` (Fase 2).

| ID Regla | Severidad | Descripción | Auto-fix |
| --- | --- | --- | --- |
| NAMING-001 | Error | Nombre de archivo no sigue patrón TYPE-YYYY-MM-DD-NNN-description.md | No |
| META-001 | Error | Campos obligatorios de frontmatter faltantes (id, title, status, created, agent, confidence, review_required, risk_level) | No |
| META-002 | Error | Campo `id` no coincide con prefijo del nombre de archivo | Sí (`--fix`) |
| META-003 | Error | Campo `status` tiene valor no válido | No |
| CROSS-001 | Error | `risk_level: high\ | critical` pero `review_required` no es `true` | Sí (`--fix`) |
| CROSS-002 | Error | `eu_ai_act_risk: high` pero `review_required` no es `true` | Sí (`--fix`) |
| CROSS-003 | Error | Tipo SEC/MCARD/DPIA sin `review_required: true` | Sí (`--fix`) |
| TYPE-001 | Error | INC sin campo `severity` | No |
| TYPE-002 | Warning | ETH con sección "Data Privacy" sin campo `gdpr_legal_basis` | No |
| REF-001 | Warning | Documentos en `related:` no encontrados en `.devtrail/` | No |
| SEC-001 | Error | Información sensible detectada en el documento | No |
| OBS-001 | Warning | Documento con tag `observabilidad` sin sección de alcance o riesgos de instrumentación | No |

---

*Plan de implementación elaborado por Claude Opus 4.6.*
*Creado el 23 de marzo de 2026. Actualizado el 25 de marzo de 2026 — revisión de congruencia y correcciones post-integración OpenTelemetry.*
