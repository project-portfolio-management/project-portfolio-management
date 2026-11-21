# Testes de segurança independentes

*Para um comprador, um avaliador ou um testador: o que os testes de segurança independentes deste software devem cobrir, o que o projeto fornece para ajudar, o que foi e o que não foi feito, e o que fica por sua conta. Isto é uma orientação, não um relatório de testes e não é aconselhamento jurídico.*

## Onde estamos

**Nenhum teste de segurança independente foi feito.** Não existe nenhum teste de intrusão, auditoria de terceiros, revisão independente do código ou certificação para este software, e o projeto não alega nenhum ([caminho para a produção](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). Uma varredura passiva é executada na integração contínua do projeto (regras em `.zap/rules.tsv`), e os testes do próprio projeto verificam os controles listados abaixo; nenhum dos dois substitui um testador que não escreveu o software.

Este é o item 1 e o item 9 da [lista de verificação de garantia](../../../../governance/assurance-checklist.md). Cabe a quem implanta, com um testador certificado, contra a **sua** implantação: o projeto não pode testar a sua hospedagem, o seu provedor de identidade, a sua rede nem as suas chaves.

## O que testar

Teste a implantação como ela vai funcionar, com o login obrigatório e o seu provedor de identidade na frente. Escopo sugerido, a partir dos controles que o software declara:

| Área | O que o software declara | O que um testador deve tentar |
|---|---|---|
| Autenticação | O login vem ativado por padrão; o serviço não inicia sem as chaves de verificação ou, em produção, com o login desativado; os tokens são verificados offline com chaves publicadas | Tokens forjados, expirados, reutilizados e com assinatura incorreta; emissor ou público errado; rotação de chaves; um conjunto de chaves ausente |
| Autorização | Política baseada em atributos; regras no nível do registo | Escalada de privilégio horizontal e vertical em cada rota; troca de identificadores em caminhos e corpos de requisição; atributos que quem consulta não deveria poder adicionar |
| Planos privados e marcas de tratamento | Um plano privado, e um plano acima da credencial de quem consulta, não está em nenhuma lista, contagem, total, resultado de busca, feed ou exportação para essa pessoa, administradores inclusive | Qualquer rota que revele a existência, um nome, uma contagem ou um total de um plano que quem consulta não pode ler; mensagens de erro e tempos de resposta que diferem entre "oculto" e "inexistente" |
| Acesso de emergência | Somente pessoas elegíveis, um motivo declarado, no máximo quatro horas, uma segunda pessoa avisada | Uso sem elegibilidade; prorrogação de uma concessão; agir sem que a segunda pessoa seja avisada |
| Registos congelados e que só recebem acréscimos | A trilha de auditoria, os pacotes da direção emitidos, o histórico de confiança na entrega, o histórico de status de programas e os pulmões de projeto recusam alteração e exclusão no banco de dados | Atualizar ou excluir pela API, pela importação em lote e por qualquer rota que toque as mesmas linhas |
| Separação de funções | Quem solicita não pode aprovar a própria solicitação de mudança | Aprovar como o solicitante, como um administrador e trocando o aprovador depois do envio |
| Envios e anexos | O conteúdo deve corresponder ao tipo declarado; programas são recusados; um gancho opcional de verificação de vírus falha de forma fechada | Ficheiros poliglotas, programas renomeados, ficheiros grandes demais e malformados, artifícios de caminho nos nomes, e se o seu verificador é realmente chamado |
| Requisições de saída | Destinos informados pelo utilizador são limitados a endereços `https` públicos | Falsificação de requisição do lado do servidor (SSRF) para endereços privados, link-local e de metadados, redirecionamentos, DNS rebinding |
| Sessões e o navegador | Tempo limite de inatividade, um limite máximo, um cookie `httpOnly`, uma Content Security Policy rígida, uma lista de origens permitidas do CORS | Fixação e reutilização de sessão, falsificação de requisição entre sites (CSRF), injeção na interface, clickjacking |
| Limitação de taxa | Limites por utilizador | Contorno trocando identidade, cabeçalho ou caminho; se os limites sobrevivem ao seu proxy |
| Ferramentas para titulares de dados | Exportação e pseudonimização, somente administradores, prévia antes | Uso por quem não é administrador; completude em relação ao [inventário de dados pessoais](../../../../governance/personal-data-inventory.csv) |
| Backups | Criptografia opcional, restauração testada | Acesso aos ficheiros de backup e às chaves; restauração em um ambiente limpo |

## O que o projeto fornece

- O código-fonte, a especificação e os testes, para que uma revisão de código possa partir das declarações.
- Uma descrição legível por máquina da API (`/api-docs/openapi.json`), com esquemas de requisição e resposta para o trabalho dos pacotes da direção; a lista de rotas é conferida com o código por testes.
- Uma configuração descartável com o login desativado (`dev-insecure`, veja o [README](../../../../README.md)) **somente para a máquina do próprio testador**, e um script que gera um conjunto de chaves de teste (`scripts/smoke-keys.sh`) para que tokens com atributos escolhidos possam ser emitidos contra uma implantação de teste.
- O [diagrama de fluxo de dados](../../../../governance/data-flow.md), o [manual de operação](../../../../operations/runbook.md) e as [metas de recuperação](../../../../operations/recovery-objectives.md).
- Uma forma privada de relatar o que for encontrado: [SECURITY.md](../../../../SECURITY.md).

## Regras de engajamento

Combine estas regras por escrito antes de qualquer teste; o projeto não pode defini-las para o seu ambiente.

- **Ambiente.** Teste uma cópia com dados realistas, mas não reais. Não teste a produção, a menos que você tenha decidido fazê-lo, por escrito.
- **Autorização.** O testador tem a sua permissão por escrito, um contato identificado e uma janela de tempo declarada.
- **Fora dos limites.** Negação de serviço por volume, engenharia social contra empregados e serviços de terceiros (o seu provedor de identidade, a sua hospedagem) ficam fora do escopo, a menos que o dono de cada um concorde. O escopo do próprio projeto está em [SECURITY](../../../../SECURITY.md).
- **Dados.** Dados pessoais reais não são usados, não são copiados para fora e não entram no relatório.
- **Chaves.** As chaves de teste são separadas das de produção e destruídas depois.

## Relatório e novo teste

- Achados sobre este software vão ao mantenedor de forma privada, como diz o [SECURITY](../../../../SECURITY.md), com a forma de reproduzir, a versão ou o commit e o impacto. As metas do projeto são confirmação em 5 dias úteis e um plano em 15; são metas, não garantias. Há um único mantenedor e nenhum acordo de nível de serviço.
- Achados sobre a sua implantação vão ao seu próprio responsável.
- Peça ao testador que classifique cada achado, diga o que **não** foi testado e teste novamente as correções. Um relatório que lista apenas o que foi encontrado não diz o que passou despercebido.
- Guarde o relatório, o escopo, as datas e o novo teste nos seus próprios registos. O projeto não pode guardá-los por você.

## Limites deste documento

Ele lista áreas a cobrir, não um plano de testes completo, e as declarações da tabela são do próprio projeto. Não certifica nada, e um teste sem achados é evidência sobre uma implantação em uma data.

## Decisões que cabem a você

Quem testa, quando, contra o quê, com quais dados e sob quais regras; como os achados são classificados e até quando devem ser corrigidos; e se o resultado é partilhado com os seus avaliadores.

## Leia a seguir

[Lista de verificação de garantia](../../../../governance/assurance-checklist.md) · [Caminho para a produção](../../path-to-production/index.md) · [Diretor de Segurança](../chief-security-officer/index.md) · [Governação, risco e conformidade](../governance-risk-compliance/index.md)
