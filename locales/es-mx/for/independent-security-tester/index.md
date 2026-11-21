# Pruebas de seguridad independientes

*Para quien compra, evalúa o prueba: qué debería cubrir una prueba de seguridad independiente de este software, qué aporta el proyecto para ayudar, qué se ha hecho y qué no, y qué queda por su cuenta. Esto es una guía, no un informe de pruebas ni asesoría jurídica.*

## Dónde estamos

**No se ha hecho ninguna prueba de seguridad independiente.** No existe ninguna prueba de penetración, auditoría de terceros, revisión independiente del código ni certificación de este software, y el proyecto no declara ninguna ([camino a producción](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). En la integración continua del proyecto se ejecuta un análisis pasivo (reglas en `.zap/rules.tsv`), y las pruebas del propio proyecto verifican los controles que se enumeran abajo; ninguno de los dos sustituye a quien prueba sin haber escrito el software.

Este es el punto 1 y el punto 9 de la [lista de verificación de aseguramiento](../../../../governance/assurance-checklist.md). Corresponde a quien despliega, con una persona probadora certificada, sobre **su** despliegue: el proyecto no puede probar su alojamiento, su proveedor de identidad, su red ni sus llaves.

## Qué probar

Pruebe el despliegue tal como funcionará, con el inicio de sesión exigido y su proveedor de identidad delante. Alcance sugerido, a partir de los controles que el software declara:

| Área | Qué declara el software | Qué debería intentar quien prueba |
|---|---|---|
| Autenticación | El inicio de sesión está activado de forma predeterminada; el servicio no arranca sin llaves de verificación ni, en producción, con el inicio de sesión desactivado; los tokens se verifican sin conexión contra llaves publicadas | Tokens falsificados, vencidos, reproducidos y con firma incorrecta; emisor o audiencia incorrectos; rotación de llaves; un conjunto de llaves ausente |
| Autorización | Política basada en atributos; reglas a nivel de registro | Escalamiento horizontal y vertical de privilegios en cada ruta; cambiar identificadores en rutas y cuerpos; atributos que quien consulta no debería poder agregar |
| Planes privados y marcas de manejo | Un plan privado, y un plan por encima de la autorización de quien consulta, no aparece para esa persona en ninguna lista, recuento, total, resultado de búsqueda, canal ni exportación, administradores incluidos | Cualquier ruta que revele la existencia, el nombre, el recuento o el total de un plan que quien consulta no puede leer; mensajes de error y tiempos que difieran entre «oculto» y «inexistente» |
| Acceso de emergencia (break-glass) | Solo personas elegibles, un motivo declarado, un máximo de cuatro horas, se avisa a una segunda persona | Usarlo sin elegibilidad; ampliar un permiso; actuar sin que se avise a la segunda persona |
| Registros congelados y de solo anexado | El registro de auditoría, los paquetes de dirección emitidos, el historial de confianza de entrega, el historial de estado de programas y los amortiguadores de proyecto rechazan la modificación y la eliminación en la base de datos | Actualizar o eliminar mediante la API, mediante la importación masiva y mediante cualquier ruta que toque las mismas filas |
| Separación de funciones | Quien presenta una solicitud no puede aprobar su propia solicitud de cambio | Aprobar como quien la presentó, como administrador y cambiando a la persona aprobadora después del envío |
| Cargas y archivos adjuntos | El contenido debe coincidir con el tipo declarado; se rechazan los programas; un enlace opcional de análisis de virus falla de forma segura | Archivos políglotas, programas renombrados, archivos demasiado grandes o mal formados, trucos de rutas en los nombres, y si su analizador realmente se invoca |
| Solicitudes salientes | Los destinos indicados por los usuarios se limitan a direcciones `https` públicas | Falsificación de solicitudes del lado del servidor hacia direcciones privadas, de enlace local y de metadatos, redirecciones, DNS rebinding |
| Sesiones y navegador | Tiempo de espera por inactividad, un límite máximo, una cookie `httpOnly`, una política estricta de seguridad de contenido, una lista de orígenes permitidos para CORS | Fijación y reutilización de sesión, falsificación de solicitudes entre sitios, inyección en la interfaz, clickjacking |
| Limitación de tasa | Límites por quien consulta | Evasión cambiando la identidad, el encabezado o la ruta; si los límites sobreviven a su proxy |
| Herramientas para titulares de datos | Exportación y seudonimización, solo administradores, con vista previa primero | Uso por quien no es administrador; integridad frente al [inventario de datos personales](../../../../governance/personal-data-inventory.csv) |
| Respaldos | Cifrado opcional, una restauración probada | Acceso a los archivos de respaldo y a las llaves; restauración en un entorno limpio |

## Qué aporta el proyecto

- El código fuente, la especificación y las pruebas, para que una revisión de código pueda partir de lo declarado.
- Una descripción legible por máquina de la API (`/api-docs/openapi.json`), con esquemas de solicitud y de respuesta para el trabajo de los paquetes de dirección; las pruebas contrastan la lista de rutas con el código.
- Una configuración desechable con el inicio de sesión desactivado (`dev-insecure`, véase el [README](../../../../README.md)) **solo para la máquina de quien prueba**, y un guion que genera un conjunto de llaves de prueba (`scripts/smoke-keys.sh`) para emitir tokens con los atributos elegidos contra un despliegue de prueba.
- El [diagrama de flujo de datos](../../../../governance/data-flow.md), el [manual de operación](../../../../operations/runbook.md) y los [objetivos de recuperación](../../../../operations/recovery-objectives.md).
- Una vía privada para reportar lo que se encuentre: [SECURITY.md](../../../../SECURITY.md).

## Reglas de participación

Acuérdelas por escrito antes de cualquier prueba; el proyecto no puede fijarlas para su entorno.

- **Entorno.** Pruebe una copia con datos realistas pero no reales. No pruebe producción salvo que lo haya decidido por escrito.
- **Autorización.** Quien prueba tiene su permiso por escrito, una persona de contacto identificada y una ventana de tiempo declarada.
- **Fuera de límites.** La denegación de servicio por volumen, la ingeniería social contra el personal y los servicios de terceros (su proveedor de identidad, su proveedor de alojamiento) quedan fuera del alcance salvo que la parte propietaria de cada uno lo acepte. El alcance del propio proyecto está en [SECURITY](../../../../SECURITY.md).
- **Datos.** No se usan datos personales reales, no se copian hacia afuera ni se incluyen en el informe.
- **Llaves.** Las llaves de prueba son distintas de las de producción y se destruyen después.

## Reporte y nueva prueba

- Los hallazgos sobre este software se envían de forma privada a quien lo mantiene, como indica [SECURITY](../../../../SECURITY.md), con cómo reproducirlos, la versión o el commit y el impacto. Los objetivos del proyecto son acusar recibo en 5 días hábiles y presentar un plan en 15; son objetivos, no garantías. Hay una sola persona a cargo del mantenimiento y ningún acuerdo de nivel de servicio.
- Los hallazgos sobre su despliegue se envían a su propia persona responsable.
- Pida a quien prueba que califique cada hallazgo, que diga qué **no** se probó y que vuelva a probar las correcciones. Un informe que enumera solo lo que se encontró no le dice lo que se pasó por alto.
- Conserve el informe, el alcance, las fechas y la nueva prueba con sus propios registros. El proyecto no puede guardarlos por usted.

## Límites de este documento

Enumera áreas por cubrir, no un plan de pruebas completo, y lo declarado en la tabla es del propio proyecto. No certifica nada, y una prueba limpia es evidencia sobre un despliegue en una fecha.

## Decisiones que le corresponden

Quién prueba, cuándo, contra qué, con qué datos y bajo qué reglas; cómo se califican los hallazgos y para cuándo deben corregirse; y si el resultado se comparte con quienes lo evalúan.

## Lecturas siguientes

[Lista de verificación de aseguramiento](../../../../governance/assurance-checklist.md) · [Camino a producción](../../path-to-production/index.md) · [Dirección de Seguridad (CSO)](../chief-security-officer/index.md) · [Gobernanza, riesgo y cumplimiento](../governance-risk-compliance/index.md)
