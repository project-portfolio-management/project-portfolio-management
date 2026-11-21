# Para o Diretor de Operações

*Diretor de Operações (COO): o que você conduz no dia a dia, o que o software faz pela entrega e o que é preciso para operá-lo.*

## O que ele faz pelas operações

- **Planeamento e controle da entrega:** planos, tarefas, dependências, um cálculo do caminho crítico, marcos entre planos, sprints e fases, burndown e métricas de fluxo.
- **Capacidade:** alocações, disponibilidade, licenças e períodos sem trabalho, funções de recursos, carga de trabalho e utilização derivadas. (Um mapa de calor de capacidade por grupo de habilidades e mês para o pacote da direção não foi construído: `DP-10`.)
- **Tempo e custo:** folhas de horas, tempo registado, taxas de custo (restritas às finanças), orçamentos e custo derivado.
- **Fluxo de governação:** registo de problemas, solicitações de mudança com separação de funções (quem solicita não pode aprovar), aprovações, revisões, casos de admissão, mudança e desligamento.
- **Exceções:** uma confiança na entrega vermelha ou âmbar-vermelho abre uma exceção que exige uma causa, a decisão necessária, um responsável identificado e um prazo de decisão de até 48 horas (janela configurável, de 1 a 720 horas). As violações aparecem no pacote da direção e em `GET /api/exceptions`.

## O que é preciso para operá-lo

| Necessidade | Detalhe |
|---|---|
| Serviço | Um serviço em Rust, PostgreSQL 18, Redis opcional (cache de produção, sessões partilhadas opcionais), uma interface SvelteKit |
| Manual de operação | [pacote de operações](../../../../operations/README.md): implantação, manual de operação, backup e restauração, metas de recuperação, atualização e migração |
| Tarefas agendadas | Nada se agenda sozinho. A limpeza de retenção, os avisos de violação de exceção e tarefas semelhantes devem ser colocados em um agendador por seus operadores |
| Suporte | Um único mantenedor, sem SLA; reserve orçamento para o seu próprio suporte de primeiro nível |

## Conheça os limites

- Um plano com vários milhares de tarefas demora para ser programado; a resposta completa é recusada acima de 5.000 tarefas, a menos que seja pedida (`?full=true` ou `?only=critical`). Os valores e resguardadas estão em [`BENCHMARKS.md`](../../../../BENCHMARKS.md); são uma ordem de grandeza obtida em uma só máquina, não capacidade.
- Existem ecrãs para boa parte do espaço de trabalho, mas **ainda não** para solicitações de mudança, problemas, pacotes ou confiança na entrega; hoje esses itens só existem pela API.

## Decisões que cabem a você

Quem é responsável por cada exceção, quem emite o pacote mensal (o ritmo e os lembretes são `DP-12`, não construído) e qual cadência de revisão a regra de exceção alimenta.

## Leia a seguir

[Pacote de operações](../../../../operations/README.md) · [Manual de operação](../../../../operations/runbook.md) · [Metas de recuperação](../../../../operations/recovery-objectives.md)
