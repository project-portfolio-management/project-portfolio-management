# Para o Diretor de Estratégia

*Diretor de Estratégia (CSO): como o software liga a estratégia ao trabalho, o que ele mostra a você e o que ainda não consegue fazer.*

## O que ele faz pela estratégia

- **Um único registro de tudo o que é financiado.** Portfólios, programas, projetos e produtos são um único registro recursivo, de modo que todo o patrimônio pode ser lido como uma árvore ([propósito e visão](../../../../spec/01-purpose-and-vision.md)).
- **Detecção de duplicidades.** A correspondência explicável mostra quando duas partes da organização instituíram a mesma iniciativa, antes de ambas serem financiadas.
- **Objetivos estratégicos e o mapa do portfólio.** Você marca objetivos como objetivos estratégicos e indica um objetivo principal para cada programa. `GET /api/portfolio-map` e o pacote mensal da diretoria agrupam os programas por objetivo principal. Um programa aparece uma vez, com seus outros objetivos citados; um programa sem objetivo principal é listado em "sem objetivo estratégico", e os totais nunca são contados em duplicidade.
- **Confiança na entrega ao lado do status calculado.** As pessoas declaram a sua confiança na entrega; o software a mostra junto do vermelho/âmbar/verde calculado e sinaliza uma diferença de dois níveis. Um plano que ninguém avaliou mostra "não avaliado", nunca verde.
- **Um pacote mensal congelado.** O que foi informado aos diretores é mantido exatamente como emitido, com a tendência medida em relação ao pacote emitido anterior.

## O que ele ainda não faz

- O pacote ainda não traz decisões de portão, financiamento por exercício fiscal, o gráfico de febre dos pulmões, o valor agregado, os benefícios validados nem o mapa de calor de capacidade (`DP-5` a `DP-10` em [`tasks.md`](../../../../tasks.md)). O pacote informa quais seções não foram construídas.
- "Programa" significa um plano cujo tipo é `Program`, porque os dados não têm outra definição; os objetivos de "nível mais alto" são os que você marcar, já que os objetivos não têm hierarquia.
- Não há planejamento de cenários, e ainda não há tela para o mapa nem para o pacote: eles estão disponíveis como dados.
- Não é certificado e tem um único mantenedor ([caminho para a produção](../../path-to-production/index.md)).

## Decisões que cabem a você

Quais objetivos são objetivos estratégicos; o objetivo principal de cada programa; quem declara e quem avalia de forma independente a confiança na entrega; quantos meses de tendência os diretores devem ver.

## Leia a seguir

[Roteiro](../../../../spec/15-roadmap.md) · [Perguntas em aberto](../../../../spec/16-open-questions.md) · [Escopo](../../../../spec/02-scope.md)
