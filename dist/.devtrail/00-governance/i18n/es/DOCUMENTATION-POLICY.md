# Política de Documentación - DevTrail

**Idiomas**: [English](../../DOCUMENTATION-POLICY.md) | Español

---

## 1. Convención de Nomenclatura de Archivos

### Formato Estándar

```
[TIPO]-[YYYY-MM-DD]-[NNN]-[descripcion].md
```

| Componente | Descripción | Ejemplo |
|------------|-------------|---------|
| `TIPO` | Prefijo del tipo de documento | `AILOG`, `AIDEC`, `ADR` |
| `YYYY-MM-DD` | Fecha de creación | `2025-01-27` |
| `NNN` | Número secuencial del día | `001`, `002` |
| `descripcion` | Breve descripción en kebab-case | `implementar-oauth` |

### Ejemplos

```
AILOG-2025-01-27-001-implementar-oauth.md
AIDEC-2025-01-27-001-seleccion-framework-testing.md
ADR-2025-01-27-001-arquitectura-microservicios.md
REQ-2025-01-27-001-autenticacion-usuarios.md
```

---

## 2. Metadatos Requeridos (Frontmatter)

Todos los documentos deben incluir metadatos YAML al principio:

```yaml
---
id: AILOG-2025-01-27-001
title: Implementación de Autenticación OAuth
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

### Campos Requeridos

| Campo | Descripción |
|-------|-------------|
| `id` | Identificador único (igual que el nombre del archivo sin .md) |
| `title` | Título descriptivo |
| `status` | Estado actual del documento |
| `created` | Fecha de creación |
| `agent` | Identificador del agente que creó el documento |
| `confidence` | Nivel de confianza del agente |
| `review_required` | Si se requiere revisión humana |
| `risk_level` | Nivel de riesgo del cambio |

### Campos Opcionales

| Campo | Descripción |
|-------|-------------|
| `updated` | Fecha de última actualización |
| `tags` | Etiquetas para categorización |
| `related` | Referencias a documentos relacionados |
| `supersedes` | ID del documento que este reemplaza |
| `superseded_by` | ID del documento que reemplaza a este |

---

## 3. Estados de Documentos

```
draft ──────► accepted ──────► deprecated
                │                   │
                │                   ▼
                └──────► superseded
```

| Estado | Descripción |
|--------|-------------|
| `draft` | En borrador, pendiente de revisión |
| `accepted` | Aprobado y vigente |
| `deprecated` | Obsoleto, pero se mantiene como referencia |
| `superseded` | Reemplazado por otro documento |

---

## 4. Niveles de Riesgo

| Nivel | Cuándo usar | Requiere revisión |
|-------|-------------|-------------------|
| `low` | Cambios cosméticos, documentación | No |
| `medium` | Nueva funcionalidad, refactoring | Recomendado |
| `high` | Seguridad, datos sensibles, APIs públicas | Sí |
| `critical` | Cambios irreversibles, producción | Obligatorio |

---

## 5. Niveles de Confianza

| Nivel | Significado | Acción |
|-------|-------------|--------|
| `high` | El agente está seguro de la decisión | Proceder |
| `medium` | El agente tiene dudas menores | Documentar alternativas |
| `low` | El agente necesita validación | Marcar `review_required: true` |

---

## 6. Estructura de Carpetas

```
.devtrail/
├── 00-governance/          # Políticas y reglas
├── 01-requirements/        # Requisitos del sistema
├── 02-design/              # Diseño y arquitectura
│   └── decisions/          # ADRs
├── 03-implementation/      # Guías de implementación
├── 04-testing/             # Estrategias de prueba
├── 05-operations/          # Operaciones
│   └── incidents/          # Post-mortems
├── 06-evolution/           # Evolución del sistema
│   └── technical-debt/     # Deuda técnica
├── 07-ai-audit/            # Auditoría de agentes IA
│   ├── agent-logs/         # AILOG
│   ├── decisions/          # AIDEC
│   └── ethical-reviews/    # ETH
└── templates/              # Plantillas
```

---

## 7. Referencias Cruzadas

Usa el formato `[TIPO-ID]` para referencias:

```markdown
Esta decisión se basa en los requisitos definidos en [REQ-2025-01-15-003].
Ver también [ADR-2025-01-20-001] para contexto arquitectónico.
```

---

*DevTrail v1.0.0 | [Strange Days Tech](https://strangedays.tech)*
