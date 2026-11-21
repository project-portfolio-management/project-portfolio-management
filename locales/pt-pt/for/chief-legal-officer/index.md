# Para o Diretor Jurídico

*Diretor Jurídico (CLO): licença, proteção de dados, acessibilidade, deveres de idioma e o que o projeto não garante. Isto não é aconselhamento jurídico.*

## Licença

Oferecido sob qualquer uma das licenças MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, GPL-3.0-only, OGL-UK-3.0 ou CC-BY-NC-SA-4.0, ou sob termos personalizados mediante acordo. A escolha e os textos estão em [`LICENSE.md`](../../../../LICENSE.md) e em `LICENSES/`; um teste mantém todos os manifestos coerentes com eles.

## Proteção de dados (a lei que se aplica a você)

Escrito para qualquer lei de proteção de dados: a [especificação de conformidade §12.2a](../../../../spec/12-compliance.md) relaciona as mais comuns por região (o GDPR e o UK GDPR, a LGPD, a PIPL, a DPDP Act, a PIPEDA, a CCPA e outras) ao que cada uma exige aqui, a partir de conhecimento geral e sem conferência com o texto de cada lei. São documentos pré-preenchidos que você adapta; cada marcador `‹buyer›` é uma decisão sua (controlador, base legal, prazos de retenção, operadores, aprovação):

- [Modelo de DPIA](../../../../governance/dpia.md) e [registo de tratamento](../../../../governance/record-of-processing.md)
- [Inventário de dados pessoais](../../../../governance/personal-data-inventory.csv): toda coluna que pode conter dados pessoais
- [Calendário de retenção](../../../../governance/retention-schedule.md): propostas, com o que o código aplica hoje
- [Diagrama de fluxo de dados](../../../../governance/data-flow.md)

O software mantém pessoas como referências opacas, mais tempo registado, disponibilidade e taxas de custo, o que se aproxima de monitoramento e, por isso, justifica uma DPIA. Oferece exportação e eliminação para o titular dos dados por pseudonimização ou, se a implantação optar, por exclusão (somente administradores, com prévia antes; WW-1.8). Pacotes congelados da direção e o histórico de confiança, que só recebe acréscimos, têm o nome de quem redigiu ou avaliou apagado, nunca excluído, de modo que um pedido de eliminação pode ser atendido sem quebrar o registo. Ele **não** encontra nomes digitados em texto livre, e os backups guardam registos que a aplicação já excluiu.

## Regimes de referência

A HIPAA e os regimes de certificação de dados de saúde dos EUA **não se aplicam** (não há dados de pacientes); o regime de dispositivos clínicos não se aplica. A [lista de verificação do DTAC](../../../../governance/dtac-checklist.md) responde aos Digital Technology Assessment Criteria do NHS do Reino Unido, um esquema nacional mantido como exemplo prático; para esquemas de outros países e setores (ISO/IEC 27001, SOC 2, esquemas nacionais de nuvem e do setor público), comece pela [lista de verificação de garantia](../../../../governance/assurance-checklist.md). Nada é certificado.

## Acessibilidade e idioma

A WCAG 2.2 AA é testada automaticamente em cada rota; nenhum teste manual ou com tecnologia assistiva foi feito ([declaração](../../../../governance/accessibility-statement.md)). O galês e outras traduções são rascunhos que aguardam um tradutor profissional ([declaração de idioma](../../../../governance/language-statement.md)); isso importa se você tem deveres legais de idioma.

## Diligência prévia do fornecedor

Um único mantenedor, nenhum suporte contratual, nenhuma garantia alegada, ferramentas de IA usadas no desenvolvimento e divulgadas na [declaração sobre IA](../../../../AI_STATEMENT.md). O produto não contém nenhum modelo de IA. Relato de vulnerabilidades: [SECURITY.md](../../../../SECURITY.md).

## Decisões que cabem a você

Base legal e prazos de retenção; se você precisa de um procedimento de retenção legal (nenhum foi construído); termos de contrato e de suporte com o mantenedor.

## Leia a seguir

[Lista de verificação de garantia](../../../../governance/assurance-checklist.md) · [Índice do pacote de governação](../../../../governance/README.md)
