# Para la Dirección de Riesgos (CRO)

*Dirección de Riesgos (CRO): los riesgos que el software le ayuda a ver y los riesgos de usarlo.*

## Riesgos que le ayuda a gestionar

- **Riesgos, problemas y solicitudes de cambio** registrados por plan; un registro de problemas de todos los planes legibles; solicitudes de cambio con separación de funciones.
- **Confianza de entrega y excepciones:** la confianza declarada se mantiene separada del RAG calculado y se señala cuando divergen en dos niveles, con un historial de solo anexado, y un reloj de excepción de 48 horas que no se reinicia cuando empeora una calificación y que solo se cierra con una decisión registrada.
- **Ausencia honesta:** los datos faltantes se muestran como «sin evaluar» o como nulos con un motivo, nunca como verde o cero.
- **Un paquete mensual congelado y versionado,** de modo que lo que se informó a la dirección pueda mostrarse después exactamente como se emitió.

## Riesgos de usarlo (el registro del propio proyecto)

| Riesgo | Estado |
|---|---|
| Sin prueba de penetración, auditoría de terceros ni certificación | Abierto: corresponde encargarlo a quien compra ([lista de verificación de aseguramiento](../../../../governance/assurance-checklist.md)) |
| Retención aplicada solo a una parte de los datos | Abierto; la depuración debe programarse |
| La herramienta para titulares de datos no busca en texto libre, archivos adjuntos ni copias de seguridad | Abierto |
| Un administrador de la base de datos puede vulnerar el registro de auditoría; no hay cadena de hashes | Declarado en la [DPIA](../../../../governance/dpia.md) |
| Las marcas de manejo ocultan y etiquetan, no cifran | Declarado |
| El análisis de archivos adjuntos requiere un analizador que ejecute quien despliega | Abierto |
| Una sola persona a cargo del mantenimiento, sin SLA | Abierto |
| Las traducciones son borradores | Abierto |
| Solo no clínico; sin caso de seguridad clínica | No capture contenido clínico |

## Controles que el software aplica

Inicio de sesión de forma predeterminada, limitación de tasa, encabezados de seguridad estrictos, un tiempo de espera de sesión por inactividad (30 minutos) y un límite máximo de sesión, destinos salientes restringidos a `https` público, archivos adjuntos con firma verificada, filas de auditoría inmutables y acceso de emergencia (break-glass) con auditoría. Cada uno está cubierto por una prueba citada en [camino a producción](../../path-to-production/index.md).

## Decisiones que le corresponden

El apetito de riesgo para un piloto; la ventana de excepción (48 horas de forma predeterminada); para qué nivel de marcado se asegura el despliegue; quién revisa la lista de accesos y con qué frecuencia.

## Lecturas siguientes

[Camino a producción](../../path-to-production/index.md) · [DPIA](../../../../governance/dpia.md) · [Cumplimiento](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
