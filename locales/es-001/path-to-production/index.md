# Camino a producción

*Qué significa «producción» para este software y cuánto le falta para llegar, punto por punto.*

Aquí «producción» significa que una organización usa el software para trabajo real. De él dependen planes reales, nombres
de personas reales y decisiones reales; una caída o una fuga tiene consecuencias, y alguien responde por ello. En un gran
organismo de salud o un departamento del gobierno, producción también significa haber superado la evaluación propia de la
organización: una prueba de seguridad, una evaluación de impacto en la protección de datos, una auditoría de
accesibilidad y, en el Servicio Nacional de Salud británico, los Digital Technology Assessment Criteria (DTAC).

## Dónde está

- No se declara ninguna certificación, aprobación ni auditoría. Esta página es la estimación del propio proyecto.
- «Médico» significa una herramienta de cartera y proyectos **no clínica**: sin historias clínicas y sin apoyo a
  decisiones clínicas.
- El proyecto sugiere empezar con un piloto sobre datos no clínicos y no secretos, a cargo de una organización que
  complete su propia evaluación.

## Madurez, punto por punto

Cada cifra es la estimación del proyecto a 13 de noviembre de 2026, no una medición. Cada control deslizante empieza en
esa estimación. Mover un control deslizante anota su propia opinión mientras lee; no se guarda ni se envía nada.

- **Garantía de seguridad independiente** — 0%. Sin prueba de penetración, sin auditoría de terceros y sin certificación. Un comprador o un proveedor tiene que encargarlas (RG-13).
- **Accesibilidad** — 40%. En cada cambio se ejecuta una auditoría automática WCAG 2.2 AA de cada ruta. No se ha hecho ninguna prueba manual, ni con tecnologías de apoyo ni con personas con discapacidad (RG-13).
- **Solicitudes de los interesados** — 50%. Los administradores pueden exportar y seudonimizar a una persona en cada referencia del esquema, con una vista previa antes (RG-6). Los nombres escritos en títulos, comentarios o documentos, los archivos subidos, las instantáneas de auditoría y las copias de seguridad no se encuentran automáticamente.
- **Conservación** — 35%. Una purga, en modo de prueba por defecto, abarca notificaciones, registros de entrega, claves de idempotencia e instantáneas de tendencias y de adopción, con plazos fijados por instalación (RG-4). Los planes, tareas, registros de tiempo, documentos y la pista de auditoría se conservan hasta que un operador los borre.
- **Marcas de tratamiento** — 60%. Una clasificación configurable oculta los planes por encima de la habilitación de quien consulta, también a los administradores, y etiqueta cada respuesta (RG-7). La marca no se estampa en los archivos generados, y la habilitación solo es tan fiable como el atributo del proveedor de identidad.
- **Seguridad de los adjuntos** — 50%. El contenido debe coincidir con su tipo declarado, se rechazan los programas y un análisis opcional con `clamd` falla de forma segura (RG-8). No se incluye ningún antivirus, y el enlace solo se ha probado contra un sustituto.
- **Revisión de accesos y acceso de emergencia** — 45%. Un informe de los permisos vigentes apoya la recertificación (RG-9), y el acceso de emergencia tiene un límite de tiempo y exige un motivo declarado (RG-10). El informe muestra lo registrado, no lo que impone el proveedor de identidad; no hay ninguna ruta para registrar la decisión de un revisor; y un aviso de acceso de emergencia puede nombrar a alguien que no recibe nada mientras el permiso sigue vigente.
- **Rendimiento y capacidad** — 40%. En [`BENCHMARKS.md`](../../../BENCHMARKS.md) hay una prueba de carga y de resistencia en una sola máquina: las peticiones normales responden en decenas de milisegundos, un plan de unas 12.700 tareas tarda en programarse y el proceso crece hasta cerca de 1 GB. No hay plan de capacidad, ni prueba con muchos usuarios, ni ejecución en varias máquinas.
- **Sesiones en varios servidores de interfaz** — 70%. Las sesiones se guardan en memoria salvo que se configure Redis (RG-12); con Redis, el inicio de sesión depende de él.
- **Integridad de la pista de auditoría** — 50%. Las escrituras se auditan y un disparador de la base de datos rechaza cambios y borrados, con excepciones declaradas (RG-3). Un administrador de la base de datos puede eliminar el disparador, y no hay cadena de hashes.
- **Copias de seguridad y recuperación** — 80%. La restauración se prueba en integración continua, sin cifrar y cifrada, frente a objetivos de recuperación declarados (RG-5). El cifrado es opcional, y el simulacro se ejecuta en la integración continua del propio proyecto, no en el alojamiento de quien lo despliega.
- **Soporte y continuidad** — 10%. Un único responsable de mantenimiento y ningún acuerdo de nivel de servicio ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Traducciones** — 20%. El texto en idiomas distintos del inglés está escrito por máquina hasta que lo revise un hablante, y las obligaciones legales de idioma exigen texto revisado ([declaración de idiomas](../../../governance/language-statement.md)).
- **Uso clínico** — 0%. No se ha redactado ningún caso de seguridad clínica, y el uso clínico queda fuera del alcance.

## Qué debe hacer un primer despliegue

1. Completar la [lista de comprobación de garantías](../../../governance/assurance-checklist.md), empezando por una prueba de penetración y una auditoría de accesibilidad.
2. Rellenar cada fila «Buyer to complete» de la [lista de comprobación DTAC](../../../governance/dtac-checklist.md) y las marcas de la [DPIA](../../../governance/dpia.md).
3. Acordar los plazos de conservación con el responsable de archivos y programar la purga.
4. Activar las copias de seguridad cifradas y ejecutar el simulacro de restauración en el propio alojamiento del despliegue.
5. Mantener los datos no clínicos y por debajo del nivel de marca para el que se evaluó el despliegue.
6. Planificar las carencias anteriores o financiar su eliminación.

Las pruebas de cada punto están en la [evaluación](../../../governance/production-readiness.md).
