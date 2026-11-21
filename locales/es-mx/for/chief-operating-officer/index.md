# Para la Dirección de Operaciones (COO)

*Dirección de Operaciones (COO): lo que usted dirige día a día, lo que el software hace por la entrega y lo que exige operarlo.*

## Qué hace por las operaciones

- **Planeación y control de la entrega:** planes, tareas, dependencias, un cálculo de ruta crítica, hitos entre planes, sprints y fases, burndown y métricas de flujo.
- **Capacidad:** asignaciones, disponibilidad, licencias y periodos no laborables, roles de recursos, carga de trabajo y utilización derivadas. (Un mapa de calor de capacidad por grupo de habilidades y mes para el paquete de dirección no está construido: `DP-10`.)
- **Tiempo y costos:** hojas de tiempo, tiempo registrado, tarifas de costo (restringidas a finanzas), presupuestos y costo derivado.
- **Flujo de gobernanza:** registro de problemas, solicitudes de cambio con separación de funciones (quien las presenta no puede aprobarlas), aprobaciones, revisiones, casos de altas, cambios y bajas de personal.
- **Excepciones:** una confianza de entrega roja o ámbar-rojo abre una excepción que requiere una causa, la decisión necesaria, una persona responsable identificada y una fecha límite de decisión dentro de 48 horas (ventana configurable, de 1 a 720 horas). Los incumplimientos aparecen en el paquete de dirección y en `GET /api/exceptions`.

## Qué exige operarlo

| Necesidad | Detalle |
|---|---|
| Servicio | Un servicio en Rust, PostgreSQL 18, Redis opcional (caché de producción, sesiones compartidas opcionales) y una interfaz SvelteKit |
| Manual de operación | [paquete de operaciones](../../../../operations/README.md): despliegue, manual de operación, respaldo y restauración, objetivos de recuperación, actualización y migración |
| Tareas programadas | Nada se programa solo. La depuración por retención, los avisos de incumplimiento de excepciones y tareas similares deben ponerse en un programador por quienes operan el sistema |
| Soporte | Una sola persona a cargo del mantenimiento, sin SLA; presupueste su propio soporte de primer nivel |

## Conozca los límites

- Calendarizar un plan de varios miles de tareas es lento; la respuesta completa se rechaza por encima de 5,000 tareas salvo que se pida expresamente (`?full=true` o `?only=critical`). Las cifras y salvedades están en [`BENCHMARKS.md`](../../../../BENCHMARKS.md); son un orden de magnitud obtenido en una sola máquina, no capacidad.
- Existen pantallas para gran parte del espacio de trabajo, pero **todavía no** para solicitudes de cambio, problemas, paquetes ni confianza de entrega; hoy solo están disponibles por la API.

## Decisiones que le corresponden

Quién es responsable de cada excepción, quién emite el paquete mensual (el ritmo y los recordatorios son `DP-12`, no construido) y qué cadencia de revisión alimenta la regla de excepciones.

## Lecturas siguientes

[Paquete de operaciones](../../../../operations/README.md) · [Manual de operación](../../../../operations/runbook.md) · [Objetivos de recuperación](../../../../operations/recovery-objectives.md)
