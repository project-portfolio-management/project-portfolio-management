# Para o Diretor de Segurança

*Diretor de Segurança (CSO): o que o software aplica, o que o projeto verificou e o que você precisa testar e operar por conta própria.*

## O que o software aplica

| Área | Comportamento |
|---|---|
| Autenticação | O login vem ativado por padrão. O serviço não inicia sem as chaves de verificação de tokens e não inicia em produção com o login desativado. Autenticação por meio de um provedor de identidade como o Microsoft Entra ID; a autenticação multifator fica com o provedor ([guia de login](../../../../operations/sign-in-with-entra.md)) |
| Autorização | Política baseada em atributos do token. Planos privados ficam fora de todo agregado para todos; marcas de tratamento ocultam planos acima da credencial de quem consulta, administradores inclusive |
| Acesso de emergência | O acesso a planos privados exige elegibilidade, um motivo declarado, no máximo quatro horas e uma segunda pessoa avisada; toda solicitação é auditada |
| Sessões | Tempo limite de inatividade (30 minutos por padrão) e um limite máximo de sessão |
| Transporte e navegador | Cabeçalhos de segurança, uma Content Security Policy rígida, uma lista de origens permitidas do CORS, limitação de taxa por usuário |
| Saída | Destinos informados pelo usuário são restritos a endereços `https` públicos |
| Envios | O conteúdo deve corresponder ao tipo declarado; programas são recusados; uma verificação opcional com `clamd` falha de forma fechada |
| Auditoria | A trilha de auditoria recusa alteração e exclusão no banco de dados |
| Registros congelados | Pacotes da diretoria emitidos e avaliações de confiança na entrega são imutáveis ou só recebem acréscimos, por gatilho do banco de dados |
| Cadeia de suprimentos | `cargo deny`, uma lista de materiais de software, proveniência da compilação na publicação, uma [política de segurança](../../../../SECURITY.md) |

## O que não foi feito

- Nenhum teste de intrusão, auditoria de terceiros, revisão independente do código ou certificação.
- Nenhum verificador de malware incluído; o gancho do `clamd` foi testado apenas com um substituto.
- A trilha de auditoria pode ser burlada por um administrador do banco de dados; não há cadeia de hashes.
- Marcas de tratamento ocultam e rotulam; não criptografam, e a marca não é gravada nos arquivos gerados.
- As sessões ficam na memória, a menos que o Redis esteja configurado.
- A ferramenta para titulares de dados encontra referências, não nomes digitados em texto livre, em arquivos enviados ou em backups.

## Como operá-lo

Os backups têm restauração testada, criptografia opcional e metas de recuperação declaradas ([pacote de operações](../../../../operations/README.md)). O [manual de operação](../../../../operations/runbook.md) traz os passos técnicos para incidentes; o processo de incidentes e de violação de dados é seu. Relate uma vulnerabilidade de forma privada, como o [SECURITY.md](../../../../SECURITY.md) descreve; as metas são de confirmação em 5 dias úteis, não garantias.

## Decisões que cabem a você

Quem executa o teste de intrusão e quando; o acesso condicional do provedor de identidade e o mapeamento de funções; o nível de marcação para o qual esta implantação tem garantia; o verificador por trás do gancho de anexos; com que frequência o acesso é recertificado.

## Leia a seguir

[Lista de verificação de garantia](../../../../governance/assurance-checklist.md) · [Caminho para a produção](../../path-to-production/index.md) · [Fluxo de dados](../../../../governance/data-flow.md)
