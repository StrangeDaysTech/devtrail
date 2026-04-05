# DevTrail - Flujos de Trabajo Recomendados

**Patrones y cadencias para usar DevTrail en el día a día.**

[![Strange Days Tech](https://img.shields.io/badge/by-Strange_Days_Tech-purple.svg)](https://strangedays.tech)

**Idiomas**: [English](../../../adopters/WORKFLOWS.md) | Español | [简体中文](../../zh-CN/adopters/WORKFLOWS.md)

---

## Tabla de Contenidos

1. [Después de la Configuración Inicial](#después-de-la-configuración-inicial)
2. [Desarrollo Diario](#desarrollo-diario)
3. [Mantener DevTrail Actualizado](#mantener-devtrail-actualizado)
4. [Verificar el Estado del Proyecto](#verificar-el-estado-del-proyecto)
5. [Usar Skills (Documentación Activa)](#usar-skills-documentación-activa)
6. [Patrones de Equipo](#patrones-de-equipo)
7. [Entender las Versiones](#entender-las-versiones)

---

## Después de la Configuración Inicial

Ejecutaste `devtrail init .` e hiciste commit del resultado. ¿Ahora qué?

1. **Abre tu proyecto** con tu asistente de codificación IA (Claude Code, Cursor, Gemini CLI, etc.)
2. El asistente **leerá automáticamente** las directivas de DevTrail (`CLAUDE.md`, `GEMINI.md`, etc.)
3. A partir de este punto, el asistente **crea documentación** en `.devtrail/` como parte de su flujo de trabajo normal
4. **No se necesita configuración adicional** — DevTrail funciona de forma pasiva a través de los archivos de directivas

---

## Desarrollo Diario

### El Ciclo Pasivo

1. Trabaja normalmente con tu asistente IA — escribe features, corrige bugs, refactoriza
2. La IA crea documentos en `.devtrail/` según las reglas de gobernanza:
   - **AILOG** para implementaciones significativas (>10 líneas cambiadas)
   - **AIDEC** al elegir entre alternativas
   - **ADR** para decisiones arquitectónicas
   - **ETH** cuando surgen preocupaciones éticas
3. Revisa los documentos marcados con `review_required: true`
4. Haz commit de la documentación junto con los cambios de código correspondientes

### Cuándo Crear Documentos Manualmente

Usa el sistema activo (skills) cuando:

- La IA omitió documentar un cambio significativo
- Tú (un humano) tomaste una decisión que debería registrarse
- Quieres crear un documento REQ, TES, TDE o INC
- Quieres verificar el cumplimiento de documentación

---

## Mantener DevTrail Actualizado

### Cadencia Recomendada

- **Mensualmente** o cuando veas un nuevo release en GitHub
- Consulta la [página de releases](https://github.com/StrangeDaysTech/devtrail/releases) para changelogs

### Comandos de Actualización

| Objetivo | Comando |
|----------|---------|
| Actualizar framework y CLI | `devtrail update` |
| Actualizar solo plantillas y docs de gobernanza | `devtrail update-framework` |
| Actualizar solo el binario CLI | `devtrail update-cli` |

Framework y CLI tienen **versiones independientes** — puedes actualizar uno sin el otro. Ver [Entender las Versiones](#entender-las-versiones).

### Después de Actualizar

1. Revisa los cambios en archivos de directivas y docs de gobernanza
2. Haz commit de los archivos actualizados: `git add .devtrail/ && git commit -m "chore: update DevTrail framework"`
3. Si personalizaste archivos del framework, verifica si hay conflictos

---

## Verificar el Estado del Proyecto

### Estado via CLI

```bash
devtrail status
```

Muestra: versión del framework, versión del CLI, integridad de la estructura de directorios y estadísticas de documentos por tipo. Úsalo para verificar que la instalación está saludable.

### Cumplimiento de Documentación (Skill)

```bash
/devtrail-status
```

El skill `/devtrail-status` (disponible en Claude Code y Gemini CLI) analiza:

- Qué cambios de código recientes carecen de documentación correspondiente
- Cumplimiento de documentos contra las reglas de gobernanza
- Estado general de documentación

---

## Usar Skills (Documentación Activa)

DevTrail tiene dos sistemas de documentación:

| Sistema | Cómo funciona | Cuándo usar |
|---------|---------------|-------------|
| **Pasivo** | La IA auto-documenta via archivos de directivas | Por defecto — sucede automáticamente |
| **Activo** | El usuario invoca skills para crear docs | Cuando el pasivo omitió algo, o para decisiones humanas |

### Skills Disponibles

| Skill | Propósito |
|-------|-----------|
| `/devtrail-status` | Verificar cumplimiento de documentación |
| `/devtrail-new` | Crear cualquier tipo de documento (sugiere el más adecuado) |
| `/devtrail-ailog` | Creación rápida de AILOG |
| `/devtrail-aidec` | Creación rápida de AIDEC |
| `/devtrail-adr` | Creación rápida de ADR |

Para detalles completos de skills, consulta el [README](../README.md#skills).

---

## Patrones de Equipo

### Revisión de PRs

- Verifica que los cambios de código significativos incluyan documentos correspondientes en `.devtrail/`
- Revisa cualquier documento con `review_required: true`
- Verifica que los AILOGs describan con precisión lo que hizo la IA

### Onboarding de Nuevos Miembros

1. Apúntalos a `.devtrail/QUICK-REFERENCE.md` para una vista rápida
2. Pídeles que lean los ADRs recientes para entender el contexto arquitectónico
3. Muéstrales AILOGs de features recientes para ver cómo funciona la documentación en la práctica

### Retrospectivas de Sprint

- Revisa AILOGs y AIDECs del sprint para entender patrones de contribución de la IA
- Identifica decisiones no documentadas que deberían haberse registrado
- Revisa documentos TDE para deuda técnica acumulada

### Uso Compartido de Asistentes IA

Cuando múltiples miembros del equipo usan asistentes IA en el mismo proyecto:

- Cada sesión de asistente produce sus propios documentos
- El campo `agent` en los metadatos identifica qué asistente creó cada documento
- Revisa AIDECs superpuestos o contradictorios durante la revisión de PRs

---

## Entender las Versiones

DevTrail usa **versionado independiente** para sus dos componentes:

| Componente | Prefijo de tag | Contiene | Se actualiza con |
|------------|---------------|----------|-----------------|
| **Framework** | `fw-` | Plantillas, docs de gobernanza, directivas, scripts | `devtrail update-framework` |
| **CLI** | `cli-` | El binario `devtrail` | `devtrail update-cli` |

### ¿Por Qué Versiones Independientes?

- Los cambios de framework (nuevas plantillas, reglas actualizadas) son más frecuentes
- Los cambios de CLI (nuevos comandos, corrección de bugs) siguen una cadencia diferente
- Puedes actualizar docs de gobernanza sin necesitar un nuevo binario del CLI

### Verificar Tus Versiones

```bash
devtrail about     # Verificación rápida de versiones
devtrail status    # Reporte completo de salud incluyendo versiones
```

Para información detallada del CLI, consulta la [Referencia CLI](CLI-REFERENCE.md#versionado).

---

<div align="center">

**DevTrail** — Porque cada cambio cuenta una historia.

[Volver a docs](../../README.md) • [README](../README.md) • [Strange Days Tech](https://strangedays.tech)

</div>
