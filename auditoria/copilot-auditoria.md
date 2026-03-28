# Auditoría de alineación DevTrail

## Alcance y método
- Fuentes revisadas: `README.md`, `docs/README.md`, `docs/adopters/CLI-REFERENCE.md`, `dist/.devtrail/QUICK-REFERENCE.md`, `dist/.devtrail/00-governance/AGENT-RULES.md`, plantillas `TEMPLATE-AILOG.md` y `TEMPLATE-ADR.md`.
- Análisis del CLI (código Rust en `cli/src`) con verificación de comandos, validaciones y chequeos de cumplimiento. Carpeta `auditoria/` excluida.

## Resumen ejecutivo
DevTrail entrega lo que promete: el framework provee plantillas y políticas alineadas con ISO/IEC 42001, EU AI Act y NIST AI RMF, y el CLI implementa los comandos y salidas descritos. Único gap relevante: el comando `analyze-complexity` se menciona en reglas para agentes pero no está expuesto en el CLI (solo existe como librería interna). Riesgo bajo/medio por expectativa incumplida; resto coherente.

## Hallazgos del framework
- Cobertura de 12 tipos de documentos, con nombres y ubicaciones coherentes entre README, Quick Reference y AGENT-RULES.
- Plantillas incluyen metadatos de riesgo (EU AI Act, ISO 42001 clause, NIST GenAI) y guías explícitas de revisión, observabilidad y calidad (ISO 25010). Se alinean con las instrucciones de uso en README/Quick Reference.
- Reglas de autonomía y revisión (AGENT-RULES) coinciden con los niveles indicados en README/Quick Reference. No se detectaron inconsistencias en los triggers de documentación.
- No existen directorios `.agent/`, `.claude/` o `.gemini/` en el repo base; se asume que los crea `devtrail init` al inyectar directivas.

## Hallazgos del CLI
- Comandos implementados: init, update, update-framework, update-cli, remove, status, repair, validate, compliance, metrics, audit, explore (feature `tui` activada por defecto) y about. Salidas y flags coinciden con la referencia de CLI.
- Validación: 13 reglas efectivamente codificadas (naming, metadatos obligatorios, dependencias de riesgo/review, campos específicos por tipo, detección de secretos, observabilidad). Soporta `--fix` para inyectar `review_required` y prefijos.
- Cumplimiento: cheques y scoring para ISO/IEC 42001 (4), EU AI Act (4), NIST AI RMF/600-1 (5) con salidas text/markdown/json y HTML en audit. Evaluación basada en existencia/consistencia, no en calidad de contenido.
- Métricas y auditoría generan tablas, timelines y trazabilidad a partir de `related`; no hay inferencia automática de dependencias fuera de ese campo.
- Gap: función `analyze_complexity` existe en código pero no hay comando ni flag asociado; AGENT-RULES la referencian como opción opcional.

## Desviaciones y riesgos
- `analyze-complexity`: expectativa creada en reglas de agentes pero no invocable por usuarios; puede generar confusión sobre umbrales de documentación.
- Trazabilidad depende de `related`; sin ese campo la cadena REQ→ADR→AILOG→TES no se reconstruye, lo que puede afectar reportes de auditoría.
- Las validaciones de cumplimiento verifican presencia de evidencias, no su suficiencia; riesgo de “falsos positivos” si el contenido es débil.

## Recomendaciones
1) Exponer `analyze-complexity` como comando (`devtrail analyze-complexity --path ... --threshold ...`) o removerlo de AGENT-RULES.  
2) Documentar en README/CLI que la trazabilidad depende de `related` y ofrecer un chequeo adicional para relaciones faltantes.  
3) Considerar validaciones de contenido mínimo (p.ej., campos clave no vacíos en secciones críticas) para reducir riesgo de cumplimiento nominal.  
4) Confirmar generación automática de directorios de skills durante `devtrail init` o añadirlos al dist para alinearse con la sección “Multi-Agent Architecture”.  
