# Gestão de portfólio de projetos

*Um registro do trabalho financiado, com detecção de duplicidades e um espaço de trabalho para gerenciamento de projetos.*

Gestão de portfólio de projetos é um software de código aberto para registrar os planos que uma organização financia e gerenciar o trabalho dentro deles. Abrange tarefas, cronogramas, custos, recursos, riscos e governança. Compara planos para identificar possíveis duplicidades e mostra as evidências de cada correspondência. Quando um valor não pode ser calculado, o software informa o motivo.

## Principais benefícios

- **Registro único do portfólio.** Portfólios, programas, projetos e produtos usam um único tipo de registro, e qualquer plano pode conter outro. Os relatórios partem de uma única lista, abrangendo todos os departamentos.
- **Detecção de duplicidades.** A correspondência compara planos registrados com nomes diferentes e mostra as evidências de cada resultado, para que os revisores possam agir antes que ambos sejam financiados.
- **Valores rastreáveis.** Um valor ausente é exibido com o motivo da ausência. Um plano sem avaliação é exibido como "não avaliado".
- **Trilha de auditoria.** Toda alteração é auditada. Quem abre uma solicitação de mudança não pode aprová-la.
- **Controle de acesso.** O login vem ativado por padrão, e o serviço não é executado em produção com o login desativado.
- **Implantação e licenciamento.** O software é hospedado por conta própria e oferecido sob cinco licenças de código aberto. É possível importar trabalho do Jira, do Asana e do Microsoft Project.

[Ver todos os benefícios](topics/benefícios/index.md)

## Áreas funcionais

- **Planos e correspondência.** Registro recursivo, detecção explicável de duplicidades e unificação.
- **Entrega.** Quadros kanban, gráficos de Gantt, calendários, marcos, caminho crítico e corrente crítica.
- **Finanças.** Orçamentos, livro de financiamento, valor agregado e risco de custo e de cronograma.
- **Recursos.** Grupos de habilidades, mapa de calor de capacidade e pessoas-chave.
- **Estratégia.** Entrada de demandas, ideias, objetivos, metas e resultados-chave, e cenários.
- **Governança.** Riscos, problemas, revisões de portão, escalonamento e pacotes mensais para a diretoria.

[Ver todas as funcionalidades](topics/funcionalidades/index.md)

## Usuários previstos

- **Escritórios de portfólio.** Relatórios abrangendo programas e projetos.
- **Gerentes de projeto e de entrega.** Gestão de cronograma, custo e risco.
- **Membros de equipe.** Uma visão pessoal do trabalho atribuído.
- **Líderes seniores.** Os [resumos por função](for/) descrevem o que o software oferece para cada função, seus limites e as decisões necessárias.

## Situação atual

O software está em estágio inicial. Tem um único mantenedor, nenhuma auditoria de terceiros e nenhum contrato de suporte. O que seria necessário para o uso em produção, item por item e com uma estimativa para cada um, está na página [caminho para a produção](path-to-production/index.md). As limitações conhecidas estão listadas na [página de comparações](topics/comparações/index.md).

## Demonstração

Uma demonstração local usa uma autoridade de transportes fictícia e requer Git e Podman. Os passos estão no [tutorial](topics/tutorial/index.md).
