# Reglas para Agentes IA - DevTrail

> Este documento define las reglas que todos los agentes de IA deben seguir cuando trabajan en proyectos bajo DevTrail.

**Idiomas**: [English](../../AGENT-RULES.md) | Español

---

## 1. Identificación Obligatoria

### Al Iniciar una Sesión

Cada agente debe identificarse con:
- Nombre del agente (ej.: `claude-code-v1.0`, `cursor-v1.0`, `gemini-cli-v1.0`)
- Versión del agente si está disponible

### En Cada Documento

Incluir en el frontmatter:
```yaml
agent: agent-name-v1.0
confidence: high | medium | low
```

---

## 2. Cuándo Documentar

### OBLIGATORIO - Crear documento

| Situación | Tipo | Notas |
|-----------|------|-------|
| >10 líneas de código en lógica de negocio | AILOG | - |
| Decisión entre 2+ alternativas técnicas | AIDEC | Documentar alternativas |
| Cambios en seguridad/autenticación | AILOG | `risk_level: high` |
| Manejo de datos personales (PII/GDPR) | AILOG + ETH | ETH requiere aprobación |
| Integración con servicio externo | AILOG | - |
| Cambio en API pública | AILOG | `risk_level: medium+` |
| Cambio en esquema de base de datos | AILOG | `risk_level: medium+` |

### PROHIBIDO - No documentar

- Credenciales, tokens, API keys
- Información personal identificable
- Secretos de cualquier tipo

### OPCIONAL - No requiere documento

- Cambios de formato (espacios, indentación)
- Correcciones de erratas
- Comentarios de código
- Cambios menores de estilo

---

## 3. Límites de Autonomía

### Crear Libremente

| Tipo | Descripción |
|------|-------------|
| AILOG | Logs de acciones realizadas |
| AIDEC | Decisiones técnicas tomadas |

### Crear Borrador → Requiere Aprobación Humana

| Tipo | Descripción |
|------|-------------|
| ETH | Revisiones éticas |
| ADR | Decisiones arquitectónicas |

### Proponer → Requiere Validación Humana

| Tipo | Descripción |
|------|-------------|
| REQ | Requisitos del sistema |
| TES | Planes de prueba |

### Solo Identificar → Humano Prioriza

| Tipo | Descripción |
|------|-------------|
| TDE | Deuda técnica |
| INC | Conclusiones de incidentes |

---

## 4. Cuándo Solicitar Revisión Humana

Marcar `review_required: true` cuando:

1. **Baja confianza**: `confidence: low`
2. **Alto riesgo**: `risk_level: high | critical`
3. **Decisiones de seguridad**: Cualquier cambio en auth/authz
4. **Cambios irreversibles**: Migraciones, eliminaciones
5. **Impacto en usuarios**: Cambios que afectan UX
6. **Preocupaciones éticas**: Privacidad, sesgo, accesibilidad

---

## 5. Formato de Documentos

### Usar Plantillas

Antes de crear un documento, cargar la plantilla correspondiente:

```
.devtrail/templates/TEMPLATE-[TIPO].md
```

### Convención de Nomenclatura

```
[TIPO]-[YYYY-MM-DD]-[NNN]-[descripcion].md
```

### Ubicación

| Tipo | Carpeta |
|------|---------|
| AILOG | `.devtrail/07-ai-audit/agent-logs/` |
| AIDEC | `.devtrail/07-ai-audit/decisions/` |
| ETH | `.devtrail/07-ai-audit/ethical-reviews/` |
| ADR | `.devtrail/02-design/decisions/` |
| REQ | `.devtrail/01-requirements/` |
| TES | `.devtrail/04-testing/` |
| INC | `.devtrail/05-operations/incidents/` |
| TDE | `.devtrail/06-evolution/technical-debt/` |

---

## 6. Comunicación con Humanos

### Ser Transparente

- Explicar el razonamiento detrás de las decisiones
- Documentar alternativas consideradas
- Admitir incertidumbre cuando existe

### Ser Conciso

- Ir al grano
- Evitar jerga innecesaria
- Usar listas y tablas cuando sea apropiado

### Ser Proactivo

- Identificar riesgos potenciales
- Sugerir mejoras cuando sean evidentes
- Alertar sobre deuda técnica

---

## 7. Manejo de Errores

Si el agente comete un error:

1. **Documentar** el error en un AILOG
2. **Explicar** qué salió mal
3. **Proponer** corrección
4. **Marcar** `review_required: true`

---

## 8. Actualización de Documentos

### Crear Nuevo vs Actualizar

| Situación | Acción |
|-----------|--------|
| Corrección menor | Actualizar documento existente |
| Cambio significativo | Crear nuevo documento |
| Documento obsoleto | Marcar como `deprecated` |
| Reemplazo completo | Crear nuevo + marcar anterior como `superseded` |

### Al Actualizar

- Actualizar el campo `updated` en frontmatter
- Agregar nota en la sección de historial si existe
- Mantener consistencia con documentos relacionados

---

*DevTrail v1.0.0 | [Strange Days Tech](https://strangedays.tech)*
