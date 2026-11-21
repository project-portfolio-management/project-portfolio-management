# Gobernanza, riesgo y cumplimiento

*Para quienes dirigen la gobernanza, el riesgo y el cumplimiento: lo que el software le ayuda a evidenciar, lo que aplica y lo que usted debe aportar. No es asesoría jurídica.*

## Gobernanza

- **Las decisiones quedan registradas.** Las solicitudes de cambio requieren una persona aprobadora identificada que no sea quien las presentó, un motivo declarado de al menos 10 caracteres, y se auditan por actor; la base de datos rechaza una aprobación sin quien decide ni motivo.
- **Un paquete mensual de dirección congelado.** Se redacta una vez, lo emite una persona, es inmutable una vez emitido; las correcciones son una reemisión con una nota. El acceso de lectura es por destinatarios.
- **Confianza de entrega y excepciones.** La confianza declarada es de solo anexado y se muestra junto al estado calculado, nunca mezclada con él. Una calificación roja o ámbar-rojo abre una excepción con un reloj de 48 horas (configurable) que solo cierra una decisión registrada.
- **Gobernanza del proyecto del propio software:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [control de cambios](../../../../spec/18-change-control.md), [declaración sobre IA](../../../../AI_STATEMENT.md).

## Riesgo

- Riesgos, problemas y solicitudes de cambio por plan; un registro de problemas de todos los planes legibles.
- Los datos faltantes se muestran como «sin evaluar», nunca como verde.
- Los riesgos abiertos del propio proyecto (sin prueba de penetración, retención parcial, una sola persona a cargo del mantenimiento, traducciones en borrador) están en la página [camino a producción](../../path-to-production/index.md), punto por punto y con una estimación para cada uno.

## Cumplimiento

| Área | Dónde | Estado |
|---|---|---|
| Protección de datos (la ley aplicable; la [§12.2a](../../../../spec/12-compliance.md) asigna GDPR, UK GDPR, LGPD, PIPL, DPDP, PIPEDA, CCPA y otras) | [DPIA](../../../../governance/dpia.md), [registro de tratamiento](../../../../governance/record-of-processing.md), [inventario de datos personales](../../../../governance/personal-data-inventory.csv), [calendario de retención](../../../../governance/retention-schedule.md), [flujo de datos](../../../../governance/data-flow.md) | Prellenado; las marcas `‹buyer›` le corresponden; retención aplicada solo en parte |
| DTAC del NHS del Reino Unido (un esquema nacional, un ejemplo desarrollado; otros esquemas parten de la [lista de verificación de aseguramiento](../../../../governance/assurance-checklist.md)) | [Lista de verificación DTAC](../../../../governance/dtac-checklist.md) | Respondida sección por sección; filas de comprador y proveedor abiertas |
| Accesibilidad | [declaración](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA automatizada; sin prueba manual |
| Obligaciones lingüísticas | [declaración sobre idiomas](../../../../governance/language-statement.md) | Las traducciones son borradores |
| ISO/IEC 27001 | [especificación de cumplimiento](../../../../spec/12-compliance.md) | Controles del lado del despliegue; no certificado |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [especificación de cumplimiento](../../../../spec/12-compliance.md) §12.4 | No aplican (sin datos de pacientes); declarado, no fabricado |

## Evidencia y aseguramiento

La [lista de verificación de aseguramiento](../../../../governance/assurance-checklist.md) enumera 14 puntos que pedirá una persona evaluadora, con responsable y estado. La mayoría están abiertos y corresponden a quien despliega: prueba de penetración, auditoría de accesibilidad, ejercicio de recuperación ante desastres, aseguramiento del alojamiento, configuración del proveedor de identidad, recertificación de accesos, proceso de incidentes, gestión de documentos. El registro de auditoría rechaza la modificación y la eliminación en la base de datos, pero un administrador de la base de datos puede vulnerarlo; no hay cadena de hashes.

## Decisiones que le corresponden

Base jurídica y plazos de retención; ventana de excepciones; nivel de marcado del despliegue; quién revisa los accesos y con qué frecuencia; si hace falta un procedimiento de conservación por motivos legales (no hay ninguno construido).

## Lecturas siguientes

[Índice del paquete de gobernanza](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Especificación de cumplimiento](../../../../spec/12-compliance.md)
