# Para la Dirección de Tecnología (CTO)

*Dirección de Tecnología (CTO): arquitectura, calidad, cómo extenderlo y dónde es débil la ingeniería.*

## Arquitectura

Tres subproyectos, una sola entidad ([arquitectura](../../../../spec/08-architecture.md)):

| Parte | Tecnología |
|---|---|
| Biblioteca de comparación | Rust; comparación por pares determinista y probabilística, explicable; incorporada por el servicio |
| Servicio | Rust, axa sobre axum, sqlx, PostgreSQL 18; REST bajo `/api`, OpenAPI derivado de las rutas |
| Interfaz | SvelteKit 3, SPA de Svelte 5 con un backend-for-frontend; catálogos de i18n |

La cadena de herramientas está fijada en [`rust-toolchain.toml`](../../../../rust-toolchain.toml). La persistencia está en la [especificación 10](../../../../spec/10-persistence.md); la API, en la [especificación 09](../../../../spec/09-api-surface.md).

## Fortalezas de ingeniería que conviene conocer

- **Los invariantes viven en la base de datos, donde importan:** el registro de auditoría rechaza la modificación y la eliminación; los paquetes de dirección emitidos y las evaluaciones de confianza de entrega son inmutables o de solo anexado mediante disparadores; las restricciones CHECK respaldan las reglas del servicio.
- **Pruebas de resguardo:** las pruebas de censo hacen fallar la compilación cuando una nueva ruta de agregación omite el filtro de planes privados, una ruta de lista no está paginada, una columna de datos personales falta en el inventario o una ruta falta en OpenAPI. Se corrigen clasificando lo nuevo, no debilitando la prueba.
- **Pruebas con tokens reales:** la autorización se prueba de extremo a extremo con tokens PASETO firmados (cada prueba en su propio proceso, porque la configuración de autenticación se lee una vez por proceso).
- **Cadena de suministro:** `cargo deny`, una lista de materiales de software, procedencia de la compilación en cada versión, una política de seguridad.
- **La CI** compila y prueba todo en cada push; véase `.github/workflows/`.

## Dónde es débil

- Una sola persona a cargo del mantenimiento, por lo que el factor autobús es uno ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- El rendimiento se conoce solo de forma aproximada; la caché de respuestas cubre solo el punto de acceso de la ruta crítica; los planes muy grandes son lentos.
- Las sesiones están en memoria salvo que se configure Redis.
- Varias secciones del paquete de dirección y todas las pantallas del paquete no están construidas ([`tasks.md`](../../../../tasks.md)).
- Se usan herramientas de IA para construirlo; el producto no contiene ningún modelo. Véase la [declaración sobre IA](../../../../AI_STATEMENT.md).

## Decisiones que le corresponden

El alojamiento y la topología de Redis, si bifurcar el proyecto o adoptar el original, quién revisa el código de forma independiente y cómo encaja el servicio con su proveedor de identidad (conjunto de claves PASETO, u OIDC mediante Keycloak o Entra).

## Lecturas siguientes

[Arquitectura](../../../../spec/08-architecture.md) · [Estrategia de pruebas](../../../../spec/11-testing-strategy.md) · [Estado de la implementación](../../../../spec/14-implementation-status.md) · [Pruebas de rendimiento](../../../../BENCHMARKS.md)
