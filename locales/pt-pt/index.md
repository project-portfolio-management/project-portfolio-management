# Gestão de portfólio de projetos

*Um registo do trabalho financiado, com deteção de duplicidades e um espaço de trabalho para gestão de projetos.*

Gestão de portfólio de projetos é um software de código aberto para registar os planos que uma organização financia e gerir o trabalho dentro deles. Abrange tarefas, cronogramas, custos, recursos, riscos e governação. Compara planos para identificar possíveis duplicidades e mostra as evidências de cada correspondência. Quando um valor não pode ser calculado, o software informa o motivo.

## Principais benefícios

- **Registo único do portfólio.** Portfólios, programas, projetos e produtos usam um único tipo de registo, e qualquer plano pode conter outro. Os relatórios partem de uma única lista, abrangendo todos os departamentos.
- **Deteção de duplicidades.** A correspondência compara planos registados com nomes diferentes e mostra as evidências de cada resultado, para que os revisores possam agir antes que ambos sejam financiados.
- **Valores rastreáveis.** Um valor ausente é exibido com o motivo da ausência. Um plano sem avaliação é exibido como "não avaliado".
- **Trilha de auditoria.** Toda alteração é auditada. Quem abre uma solicitação de mudança não pode aprová-la.
- **Controle de acesso.** O login vem ativado por padrão, e o serviço não é executado em produção com o login desativado.
- **Implantação e licenciamento.** O software é hospedado por conta própria e oferecido sob cinco licenças de código aberto. É possível importar trabalho do Jira, do Asana e do Microsoft Project.

[Ver todos os benefícios](topics/benefícios/index.md)

## Áreas funcionais

- **Planos e correspondência.** Registo recursivo, deteção explicável de duplicidades e unificação.
- **Entrega.** Quadros kanban, gráficos de Gantt, calendários, marcos, caminho crítico e corrente crítica.
- **Finanças.** Orçamentos, livro de financiamento, valor agregado e risco de custo e de cronograma.
- **Recursos.** Grupos de habilidades, mapa de calor de capacidade e pessoas-chave.
- **Estratégia.** Entrada de demandas, ideias, objetivos, metas e resultados-chave, e cenários.
- **Governação.** Riscos, problemas, revisões de portão, escalonamento e pacotes mensais para a direção.

[Ver todas as funcionalidades](topics/funcionalidades/index.md)

## Utilizadores previstos

- **Escritórios de portfólio.** Relatórios abrangendo programas e projetos.
- **Gerentes de projeto e de entrega.** Gestão de cronograma, custo e risco.
- **Membros de equipa.** Uma visão pessoal do trabalho atribuído.
- **Líderes seniores.** Os [resumos por função](for/) descrevem o que o software oferece para cada função, seus limites e as decisões necessárias.

## Situação atual

O software está em estágio inicial. Tem um único mantenedor, nenhuma auditoria de terceiros e nenhum contrato de suporte. O que seria necessário para o uso em produção, item por item e com uma estimativa para cada um, está na página [caminho para a produção](path-to-production/index.md). As limitações conhecidas estão listadas na [página de comparações](topics/comparações/index.md).

## Demonstração

Uma demonstração local usa uma autoridade de transportes fictícia e requer Git e Podman. Os passos estão no [tutorial](topics/tutorial/index.md).
