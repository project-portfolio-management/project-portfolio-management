# Para o Presidente Executivo

*Presidente Executivo (CEO): o que é este software, o que ele oferece, o que não oferece e as decisões que cabem a você.*

## O que é, em um parágrafo

Um registo de todo **plano** que a organização financia (portfólios, programas, projetos, produtos e assim por diante, em um único registo recursivo), com um espaço de trabalho de gestão de projetos por baixo (tarefas, cronograma, custos, recursos, riscos, problemas, solicitações de mudança, governação). Ele encontra iniciativas duplicadas por correspondência explicável, de modo que dois departamentos que instituem o mesmo trabalho apareçam antes de ambos serem financiados. Veja [propósito e visão](../../../../spec/01-purpose-and-vision.md).

## No que você pode confiar

- **Um único lugar para perguntar "o que estamos fazendo?"** Os planos se aninham em qualquer profundidade; planos privados ficam fora de todos os totais e contagens, para todos, por uma regra que os testes verificam.
- **Um pacote mensal para a direção** que é redigido uma vez, emitido por uma pessoa identificada e depois **congelado pelo banco de dados**: um pacote emitido não pode ser editado; uma correção é uma nova versão reemitida, com uma nota. Hoje ele traz o mapa do portfólio por objetivo estratégico, a confiança na entrega, exceções, problemas e solicitações de mudança.
- **Confiança na entrega declarada por pessoas, exibida ao lado do status calculado, nunca misturada com ele.** Um plano que ninguém avaliou mostra "não avaliado", nunca verde. Uma classificação âmbar-vermelho ou vermelho abre uma exceção com um relógio de 48 horas (configurável); um prazo perdido aparece no pacote.
- **Decisões deixam rastro.** Toda alteração é auditada, a trilha de auditoria não pode ser modificada por meio do serviço, e uma solicitação de mudança não pode ser aprovada por quem a abriu.

## O que ele ainda não oferece

- O pacote **ainda não** inclui decisões de portão, financiamento e dinheiro por exercício fiscal, o gráfico de febre dos pulmões, o valor agregado, os benefícios validados nem o mapa de calor de capacidade. Eles estão listados como `DP-5` a `DP-10` em [`tasks.md`](../../../../tasks.md), e o pacote informa quais seções ainda não foram construídas em vez de deixar um espaço em branco.
- Nenhum PDF ou ecrã para o pacote: ele é entregue como dados (`DP-11`).
- Ele **não é certificado nem auditado** por ninguém. O que seria necessário para o uso em produção, item por item e com uma estimativa para cada um, está na página [caminho para a produção](../../path-to-production/index.md).
- Um único mantenedor, nenhum acordo de nível de serviço ([governação](../../../../GOVERNANCE.md)).

## Decisões que cabem a você

1. Se fará um piloto e em qual parte do portfólio.
2. Quem é o **patrocinador** para a regra de exceção de 48 horas e quem pode emitir o pacote mensal.
3. Se financiará as lacunas acima (avaliação, teste de intrusão, acordo de suporte) ou as aceitará.

## Leia a seguir

[Caminho para a produção](../../path-to-production/index.md) · [Roteiro](../../../../spec/15-roadmap.md) · [Perguntas em aberto](../../../../spec/16-open-questions.md)
