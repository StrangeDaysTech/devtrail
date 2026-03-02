# Contribuir a DevTrail

¡Gracias por tu interés en contribuir a DevTrail! Este documento proporciona guías e información para contribuidores.

**Idiomas**: [English](../../../CONTRIBUTING.md) | Español

## Tabla de Contenidos

- [Código de Conducta](#código-de-conducta)
- [¿Cómo Puedo Contribuir?](#cómo-puedo-contribuir)
- [Configuración de Desarrollo](#configuración-de-desarrollo)
- [Proceso de Pull Request](#proceso-de-pull-request)
- [Guías de Estilo](#guías-de-estilo)
- [Estándares de Documentación](#estándares-de-documentación)

---

## Código de Conducta

Este proyecto se adhiere a un Código de Conducta que todos los contribuidores deben seguir. Por favor sé respetuoso, inclusivo y constructivo en todas las interacciones.

### Nuestros Estándares

- Sé acogedor e inclusivo
- Sé respetuoso con diferentes puntos de vista
- Acepta críticas constructivas con gracia
- Enfócate en lo mejor para la comunidad
- Muestra empatía hacia otros miembros de la comunidad

---

## ¿Cómo Puedo Contribuir?

### Reportar Bugs

Antes de crear un reporte de bug, por favor revisa los issues existentes para evitar duplicados.

**Al reportar un bug, incluye:**

- Un título claro y descriptivo
- Pasos para reproducir el comportamiento
- Comportamiento esperado
- Comportamiento real
- Capturas de pantalla (si aplica)
- Detalles del entorno (SO, plataforma de IA, etc.)

### Sugerir Características

¡Las sugerencias de características son bienvenidas! Por favor incluye:

- Una descripción clara de la característica
- El problema que resuelve
- Posibles enfoques de implementación
- Alternativas que hayas considerado

### Mejorar Documentación

Las mejoras de documentación son muy valoradas:

- Corregir erratas o redacción poco clara
- Agregar ejemplos
- Mejorar explicaciones
- Traducir a otros idiomas

### Enviar Código

Las contribuciones de código deben:

- Corregir un bug o implementar una característica
- Incluir pruebas apropiadas (si aplica)
- Seguir las guías de estilo del proyecto
- Actualizar documentación según sea necesario

---

## Configuración de Desarrollo

### Prerrequisitos

- Git
- Un editor de texto (VS Code recomendado)
- Bash (para scripts de validación en Linux/Mac)
- PowerShell (para scripts de validación en Windows)
- Node.js (opcional, para markdownlint)

### Pasos de Configuración

1. **Fork del repositorio**

   Haz clic en "Fork" en la página del repositorio de GitHub.

2. **Clonar tu fork**
   ```bash
   git clone https://github.com/tu-usuario/devtrail.git
   cd devtrail-framework
   ```

3. **Crear una rama**
   ```bash
   git checkout -b feature/nombre-de-tu-caracteristica
   # o
   git checkout -b fix/tu-correccion-de-bug
   ```

4. **Instalar herramientas de desarrollo (opcional)**
   ```bash
   npm install -g markdownlint-cli
   ```

5. **Hacer tus cambios**

6. **Validar tus cambios**
   ```bash
   # Linux/Mac
   bash scripts/pre-commit-docs.sh

   # Windows
   .\scripts\validate-docs.ps1
   ```

---

## Proceso de Pull Request

### Antes de Enviar

- [ ] Ejecutar scripts de validación exitosamente
- [ ] Actualizar documentación si es necesario
- [ ] Agregarte a CONTRIBUTORS.md (si aplica)
- [ ] Escribir una descripción clara del PR

### Formato de Título del PR

Usa formato de commit convencional:

```
tipo(alcance): descripción

Ejemplos:
feat(templates): agregar plantilla para revisiones de seguridad
fix(validation): corregir regex para nomenclatura de archivos
docs(readme): clarificar pasos de instalación
chore(ci): actualizar flujo de trabajo de GitHub Actions
```

**Tipos:**
- `feat` - Nueva característica
- `fix` - Corrección de bug
- `docs` - Cambios de documentación
- `chore` - Tareas de mantenimiento
- `refactor` - Refactorización de código
- `test` - Adiciones o correcciones de pruebas

### Plantilla de Descripción del PR

```markdown
## Resumen
Breve descripción de los cambios

## Motivación
¿Por qué se necesita este cambio?

## Cambios
- Cambio 1
- Cambio 2

## Pruebas
¿Cómo se probaron estos cambios?

## Lista de Verificación
- [ ] Scripts de validación pasan
- [ ] Documentación actualizada
- [ ] Sin información sensible incluida
```

### Proceso de Revisión

1. Un mantenedor revisará tu PR
2. Atiende cualquier cambio solicitado
3. Una vez aprobado, un mantenedor hará merge

---

## Guías de Estilo

### Markdown

- Usa encabezados estilo ATX (`#`, `##`, etc.)
- Usa bloques de código cercados con identificadores de lenguaje
- Usa tablas para datos estructurados
- Mantén líneas bajo 120 caracteres cuando sea práctico
- Usa líneas en blanco para separar secciones

### Front-matter YAML

```yaml
---
id: TYPE-YYYY-MM-DD-NNN
title: Título claro y descriptivo
status: draft | accepted | deprecated
created: YYYY-MM-DD
# Campos adicionales según sea necesario
---
```

### Nomenclatura de Archivos

Documentos DevTrail:
```
[TIPO]-[YYYY-MM-DD]-[NNN]-[descripcion].md
```

- Usa minúsculas para la descripción
- Usa guiones para separar palabras
- Mantén las descripciones concisas pero claras

### Código en Scripts

- Usa nombres de variables claros
- Agrega comentarios para lógica compleja
- Sigue las mejores prácticas de shell/PowerShell

---

## Estándares de Documentación

### Agregar un Nuevo Tipo de Documento

Si estás proponiendo un nuevo tipo de documento:

1. **Crear la plantilla**
   - Agregar `TEMPLATE-NUEVOTIPO.md` a `.devtrail/templates/`
   - Seguir patrones de plantillas existentes

2. **Actualizar docs de gobernanza**
   - `.devtrail/00-governance/DOCUMENTATION-POLICY.md`
   - `.devtrail/00-governance/AGENT-RULES.md`
   - `.devtrail/QUICK-REFERENCE.md`

3. **Actualizar configs de agente**
   - `CLAUDE.md`
   - `GEMINI.md`
   - `.cursorrules`
   - `.github/copilot-instructions.md`

4. **Actualizar validación**
   - `scripts/pre-commit-docs.sh`
   - `scripts/validate-docs.ps1`
   - `.github/workflows/docs-validation.yml`

5. **Documentar el cambio**
   - Crear un ADR explicando el nuevo tipo
   - Actualizar el README si es necesario

### Escribir Plantillas

Las plantillas deben incluir:

- Front-matter YAML completo con todos los campos
- Encabezados de sección claros
- Texto placeholder explicando qué va en cada sección
- Ejemplos donde sea útil

### Escribir Documentos de Gobernanza

- Sé claro y sin ambigüedades
- Usa tablas para información de referencia
- Incluye ejemplos
- Mantén las reglas accionables

---

## Desarrollo del CLI

El CLI de DevTrail está escrito en Rust y se encuentra en el directorio `cli/`.

### Compilar

```bash
cd cli
cargo build
```

### Ejecutar Tests

```bash
cd cli
cargo test
```

### Build de Release

```bash
cd cli
cargo build --release
```

---

## ¿Preguntas?

Si tienes preguntas sobre contribuir:

1. Revisa [Issues](https://github.com/StrangeDaysTech/devtrail/issues) existentes
2. Revisa [Discussions](https://github.com/StrangeDaysTech/devtrail/discussions)
3. Abre una nueva Discussion para preguntas generales
4. Abre un Issue para bugs o características específicas

---

## Reconocimiento

Los contribuidores son reconocidos en:

- Gráfico de contribuidores de GitHub
- Notas de release para contribuciones significativas
- CONTRIBUTORS.md (para contribuidores recurrentes)

¡Gracias por ayudar a mejorar DevTrail!

---

*DevTrail — Porque cada cambio cuenta una historia.*

[Strange Days Tech](https://strangedays.tech)
