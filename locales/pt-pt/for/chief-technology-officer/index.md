# Para o Diretor de Tecnologia

*Diretor de Tecnologia (CTO): arquitetura, qualidade, como estendê-lo e onde a engenharia é frágil.*

## Arquitetura

Três subprojetos, uma entidade ([arquitetura](../../../../spec/08-architecture.md)):

| Parte | Tecnologia |
|---|---|
| Biblioteca de correspondência | Rust; correspondência par a par determinística e probabilística, explicável; incorporada ao serviço |
| Serviço | Rust, axa sobre axum, sqlx, PostgreSQL 18; REST em `/api`, OpenAPI derivado das rotas |
| Interface | SvelteKit 3, SPA Svelte 5 com um backend-for-frontend; catálogos de i18n |

A cadeia de ferramentas está fixada em [`rust-toolchain.toml`](../../../../rust-toolchain.toml). A persistência está na [especificação 10](../../../../spec/10-persistence.md); a API, na [especificação 09](../../../../spec/09-api-surface.md).

## Pontos fortes da engenharia que vale conhecer

- **As invariantes ficam no banco de dados onde importam:** o log de auditoria recusa alteração e exclusão; pacotes da direção emitidos e avaliações de confiança na entrega são imutáveis ou só recebem acréscimos por gatilho; restrições CHECK sustentam as regras do serviço.
- **Testes de guarda:** testes de censo fazem a compilação falhar quando uma nova rota de agregação ignora o filtro de planos privados, uma rota de lista não é paginada, uma coluna de dados pessoais falta no inventário ou uma rota falta no OpenAPI. A correção é classificar o item novo, não enfraquecer o teste.
- **Testes com tokens reais:** a autorização é testada de ponta a ponta com tokens PASETO assinados (cada teste em um processo próprio, porque as configurações de autenticação são lidas uma só vez por processo).
- **Cadeia de suprimentos:** `cargo deny`, uma lista de materiais de software, proveniência da compilação na publicação, uma política de segurança.
- **A CI** compila e testa tudo a cada push; veja `.github/workflows/`.

## Onde é frágil

- Um único mantenedor, de modo que o fator autocarro (bus factor) é um ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- O desempenho é conhecido apenas de forma aproximada; o cache de respostas cobre só a rota do caminho crítico; planos muito grandes são lentos.
- As sessões ficam na memória, a menos que o Redis esteja configurado.
- Várias seções do pacote da direção e todas as ecrãs do pacote não foram construídas ([`tasks.md`](../../../../tasks.md)).
- Ferramentas de IA são usadas para construí-lo; o produto não contém nenhum modelo. Veja a [declaração sobre IA](../../../../AI_STATEMENT.md).

## Decisões que cabem a você

A topologia de hospedagem e do Redis, se fará um fork ou adotará o upstream, quem revisa o código de forma independente e como o serviço se encaixa no seu provedor de identidade (conjunto de chaves PASETO, ou OIDC por meio do Keycloak ou do Entra).

## Leia a seguir

[Arquitetura](../../../../spec/08-architecture.md) · [Estratégia de testes](../../../../spec/11-testing-strategy.md) · [Situação da implementação](../../../../spec/14-implementation-status.md) · [Benchmarks](../../../../BENCHMARKS.md)
