# Governação, risco e conformidade

*Para quem conduz governação, risco e conformidade: o que o software ajuda a comprovar, o que ele aplica e o que você deve fornecer. Não é aconselhamento jurídico.*

## Governação

- **As decisões são registadas.** Solicitações de mudança exigem um aprovador identificado que não seja quem as solicitou, um motivo declarado de pelo menos 10 caracteres, e são auditadas por autor; o banco de dados recusa uma aprovação sem decisor e sem motivo.
- **Um pacote mensal congelado para a direção.** Redigido uma vez, emitido por uma pessoa, imutável depois de emitido; as correções são uma reemissão com uma nota. O acesso de leitura é concedido por público.
- **Confiança na entrega e exceções.** A confiança declarada só recebe acréscimos e é exibida ao lado do status calculado, nunca misturada. Uma classificação vermelha ou âmbar-vermelho abre uma exceção com um relógio de 48 horas (configurável) que só uma decisão registada encerra.
- **Governação do próprio projeto do software:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [controle de mudanças](../../../../spec/18-change-control.md), [declaração sobre IA](../../../../AI_STATEMENT.md).

## Risco

- Riscos, problemas e solicitações de mudança por plano; um registo de problemas entre os planos legíveis.
- Dados ausentes são exibidos como "não avaliado", nunca como verde.
- Os riscos em aberto do próprio projeto (nenhum teste de intrusão, retenção parcial, um único mantenedor, traduções em rascunho) estão na página [caminho para a produção](../../path-to-production/index.md), item por item e com uma estimativa para cada um.

## Conformidade

| Área | Onde | Situação |
|---|---|---|
| Proteção de dados (a lei aplicável; a [§12.2a](../../../../spec/12-compliance.md) relaciona GDPR, UK GDPR, LGPD, PIPL, DPDP, PIPEDA, CCPA e outras) | [DPIA](../../../../governance/dpia.md), [registo de tratamento](../../../../governance/record-of-processing.md), [inventário de dados pessoais](../../../../governance/personal-data-inventory.csv), [calendário de retenção](../../../../governance/retention-schedule.md), [fluxo de dados](../../../../governance/data-flow.md) | Pré-preenchido; os marcadores `‹buyer›` são seus; retenção aplicada só em parte |
| DTAC do NHS do Reino Unido (um esquema nacional, um exemplo prático; outros esquemas começam pela [lista de verificação de garantia](../../../../governance/assurance-checklist.md)) | [Lista de verificação do DTAC](../../../../governance/dtac-checklist.md) | Respondida seção por seção; linhas do comprador e do fornecedor em aberto |
| Acessibilidade | [declaração](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA automatizada; nenhum teste manual |
| Deveres de idioma | [declaração de idioma](../../../../governance/language-statement.md) | As traduções são rascunhos |
| ISO/IEC 27001 | [especificação de conformidade](../../../../spec/12-compliance.md) | Controles do lado da implantação; não certificado |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [especificação de conformidade](../../../../spec/12-compliance.md) §12.4 | Não se aplicam (sem dados de pacientes); declarado, não fabricado |

## Evidências e garantia

A [lista de verificação de garantia](../../../../governance/assurance-checklist.md) enumera 14 itens que um avaliador vai pedir, com responsável e situação. A maioria está em aberto e cabe a quem implanta: teste de intrusão, auditoria de acessibilidade, exercício de recuperação de desastres, garantia da hospedagem, configuração do provedor de identidade, recertificação de acessos, processo de incidentes, gestão de documentos. A trilha de auditoria recusa alteração e exclusão no banco de dados, mas um administrador do banco de dados pode burlá-la; não há cadeia de hashes.

## Decisões que cabem a você

Base legal e prazos de retenção; janela de exceção; nível de marcação da implantação; quem revisa os acessos e com que frequência; se é necessário um procedimento de retenção legal (nenhum foi construído).

## Leia a seguir

[Índice do pacote de governação](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Especificação de conformidade](../../../../spec/12-compliance.md)
