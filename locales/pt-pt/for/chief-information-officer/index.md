# Para o Diretor de Informação

*Diretor de Informação (CIO): integração, identidade, dados, hospedagem e as informações que lhe serão pedidas.*

## Identidade e acesso

- O login é **obrigatório por padrão**; o serviço se recusa a iniciar sem as chaves de verificação de tokens e, em produção, com o login desativado.
- Autenticação por meio de um provedor de identidade (incluindo o Microsoft Entra ID); a autenticação multifator fica com o provedor ([guia de login](../../../../operations/sign-in-with-entra.md)).
- A autorização usa atributos do token (por exemplo `access`, `directorpack`, `confidence`) com uma política baseada em atributos. Planos privados ficam ocultos de todo agregado; marcas de tratamento ocultam planos acima da credencial de quem consulta, administradores inclusive.
- O acesso de emergência (break-glass) a planos privados exige elegibilidade, um motivo declarado, no máximo quatro horas e uma segunda pessoa avisada.
- Administradores podem listar as concessões de acesso vigentes (um apoio à recertificação) e exportar ou pseudonimizar os dados de uma pessoa.

## Dados e integração

| Tema | Posição |
|---|---|
| Dados mantidos | Registos de plano, tarefa, risco, orçamento, marco, tempo e custo; pessoas são referências opacas, não nomes. Nenhum dado de pacientes, nenhum dado de categoria especial por concepção |
| Inventário | Toda coluna de dados pessoais está listada no [inventário de dados pessoais](../../../../governance/personal-data-inventory.csv); um teste o mantém fiel ao esquema |
| Interfaces | REST com um documento OpenAPI derivado; importação e exportação em lote; feeds de calendário; eventos de saída e webhooks apenas para destinos `https` públicos |
| Backups | Backup por script com restauração testada, cifragem opcional e metas de recuperação declaradas ([backup e restauração](../../../../operations/backup-and-restore.md)) |
| Hospedagem | Imagens de contentor e `compose.yaml`; o software não faz transferências por conta própria, de modo que o local dos dados é uma escolha de hospedagem sua |

## Lacunas que você deve fechar

- Teste de intrusão, revisão independente do código e certificação: nenhum foi feito.
- A retenção é aplicada só em parte; a limpeza deve ser agendada (veja o [calendário de retenção](../../../../governance/retention-schedule.md)).
- A ferramenta para titulares de dados encontra referências, não nomes digitados em texto livre ou em ficheiros enviados.
- A trilha de auditoria não é à prova de adulteração por um administrador do banco de dados.

## Leia a seguir

[Diagrama de fluxo de dados](../../../../governance/data-flow.md) · [Lista de verificação de garantia](../../../../governance/assurance-checklist.md) · [Implantação](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
