# Para la Dirección de Sistemas de Información (CIO)

*Dirección de Sistemas de Información (CIO): integración, identidad, datos, alojamiento y la información que le pedirán entregar.*

## Identidad y acceso

- El inicio de sesión se **exige de forma predeterminada**; el servicio se niega a arrancar sin las llaves de verificación de tokens, y en producción con el inicio de sesión desactivado.
- Acceso mediante un proveedor de identidad (incluido Microsoft Entra ID); la autenticación multifactor queda en manos del proveedor ([guía de inicio de sesión](../../../../operations/sign-in-with-entra.md)).
- La autorización usa atributos del token (por ejemplo `access`, `directorpack`, `confidence`) con una política de atributos. Los planes privados se ocultan de todo agregado; las marcas de manejo ocultan los planes por encima de la autorización de quien consulta, administradores incluidos.
- El acceso de emergencia (break-glass) a planes privados requiere elegibilidad, un motivo declarado, un máximo de cuatro horas y que se avise a una segunda persona.
- Los administradores pueden listar los accesos permanentes concedidos (una ayuda para la recertificación) y exportar o seudonimizar los datos de una persona.

## Datos e integración

| Tema | Postura |
|---|---|
| Datos conservados | Registros de planes, tareas, riesgos, presupuestos, hitos, tiempo y costos; las personas son referencias opacas, no nombres. Sin datos de pacientes ni datos de categorías especiales por diseño |
| Inventario | Cada columna de datos personales figura en el [inventario de datos personales](../../../../governance/personal-data-inventory.csv); una prueba lo mantiene fiel al esquema |
| Interfaces | REST con un documento OpenAPI derivado; importación y exportación masivas; canales de calendario; eventos salientes y webhooks solo hacia destinos `https` públicos |
| Respaldos | Respaldo con guion y restauración probada, cifrado opcional y objetivos de recuperación declarados ([respaldo y restauración](../../../../operations/backup-and-restore.md)) |
| Alojamiento | Imágenes de contenedor y `compose.yaml`; el software no hace transferencias por sí mismo, así que la ubicación de los datos depende de su elección de alojamiento |

## Brechas que debe cerrar

- Prueba de penetración, revisión independiente del código y certificación: ninguna realizada.
- La retención se aplica solo en parte; la depuración debe programarse (véase el [calendario de retención](../../../../governance/retention-schedule.md)).
- La herramienta para titulares de datos encuentra referencias, no nombres escritos en texto libre ni en archivos cargados.
- El registro de auditoría no resiste la manipulación por parte de un administrador de la base de datos.

## Lecturas siguientes

[Diagrama de flujo de datos](../../../../governance/data-flow.md) · [Lista de verificación de aseguramiento](../../../../governance/assurance-checklist.md) · [Despliegue](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
