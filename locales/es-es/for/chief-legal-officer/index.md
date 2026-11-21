# Para la Dirección Jurídica (CLO)

*Dirección Jurídica (CLO): licencia, protección de datos, accesibilidad, obligaciones lingüísticas y lo que el proyecto no garantiza. Esto no es asesoría jurídica.*

## Licencia

Se ofrece bajo cualquiera de las siguientes: MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, GPL-3.0-only, OGL-UK-3.0 o CC-BY-NC-SA-4.0, o con términos a la medida por acuerdo. La elección y los textos están en [`LICENSE.md`](../../../../LICENSE.md) y en `LICENSES/`; una prueba mantiene coherentes con ellos todos los manifiestos.

## Protección de datos (la ley que le aplica)

Está escrito para cualquier ley de protección de datos: la [especificación de cumplimiento §12.2a](../../../../spec/12-compliance.md) asigna las más comunes por región (el GDPR y el UK GDPR, la LGPD, la PIPL, la ley DPDP, la PIPEDA, la CCPA y otras) a lo que cada una exige aquí, a partir de conocimiento general y sin contrastarlas con el texto de cada ley. Son documentos prellenados que usted adapta; cada marca `‹buyer›` es una decisión que le corresponde (responsable del tratamiento, base jurídica, plazos de retención, encargados, aprobación):

- [Plantilla de DPIA](../../../../governance/dpia.md) y [registro de tratamiento](../../../../governance/record-of-processing.md)
- [Inventario de datos personales](../../../../governance/personal-data-inventory.csv): cada columna que puede contener datos personales
- [Calendario de retención](../../../../governance/retention-schedule.md): propuestas, con lo que el código aplica hoy
- [Diagrama de flujo de datos](../../../../governance/data-flow.md)

El software conserva a las personas como referencias opacas junto con tiempo registrado, disponibilidad y tarifas de coste, lo cual es afín a la vigilancia y por tanto justifica una DPIA. Ofrece exportación de datos y supresión para titulares mediante seudonimización o, si el despliegue así lo decide, eliminación (solo administradores, con vista previa primero; WW-1.8). En los paquetes de dirección congelados y en el historial de confianza de solo anexado, se borra el nombre de quien redactó o evaluó, nunca el registro, de modo que una solicitud de supresión puede atenderse sin romper el registro. **No** encuentra nombres escritos en texto libre, y los copias de seguridad conservan registros que la aplicación ya eliminó.

## Marcos normativos

HIPAA y los regímenes estadounidenses de certificación de datos de salud **no aplican** (no hay datos de pacientes); tampoco aplica el régimen de dispositivos clínicos. La [lista de verificación DTAC](../../../../governance/dtac-checklist.md) responde a los Digital Technology Assessment Criteria del NHS del Reino Unido, un esquema nacional que se conserva como ejemplo desarrollado; para los esquemas de otros países y sectores (ISO/IEC 27001, SOC 2, esquemas nacionales de nube y de sector público), parta de la [lista de verificación de aseguramiento](../../../../governance/assurance-checklist.md). Nada está certificado.

## Accesibilidad e idioma

WCAG 2.2 AA se prueba automáticamente en cada ruta; no se ha hecho ninguna prueba manual ni con tecnologías de apoyo ([declaración](../../../../governance/accessibility-statement.md)). El galés y otras traducciones son borradores a la espera de una persona traductora profesional ([declaración sobre idiomas](../../../../governance/language-statement.md)); es relevante si tiene obligaciones lingüísticas establecidas por ley.

## Debida diligencia del proveedor

Una sola persona a cargo del mantenimiento, sin soporte contractual, sin garantía alguna, con herramientas de IA usadas en el desarrollo y declaradas en la [declaración sobre IA](../../../../AI_STATEMENT.md). El producto no contiene ningún modelo de IA. Informe de vulnerabilidades: [SECURITY.md](../../../../SECURITY.md).

## Decisiones que le corresponden

Base jurídica y plazos de retención; si necesita un procedimiento de conservación por motivos legales (no hay ninguno construido); términos de contrato y de soporte con quien mantiene el proyecto.

## Lecturas siguientes

[Lista de verificación de aseguramiento](../../../../governance/assurance-checklist.md) · [Índice del paquete de gobernanza](../../../../governance/README.md)
