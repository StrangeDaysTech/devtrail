# DevTrail - Referencia Rápida

> Referencia de una página para agentes IA y desarrolladores.

**Idiomas**: [English](../../QUICK-REFERENCE.md) | Español

---

## Configuración de Idioma

**Archivo**: `.devtrail/config.yml`

```yaml
language: en  # Opciones: en, es (por defecto: en)
```

| Idioma | Ruta de Templates |
|--------|-------------------|
| `en` | `.devtrail/templates/TEMPLATE-*.md` |
| `es` | `.devtrail/templates/i18n/es/TEMPLATE-*.md` |

---

## Convención de Nomenclatura

```
[TIPO]-[YYYY-MM-DD]-[NNN]-[descripcion].md
```

**Ejemplo**: `AILOG-2025-01-27-001-implementar-oauth.md`

---

## Tipos de Documentos

| Tipo | Nombre | Ubicación | Autonomía del Agente |
|------|--------|-----------|---------------------|
| `AILOG` | Log de Acción IA | `07-ai-audit/agent-logs/` | Crear libremente |
| `AIDEC` | Decisión IA | `07-ai-audit/decisions/` | Crear libremente |
| `ETH` | Revisión Ética | `07-ai-audit/ethical-reviews/` | Solo borrador |
| `ADR` | Decisión de Arquitectura | `02-design/decisions/` | Requiere revisión |
| `REQ` | Requisito | `01-requirements/` | Proponer |
| `TES` | Plan de Pruebas | `04-testing/` | Proponer |
| `INC` | Post-mortem de Incidente | `05-operations/incidents/` | Contribuir |
| `TDE` | Deuda Técnica | `06-evolution/technical-debt/` | Identificar |

---

## Cuándo Documentar

| Situación | Acción |
|-----------|--------|
| >10 líneas lógica de negocio | AILOG |
| Decisión entre alternativas | AIDEC |
| Seguridad/autenticación | AILOG + `risk_level: high` |
| Datos personales (PII) | AILOG + ETH |
| Integración externa | AILOG |
| Cambio en API/BD | AILOG |

**NO documentar**: espacios, erratas, credenciales.

---

## Metadatos Mínimos

```yaml
---
id: AILOG-2025-01-27-001
title: Descripción breve
status: accepted
created: 2025-01-27
agent: agent-name-v1.0
confidence: high | medium | low
review_required: true | false
risk_level: low | medium | high | critical
---
```

---

## Revisión Humana Requerida

Marcar `review_required: true` cuando:

- `confidence: low`
- `risk_level: high | critical`
- Decisiones de seguridad
- Cambios irreversibles
- Documentos ETH, ADR, REQ

---

## Estructura de Carpetas

```
.devtrail/
├── 00-governance/           ← Políticas
├── 01-requirements/         ← REQ
├── 02-design/decisions/     ← ADR
├── 03-implementation/       ← Guías
├── 04-testing/              ← TES
├── 05-operations/incidents/ ← INC
├── 06-evolution/technical-debt/ ← TDE
├── 07-ai-audit/
│   ├── agent-logs/          ← AILOG
│   ├── decisions/           ← AIDEC
│   └── ethical-reviews/     ← ETH
└── templates/               ← Plantillas
```

---

## Flujo de Trabajo

```
1. EVALUAR → ¿Requiere documentación?
       ↓
2. CARGAR → Plantilla correspondiente
       ↓
3. CREAR → Con nomenclatura correcta
       ↓
4. MARCAR → review_required si aplica
```

---

## Niveles

### Confianza
| Nivel | Acción |
|-------|--------|
| `high` | Proceder |
| `medium` | Documentar alternativas |
| `low` | `review_required: true` |

### Riesgo
| Nivel | Ejemplos |
|-------|----------|
| `low` | Docs, formato |
| `medium` | Nueva funcionalidad |
| `high` | Seguridad, APIs |
| `critical` | Producción, irreversible |

---

## Skills (Claude Code)

| Comando | Propósito |
|---------|-----------|
| `/devtrail-status` | Verificar estado y cumplimiento de documentación |

---

*DevTrail v1.0.0 | [Strange Days Tech](https://strangedays.tech)*
