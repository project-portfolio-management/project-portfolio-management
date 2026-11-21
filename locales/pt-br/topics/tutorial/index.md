# Tutorial

*Passos para executar a demonstração em um computador local e examinar um portfólio de exemplo.*

A demonstração é uma cópia completa do software, carregada com dados de uma organização fictícia, a Northgate Transport Authority. Todas as pessoas, planos, valores e datas são fictícios, e os dados podem ser alterados livremente.

**Nota de segurança.** A demonstração confia em uma chave de teste publicada. Ela não deve ser executada em uma rede compartilhada nem usada com dados reais.

## Requisitos

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) com Compose

## Passo 1: baixar o código

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Passo 2: iniciar a demonstração

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Este comando compila o software, inicia o banco de dados, carrega o portfólio fictício e inicia a interface. A primeira compilação demora mais do que as seguintes.

## Passo 3: entrar

Abra <http://localhost:3000/pt-br/signin>, deixe o endereço como `example@example.com` e pressione **Sign in to the demonstration**. Nenhum e-mail é enviado.

A demonstração ainda mostra os rótulos da interface em inglês, como este botão; esta página os mantém como aparecem na tela.

A primeira página é o **demonstration tour**, que lista todas as funcionalidades por grupo, com uma descrição e um link. Uma faixa amarela em todas as páginas informa que os dados são fictícios.

## Passo 4: examinar o portfólio

A partir do tour, abra **Plans** para ver o portfólio, seus programas e seus projetos. Depois abra o **Portfolio dashboard**.

## Passo 5: examinar um projeto

Abra o projeto **North Line interlocking** pelo grupo de entrega do tour. Examine o gráfico de **Gantt**, o **Critical path** e o **Earned value**.

## Passo 6: examinar o trabalho pessoal

Abra **My work** e **My kanban board**. A sessão está aberta como Example Person, que tem tarefas em aberto.

## Passo 7: examinar capacidade, controle de início e relatórios

Abra **Capacity heatmap** para ver os recursos com excesso de alocação. Abra **Start queue**, onde o limite de trabalho em andamento segurou um programa. Abra **Director packs**, o resumo mensal que é congelado depois de emitido.

## Passo 8: encerrar a demonstração

Este comando encerra todos os serviços e apaga os dados da demonstração:

```sh
podman compose --profile demo down -v
```

## Próximos passos

- [Guia de instalação](../../../../INSTALL.md), que cobre o login e os provedores de identidade
- [Resumos por função](../../for/) para funções seniores
- [Guia da demonstração](../../../../operations/demo.md)
- [Lista de funcionalidades](../funcionalidades/index.md)

Problemas podem ser relatados pelo [rastreador de issues](https://github.com/project-portfolio-management/project-portfolio-management/issues).
