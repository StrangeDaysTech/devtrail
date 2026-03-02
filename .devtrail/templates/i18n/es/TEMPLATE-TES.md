---
id: TES-YYYY-MM-DD-NNN
title: [Título del plan de pruebas]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium
review_required: true
risk_level: low | medium
tags: []
related: []
---

# TES: [Título del Plan de Pruebas]

> **PROPUESTA**: Este plan fue creado por un agente de IA y requiere validación.

## Alcance

### Dentro del Alcance
- [Funcionalidad/componente a probar 1]
- [Funcionalidad/componente a probar 2]

### Fuera del Alcance
- [Qué no se probará y por qué]

## Estrategia de Pruebas

### Tipos de Pruebas

| Tipo | Cobertura | Herramienta |
|------|-----------|-------------|
| Unitarias | [%] | [Jest/Vitest/etc] |
| Integración | [%] | [Herramienta] |
| E2E | [Casos críticos] | [Cypress/Playwright/etc] |
| Rendimiento | [Si aplica] | [Herramienta] |

### Enfoque
[Descripción del enfoque general de pruebas]

## Casos de Prueba

### Funcionalidad: [Nombre]

| ID | Caso | Precondiciones | Pasos | Resultado Esperado | Prioridad |
|----|------|----------------|-------|-------------------|-----------|
| TC-001 | [Nombre] | [Setup] | 1. [Paso] | [Esperado] | Alta |
| TC-002 | [Nombre] | [Setup] | 1. [Paso] | [Esperado] | Media |

### Casos Negativos

| ID | Caso | Entrada Inválida | Resultado Esperado |
|----|------|------------------|-------------------|
| TC-N01 | [Nombre] | [Entrada] | [Error esperado] |

### Casos de Borde

| ID | Caso | Condición | Resultado Esperado |
|----|------|-----------|-------------------|
| TC-E01 | [Nombre] | [Condición límite] | [Esperado] |

## Datos de Prueba

### Fixtures Requeridos
- [Fixture 1]: [Descripción]
- [Fixture 2]: [Descripción]

### Mocks Requeridos
- [Mock 1]: [Qué simula]
- [Mock 2]: [Qué simula]

## Criterios de Aceptación

- [ ] Cobertura mínima de [X]%
- [ ] Todos los casos críticos pasan
- [ ] Sin regresiones en funcionalidad existente
- [ ] Rendimiento dentro de umbrales aceptables

## Riesgos y Mitigaciones

| Riesgo | Probabilidad | Impacto | Mitigación |
|--------|--------------|---------|------------|
| [Riesgo] | [Alta/Media/Baja] | [Alto/Medio/Bajo] | [Acción] |

## Entorno de Pruebas

- **Entorno**: [Local/CI/Staging]
- **Configuración especial**: [Si aplica]
- **Dependencias externas**: [Lista]

---

## Validación

| Campo | Valor |
|-------|-------|
| Validado por | [Nombre] |
| Fecha | [YYYY-MM-DD] |
| Comentarios | [Notas] |

<!-- Template: DevTrail | https://strangedays.tech -->
