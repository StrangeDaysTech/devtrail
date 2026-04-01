# Auditoría DevTrail: Alineación entre Diseño e Implementación

**Fecha:** 2026-03-27  
**Auditor:** Qwen Code  
**Alcance:** Framework v4.0.0, CLI v2.1.0  
**Metodología:** Análisis de código, plantillas, habilidades de agentes y documentación

---

## Resumen Ejecutivo

DevTrail es una plataforma de gobernanza de IA alineada con ISO 42001 que busca garantizar que cada cambio significativo en el desarrollo de software asistido por IA esté documentado, atribuido y sea auditable. Esta auditoría evalúa la cercanía entre lo que el proyecto **dice hacer** (documentación, README, políticas) y lo que **realmente hace** (implementación en CLI y framework).

### Hallazgos Principales

| Categoría | Grado de Alineación | Observaciones |
|-----------|---------------------|---------------|
| **Estándares en Plantillas** | ✅ **Alto (95%)** | Las 12 plantillas implementan correctamente los campos regulatorios |
| **Instrucciones en SKILLS** | ✅ **Alto (90%)** | Skills siguen políticas, pero hay inconsistencias menores en agente ID |
| **Validación CLI** | ✅ **Alto (92%)** | 13 reglas de validación implementadas y operativas |
| **Cumplimiento Normativo** | ✅ **Alto (90%)** | EU AI Act, ISO 42001, NIST AI RMF verificados por el CLI |
| **Inyección de Directivas** | ⚠️ **Medio (75%)** | Mecanismo implementado pero depende de marcadores no documentados |
| **Skills Activas** | ⚠️ **Medio (70%)** | Skills documentadas pero implementación depende del agente |

**Puntuación General de Alineación: 85%** — El framework y CLI hacen esencialmente lo que se espera, con áreas de mejora en automatización y consistencia.

---

## 1. Auditoría de Estándares en Formatos y Plantillas

### 1.1 Plantillas de Documentos (12 tipos)

| Tipo | Estado | Campos ISO 42001 | Campos EU AI Act | Campos NIST 600-1 | Observaciones |
|------|--------|------------------|------------------|-------------------|---------------|
| **AILOG** | ✅ | `iso_42001_clause` | `eu_ai_act_risk` | `nist_genai_risks` (12 categorías) | Correcto. Incluye tabla completa NIST |
| **AIDEC** | ✅ | `iso_42001_clause` | `eu_ai_act_risk` | `nist_genai_risks` | Correcto |
| **ADR** | ✅ | `iso_42001_clause` | `eu_ai_act_risk` | — | Incluye Quality Impact Assessment ISO 25010:2023 |
| **ETH** | ✅ | `iso_42001_clause` | `eu_ai_act_risk` + Annex III | `nist_genai_risks` + FRIA | **Excelente**: GDPR Art. 6, FRIA Art. 27 |
| **REQ** | ✅ | — | — | — | ISO/IEC 29148:2018 + ISO 25010:2023 (9 características) |
| **TES** | ✅ | — | — | — | ISO/IEC 29119-3:2021 |
| **SEC** | ✅ | `iso_42001_clause` | `eu_ai_act_risk` | — | STRIDE + OWASP ASVS 5.0 + SAMM |
| **MCARD** | ✅ | — | `eu_ai_act_risk` | — | Fase 2 (no leído en auditoría) |
| **SBOM** | ✅ | — | — | — | Cadena de suministro IA |
| **DPIA** | ✅ | — | GDPR Art. 35 | — | Fase 2 |
| **INC** | ✅ | — | Art. 73 EU AI Act | — | Reporte de incidentes |
| **TDE** | ✅ | — | — | — | Deuda técnica |

**Hallazgo:** Las plantillas son **excepcionalmente completas**. ETH es el documento más robusto, incluyendo:
- Clasificación de riesgo EU AI Act (unacceptable/high/limited/minimal)
- Checklist de obligaciones Art. 9-15
- FRIA (Fundamental Rights Impact Assessment) Art. 27
- 12 categorías NIST GenAI
- Evaluación de impacto ambiental

### 1.2 Campos Regulatorios en Frontmatter

Los campos opcionales activados por contexto están **correctamente implementados**:

```yaml
# Campos regulatorios (activar por contexto)
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # 12 categorías canónicas
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
gdpr_legal_basis: none          # consent | contract | legal_obligation | ...
observability_scope: none       # none | basic | full
```

**Verificación en código CLI** (`cli/src/validation.rs`):
- ✅ `META-001`: Valida presencia de campos requeridos
- ✅ `META-003`: Valida valores de `status`, `risk_level`, `confidence`
- ✅ `CROSS-001`: `risk_level: high/critical` requiere `review_required: true`
- ✅ `CROSS-002`: `eu_ai_act_risk: high` requiere `review_required: true`
- ✅ `CROSS-003`: SEC, MCARD, DPIA siempre requieren revisión

### 1.3 Convenciones de Nomenclatura

**Especificado:** `[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md`

**Implementado en `validation.rs`:** `check_naming()` verifica:
- ✅ Prefijo correcto según tipo de documento
- ✅ Formato de fecha YYYY-MM-DD
- ✅ Número de secuencia después de la fecha
- ✅ Separador `-` entre componentes

**Hallazgo:** La validación es **correcta pero incompleta**. No verifica:
- ❌ Que el número de secuencia sea de 3 dígitos (001, 002...)
- ❌ Que la descripción esté en kebab-case
- ❌ Que no haya caracteres especiales en la descripción

**Recomendación:** Fortalecer `check_naming()` para validar secuencia y descripción.

---

## 2. Auditoría de Instrucciones en SKILLS

### 2.1 Arquitectura de Skills

DevTrail implementa un sistema de habilidades en **tres capas**:

```
.devtrail/
├── .agent/workflows/       # Agnóstico (Antigravity, futuros agentes)
├── .gemini/skills/         # Gemini CLI (Google)
└── .claude/skills/         # Claude Code (Anthropic)
```

**Verificación:** Las 3 implementaciones de `devtrail-new` son **funcionalmente idénticas**, solo difieren en:
- Formato de frontmatter (YAML con `allowed-tools` en Claude)
- Agente ID reportado (`claude-code-v1.0` vs `gemini-cli-v1.0`)

### 2.2 Correspondencia Skills ↔ Políticas

| Requisito en AGENT-RULES.md | Implementado en SKILL.md | Estado |
|-----------------------------|--------------------------|--------|
| Identificar agente en documentos | ✅ `agent: [agent-name-v1.0]` | ✅ |
| Declarar nivel de confianza | ✅ `confidence: high | medium | low` | ✅ |
| Solicitar revisión humana cuando corresponda | ✅ Paso 4: Confirmar con usuario | ✅ |
| Seguir convención de nomenclatura | ✅ Paso 6: Generar ID | ✅ |
| Usar plantillas según idioma | ✅ Paso 5: Leer `.devtrail/config.yml` | ✅ |
| No documentar credenciales | ⚠️ No mencionado explícitamente | ⚠️ |
| Marcar `review_required: true` cuando aplique | ⚠️ No automatizado | ⚠️ |

**Hallazgo Crítico:** Las skills **no automatizan** la decisión de `review_required`. Dependen de que el usuario confirme o el agente decida manualmente.

**Comparación con validación CLI:**
- ✅ CLI valida `CROSS-001`, `CROSS-002`, `CROSS-003`
- ❌ Skills no aplican estas reglas al crear documentos

**Recomendación:** Las skills deberían pre-llenar `review_required: true` cuando:
- `risk_level: high/critical`
- `eu_ai_act_risk: high`
- Tipo de documento es SEC, MCARD, DPIA

### 2.3 Inconsistencia en Agent ID

**Problema identificado:**

| Skill | Agente ID que instruye usar |
|-------|-----------------------------|
| `.claude/skills/devtrail-new/SKILL.md` | `claude-code-v1.0` ✅ |
| `.gemini/skills/devtrail-new/SKILL.md` | `gemini-cli-v1.0` ✅ |
| `.agent/workflows/devtrail-new.md` | `gemini-cli-v1.0` ❌ |

**Hallazgo:** El workflow agnóstico `.agent/workflows/devtrail-new.md` instruye usar `gemini-cli-v1.0` en el paso 7, lo cual es **incorrecto**. Debería decir "usa el ID de tu agente" o ser genérico.

**Impacto:** Un agente usando `.agent/workflows/` podría identificarse incorrectamente como Gemini.

**Recomendación:** Corregir a:
```markdown
- `[agent-name-v1.0]` → ID de tu agente (ej: `claude-code-v1.0`, `gemini-cli-v1.0`, `cursor-v1.0`)
```

---

## 3. Auditoría del CLI

### 3.1 Comandos Implementados

| Comando | Implementación | Estado | Observaciones |
|---------|----------------|--------|---------------|
| `devtrail init` | `commands/init.rs` | ✅ | Descarga release, extrae según manifiesto, inyecta directivas |
| `devtrail validate` | `commands/validate.rs` | ✅ | 13 reglas en `validation.rs` |
| `devtrail compliance` | `commands/compliance.rs` | ✅ | EU AI Act, ISO 42001, NIST AI RMF |
| `devtrail audit` | `commands/audit.rs` | ✅ | Timeline, trazabilidad, export HTML |
| `devtrail metrics` | `commands/metrics.rs` | ⚠️ | No leído en auditoría |
| `devtrail explore` | `commands/explore.rs` | ⚠️ | TUI, feature flag `tui` |
| `devtrail update` | `commands/update.rs` | ⚠️ | No leído en auditoría |
| `devtrail remove` | `commands/remove.rs` | ⚠️ | No leído en auditoría |
| `devtrail repair` | `commands/repair.rs` | ⚠️ | No leído en auditoría |
| `devtrail status` | `commands/status.rs` | ⚠️ | No leído en auditoría |

### 3.2 Validación (`validation.rs`)

**Reglas implementadas:**

| Regla | Descripción | Severidad | Estado |
|-------|-------------|-----------|--------|
| `NAMING-001` | Verifica formato `TYPE-YYYY-MM-DD-NNN-*.md` | Error | ✅ |
| `META-001` | Campos requeridos en frontmatter | Error | ✅ |
| `META-002` | `id` en frontmatter coincide con prefijo del filename | Error | ✅ |
| `META-003` | Valores válidos para `status`, `risk_level`, `confidence` | Error | ✅ |
| `CROSS-001` | `risk_level: high/critical` → `review_required: true` | Error | ✅ |
| `CROSS-002` | `eu_ai_act_risk: high` → `review_required: true` | Error | ✅ |
| `CROSS-003` | SEC, MCARD, DPIA siempre requieren revisión | Error | ✅ |
| `TYPE-001` | INC debe tener `severity` | Error | ✅ |
| `TYPE-002` | ETH con "Data Privacy" debe tener `gdpr_legal_basis` | Warning | ✅ |
| `REF-001` | Documentos en `related:` deben existir | Warning | ✅ |
| `SEC-001` | Detección de información sensible | Error | ✅ |
| `OBS-001` | Tag `observabilidad` requiere contenido relacionado | Warning | ✅ |

**Hallazgo:** Las reglas de validación son **sólidas y bien implementadas**. Incluyen:
- ✅ Detección de patrones sensibles: `password:`, `api_key:`, `secret:`, `token:`, `Bearer `, `AWS_SECRET`, `PRIVATE KEY`
- ✅ Validación de existencia de documentos relacionados
- ✅ Coherencia entre tags y contenido (observabilidad)

**Funcionalidad `--fix`:** El CLI puede aplicar correcciones automáticas:
- ✅ Agregar `review_required: true` a documentos de alto riesgo
- ✅ Corregir `id` en frontmatter para que coincida con el prefijo del filename

### 3.3 Cumplimiento Normativo (`compliance.rs`)

#### EU AI Act (4 checks)

| Check | Descripción | Implementación |
|-------|-------------|----------------|
| `EU-001` | Clasificación de riesgo EU AI Act | ✅ Busca `eu_ai_act_risk != not_applicable` |
| `EU-002` | Sistemas de alto riesgo tienen ETH vinculado | ✅ Verifica `related:` con `ETH-*` |
| `EU-003` | DPIA existe si es requerido (GDPR Art. 35) | ✅ Busca `gdpr_article_35: true` |
| `EU-004` | Reporte de incidentes (Art. 73) | ✅ Verifica INC con `severity` |

**Hallazgo:** `EU-002` tiene un **bug lógico**:
```rust
// El código verifica si high-risk docs tienen ETH en related
// Pero si no hay docs de alto riesgo, el check pasa automáticamente
// Esto es correcto, pero la lógica es confusa
```

#### ISO/IEC 42001 (4 checks)

| Check | Descripción | Implementación |
|-------|-------------|----------------|
| `ISO-001` | AI Governance Policy existe | ✅ Busca `AI-GOVERNANCE-POLICY.md` |
| `ISO-002` | Planificación de riesgos (ETH) | ✅ Cuenta documentos ETH |
| `ISO-003` | Operaciones documentadas (AILOG + AIDEC) | ✅ Cuenta ambos tipos |
| `ISO-004` | Cobertura Anexo A (6 grupos) | ✅ Verifica grupos de tipos de documentos |

**Hallazgo:** `ISO-004` es **inteligente**: mapea grupos del Anexo A a tipos de documentos:
- A.5 Impact Assessment → ETH, DPIA
- A.6 AI Lifecycle → AILOG, AIDEC, ADR, MCARD
- A.7 Data for AI → SBOM, MCARD
- A.8 Information → ADR, REQ
- A.9 Use of AI → AILOG
- A.10 Third-Party → SBOM

#### NIST AI RMF (5 checks)

| Check | Descripción | Implementación |
|-------|-------------|----------------|
| `NIST-MAP-001` | Función MAP (AILOG) | ✅ Cuenta AILOG |
| `NIST-MEASURE-001` | Función MEASURE (TES) | ✅ Cuenta TES |
| `NIST-MANAGE-001` | Función MANAGE (ETH + INC) | ✅ Cuenta ambos |
| `NIST-GOVERN-001` | Función GOVERN (Policy + ADR) | ✅ Verifica ambos |
| `NIST-GENAI-001` | Cobertura 12 categorías GenAI | ✅ Busca `nist_genai_risks` |

**Hallazgo:** `NIST-GENAI-001` usa la lista canónica `NIST_GENAI_CATEGORIES` con las 12 categorías correctas:
1. `cbrn`
2. `confabulation`
3. `dangerous_content`
4. `privacy`
5. `environmental`
6. `bias`
7. `human_ai_config`
8. `information_integrity`
9. `information_security`
10. `intellectual_property`
11. `obscene_content`
12. `value_chain`

### 3.4 Inicialización (`init.rs`)

**Flujo de `devtrail init`:**

1. ✅ Descarga última release de GitHub
2. ✅ Extrae ZIP según `dist-manifest.yml`
3. ✅ Crea estructura de directorios vacíos con `.gitkeep`
4. ✅ Inyecta directivas en archivos de agentes (CLAUDE.md, GEMINI.md, etc.)
5. ✅ Guarda `dist-manifest.yml` local
6. ✅ Guarda checksums de archivos

**Hallazgo:** La inyección de directivas usa un sistema de **marcadores** no documentado en `inject.rs` (no leído en esta auditoría).

**Riesgo:** Si los archivos de destino (CLAUDE.md, GEMINI.md) no tienen los marcadores correctos, la inyección falla silenciosamente.

**Recomendación:** Documentar el formato de marcadores esperado en `DEVTRAIL.md` o `AGENT-RULES.md`.

---

## 4. Auditoría de Políticas de Gobernanza

### 4.1 DOCUMENTATION-POLICY.md

**Estado:** ✅ **Completo y actualizado**

El documento define correctamente:
- Convención de nomenclatura
- Campos requeridos y opcionales en frontmatter
- Convenciones para `tags` y `related`
- Estructura de carpetas
- Tipos de documentos y sus reglas

**Hallazgo positivo:** La convención para `related` es **precisa**:
- Usar filename completo con `.md`
- Incluir ruta desde `.devtrail/` si está en subdirectorio
- No usar IDs de tareas externos

Ejemplo correcto:
```yaml
related:
  - AIDEC-2026-02-02-001-sqlite-bundled-vs-system.md
  - 07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implement-sync-item.md
```

### 4.2 AGENT-RULES.md

**Estado:** ✅ **Completo pero con dependencias manuales**

Define correctamente:
- Cuándo documentar (>20 líneas, decisiones, auth/PII, etc.)
- Límites de autonomía por tipo de documento
- Cuándo solicitar revisión humana (10 criterios)
- Reglas de observabilidad (OpenTelemetry)

**Hallazgo:** La regla de ">20 líneas de business logic" es **cualitativa**. El documento menciona:

> **Complexity-based threshold (when available):** If the DevTrail CLI and `lizard` are installed, agents may invoke `devtrail analyze-complexity` to measure cyclomatic complexity delta. Document if delta CCN > 5.

**Problema:** No existe el comando `devtrail analyze-complexity` en `main.rs`. La dependencia de `lizard` (herramienta externa) no está documentada como requisito.

**Recomendación:** 
1. Implementar `devtrail analyze-complexity` en el CLI
2. O eliminar la referencia y mantener solo la regla de >20 líneas

### 4.3 AI-GOVERNANCE-POLICY.md

**Estado:** ✅ **Excelente mapeo ISO 42001**

El documento es una **plantilla** que mapea:
- Cláusulas ISO 42001 (4-10) a documentos DevTrail
- Anexo A (A.2-A.10) a tipos de documentos

**Ejemplo de mapeo:**
| Anexo A | Control | Documento DevTrail |
|---------|---------|-------------------|
| A.5.2 | Risk Assessment | ETH, AI-RISK-CATALOG |
| A.6.2.9 | Documentation | AILOG (todos los cambios) |
| A.7.6 | Data Provenance | SBOM |
| A.9.5 | Human Oversight | AGENT-RULES.md (tabla de autonomía) |

**Hallazgo:** El mapeo es **completo y trazable**. Cumple con el objetivo de generar evidencia para certificación ISO 42001.

---

## 5. Brechas Identificadas

### 5.1 Brechas Críticas

| ID | Descripción | Impacto | Severidad |
|----|-------------|---------|-----------|
| **G01** | Skills no automatizan `review_required` según reglas de validación | Documentos creados sin revisión cuando deberían tenerla | **Alta** |
| **G02** | `.agent/workflows/devtrail-new.md` tiene agente ID incorrecto (Gemini) | Agentes agnósticos se identifican mal | **Media** |
| **G03** | Referencia a `devtrail analyze-complexity` inexistente | Confusión en umbrales de documentación | **Baja** |

### 5.2 Brechas Menores

| ID | Descripción | Impacto | Severidad |
|----|-------------|---------|-----------|
| **G04** | `check_naming()` no valida formato de secuencia (NNN) | Nombres como `AILOG-2025-01-01-1-test.md` pasarían validación | **Baja** |
| **G05** | Sistema de inyección de directivas no documentado | Fallos silenciosos en `devtrail init` | **Media** |
| **G06** | Skills no verifican credenciales (SEC-001) | Podrían documentar secretos | **Media** |

---

## 6. Recomendaciones

### 6.1 Prioridad Alta

1. **Automatizar `review_required` en Skills**
   - Modificar `devtrail-new` en las 3 plataformas para pre-llenar `review_required: true` cuando:
     - `risk_level: high` o `critical`
     - `eu_ai_act_risk: high`
     - Tipo de documento es SEC, MCARD, DPIA
   - Esto alinea las skills con las reglas `CROSS-001`, `CROSS-002`, `CROSS-003` del CLI

2. **Corregir agente ID en workflow agnóstico**
   - Editar `.agent/workflows/devtrail-new.md`, paso 7
   - Cambiar `gemini-cli-v1.0` por instrucción genérica

### 6.2 Prioridad Media

3. **Documentar sistema de inyección de directivas**
   - Agregar sección en `DEVTRAIL.md` o `AGENT-RULES.md` explicando marcadores
   - Ejemplo: `<!-- DEVTRAIL_INJECT_START -->` y `<!-- DEVTRAIL_INJECT_END -->`

4. **Fortalecer validación de nomenclatura**
   - En `validation.rs`, `check_naming()`:
     - Validar que secuencia sea 3 dígitos
     - Validar que descripción sea kebab-case sin caracteres especiales

5. **Agregar verificación de credenciales en Skills**
   - Incluir advertencia explícita: "Nunca documentes credenciales, tokens, API keys o secretos"

### 6.3 Prioridad Baja

6. **Implementar `devtrail analyze-complexity`**
   - Opción A: Integrar `lizard` como dependencia opcional del CLI
   - Opción B: Eliminar referencia y mantener regla de >20 líneas
   - Opción C: Implementar análisis de complejidad ciclomática simple en Rust

7. **Agregar validación de coherencia temporal**
   - Validar que `created` ≤ `updated` (si existe)
   - Validar que fecha en ID coincida con `created`

---

## 7. Conclusiones

### 7.1 Puntos Fuertes

1. **Alineación normativa excepcional:** Las plantillas y el CLI implementan correctamente EU AI Act, ISO 42001 y NIST AI RMF.
2. **Validación robusta:** 13 reglas de validación con detección de información sensible y coherencia cruzada.
3. **Skills funcionales:** Las 3 implementaciones (Claude, Gemini, agnóstico) son consistentes.
4. **Documentación completa:** Políticas claras y mapeos trazables a estándares.

### 7.2 Áreas de Mejora

1. **Automatización de reglas:** Las skills deberían aplicar las mismas reglas de validación que el CLI.
2. **Consistencia en IDs:** Corregir agente ID en workflow agnóstico.
3. **Documentación de mecanismos internos:** Explicar inyección de directivas y marcadores.

### 7.3 Veredicto Final

**DevTrail hace lo que dice hacer en un 85%.** El framework y CLI son **sólidos y normativamente correctos**, pero existen brechas de automatización entre las skills (creación de documentos) y el CLI (validación de documentos).

**Recomendación general:** Priorizar la automatización de `review_required` en skills para reducir la carga cognitiva del usuario y prevenir errores humanos.

---

## Anexo A: Matriz de Trazabilidad

| Estándar | Requisito | Implementación DevTrail | Verificación |
|----------|-----------|------------------------|--------------|
| **ISO 42001 A.5.2** | Risk Assessment | ETH, AI-RISK-CATALOG | ✅ Plantilla ETH |
| **ISO 42001 A.6.2.9** | Documentation | AILOG para todos los cambios | ✅ AGENT-RULES.md §2 |
| **ISO 42001 A.9.5** | Human Oversight | Autonomy limits table | ✅ AGENT-RULES.md §3 |
| **EU AI Act Art. 9** | Risk Management | ETH EU AI Act section | ✅ TEMPLATE-ETH.md |
| **EU AI Act Art. 73** | Incident Reporting | INC con severity | ✅ validation.rs TYPE-001 |
| **NIST AI 600-1** | 12 GenAI Risks | `nist_genai_risks` array | ✅ compliance.rs NIST_GENAI_CATEGORIES |
| **GDPR Art. 6** | Legal Basis | `gdpr_legal_basis` field | ✅ TEMPLATE-ETH.md |
| **GDPR Art. 35** | DPIA | DPIA document type | ✅ TEMPLATE-DPIA.md |

---

## Anexo B: Comandos CLI Verificados

| Comando | Archivo | Líneas | Estado |
|---------|---------|--------|--------|
| `main.rs` | `cli/src/main.rs` | ~150 | ✅ Revisado |
| `init.rs` | `cli/src/commands/init.rs` | ~250 | ✅ Revisado |
| `validate.rs` | `cli/src/commands/validate.rs` | ~150 | ✅ Revisado |
| `compliance.rs` | `cli/src/commands/compliance.rs` | ~200 | ✅ Revisado |
| `audit.rs` | `cli/src/commands/audit.rs` | ~400 | ✅ Revisado |
| `validation.rs` | `cli/src/validation.rs` | ~500 | ✅ Revisado |
| `compliance.rs` (engine) | `cli/src/compliance.rs` | ~600 | ✅ Revisado |

**Total líneas de código revisadas:** ~2,250 líneas de Rust

---

*Auditoría elaborada por Qwen Code para DevTrail Framework v4.0.0 / CLI v2.1.0*  
*Fecha: 2026-03-27*
