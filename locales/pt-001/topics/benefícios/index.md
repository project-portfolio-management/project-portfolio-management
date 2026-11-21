# Benefícios

*Os principais benefícios para uma organização, por área.*

## Visibilidade do portfólio

As informações do portfólio costumam estar espalhadas por vários lugares. Neste software, todo portfólio, programa, projeto e produto é um único tipo de registro, chamado **plano**. Qualquer plano pode conter qualquer outro, em qualquer profundidade, de modo que a estrutura pode acompanhar a organização. Um rótulo como "Projeto" é apenas descritivo e não impõe nenhuma restrição ao plano.

## Redução de duplicidades

Dois departamentos podem instituir a mesma iniciativa com nomes diferentes. O comparador examina os atributos que identificam um plano: nome, objetivos, responsável, patrocinador, plano superior, período, palavras-chave e identificadores. Ele funciona com todos os tipos de plano e mostra as evidências de cada correspondência, e a decisão cabe a uma pessoa. Dois registros de um mesmo plano podem ser unificados.

## Qualidade dos relatórios

- Um valor ausente é exibido com um motivo, por exemplo "sem previsão, porque…".
- Um plano sem avaliação é exibido como "não avaliado".
- A confiança na entrega registrada por pessoas é exibida ao lado do status calculado, e os dois são mantidos separados.
- Planos privados ficam fora de todos os totais e contagens para todos os usuários. Testes verificam essa regra.
- Cada duração informa se está em dias corridos ou em dias úteis.

## Registro de decisões

Toda alteração é auditada, e a trilha de auditoria não pode ser modificada por meio do serviço. Quem abre uma solicitação de mudança não pode aprová-la. Um pacote mensal para a diretoria é redigido uma vez, emitido por uma pessoa identificada e depois congelado pelo banco de dados. Uma correção é emitida como uma nova versão, com uma nota.

## Segurança

O login vem ativado por padrão, e o serviço não inicia em produção com o login desativado. Cabeçalhos de segurança, limitação de taxa de requisições, um plano de recuperação com metas definidas e um pacote de governança da informação estão incluídos no repositório.

## Controle e portabilidade

- **Licenciamento.** O software é de código aberto, à escolha entre MIT, Apache-2.0, BSD-3-Clause, GPL-2.0 ou GPL-3.0.
- **Hospedagem.** O software é hospedado por conta própria: um serviço em Rust sobre PostgreSQL, com uma interface em SvelteKit.
- **Importação de dados.** Arquivos CSV do Jira e do Asana podem ser importados, com uma prévia e o motivo declarado para qualquer item excluído. O XML do Microsoft Project pode ser importado e exportado.
- **Idiomas.** A interface está disponível em dez localidades, incluindo o árabe, escrito da direita para a esquerda. As traduções são rascunhos que aguardam revisão por falantes.

## Páginas relacionadas

Veja também [funcionalidades](../funcionalidades/index.md), [comparações](../comparações/index.md) e o [tutorial](../tutorial/index.md).
