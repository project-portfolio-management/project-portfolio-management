# Comparaciones

*Un resumen de cómo se relaciona el software con otros productos.*

Los productos siguientes sirven a fines distintos y no se califican. Estas notas se basan en el material público de cada proyecto. **No se instaló ni se ejecutó ningún otro producto**, y cada uno puede haber cambiado desde entonces. Antes de tomar una decisión, conviene revisar la documentación vigente del producto.

| Proyecto | Qué es | En qué se diferencia este proyecto |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Gestión de proyectos para personas que no son gerentes de proyectos | Este proyecto incluye además las funciones de calendarización, costos y gobernanza que usa una oficina de cartera |
| [Redmine](https://www.redmine.org/) | Un gestor de propósito general configurable, con tipos de problema, estados y roles definidos por la organización | Los tipos de problema, los estados y los flujos de trabajo los define la organización. Los roles llevan permisos sobre los problemas, asignados plan por plan, dentro de lo que permite el inicio de sesión de la persona. Véase la [comparación con Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (mediante su bifurcación, Platform Collective) | Un marco de aplicaciones empresariales sobre muchos servicios | Huly es una plataforma de varios servicios. Este proyecto es un solo servicio sobre PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Una alternativa de escritorio a Microsoft Project, para una persona y un proyecto | Este proyecto es un servicio alojado, con varios usuarios y control de acceso |

## Comparaciones de productos

Páginas de comparación individuales: [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interoperabilidad

- **Jira y Asana:** importación de CSV, con una vista previa y el motivo declarado de todo lo que se deja fuera.
- **Microsoft Project:** importación y exportación de XML MSPDI, que abarca tareas, esquema, fechas, duraciones, restricciones, dependencias y el calendario. Se excluyen las personas asignadas, los recursos y las líneas base, y el archivo lo registra.

## Diferencias de diseño

- **Motivos declarados.** Una cifra faltante se muestra con su motivo, por ejemplo «sin pronóstico, porque…».
- **Detección explicable de duplicados.** La comparación funciona con todos los tipos de plan y muestra su evidencia.
- **Valores seguros predeterminados.** El inicio de sesión está activado de forma predeterminada, y el repositorio incluye material de seguridad.
- **Duraciones explícitas.** Cada duración se indica en días naturales o en días hábiles.
- **Límites registrados.** El pendiente registra los límites de cada elemento.

## Limitaciones conocidas

- Sin aplicación móvil ni de escritorio.
- Sin coedición en tiempo real.
- Una sola persona a cargo del mantenimiento, un historial limitado en producción y ninguna auditoría de terceros.
- La prueba de carga publicada se ejecutó en una sola máquina. Indica un orden de magnitud y no es un plan de capacidad.
- Las traducciones son borradores pendientes de revisión.
- Sin pantalla para redactar o resolver solicitudes de cambio.
- Sin ecosistema de complementos ni mercado.

Hay más detalle en las [notas de comparación](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Las correcciones se pueden enviar como un reporte con una fuente.
