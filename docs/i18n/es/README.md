<div align="center">

# DevTrail

**Gobernanza de Documentación para Desarrollo de Software Asistido por IA**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../../../LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Handbook](https://img.shields.io/badge/docs-Handbook-orange.svg)](../../../dist/.devtrail/QUICK-REFERENCE.md)
[![Strange Days Tech](https://img.shields.io/badge/by-Strange_Days_Tech-purple.svg)](https://strangedays.tech)

[Inicio Rápido](#inicio-rápido) •
[Características](#características) •
[Documentación](#documentación) •
[Contribuir](#contribuir)

**Idiomas**: [English](../../../README.md) | Español

</div>

---

## El Problema

A medida que los asistentes de codificación con IA se vuelven parte integral del desarrollo de software, surge una brecha crítica:

- **¿Quién hizo este cambio?** ¿Fue un desarrollador o un asistente de IA?
- **¿Por qué se tomó esta decisión?** ¿Qué alternativas se consideraron?
- **¿Debería haberse revisado esto?** ¿Fue apropiada la supervisión humana?
- **¿Cuál es el impacto?** ¿Qué tan riesgoso es este cambio?

Sin documentación estructurada, el desarrollo asistido por IA se convierte en una caja negra.

## La Solución

DevTrail proporciona un **sistema de gobernanza de documentación** que asegura:

> **"Ningún cambio significativo sin un rastro documentado."**

Cada cambio significativo—ya sea hecho por un humano o una IA—está documentado, atribuido y puede ser revisado.

---

## Características

### 📋 Documentación Estructurada

Ocho tipos de documentos que cubren todo el ciclo de vida del desarrollo:

| Tipo | Propósito | Ejemplo |
|------|-----------|---------|
| **REQ** | Requisitos | Requisitos del sistema, historias de usuario |
| **ADR** | Decisiones de Arquitectura | Elecciones tecnológicas, patrones de diseño |
| **TES** | Planes de Prueba | Estrategias de prueba, objetivos de cobertura |
| **INC** | Post-mortems de Incidentes | Análisis de causa raíz, lecciones aprendidas |
| **TDE** | Deuda Técnica | Deuda identificada, planes de remediación |
| **AILOG** | Logs de Acciones de IA | Qué hicieron los asistentes de IA y por qué |
| **AIDEC** | Decisiones de IA | Elecciones hechas por IA con alternativas |
| **ETH** | Revisiones Éticas | Privacidad, sesgo, IA responsable |

### 🤖 Soporte para Agentes IA

Pre-configurado para asistentes de codificación con IA populares:

- **Claude Code** (Anthropic) → `CLAUDE.md`
- **Cursor** → `.cursorrules`
- **GitHub Copilot CLI** → `.github/copilot-instructions.md`
- **Gemini CLI** (Google) → `GEMINI.md`

Cada configuración instruye a la IA a:
- Identificarse en cada documento
- Declarar niveles de confianza
- Solicitar revisión humana cuando sea apropiado
- Seguir convenciones de nomenclatura
- **Seguir estrategia de branching Git** (nunca hacer commit directamente a `main`)

### 👁️ Supervisión Humana

Salvaguardas incorporadas que aseguran que los humanos mantengan el control:

- **Niveles de autonomía**: Algunos tipos de documentos requieren aprobación humana
- **Disparadores de revisión**: Baja confianza o alto riesgo → revisión obligatoria
- **Revisiones éticas**: Preocupaciones de privacidad y sesgo marcadas para decisión humana

### ✅ Validación y CI/CD

Herramientas de validación automatizadas:

- **Hooks pre-commit** (Bash) - Validar antes de commit
- **Script PowerShell** - Validación compatible con Windows
- **GitHub Actions** - Flujo de trabajo de validación de PR

---

## Inicio Rápido

### Opción 1: CLI (Recomendado)

**Instalación rápida (binario precompilado):**

```bash
# Linux / macOS
curl -fsSL https://raw.githubusercontent.com/StrangeDaysTech/devtrail/main/install.sh | sh
```

```powershell
# Windows (PowerShell)
irm https://raw.githubusercontent.com/StrangeDaysTech/devtrail/main/install.ps1 | iex
```

O instalar desde el código fuente con Cargo:

```bash
cargo install devtrail-cli
```

Luego inicializa en tu proyecto:

```bash
cd tu-proyecto
devtrail init .
```

El CLI descarga la última versión de DevTrail, configura el framework y los archivos de directivas de agentes IA automáticamente.

```bash
# Actualizar documentos DevTrail a la última versión
devtrail update

# Actualizar el binario del CLI
devtrail update-cli

# Mostrar información de autoría y licencia
devtrail about

# Eliminar DevTrail
devtrail remove
```

### Opción 2: Configuración Manual

```bash
# Descargar el último release ZIP de GitHub
# https://github.com/StrangeDaysTech/devtrail/releases/latest

# Extraer y copiar a tu proyecto
cp -r .devtrail tu-proyecto/
cp DEVTRAIL.md tu-proyecto/
cp -r scripts tu-proyecto/

# Commit
git add .devtrail/ DEVTRAIL.md scripts/
git commit -m "chore: adoptar DevTrail"
```

**Ver [ADOPTION-GUIDE.md](adopters/ADOPTION-GUIDE.md) para instrucciones detalladas, estrategias de migración y planes de implementación en equipos.**

---

## Documentación

La documentación de DevTrail está organizada por audiencia:

| Track | Para | Empieza aquí |
|-------|------|--------------|
| [**Adoptantes**](adopters/) | Equipos que adoptan DevTrail en sus proyectos | [ADOPTION-GUIDE.md](adopters/ADOPTION-GUIDE.md) |
| [**Contribuidores**](../../../docs/contributors/) | Desarrolladores que contribuyen a DevTrail | [TRANSLATION-GUIDE.md](../../../docs/contributors/TRANSLATION-GUIDE.md) |

**Adoptantes**: Sigue la [Guía de Adopción](adopters/ADOPTION-GUIDE.md) para instrucciones paso a paso, estrategias de migración para proyectos existentes y planes de implementación en equipos.

**Contribuidores**: Consulta [CONTRIBUTING.md](CONTRIBUTING.md) para guías de desarrollo, y la [Guía de Traducción](../../../docs/contributors/TRANSLATION-GUIDE.md) para agregar nuevos idiomas.

### Referencias Clave

| Documento | Descripción |
|-----------|-------------|
| [**📘 Referencia Rápida**](../../../dist/.devtrail/QUICK-REFERENCE.md) | Resumen de tipos de documentos y nomenclatura |
| [DEVTRAIL.md](../../../dist/DEVTRAIL.md) | Reglas de gobernanza unificadas (fuente de verdad) |
| [ADOPTION-GUIDE.md](adopters/ADOPTION-GUIDE.md) | Guía de adopción para proyectos nuevos/existentes |

### Estructura Interna

Una vez adoptado, DevTrail crea un directorio `.devtrail/` en tu proyecto para gobernanza de desarrollo:

```
.devtrail/
├── 00-governance/           # Políticas y reglas
├── 01-requirements/         # Documentos REQ
├── 02-design/decisions/     # Documentos ADR
├── 03-implementation/       # Guías de implementación (incl. estrategia Git)
├── 04-testing/              # Documentos TES
├── 05-operations/incidents/ # Documentos INC
├── 06-evolution/technical-debt/ # Documentos TDE
├── 07-ai-audit/
│   ├── agent-logs/          # Documentos AILOG
│   ├── decisions/           # Documentos AIDEC
│   └── ethical-reviews/     # Documentos ETH
└── templates/               # Plantillas de documentos
```

### Convención de Nomenclatura

```
[TIPO]-[YYYY-MM-DD]-[NNN]-[descripcion].md
```

Ejemplo: `ADR-2025-01-27-001-usar-postgresql-para-persistencia.md`

---

## Cómo Funciona

### 1. La IA Hace un Cambio

Un asistente de IA trabajando en tu código automáticamente:

```yaml
# Crea: .devtrail/07-ai-audit/agent-logs/AILOG-2025-01-27-001-implementar-auth.md
---
id: AILOG-2025-01-27-001
title: Implementar autenticación JWT
agent: claude-code-v1.0
confidence: high
risk_level: high
review_required: true
---
```

### 2. Humano Revisa (Cuando es Necesario)

Cambios de alto riesgo o baja confianza son marcados:

```
AILOG-2025-01-27-001-implementar-auth.md
   Agent: claude-code-v1.0
   Confidence: high
   Risk Level: high
   Review Required: YES
```

### 3. Las Decisiones se Preservan

Al elegir entre alternativas, las decisiones se documentan:

```yaml
# Crea: .devtrail/07-ai-audit/decisions/AIDEC-2025-01-27-001-estrategia-auth.md
---
id: AIDEC-2025-01-27-001
title: Elegir JWT sobre autenticación basada en sesiones
alternatives_considered:
  - JWT tokens (elegido)
  - Session cookies
  - Solo OAuth
justification: "Requisito de arquitectura sin estado..."
---
```

### 4. Preocupaciones Éticas son Marcadas

Cuando la IA encuentra consideraciones éticas:

```yaml
# Crea: .devtrail/07-ai-audit/ethical-reviews/ETH-2025-01-27-001-datos-usuario.md
---
id: ETH-2025-01-27-001
title: Alcance de recolección de datos de usuario
status: draft  # Requiere aprobación humana
review_required: true
concerns:
  - Cumplimiento GDPR
  - Minimización de datos
---
```

---

## Validación

### Hook Pre-commit

```bash
# Instalar el hook pre-commit
cp scripts/pre-commit-docs.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

### Validación Manual

```bash
# Linux/Mac
bash scripts/pre-commit-docs.sh

# Windows PowerShell
.\scripts\validate-docs.ps1
```

### GitHub Actions

El flujo de trabajo incluido (`.github/workflows/docs-validation.yml`) valida automáticamente:
- Convenciones de nomenclatura de archivos
- Campos de metadatos requeridos
- Detección de información sensible
- Formato Markdown
- Integridad de enlaces internos

---

## Skills

DevTrail incluye skills para agentes IA que habilitan la **creación activa de documentación**.

> **Sistema Binario**: DevTrail usa un sistema pasivo (agentes auto-documentan via instrucciones de contexto) y un sistema activo (usuarios invocan skills para crear documentación manualmente o cuando el agente omitió algo).

### Skills Disponibles

| Skill | Propósito | Claude | Gemini |
|-------|-----------|--------|--------|
| `/devtrail-status` | Verificar cumplimiento de documentación | ✅ | ✅ |
| `/devtrail-new` | Crear cualquier tipo de documento (unificado) | ✅ | ✅ |
| `/devtrail-ailog` | Creación rápida de AILOG | ✅ | ✅ |
| `/devtrail-aidec` | Creación rápida de AIDEC | ✅ | ✅ |
| `/devtrail-adr` | Creación rápida de ADR | ✅ | ✅ |

### Ejemplos de Uso

```bash
# Verificar estado de documentación
/devtrail-status

# Crear documentación (agente sugiere tipo)
/devtrail-new

# Forzar tipo específico
/devtrail-new ailog

# Accesos directos
/devtrail-ailog
/devtrail-aidec
/devtrail-adr
```

### Scripts Shell (Uso Manual)

Para usuarios que prefieren línea de comandos o usan agentes sin soporte de skills:

```bash
# Creación interactiva de documentos
./scripts/devtrail-new.sh

# Crear tipo específico directamente
./scripts/devtrail-new.sh ailog

# Verificar estado de documentación
./scripts/devtrail-status.sh
```

### Reporte de Agentes

Los agentes IA reportan su estado de documentación al final de cada tarea:

| Estado | Significado |
|--------|-------------|
| `DevTrail: Created AILOG-...` | Documentación fue creada |
| `DevTrail: No documentation required` | Cambio menor (<10 líneas) |
| `DevTrail: Documentation pending` | Puede necesitar revisión manual |

### Arquitectura Multi-Agente

DevTrail proporciona soporte nativo de skills para múltiples agentes IA a través de una arquitectura en capas:

```
tu-proyecto/
├── .agent/workflows/       # 🌐 Agnóstico (Antigravity, futuros agentes)
│   ├── devtrail-new.md
│   ├── devtrail-status.md
│   └── ...
├── .gemini/skills/         # 🔵 Gemini CLI (Google)
│   ├── devtrail-new/SKILL.md
│   └── ...
└── .claude/skills/         # 🟣 Claude Code (Anthropic)
    ├── devtrail-new/SKILL.md
    └── ...
```

| Directorio | Agente | Producto | Formato |
|------------|--------|----------|---------|
| `.agent/workflows/` | Antigravity, genérico | Extensiones VS Code/Cursor | `skill-name.md` con frontmatter YAML |
| `.gemini/skills/` | Gemini CLI | CLI terminal de Google | `skill-name/SKILL.md` |
| `.claude/skills/` | Claude Code | Agente de codificación de Anthropic | `skill-name/SKILL.md` |

> **Nota**: `.agent/` es el estándar **agnóstico de proveedor**. Los directorios específicos de agentes (`.gemini/`, `.claude/`) proporcionan compatibilidad para esas plataformas siguiendo sus convenciones nativas.

Todas las implementaciones de skills son **funcionalmente idénticas**—solo difiere el formato para coincidir con los requisitos de cada agente.

---

## Plataformas Soportadas

### Asistentes de Codificación IA

| Plataforma | Archivo de Config | Estado |
|------------|-------------------|--------|
| Claude Code | `CLAUDE.md` | Soporte completo |
| Cursor | `.cursorrules` | Soporte completo |
| GitHub Copilot CLI | `.github/copilot-instructions.md` | Soporte completo |
| Gemini CLI | `GEMINI.md` | Soporte completo |

### Sistemas Operativos

| SO | Script de Validación |
|----|---------------------|
| Linux | `scripts/pre-commit-docs.sh` |
| macOS | `scripts/pre-commit-docs.sh` |
| Windows | `scripts/validate-docs.ps1` |

### Plataformas CI/CD

| Plataforma | Soporte |
|------------|---------|
| GitHub Actions | Flujo de trabajo incluido |
| GitLab CI | Adaptable desde GitHub Actions |
| Azure DevOps | Adaptable desde GitHub Actions |

---

## Alineación con Estándares

DevTrail se alinea con:

- **ADR** (Architecture Decision Records) - Soporte nativo
- **IEEE 830** - Estructura de documentación de requisitos
- **ISO/IEC 25010** - Atributos de calidad en ADRs
- **GDPR** - Documentación de impacto de privacidad (ETH)
- **EU AI Act** - Transparencia de IA y supervisión humana
- **NIST AI RMF** - Documentación de riesgos

---

## Contribuir

¡Damos la bienvenida a contribuciones! Ver [CONTRIBUTING.md](CONTRIBUTING.md) para guías.

### Formas de Contribuir

- Reportar bugs
- Sugerir características
- Mejorar documentación
- Enviar pull requests
- Agregar traducciones

---

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](../../../LICENSE) para detalles.

---

## Acerca de Strange Days Tech, S.A.S.

<div align="center">

**[Strange Days Tech](https://strangedays.tech)** construye herramientas para desarrollo de software responsable asistido por IA. DevTrail es uno de nuestros proyectos de código abierto.

[Sitio Web](https://strangedays.tech) • [GitHub](https://github.com/StrangeDaysTech)

</div>

---

<div align="center">

**DevTrail** — Porque cada cambio cuenta una historia.

[Volver arriba](#devtrail)

</div>
