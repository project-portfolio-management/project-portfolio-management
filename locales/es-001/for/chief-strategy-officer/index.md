# Para la Dirección de Estrategia (CSO)

*Dirección de Estrategia (CSO): cómo el software conecta la estrategia con el trabajo, qué le muestra y qué todavía no puede hacer.*

## Qué hace por la estrategia

- **Un registro de todo lo financiado.** Las carteras, programas, proyectos y productos son un registro recursivo, de modo que todo el patrimonio puede leerse como un árbol ([propósito y visión](../../../../spec/01-purpose-and-vision.md)).
- **Detección de duplicados.** La comparación explicable muestra cuándo dos partes de la organización han dado de alta la misma iniciativa, antes de financiar ambas.
- **Objetivos estratégicos y el mapa de la cartera.** Usted marca objetivos como objetivos estratégicos y nombra un objetivo principal para cada programa. `GET /api/portfolio-map` y el paquete mensual de dirección agrupan los programas por objetivo principal. Un programa aparece una vez, con sus otros objetivos nombrados; un programa sin objetivo principal aparece bajo «sin objetivo estratégico», y los totales nunca se cuentan dos veces.
- **Confianza de entrega junto al estado calculado.** Las personas declaran su confianza en la entrega; el software la muestra junto al rojo/ámbar/verde calculado y señala una diferencia de dos niveles. Un plan que nadie ha evaluado muestra «sin evaluar», nunca verde.
- **Un paquete mensual congelado.** Lo que se informó a la dirección se conserva exactamente como se emitió, con la tendencia medida frente al paquete emitido anterior.

## Lo que todavía no hace

- El paquete aún no incluye decisiones de puerta, financiamiento por ejercicio fiscal, el diagrama de fiebre de amortiguadores, el valor ganado, los beneficios validados ni el mapa de calor de capacidad (`DP-5` a `DP-10` en [`tasks.md`](../../../../tasks.md)). El paquete indica qué secciones no están construidas.
- «Programa» significa un plan cuyo tipo es `Program`, porque los datos no tienen otra definición; los objetivos «de primer nivel» son los que usted marque, ya que los objetivos no tienen jerarquía.
- No hay planeación de escenarios, ni todavía una pantalla para el mapa o el paquete: están disponibles como datos.
- No está certificado y tiene una sola persona a cargo del mantenimiento ([camino a producción](../../path-to-production/index.md)).

## Decisiones que le corresponden

Qué objetivos son objetivos estratégicos; el objetivo principal de cada programa; quién declara y quién evalúa de forma independiente la confianza de entrega; cuántos meses de tendencia debe ver la dirección.

## Lecturas siguientes

[Hoja de ruta](../../../../spec/15-roadmap.md) · [Preguntas abiertas](../../../../spec/16-open-questions.md) · [Alcance](../../../../spec/02-scope.md)
