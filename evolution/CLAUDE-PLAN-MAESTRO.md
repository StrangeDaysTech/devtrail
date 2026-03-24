# DevTrail — Plan Maestro de Evolución 2026

> **Fecha**: 23 de marzo de 2026
> **Última actualización**: 25 de marzo de 2026 — Revisión de congruencia post-integración OpenTelemetry
> **Autor**: Claude Opus 4.6 — síntesis de análisis de Gemini CLI, GPT-Codex y Qwen Code
> **Versión del Framework**: fw-2.2.0
> **Versión del CLI**: cli-1.2.0
> **Estado**: Borrador para aprobación

---

## 1. Resumen Ejecutivo

Este plan maestro consolida y armoniza las recomendaciones de tres agentes independientes (Gemini CLI, GPT-Codex y Qwen Code) que analizaron exhaustivamente el framework DevTrail, su CLI y su alineación con estándares internacionales vigentes en 2026.

DevTrail es un framework sólido, bien estructurado, con 8 tipos documentales, soporte multi-agente, validación automatizada y un CLI funcional con 9 comandos. Sin embargo, el contexto regulatorio y de estándares de 2026 exige una evolución significativa para mantener su relevancia y valor como herramienta de gobernanza de IA.

### Diagnóstico Convergente

Los tres agentes coinciden en estos puntos críticos:

| Hallazgo | Gemini | GPT-Codex | Qwen | Prioridad |
| --- | --- | --- | --- | --- |
| IEEE 830 obsoleto → ISO/IEC/IEEE 29148:2018 | ✓ | ✓ | ✓ | **Crítica** |
| EU AI Act sin operacionalizar (agosto 2026) | ✓ | ✓ | ✓ | **Crítica** |
| NIST AI RMF sin mapeo funcional | ✓ | ✓ | ✓ | **Alta** |
| ISO/IEC 25010 → actualizar a versión 2023 | ✓ | ✓ | ✓ | **Alta** |
| ISO/IEC 42001 no integrado | — | ✓ | ✓ | **Alta** |
| Directivas de agentes demasiado minimalistas | — | ✓ | ✓ | **Alta** |
| Validación CI insuficiente (reglas cruzadas) | — | ✓ | ✓ | **Alta** |
| ISO/IEC/IEEE 29119-3 para pruebas | ✓ | ✓ | — | **Media** |
| SBOM (SPDX/CycloneDX) | ✓ | ✓ | ✓ | **Media** |
| C4 Model / documentación visual | ✓ | ✓ | — | **Media** |
| Nuevos tipos documentales (Model Card, DPIA, SEC) | — | ✓ | ✓ | **Media** |
| Métricas de gobernanza | — | — | ✓ | **Media** |

### Decisión Estratégica: Alineación con ISO/IEC 42001

Tras evaluar las recomendaciones, **este plan propone que ISO/IEC 42001:2023 (AI Management System) sea el estándar vertebral** del framework, ya que:

1. Es el único estándar certificable internacionalmente para gestión de IA
2. Subsume naturalmente requisitos del EU AI Act, NIST AI RMF y GDPR
3. Proporciona una estructura organizativa que da coherencia a todos los demás estándares
4. Es un diferenciador competitivo para adoptantes del framework

Esto no significa implementar ISO 42001 al pie de la letra (DevTrail es un framework documental, no un sistema de gestión completo), sino **alinear la estructura, vocabulario y trazabilidad** de DevTrail con las cláusulas del estándar para que los equipos que adopten DevTrail tengan evidencia documentada compatible con una certificación ISO 42001.

---

## 2. Estado Actual — Lo que ya tenemos bien

Antes de definir cambios, es importante reconocer las fortalezas que los tres agentes coinciden en destacar:

### Framework (dist/)
- **8 tipos documentales** bien diseñados con templates bilingües (EN/ES)
- **Gobernanza clara**: PRINCIPLES.md, DOCUMENTATION-POLICY.md, AGENT-RULES.md, GIT-BRANCHING-STRATEGY.md
- **Soporte multi-agente** excelente: Claude, Gemini, Cursor, Copilot
- **Skills/workflows** para creación activa de documentación
- **Validación automatizada**: pre-commit hook, PowerShell validator, GitHub Actions
- **Metadatos obligatorios** con frontmatter YAML bien definido
- **Convención de nombrado** clara: `[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md`
- **Internacionalización** con traducciones en español

### CLI (cli/)
- **9 comandos** funcionales: init, update, update-framework, update-cli, remove, status, repair, explore, about
- **Auto-actualización** del binario desde GitHub Releases
- **Actualización inteligente** del framework con detección de cambios (SHA-256)
- **TUI interactivo** para explorar documentación (ratatui + crossterm)
- **Multiplataforma**: Linux, macOS (Intel/ARM), Windows
- **Sistema de inyección** de directivas robusto con marcadores
- **CI/CD** con builds automatizados para 4 targets

### Conclusión
DevTrail no necesita una reescritura, sino una **evolución estratégica** enfocada en tres ejes: actualización de estándares, expansión de capacidades de compliance, y automatización de verificaciones.

---

## 3. Arquitectura de la Evolución

### 3.1 Nuevo Mapa de Estándares

```
┌─────────────────────────────────────────────────────────────────┐
│                    ISO/IEC 42001:2023                           │
│                 (AI Management System)                          │
│              ┌─────────┬──────────┬──────────┐                 │
│              │         │          │          │                  │
│    ┌─────────▼──┐ ┌────▼─────┐ ┌─▼────────┐ │                 │
│    │ EU AI Act  │ │ NIST AI  │ │ ISO/IEC  │ │                 │
│    │ (Annex III │ │ RMF 1.0  │ │ 23894    │ │                 │
│    │  + IV)     │ │ + 600-1  │ │ (AI Risk)│ │                 │
│    └─────┬──────┘ └────┬─────┘ └──────────┘ │                 │
│          │             │                     │                 │
│    ┌─────▼─────────────▼─────────────────────▼───────────┐     │
│    │              DevTrail Framework                      │     │
│    │  ┌──────────────────────────────────────────────┐   │     │
│    │  │ Estándares Técnicos Base                     │   │     │
│    │  │  • ISO/IEC/IEEE 29148:2018 (Requerimientos)  │   │     │
│    │  │  • ISO/IEC 25010:2023 (Calidad)              │   │     │
│    │  │  • ISO/IEC/IEEE 29119-3:2021 (Pruebas)       │   │     │
│    │  │  • ADR — Michael Nygard (Decisiones)         │   │     │
│    │  │  • GDPR (Privacidad)                         │   │     │
│    │  └──────────────────────────────────────────────┘   │     │
│    │  ┌──────────────────────────────────────────────┐   │     │
│    │  │ Estándares Complementarios                   │   │     │
│    │  │  • C4 Model (Visualización)                  │   │     │
│    │  │  • SPDX/CycloneDX (SBOM)                    │   │     │
│    │  │  • OpenTelemetry (Observabilidad operativa)  │   │     │
│    │  │  • IEEE P2863 (Gobernanza ética — monitor)   │   │     │
│    │  └──────────────────────────────────────────────┘   │     │
│    └─────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 Nuevo Mapa Documental

Los 8 tipos actuales se mantienen. Se proponen **4 nuevos tipos documentales** y **1 documento de gobernanza**:

| Tipo | Nombre | Propósito | Autonomía IA |
| --- | --- | --- | --- |
| **AILOG** | AI Action Log | Registro de acciones de IA | Crear libremente |
| **AIDEC** | AI Decision | Decisiones técnicas de IA | Crear libremente |
| **ADR** | Architecture Decision Record | Decisiones arquitectónicas | Draft → aprobación |
| **ETH** | Ethical Review | Revisión ética/privacidad | Draft → aprobación |
| **REQ** | Requirement | Requerimientos del sistema | Proponer → validación |
| **TES** | Test Plan | Planes de prueba | Proponer → validación |
| **INC** | Incident Post-mortem | Post-mortem de incidentes | Identificar → humano |
| **TDE** | Technical Debt | Deuda técnica | Identificar → humano |
| **SEC** *(nuevo)* | Security Assessment | Threat modeling y controles | Draft → aprobación |
| **MCARD** *(nuevo)* | Model/System Card | Documentación de modelos IA | Draft → aprobación |
| **SBOM** *(nuevo)* | Software Bill of Materials | Inventario de dependencias IA | Crear libremente |
| **DPIA** *(nuevo)* | Data Protection Impact Assessment | Evaluación de impacto en privacidad | Draft → aprobación |

**Documento de gobernanza nuevo**:
- `AI-GOVERNANCE-POLICY.md` en `00-governance/` — Política de gestión de IA alineada con ISO 42001 (Cláusulas 4-10). Establece contexto organizacional, roles, planificación de riesgos y mejora continua.

### 3.3 Nuevos Campos en Frontmatter

Se proponen campos adicionales opcionales para alineación regulatoria:

```yaml
# Campos existentes (se mantienen)
id: AILOG-2026-03-23-001
title: "Descriptive title"
status: accepted
created: 2026-03-23
agent: claude-code-v2.0
confidence: high
review_required: false
risk_level: medium
tags: [authentication, security]
related: []

# Campos nuevos (opcionales, activados por contexto)
eu_ai_act_risk: minimal | limited | high | unacceptable | not_applicable
nist_genai_risks: [privacy, bias, security, confabulation, ...]
iso_42001_clause: [6, 8]  # Cláusulas ISO 42001 relevantes
lines_changed: 45          # Auto-calculable por CLI/scripts
files_modified:            # Auto-calculable
  - src/auth/login.rs
observability_scope: none  # none | basic | full — nivel de instrumentación OTel (opcional)
```

### 3.4 Observabilidad con OpenTelemetry (opcional)

Tras evaluar la [propuesta-opentelemetry.md](./recomendaciones/propuesta-opentelemetry.md), se integra como **estándar complementario recomendado** para proyectos con arquitectura distribuida o requisitos de alta observabilidad.

**Principios de integración:**
- **Opcional**: no se impone; se activa cuando el proyecto lo necesita
- **Documental**: DevTrail no instrumenta código, pero guía cómo documentar decisiones de observabilidad
- **Compatible con gobernanza**: la telemetría genera evidencia operativa para ISO 42001 (Cláusula 8, A.6.2.6), EU AI Act (Art. 72 post-market monitoring) y NIST AI RMF (función MEASURE)

**Cambios propuestos:**
1. **Tag estándar** `observabilidad` para documentos relacionados con instrumentación OTel
2. **Secciones opcionales** en REQ (requisitos de observabilidad), TES (pruebas de propagación), INC (evidencia con trace-id)
3. **Reglas en AGENT-RULES**: no capturar PII en atributos/logs, registrar cambios de pipeline en AILOG
4. **AIDEC/ADR obligatorio** cuando se adopte OTel en proyectos distribuidos
5. **Guía de observabilidad** (`OBSERVABILITY-GUIDE.md`) en `00-governance/` con baseline técnico (señales, atributos, Collector, muestreo, políticas de datos)
6. **Validación CI opcional**: documentos con tag `observabilidad` deben incluir sección de alcance y riesgos

**Alineación con estándares del plan:**

| Estándar | Aporte de OTel |
| --- | --- |
| ISO 42001 Cláusula 8 + A.6.2.6 | Evidencia operativa de monitoreo continuo |
| EU AI Act Art. 72 | Datos para post-market monitoring de sistemas de alto riesgo |
| NIST AI RMF MEASURE | Métricas y trazas como insumo para evaluación |
| GDPR | Política de no-PII en telemetría, redacción y retención |
| ISO 25010:2023 | Soporta fiabilidad, eficiencia y mantenibilidad |

### 3.5 Evolución del CLI

Nuevos comandos propuestos para el CLI:

| Comando | Descripción | Fase |
| --- | --- | --- |
| `devtrail compliance [--standard X]` | Verificar cumplimiento con estándares específicos | Fase 3 |
| `devtrail metrics [--period X]` | Métricas de gobernanza documental | Fase 3 |
| `devtrail audit [--from --to]` | Generar trazas de auditoría exportables | Fase 4 |
| `devtrail validate [path]` | Validación local completa (reemplaza scripts) | Fase 2 |

---

## 4. Plan de Implementación por Fases

### Fase 1: Correcciones Críticas y Base Regulatoria
**Objetivo**: Actualizar estándares obsoletos y establecer la base para compliance regulatorio.
**Versión objetivo**: fw-3.0.0 / cli-1.3.0

#### 1.1 Actualizar referencias de estándares obsoletos
- [ ] **Reemplazar IEEE 830 por ISO/IEC/IEEE 29148:2018** en README.md, ADOPTION-GUIDE.md, y documentación general
- [ ] **Actualizar ISO/IEC 25010 a versión 2023**: adoptar terminología "Safety" (seguridad física) e "Interaction Capability" (capacidad de interacción, antes Usabilidad) en templates ADR y REQ
- [ ] **Alinear TES con ISO/IEC/IEEE 29119-3:2021**: ajustar vocabulario del template (Política → Estrategia → Plan), definir "Modelos de Prueba" y "Resultados Esperados vs. Logs de Ejecución"
- [ ] Actualizar TEMPLATE-REQ.md: agregar sección de Interfaces Externas (Hardware, Software, Comunicación) según 29148
- [ ] Actualizar la tabla de estándares en README.md y ADOPTION-GUIDE.md con las nuevas referencias

#### 1.2 Enriquecer templates existentes con campos regulatorios
- [ ] Agregar campos opcionales `eu_ai_act_risk` y `nist_genai_risks` a TEMPLATE-AILOG.md y TEMPLATE-ETH.md
- [ ] Agregar campo opcional `iso_42001_clause` a todos los templates
- [ ] TEMPLATE-ETH.md: agregar secciones de "Clasificación de Riesgo EU AI Act (Annex III)", "Base Legal GDPR", "Impacto Ambiental", y "Potencial de Doble Uso"
- [ ] TEMPLATE-ADR.md: enfatizar **inmutabilidad** de ADRs aceptados (cambios generan nuevo ADR que "supersedes")
- [ ] TEMPLATE-ADR.md: agregar sección "Criterios de Validación" con métricas objetivas
- [ ] TEMPLATE-AILOG.md: agregar secciones opcionales de "EU AI Act Considerations" y "NIST GenAI Risk Categories"
- [ ] TEMPLATE-INC.md: agregar referencia a timeline regulatorio (15 días para reporte EU AI Act)
- [ ] TEMPLATE-REQ.md: agregar sección opcional `## Observability Requirements` con campos de cobertura, calidad de trazas, latencia aceptable y retención (activada por tag `observabilidad`)
- [ ] TEMPLATE-TES.md: agregar sección opcional `## Observability Tests` con pruebas de propagación de trace context, correlación log-trace, muestreo bajo carga y redacción de datos sensibles
- [ ] TEMPLATE-INC.md: agregar campos opcionales `trace_id` y `span_id` en la sección de timeline para evidencia basada en trazas OTel
- [ ] Actualizar traducciones ES de todos los templates modificados

#### 1.3 Fortalecer directivas de agentes
- [ ] Expandir CLAUDE.md, GEMINI.md, copilot-instructions.md con:
  - Checklist pre-commit (¿cambié >20 líneas de lógica de negocio? → AILOG, ¿toqué auth/PII? → AILOG + ETH draft, etc.)
  - Snippets de frontmatter pre-llenado con campos regulatorios
  - Referencia a la lista de categorías de riesgo NIST AI 600-1
  - Regla de observabilidad: no capturar PII en atributos ni logs OTel; registrar cambios de pipeline de instrumentación en AILOG
- [ ] Verificar que `.cursorrules` existe en `dist/dist-templates/directives/` (reportado como faltante)
- [ ] Incluir bloque mínimo de reglas críticas en cada directiva (identidad, review_required, prohibición de secretos) para que funcionen independientemente de DEVTRAIL.md

#### 1.4 Refinar criterios de documentación obligatoria
- [ ] Reemplazar el umbral de "10 líneas de código" por criterios basados en:
  - Complejidad/impacto: cambios en auth/authorization/PII → siempre documentar
  - Cambios en API pública o schema de BD → siempre documentar
  - Lógica de negocio: >20 líneas (umbral más razonable)
  - Mantener el umbral anterior como referencia simplificada en QUICK-REFERENCE.md
- [ ] Actualizar AGENT-RULES.md con los nuevos criterios
- [ ] Agregar triggers de review humana faltantes: cambios en modelos de ML, cambios en prompts de IA, dependencias de seguridad

#### 1.5 Documentación y versiones
- [ ] Crear `AI-GOVERNANCE-POLICY.md` en `00-governance/` con estructura alineada a ISO 42001 (contexto, liderazgo, planificación, soporte, operaciones, evaluación, mejora)
- [ ] Actualizar QUICK-REFERENCE.md con los nuevos campos y criterios
- [ ] Actualizar DOCUMENTATION-POLICY.md con los nuevos tipos documentales (SEC, MCARD, SBOM, DPIA)
- [ ] Actualizar dist-manifest.yml para incluir nuevos archivos

---

### Fase 2: Nuevos Tipos Documentales y Validación
**Objetivo**: Expandir la cobertura documental y automatizar verificaciones críticas.
**Versión objetivo**: fw-3.1.0 / cli-1.4.0

#### 2.1 Crear nuevos templates
- [ ] **TEMPLATE-SEC.md** (Security Assessment): threat modeling basado en OWASP ASVS/SAMM, controles de seguridad, evaluación de vulnerabilidades. Requiere review humana.
- [ ] **TEMPLATE-MCARD.md** (Model/System Card): basado en "Model Cards" de Mitchell et al. y "Datasheets for Datasets" de Gebru et al. Incluye: detalles del modelo, uso previsto, métricas de rendimiento, sesgo y equidad, impacto ambiental, consideraciones de seguridad.
- [ ] **TEMPLATE-SBOM.md** (Software Bill of Materials para IA): inventario de componentes de IA, fuentes de datos de entrenamiento, servicios de IA de terceros, con formato compatible SPDX/CycloneDX.
- [ ] **TEMPLATE-DPIA.md** (Data Protection Impact Assessment): evaluación de impacto en privacidad cuando se maneje PII, alineada con Art. 35 GDPR y requisitos EU AI Act.
- [ ] Crear traducciones ES para los 4 nuevos templates
- [ ] Agregar carpetas correspondientes en la estructura de `.devtrail/`
- [ ] Actualizar AGENT-RULES.md con reglas de autonomía para los nuevos tipos

#### 2.2 Fortalecer validación automatizada
- [ ] **pre-commit-docs.sh**: agregar validación de reglas cruzadas:
  - `risk_level: high|critical` → `review_required: true` (obligatorio)
  - Validar que `id` del frontmatter coincida con el nombre de archivo
  - Validar campos obligatorios específicos por tipo (ej: `severity` para INC)
  - Validar que documentos referenciados en `related:` existan
  - Detectar cambios de código sin AILOG del mismo día (advertencia)
  - Si el documento tiene tag `observabilidad`, verificar que incluya sección de alcance y riesgos de instrumentación
- [ ] **validate-docs.ps1**: replicar las mismas validaciones en PowerShell
- [ ] **docs-validation.yml**: agregar job de EU AI Act compliance check:
  - Verificar que documentos de alto riesgo tienen ETH asociado
  - Generar reporte de métricas de gobernanza en `$GITHUB_STEP_SUMMARY`
- [ ] Alinear regex de tipos en CI con la lista oficial actualizada (agregar SEC, MCARD, SBOM, DPIA)

#### 2.3 Comando `devtrail validate`
- [ ] Implementar comando `devtrail validate [path]` en el CLI que consolide toda la lógica de validación:
  - Reemplaza la necesidad de ejecutar scripts manualmente
  - Valida naming, frontmatter, reglas cruzadas, referencias rotas
  - Detecta información sensible
  - Output con colores y resumen de errores/advertencias
  - Flags: `--fix` para correcciones automáticas simples (ej: `review_required` faltante en docs de alto riesgo)

#### 2.4 Skills para nuevos tipos
- [ ] Crear skill `devtrail-sec` para creación guiada de Security Assessments
- [ ] Crear skill `devtrail-mcard` para creación guiada de Model Cards
- [ ] Actualizar skill `devtrail-new` para incluir los 4 nuevos tipos en su menú interactivo
- [ ] Crear workflows equivalentes en `.agent/workflows/` y `.gemini/skills/`

---

### Fase 3: Compliance Automatizado y Métricas
**Objetivo**: Proveer herramientas de compliance y métricas que demuestren valor tangible.
**Versión objetivo**: fw-3.2.0 / cli-2.0.0 (major bump por nuevos comandos significativos)

#### 3.1 Comando `devtrail compliance`
- [ ] Implementar `devtrail compliance [--standard X] [--all]` con soporte para:
  - `--standard eu-ai-act`: verificar clasificación de riesgo, documentación técnica (Annex IV), evaluación de conformidad, monitoreo post-mercado
  - `--standard iso-42001`: verificar cobertura de cláusulas 4-10, políticas, registros de riesgo, KPIs
  - `--standard nist-ai-rmf`: verificar cobertura de funciones MAP/MEASURE/MANAGE/GOVERN
  - `--all`: ejecutar todas las verificaciones
- [ ] Output: reporte formateado con porcentaje de cumplimiento, brechas, y recomendaciones específicas
- [ ] Soporte para `--output markdown|json` para integración con herramientas externas

#### 3.2 Comando `devtrail metrics`
- [ ] Implementar `devtrail metrics [--period X]` con:
  - Conteo de documentos por tipo y periodo
  - Tasa de cumplimiento de reviews
  - Distribución de niveles de riesgo
  - Actividad por agente
  - Tendencias (comparación con periodo anterior)
- [ ] Soporte para `--output markdown|json`
- [ ] Integración opcional con GitHub Actions para reportes automáticos en PRs

#### 3.3 Documentos de gobernanza ISO 42001
- [ ] Crear templates de gobernanza alineados con ISO 42001 en `00-governance/`:
  - `AI-RISK-CATALOG.md`: catálogo de riesgos de IA (bias, privacidad, seguridad, impacto ambiental), mapeado a las 12 categorías de NIST AI 600-1
  - `AI-LIFECYCLE-TRACKER.md`: tracker del ciclo de vida de sistemas de IA (diseño → despliegue → monitoreo → retiro)
  - `AI-KPIS.md`: KPIs de gobernanza de IA con métricas propuestas
  - `MANAGEMENT-REVIEW-TEMPLATE.md`: plantilla para revisiones por dirección
- [ ] Estos documentos son **opcionales** — solo para equipos que buscan alineación formal con ISO 42001

#### 3.4 Mapeo NIST AI RMF
- [ ] Crear guías de implementación para las 4 funciones NIST en `03-implementation/`:
  - MAP: contexto y riesgos → mapeo a AILOG/AIDEC
  - MEASURE: métricas y evaluación → mapeo a TES/métricas CLI
  - MANAGE: tratamiento de riesgos → mapeo a ETH/INC
  - GOVERN: gobernanza organizacional → mapeo a ADR/AI-GOVERNANCE-POLICY
- [ ] Crear catálogo de las 12 categorías de riesgo de IA generativa (NIST AI 600-1) con mapeo a templates ETH/AILOG

#### 3.5 Guía de Observabilidad con OpenTelemetry
- [ ] Crear `OBSERVABILITY-GUIDE.md` en `00-governance/` con:
  - Baseline técnico: señales (trazas, métricas, logs), atributos mínimos de Resource (`service.name`, `service.version`, `deployment.environment`), propagación W3C Trace Context
  - Arquitectura de pipeline: Collector, receivers OTLP, processors (batching, redacción), exporters
  - Muestreo y retención: estrategias head/tail, retención por criticidad y costo
  - Políticas de datos y seguridad: prohibición de PII/tokens/secretos en atributos, allowlist, cifrado, control de acceso, redacción automática
  - Checklist DevTrail para proyectos con OTel (AIDEC/ADR de adopción, REQ de observabilidad, TES de propagación, INC con trazas, ETH de privacidad, AILOG de cambios de instrumentación)
  - Roadmap de adopción por fases: Fase 0 (decisión), Fase 1 (instrumentación mínima), Fase 2 (cobertura completa), Fase 3 (integración con gobernanza)
- [ ] Mapear la guía a controles ISO 42001: A.6.2.6 (Operation and Monitoring), A.5.2 (Risk Assessment — evidencia operativa)
- [ ] Crear versión ES de la guía
- [ ] Estos documentos son **opcionales** — solo para proyectos con arquitectura distribuida o requisitos de alta observabilidad

---

### Fase 4: Automatización Avanzada y Ecosistema
**Objetivo**: Consolidar el ecosistema con herramientas avanzadas de auditoría y visualización.
**Versión objetivo**: fw-4.0.0 / cli-2.1.0

#### 4.1 Comando `devtrail audit`
- [ ] Implementar `devtrail audit [--from DATE --to DATE] [--system NAME]` para generar trazas de auditoría completas:
  - Timeline cronológica de todos los documentos en un periodo
  - Filtrado por sistema/componente
  - Exportación a Markdown (para repositorios) y HTML (para presentaciones)
  - Mapa de trazabilidad: REQ → ADR → AILOG → TES → INC
- [ ] Útil para auditorías internas, certificaciones, y evidencia de compliance

#### 4.2 Documentación visual con C4 Model
- [ ] Agregar instrucciones en AGENT-RULES.md para que los agentes generen diagramas C4 usando Mermaid dentro de ADRs
- [ ] Crear guía `C4-DIAGRAM-GUIDE.md` en `00-governance/` con:
  - Niveles del C4 Model: Context, Container, Component, Code
  - Sintaxis Mermaid recomendada para cada nivel
  - Ejemplos de integración en ADRs
- [ ] Opcional: soporte para PlantUML como alternativa

#### 4.3 Integración OpenAPI/AsyncAPI
- [ ] Agregar sección en TEMPLATE-REQ.md y TEMPLATE-ADR.md para referenciar especificaciones de API
- [ ] Crear guía breve sobre cuándo y cómo vincular documentación DevTrail con specs de API
- [ ] No crear un tipo documental nuevo — integrar con REQ y ADR existentes

#### 4.4 Mejoras de ecosistema
- [ ] Reducir duplicación de reglas: generar QUICK-REFERENCE.md desde DOCUMENTATION-POLICY.md (o documentar que es derivado)
- [ ] Limpiar tipos en validación CI: alinear regex con los 12 tipos oficiales
- [ ] Modo texto plano para skills: detectar terminales que no soporten box-drawing y usar formato alternativo
- [ ] Monitorear adopción de IEEE P2863/D2 para futura integración

---

## 5. Impacto en el CLI — Detalle Técnico

### 5.1 Cambios en comandos existentes

| Comando | Cambio | Fase |
| --- | --- | --- |
| `devtrail init` | Agregar nuevas carpetas (08-security, 09-ai-models), nuevos templates y AI-GOVERNANCE-POLICY.md | Fase 1 |
| `devtrail status` | Mostrar conteo de los 12 tipos documentales (no solo 8) | Fase 1 |
| `devtrail repair` | Restaurar nuevas carpetas y templates | Fase 1 |
| `devtrail explore` | Navegar los nuevos tipos en el TUI | Fase 1 |
| `devtrail update-framework` | Manejar migración de fw-2.x a fw-3.x (nuevos campos opcionales no rompen compatibilidad) | Fase 1 |

### 5.2 Nuevos comandos

```
devtrail validate [path]           # Fase 2 — Validación completa local
devtrail compliance [--standard X] # Fase 3 — Verificación de compliance
devtrail metrics [--period X]      # Fase 3 — Métricas de gobernanza
devtrail audit [--from --to]       # Fase 4 — Trazas de auditoría
```

### 5.3 Nuevas dependencias estimadas

| Crate | Propósito | Comando |
| --- | --- | --- |
| `chrono` | Manejo de fechas para métricas y auditoría | metrics, audit |
| `tabled` o similar | Formateo de tablas en terminal | compliance, metrics |
| `serde_json` (ya existe) | Output JSON | compliance, metrics |

---

## 6. Impacto en el Framework (dist/) — Detalle

### 6.1 Archivos nuevos

```
dist/.devtrail/
├── 00-governance/
│   ├── AI-GOVERNANCE-POLICY.md          # Fase 1 — Política IA (ISO 42001)
│   ├── AI-RISK-CATALOG.md              # Fase 3 — Catálogo de riesgos
│   ├── AI-LIFECYCLE-TRACKER.md         # Fase 3 — Tracker ciclo de vida
│   ├── AI-KPIS.md                      # Fase 3 — KPIs gobernanza
│   ├── MANAGEMENT-REVIEW-TEMPLATE.md   # Fase 3 — Revisiones por dirección
│   ├── OBSERVABILITY-GUIDE.md         # Fase 3 — Guía OpenTelemetry (opcional)
│   └── C4-DIAGRAM-GUIDE.md            # Fase 4 — Guía diagramas C4
├── 08-security/                        # Fase 2 — Nueva carpeta
│   └── .gitkeep
├── 09-ai-models/                       # Fase 2 — Nueva carpeta
│   └── .gitkeep
├── templates/
│   ├── TEMPLATE-SEC.md                 # Fase 2
│   ├── TEMPLATE-MCARD.md              # Fase 2
│   ├── TEMPLATE-SBOM.md               # Fase 2
│   ├── TEMPLATE-DPIA.md               # Fase 2
│   └── i18n/es/
│       ├── TEMPLATE-SEC.md            # Fase 2
│       ├── TEMPLATE-MCARD.md          # Fase 2
│       ├── TEMPLATE-SBOM.md           # Fase 2
│       └── TEMPLATE-DPIA.md           # Fase 2
```

### 6.2 Archivos modificados

| Archivo | Cambios | Fase |
| --- | --- | --- |
| Todos los TEMPLATE-*.md | Campos opcionales regulatorios | Fase 1 |
| TEMPLATE-ETH.md | Secciones EU AI Act, NIST, impacto ambiental | Fase 1 |
| TEMPLATE-REQ.md | Interfaces externas (29148), trazabilidad a riesgos, sección opcional Observability Requirements | Fase 1 |
| TEMPLATE-TES.md | Vocabulario 29119-3, sección opcional Observability Tests | Fase 1 |
| TEMPLATE-ADR.md | Inmutabilidad, criterios de validación | Fase 1 |
| TEMPLATE-INC.md | Timeline regulatorio EU AI Act, campos opcionales trace_id/span_id/dashboard_links | Fase 1 |
| TEMPLATE-AILOG.md | Campos regulatorios, nota de instrumentación OTel | Fase 1 |
| AGENT-RULES.md | Nuevos criterios de documentación, nuevos tipos, triggers review | Fases 1-2 |
| DOCUMENTATION-POLICY.md | Nuevos tipos, campos, estándares | Fases 1-2 |
| QUICK-REFERENCE.md | Reflejo de todos los cambios | Fases 1-2 |
| DEVTRAIL.md | Actualización completa | Fases 1-2 |
| Directivas (CLAUDE.md, GEMINI.md, etc.) | Expansión con checklists y ejemplos | Fase 1 |
| pre-commit-docs.sh | Validaciones cruzadas | Fase 2 |
| validate-docs.ps1 | Validaciones cruzadas | Fase 2 |
| docs-validation.yml | Jobs de compliance y métricas | Fase 2 |
| dist-manifest.yml | Nuevos archivos y carpetas | Fases 1-4 |
| config.yml | Sin cambios de estructura | — |

---

## 7. Compatibilidad y Migración

### 7.1 Compatibilidad hacia atrás

- **Todos los campos nuevos son opcionales**: proyectos existentes con fw-2.x seguirán funcionando sin cambios
- **Los 8 tipos originales no cambian de nombre ni semántica**: solo se expanden con secciones opcionales
- **El frontmatter actual sigue siendo válido**: los nuevos campos son adiciones, no reemplazos
- **Scripts de validación existentes no se rompen**: las nuevas validaciones son adicionales

### 7.2 Estrategia de migración fw-2.x → fw-3.x

1. `devtrail update-framework` detectará la versión actual y ofrecerá migración guiada
2. Se crearán las nuevas carpetas automáticamente
3. Los templates existentes se actualizarán respetando modificaciones del usuario (checksums)
4. Los documentos existentes del usuario **no se tocan** — mantienen su formato original
5. Se mostrará un resumen de cambios y recomendaciones post-actualización

### 7.3 Versionado

| Cambio | Tipo | Justificación |
| --- | --- | --- |
| fw-2.2.0 → fw-3.0.0 | **Major** | Nuevos tipos documentales, cambios en estándares base |
| cli-1.2.0 → cli-1.3.0 | **Minor** | Solo cambios en init/status/repair para nuevas carpetas |
| cli-1.3.0 → cli-1.4.0 | **Minor** | Nuevo comando `validate` |
| cli-1.4.0 → cli-2.0.0 | **Major** | Nuevos comandos `compliance` y `metrics` |
| cli-2.0.0 → cli-2.1.0 | **Minor** | Nuevo comando `audit` |

---

## 8. Decisiones Pendientes de Aprobación

Antes de iniciar la implementación, se requiere aprobación en estos puntos:

### 8.1 Alcance

1. **¿Adoptar ISO 42001 como estándar vertebral?** Los tres agentes lo recomiendan (GPT-Codex y Qwen explícitamente). Esto implica que DevTrail se posiciona no solo como framework documental sino como habilitador de certificación ISO 42001.

Respuesta: Sí. Debe ser el estándar vertebral y, especialmente, no debe ser solamente una fuente de vocabulario, queremos que los flujos de trabajo e información se acondicionen a ese estándar. Es correcto que este no es un sistema de cumplimiento, sino documental, pero queremos que el workflow y los productos encajen bien en un sistema de cumplimiento del cliente.

2. **¿Incluir los 4 nuevos tipos documentales (SEC, MCARD, SBOM, DPIA)?** Amplían la cobertura pero aumentan la complejidad. Alternativa: mantenerlos como extensiones opcionales que se instalan con `devtrail init --extended`.

Respuesta: Incluir los 4 nuevo tipos documentales, ampliar la cobertura aunque se aumente la complejidad.

3. **¿Reemplazar el umbral de "10 líneas" por criterios basados en impacto?** Los tres agentes coinciden en que es arbitrario. Qwen propone criterios de complejidad ciclomática, pero eso puede ser difícil de medir en contexto de agentes. Propuesta intermedia: subir a 20 líneas + criterios cualitativos (auth, PII, API pública, schema BD).

Respuesta:  Podemos usar una solución híbrida. Ya tenemos un CLI que puede servir de puente entre el LLM y una herramienta como lizard para calcular la complejidad ciclomática (el cli o el llm en modo agente podrían instalar la librería o solicitar al  usuario que la instale via PIP, por ejemplo), el cli puede ser invocado por el agente para organizar un análisis a través de lizard y construir una estructura json con resultados para pasar de regreso a la skill del LLM. En ausencia de lizard o un fallo en comunicación con el CLI, un fallback a la propuesta de subir a 20 líneas + criterios cualitativos.

### 8.2 Priorización

4. **¿Priorizar EU AI Act sobre ISO 42001?** Qwen recomienda EU AI Act primero por el deadline de agosto 2026. Sin embargo, ISO 42001 proporciona el marco donde EU AI Act encaja naturalmente. Propuesta: hacer ambos en paralelo en Fase 1 (estándares base) y Fase 3 (documentos detallados).

Respuesta: ambos en paralelo en Fase 1 y Fase 3 según la propuesta de este documento.

5. **¿Implementar \****`devtrail validate`**\*\* como comando CLI o mejorar los scripts existentes?** El CLI centralizado es más mantenible a largo plazo, pero requiere más esfuerzo. Propuesta: Fase 2 para el comando, mientras se mejoran los scripts en Fase 1 como solución inmediata.

Respueta: Se acepta tu propuesta de mejora incremental, primero en scripts, luego en CLI para poder eliminar el uso de scripts.

### 8.3 Técnicas

6. **¿Bump major del framework (fw-3.0.0) en Fase 1?** Los nuevos campos opcionales técnicamente no rompen compatibilidad, pero el cambio de estándares base (IEEE 830 → ISO 29148) y nuevos tipos documentales justifican un major bump como señal de evolución significativa.

Respuesta: Sí, adelante con el cambio de versión en estos términos.

7. **¿Los documentos de gobernanza ISO 42001 (AI-RISK-CATALOG, etc.) deberían ser templates o documentos pre-llenados?** Como templates, cada proyecto los adapta. Como documentos, se instalan como guía. Propuesta: templates con contenido de ejemplo extenso.

Respuesta: no podemos pre-llenarlos porque nuestro proyecto es un framework, los proyectos de los clientes son diversos y no podemos anticiparnos al llenado. Los templates con contenido de ejemplo extenso puede ser más útil. Las SKILLS que se desarrollen para ello, deberán proporcionar al usuario la ayuda paso a paso con ejemplos basados en esos templates para poder generar el documento en forma. Debemos considerar entonces la posibilidad de que el SKILL sea interactivo y cómo se invoca eso (si acaso requiere de alguna sintáxis o llamado especial) para cada plataforma de IA soportada.

---

## 9. Cronograma Estimado

```
2026
Mar  Abr  May  Jun  Jul  Ago  Sep  Oct  Nov  Dic
 ├────┼────┼────┼────┼────┼────┼────┼────┼────┤
 │         │              │              │     │
 │  Fase 1 │   Fase 2     │   Fase 3     │  F4 │
 │ fw-3.0  │  fw-3.1      │  fw-3.2      │fw-4 │
 │cli-1.3  │ cli-1.4      │  cli-2.0     │2.1  │
 │         │              │              │     │
 │         │              ▼              │     │
 │         │         EU AI Act           │     │
 │         │        (2 ago 2026)         │     │
 │         │                             │     │
```

- **Fase 1** (Abril — Mayo 2026): Correcciones críticas, base regulatoria
- **Fase 2** (Mayo — Julio 2026): Nuevos tipos, validación, comando validate
- **Fase 3** (Julio — Octubre 2026): Compliance automatizado, métricas, ISO 42001
- **Fase 4** (Octubre — Diciembre 2026): Auditoría, C4 Model, ecosistema

> **Nota**: El deadline del EU AI Act (2 de agosto de 2026) cae entre las fases 2 y 3. Los templates con campos regulatorios (Fase 1) y las validaciones (Fase 2) estarán disponibles antes de esa fecha. Los comandos avanzados de compliance (Fase 3) llegarán poco después.

---

## 10. Métricas de Éxito

| Métrica | Objetivo | Cómo medir |
| --- | --- | --- |
| Cobertura de estándares referenciados | 100% actualizados a versiones vigentes 2026 | Auditoría manual de README + templates |
| Tipos documentales | 12 tipos con templates bilingües | Conteo en dist/templates/ |
| Validaciones automatizadas | Reglas cruzadas funcionando en CI y pre-commit | Tests de integración |
| Campos regulatorios | Disponibles en todos los templates relevantes | Revisión de frontmatter |
| Comandos CLI | 13 comandos (9 actuales + 4 nuevos) | `devtrail --help` |
| Compatibilidad | fw-2.x → fw-3.x sin ruptura de documentos existentes | Tests de migración |
| Documentación | Adoption guide y CLI reference actualizados | Revisión de docs/ |

---

## 11. Riesgos y Mitigaciones

| Riesgo | Probabilidad | Impacto | Mitigación |
| --- | --- | --- | --- |
| Sobrecarga de complejidad para adoptantes | Media | Alto | Nuevos campos/tipos son opcionales. Crear modo `--minimal` y `--extended` en init |
| Deadline EU AI Act (agosto 2026) | Alta | Alto | Priorizar campos regulatorios en Fase 1. Templates listos antes de julio |
| Inconsistencia entre scripts y CLI validate | Media | Medio | Fase 2 centraliza en CLI. Deprecar scripts progresivamente |
| Exceso de tipos documentales (12 vs 8) | Baja | Medio | Los 4 nuevos son opcionales. QUICK-REFERENCE mantiene los 8 core visibles |
| Esfuerzo de traducción ES para nuevos contenidos | Media | Bajo | Priorizar templates. Docs de gobernanza ISO 42001 pueden esperar |
| Cambios en ISO 42001 o EU AI Act durante implementación | Baja | Medio | Diseñar campos regulatorios como extensibles, no hardcoded |

---

## 12. Fuentes de las Recomendaciones

### Análisis utilizados
1. **Gemini CLI** (`recomendaciones/GEMINI-PROPUESTAS.md`) — Enfoque en estándares técnicos y actualización de referencias
2. **GPT-Codex** (`recomendaciones/GPT-CODEX-PROPUESTAS.md`) — Enfoque en precisión de instrucciones, validación CI y priorización operativa
3. **Qwen Code** (`recomendaciones/QWEN_PROPUESTAS.md`) — Enfoque en ISO 42001, EU AI Act, NIST AI RMF, nuevos comandos CLI

### Estándares y regulaciones referenciados
- **ISO/IEC 42001:2023** — AI Management System
- **EU AI Act** — Reglamento Europeo de IA (en vigor agosto 2026)
- **NIST AI RMF 1.0** — AI Risk Management Framework (enero 2023)
- **NIST AI 600-1** — Generative AI Profile (julio 2024)
- **ISO/IEC/IEEE 29148:2018** — Requirements Engineering
- **ISO/IEC 25010:2023** — Software Quality (SQuaRE)
- **ISO/IEC/IEEE 29119-3:2021** — Software Testing Documentation
- **ISO/IEC 23894** — AI Risk Management
- **IEEE P2863/D2** — Ethical Governance of AI (emergente, marzo 2026)
- **GDPR** — Reglamento General de Protección de Datos
- **OpenTelemetry** — Estándar abierto para trazas, métricas y logs (complementario, opcional)

### Propuestas adicionales evaluadas
4. **Propuesta OpenTelemetry** (`recomendaciones/propuesta-opentelemetry.md`) — Evaluada el 24 de marzo de 2026. Integrada como estándar complementario recomendado para observabilidad operativa. Aporta evidencia operativa para ISO 42001 (A.6.2.6), EU AI Act (Art. 72) y NIST AI RMF (MEASURE).

---

*Plan maestro elaborado por Claude Opus 4.6, consolidando análisis de Gemini CLI, GPT-Codex y Qwen Code.*
*Creado el 23 de marzo de 2026. Actualizado el 25 de marzo de 2026 — revisión de congruencia post-integración OpenTelemetry.*
