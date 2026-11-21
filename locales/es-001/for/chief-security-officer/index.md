# Para la Dirección de Seguridad (CSO)

*Dirección de Seguridad (CSO): lo que el software aplica, lo que el proyecto ha verificado y lo que usted debe probar y operar por su cuenta.*

## Lo que el software aplica

| Área | Comportamiento |
|---|---|
| Autenticación | El inicio de sesión está activado de forma predeterminada. El servicio no arranca sin llaves de verificación de tokens, ni en producción con el inicio de sesión desactivado. Acceso mediante un proveedor de identidad como Microsoft Entra ID; la autenticación multifactor queda en manos del proveedor ([guía de inicio de sesión](../../../../operations/sign-in-with-entra.md)) |
| Autorización | Política basada en atributos de los tokens. Los planes privados quedan fuera de todo agregado para todas las personas; las marcas de manejo ocultan los planes por encima de la autorización de quien consulta, administradores incluidos |
| Acceso de emergencia (break-glass) | El acceso a planes privados requiere elegibilidad, un motivo declarado, un máximo de cuatro horas y que se avise a una segunda persona; cada solicitud queda auditada |
| Sesiones | Tiempo de espera por inactividad (30 minutos de forma predeterminada) y un límite máximo de sesión |
| Transporte y navegador | Encabezados de seguridad, una política estricta de seguridad de contenido, una lista de orígenes permitidos para CORS, limitación de tasa por quien consulta |
| Salientes | Los destinos indicados por los usuarios se restringen a direcciones `https` públicas |
| Cargas | El contenido debe coincidir con el tipo declarado; se rechazan los programas; un análisis opcional con `clamd` falla de forma segura |
| Auditoría | El registro de auditoría rechaza la modificación y la eliminación en la base de datos |
| Registros congelados | Los paquetes de dirección emitidos y las evaluaciones de confianza de entrega son inmutables o de solo anexado mediante un disparador de la base de datos |
| Cadena de suministro | `cargo deny`, una lista de materiales de software, procedencia de la compilación en cada versión, una [política de seguridad](../../../../SECURITY.md) |

## Lo que no está hecho

- Ninguna prueba de penetración, auditoría de terceros, revisión independiente del código ni certificación.
- Ningún analizador de malware incluido; el enlace con `clamd` solo se probó contra un sustituto.
- Un administrador de la base de datos puede vulnerar el registro de auditoría; no hay cadena de hashes.
- Las marcas de manejo ocultan y etiquetan; no cifran, y la marca no se estampa en los archivos generados.
- Las sesiones se conservan en memoria salvo que se configure Redis.
- La herramienta para titulares de datos encuentra referencias, no nombres escritos en texto libre, en archivos cargados ni en respaldos.

## Operación

Los respaldos tienen una restauración probada, cifrado opcional y objetivos de recuperación declarados ([paquete de operaciones](../../../../operations/README.md)). El [manual de operación](../../../../operations/runbook.md) da los pasos técnicos ante incidentes; el proceso de incidentes y de filtraciones es suyo. Reporte una vulnerabilidad de forma privada como describe [SECURITY.md](../../../../SECURITY.md); el objetivo es acusar recibo en 5 días hábiles, no una garantía.

## Decisiones que le corresponden

Quién realiza la prueba de penetración y cuándo; el acceso condicional y la asignación de roles del proveedor de identidad; el nivel de marcado para el que se asegura este despliegue; el analizador detrás del enlace de archivos adjuntos; con qué frecuencia se recertifican los accesos.

## Lecturas siguientes

[Lista de verificación de aseguramiento](../../../../governance/assurance-checklist.md) · [Camino a producción](../../path-to-production/index.md) · [Flujo de datos](../../../../governance/data-flow.md)
