# DevTrail - Referencia CLI

**Referencia completa de la herramienta de línea de comandos `devtrail`.**

[![Strange Days Tech](https://img.shields.io/badge/by-Strange_Days_Tech-purple.svg)](https://strangedays.tech)

**Idiomas**: [English](../../../adopters/CLI-REFERENCE.md) | Español

---

## Tabla de Contenidos

1. [Instalación](#instalación)
2. [Versionado](#versionado)
3. [Comandos](#comandos)
4. [Variables de Entorno](#variables-de-entorno)
5. [Códigos de Salida](#códigos-de-salida)

---

## Instalación

Instala el CLI de DevTrail usando uno de los métodos a continuación. Para instrucciones completas de configuración, consulta el [README](../README.md#inicio-rápido).

**Instalación rápida (binario precompilado):**

```bash
# Linux / macOS
curl -fsSL https://raw.githubusercontent.com/StrangeDaysTech/devtrail/main/install.sh | sh
```

```powershell
# Windows (PowerShell)
irm https://raw.githubusercontent.com/StrangeDaysTech/devtrail/main/install.ps1 | iex
```

**Desde el código fuente:**

```bash
cargo install devtrail-cli
```

---

## Versionado

DevTrail usa **tags de versión independientes** para cada componente:

| Componente | Prefijo de tag | Ejemplo | Qué incluye |
|------------|---------------|---------|-------------|
| Framework | `fw-` | `fw-2.1.0` | Plantillas, docs de gobernanza, directivas, scripts |
| CLI | `cli-` | `cli-1.0.0` | El binario `devtrail` |

Framework y CLI se publican de forma independiente. Una actualización del framework no requiere actualización del CLI, y viceversa.

**Verificar versiones instaladas:**

```bash
devtrail about    # Muestra versión CLI + versión framework (si está instalado)
devtrail status   # Muestra estado completo de la instalación incluyendo versiones
```

---

## Comandos

### `devtrail init [path]`

Inicializa DevTrail en un directorio de proyecto.

**Argumentos:**

| Argumento | Por defecto | Descripción |
|-----------|-------------|-------------|
| `path` | `.` (directorio actual) | Directorio del proyecto destino |

**Qué hace:**

1. Descarga el último release del framework (`fw-*`) desde GitHub
2. Crea la estructura de directorios `.devtrail/`
3. Crea `DEVTRAIL.md` con las reglas de gobernanza
4. Configura archivos de directivas de agentes IA (`CLAUDE.md`, `GEMINI.md`, `.cursorrules`, etc.)
5. Copia scripts de validación y workflows de CI/CD

**Ejemplo:**

```bash
$ devtrail init .
✔ Downloaded DevTrail fw-2.1.0
✔ Created .devtrail/ directory structure
✔ Created DEVTRAIL.md
✔ Configured AI agent directives
✔ Copied validation scripts

DevTrail initialized successfully!
Next: git add .devtrail/ DEVTRAIL.md scripts/ && git commit -m "chore: adopt DevTrail"
```

---

### `devtrail update`

Actualiza **ambos** framework y CLI a sus últimas versiones. Equivale a ejecutar `update-framework` seguido de `update-cli`.

Si `.devtrail/` no existe en el directorio actual, la actualización del framework se omite con una advertencia.

**Ejemplo:**

```bash
$ devtrail update
Updating framework...
✔ Framework updated to fw-2.1.0
Updating CLI...
✔ CLI updated to cli-1.0.0
```

---

### `devtrail update-framework`

Actualiza solo los archivos del framework. Busca el último release `fw-*` en GitHub.

**Manejo de conflictos:** Si has modificado archivos del framework (ej. docs de gobernanza o plantillas), la actualización preserva tus cambios y reporta conflictos para resolución manual.

**Ejemplo:**

```bash
$ devtrail update-framework
✔ Framework updated to fw-2.1.0
```

---

### `devtrail update-cli`

Auto-actualiza el binario `devtrail`. Busca el último release `cli-*` en GitHub y reemplaza el binario actual.

**Ejemplo:**

```bash
$ devtrail update-cli
✔ CLI updated to cli-1.0.0
```

---

### `devtrail remove [--full]`

Elimina DevTrail del proyecto actual.

**Flags:**

| Flag | Descripción |
|------|-------------|
| `--full` | Elimina todo, incluyendo documentos creados por el usuario en `.devtrail/`. Pide confirmación. |

**Comportamiento por defecto** (sin `--full`): elimina la estructura del framework pero preserva los documentos que creaste dentro de `.devtrail/`.

**Ejemplo:**

```bash
$ devtrail remove
✔ DevTrail framework removed. User documents preserved in .devtrail/.

$ devtrail remove --full
⚠ This will delete all DevTrail files including your documents.
Continue? [y/N]: y
✔ DevTrail completely removed.
```

---

### `devtrail status [path]`

Muestra el estado de la instalación y estadísticas de documentación.

**Argumentos:**

| Argumento | Por defecto | Descripción |
|-----------|-------------|-------------|
| `path` | `.` (directorio actual) | Directorio del proyecto destino |

**La salida incluye:**

- Ruta del proyecto
- Versión del framework
- Versión del CLI
- Idioma configurado
- Integridad de la estructura de directorios
- Estadísticas de documentos (conteo por tipo)

**Ejemplo:**

```bash
$ devtrail status
DevTrail Status
───────────────
Path:              /home/user/my-project
Framework version: fw-2.1.0
CLI version:       cli-1.0.0
Language:          en
Structure:         ✔ Complete

Documents:
  AILOG:  12
  AIDEC:   4
  ADR:     7
  REQ:     3
  TES:     2
  TDE:     1
  INC:     0
  ETH:     1
  Total:  30
```

---

### `devtrail about`

Muestra información de versión, autoría y licencia.

**Ejemplo:**

```bash
$ devtrail about
DevTrail CLI
  CLI version:       cli-1.0.0
  Framework version: fw-2.1.0
  Author:            Strange Days Tech, S.A.S.
  License:           MIT
  Repository:        https://github.com/StrangeDaysTech/devtrail
  Website:           https://strangedays.tech
```

---

## Variables de Entorno

| Variable | Descripción |
|----------|-------------|
| `GITHUB_TOKEN` | Token de acceso personal de GitHub para solicitudes autenticadas a la API. Útil para evitar límites de tasa al descargar releases. |

---

## Códigos de Salida

| Código | Significado |
|--------|-------------|
| `0` | Éxito |
| `1` | Error (detalles impresos en stderr) |

---

<div align="center">

**DevTrail** — Porque cada cambio cuenta una historia.

[Volver a docs](../../README.md) • [README](../README.md) • [Strange Days Tech](https://strangedays.tech)

</div>
