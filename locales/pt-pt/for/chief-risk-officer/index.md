# Para o Diretor de Riscos

*Diretor de Riscos (CRO): os riscos que o software ajuda a enxergar e os riscos de usá-lo.*

## Riscos que ele ajuda a gerir

- **Riscos, problemas e solicitações de mudança** registados por plano; um registo de problemas entre os planos legíveis; solicitações de mudança com separação de funções.
- **Confiança na entrega e exceções:** a confiança declarada é mantida separada do RAG calculado, sinalizada quando os dois divergem em dois níveis, com um histórico que só recebe acréscimos, e um relógio de exceção de 48 horas que não reinicia quando uma classificação piora e só é encerrado por uma decisão registada.
- **Ausência honesta:** dados ausentes aparecem como "não avaliado" ou nulo com um motivo, nunca como verde ou zero.
- **Um pacote mensal congelado e versionado,** de modo que o que foi informado aos diretores possa ser mostrado depois exatamente como foi emitido.

## Riscos de usá-lo (o registo do próprio projeto)

| Risco | Situação |
|---|---|
| Nenhum teste de intrusão, auditoria de terceiros ou certificação | Em aberto: cabe ao comprador contratar ([lista de verificação de garantia](../../../../governance/assurance-checklist.md)) |
| Retenção aplicada apenas a parte dos dados | Em aberto; a limpeza deve ser agendada |
| A ferramenta para titulares de dados não busca em texto livre, anexos ou backups | Em aberto |
| A trilha de auditoria pode ser burlada por um administrador do banco de dados; sem cadeia de hashes | Declarado na [DPIA](../../../../governance/dpia.md) |
| Marcas de tratamento ocultam e rotulam, não criptografam | Declarado |
| A verificação de anexos exige um verificador operado por quem implanta | Em aberto |
| Um único mantenedor, sem SLA | Em aberto |
| As traduções são rascunhos | Em aberto |
| Apenas não clínico; sem caso de segurança clínica | Não insira conteúdo clínico |

## Controles que o software aplica

Login por padrão, limitação de taxa de requisições, cabeçalhos de segurança rígidos, um tempo limite de sessão ociosa (30 minutos) e um limite máximo de sessão, destinos de saída restritos a `https` público, anexos com assinatura verificada, linhas de auditoria imutáveis, acesso de emergência com auditoria. Cada um é coberto por um teste citado em [caminho para a produção](../../path-to-production/index.md).

## Decisões que cabem a você

O apetite a risco para um piloto; a janela de exceção (padrão de 48 horas); para qual nível de marcação a implantação tem garantia; quem revisa a lista de acessos e com que frequência.

## Leia a seguir

[Caminho para a produção](../../path-to-production/index.md) · [DPIA](../../../../governance/dpia.md) · [Conformidade](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
