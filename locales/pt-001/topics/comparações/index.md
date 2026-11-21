# Comparações

*Um resumo de como o software se relaciona com outros produtos.*

Os produtos abaixo atendem a finalidades diferentes e não recebem pontuação. Estas notas se baseiam no material público de cada projeto. **Nenhum outro produto foi instalado ou executado**, e cada um pode ter mudado desde então. A documentação atual do produto deve ser consultada antes de uma decisão.

| Projeto | O que é | Onde este projeto se diferencia |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Gestão de projetos para pessoas que não são gerentes de projeto | Este projeto também inclui as funções de cronograma, custo e governança usadas por um escritório de portfólio |
| [Redmine](https://www.redmine.org/) | Um rastreador configurável de uso geral, com tipos de problema, status e funções definidos pela organização | Tipos de problema, status e fluxos de trabalho são definidos pela organização. As funções carregam permissões sobre problemas, atribuídas plano a plano, dentro do que o login de cada pessoa permite. Veja a [comparação com o Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (por meio de seu fork, Platform Collective) | Uma estrutura de aplicações empresariais sobre muitos serviços | O Huly é uma plataforma de vários serviços. Este projeto é um único serviço sobre PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Uma alternativa de desktop ao Microsoft Project, para um gerente e um projeto | Este projeto é um serviço hospedado, com vários usuários e controle de acesso |

## Comparações de produtos

Páginas de comparação individuais (em inglês): [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interoperabilidade

- **Jira e Asana:** importação de CSV, com uma prévia e o motivo declarado de tudo o que ficou de fora.
- **Microsoft Project:** importação e exportação de XML MSPDI, abrangendo tarefas, estrutura de tópicos, datas, durações, restrições, dependências e o calendário. Responsáveis, recursos e linhas de base são excluídos, e o arquivo registra isso.

## Diferenças de projeto

- **Motivos declarados.** Um valor ausente é exibido com um motivo, por exemplo "sem previsão, porque…".
- **Detecção explicável de duplicidades.** A correspondência funciona em todos os tipos de plano e mostra suas evidências.
- **Padrões de segurança.** O login vem ativado por padrão, e o material de segurança está incluído no repositório.
- **Durações explícitas.** Cada duração é informada em dias corridos ou dias úteis.
- **Limites registrados.** O backlog registra os limites de cada item.

## Limitações conhecidas

- Nenhum aplicativo móvel ou de desktop.
- Nenhuma coedição em tempo real.
- Um único mantenedor, um histórico limitado em produção e nenhuma auditoria de terceiros.
- O teste de carga publicado foi executado em uma só máquina. Indica uma ordem de grandeza e não é um plano de capacidade.
- As traduções são rascunhos que aguardam revisão.
- Nenhuma tela para redigir ou decidir solicitações de mudança.
- Nenhum ecossistema de plugins ou de loja de extensões.

Mais detalhes estão nas [notas de comparação](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Correções podem ser enviadas como um issue com uma fonte.
