# Gestión de cartera de proyectos

*Un registro del trabajo financiado, con detección de duplicados y un espacio de trabajo para la gestión de proyectos.*

Project Portfolio Management es software de código abierto para registrar los planes que financia una organización y gestionar el trabajo que contienen. Abarca tareas, calendarios, costes, recursos, riesgos y gobernanza. Compara los planes para identificar posibles duplicados y muestra la evidencia de cada coincidencia. Cuando una cifra no se puede calcular, el software indica el motivo.

## Principales beneficios

- **Un único registro de la cartera.** Las carteras, programas, proyectos y productos usan un solo tipo de registro, y cualquier plan puede contener a otro. Los informes se basan en una sola lista de todos los departamentos.
- **Detección de duplicados.** La comparación contrasta planes registrados con nombres distintos y muestra la evidencia de cada resultado, para que quienes revisan puedan actuar antes de financiar ambos.
- **Cifras trazables.** Una cifra faltante se muestra junto con el motivo de su ausencia. Un plan sin evaluación se muestra como «sin evaluar».
- **Registro de auditoría.** Cada cambio queda auditado. Quien presenta una solicitud de cambio no puede aprobarla.
- **Control de acceso.** El inicio de sesión está activado de forma predeterminada, y el servicio no se ejecuta en producción con el inicio de sesión desactivado.
- **Despliegue y licencias.** El software se aloja por cuenta propia y se ofrece bajo cinco licencias de código abierto. El trabajo se puede importar desde Jira, Asana y Microsoft Project.

[Ver todos los beneficios](topics/beneficios/index.md)

## Áreas funcionales

- **Planes y coincidencias.** Registro recursivo, detección de duplicados explicable y fusión.
- **Entrega.** Tableros kanban, diagramas de Gantt, calendarios, hitos, ruta crítica y cadena crítica.
- **Finanzas.** Presupuestos, libro de financiamiento, valor ganado, y riesgo de costes y de calendario.
- **Recursos.** Grupos de habilidades, mapa de calor de capacidad y personas clave.
- **Estrategia.** Recepción de trabajo, ideas, objetivos, metas y resultados clave, y escenarios.
- **Gobernanza.** Riesgos, problemas, revisiones de puerta, escalamiento y paquetes mensuales de dirección.

[Ver todas las funciones](topics/funciones/index.md)

## Usuarios previstos

- **Oficinas de cartera.** Informes de programas y proyectos.
- **Responsables de proyectos y de entrega.** Gestión de calendario, costes y riesgos.
- **Integrantes de equipos.** Una vista personal del trabajo asignado.
- **Altos directivos.** Las [fichas por función](for/) exponen lo que el software ofrece a cada función, sus límites y las decisiones que corresponden.

## Estado actual

El software está en una etapa temprana. Tiene una sola persona a cargo del mantenimiento, ninguna auditoría de terceros y ningún acuerdo de soporte. Lo que exigiría el uso en producción, punto por punto y con una estimación para cada uno, está en la página [camino a producción](path-to-production/index.md). Las limitaciones conocidas figuran en la [página de comparaciones](topics/comparaciones/index.md).

## Demostración

Una demostración local usa una autoridad de transporte ficticia y requiere Git y Podman. Los pasos están en el [tutorial](topics/tutorial/index.md).
