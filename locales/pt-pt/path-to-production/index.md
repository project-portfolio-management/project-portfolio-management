# Caminho para a produção

*O que "produção" significa para este software e a que distância ele está dela, item por item.*

"Produção" aqui significa que uma organização usa o software em trabalho real. Planos reais, nomes de pessoas reais e
decisões reais dependem dele, uma queda ou um vazamento tem consequências, e alguém responde por isso. Em um grande
órgão de saúde ou em um ministério, produção também significa que a garantia da própria organização foi cumprida: um
teste de segurança, uma avaliação de impacto à proteção de dados, uma auditoria de acessibilidade e, no Serviço
Nacional de Saúde do Reino Unido (NHS), os Digital Technology Assessment Criteria (DTAC).

## Onde estamos

- Nenhuma certificação, aprovação ou auditoria é alegada. Esta página é a estimativa do próprio projeto.
- "Médico" significa uma ferramenta de portfólio e de projetos **não clínica**: sem prontuários de pacientes e sem apoio à decisão clínica.
- O projeto sugere começar com um piloto com dados não clínicos e não sigilosos, executado por uma organização que
  conclua a sua própria garantia.

## Prontidão, item por item

Cada valor é a estimativa do projeto em 13 de novembro de 2026, não uma medição. Cada controle deslizante começa nessa estimativa.
Mover um controle regista a sua própria opinião enquanto você lê; nada é guardado nem enviado.

- **Garantia independente de segurança** — 0%. Nenhum teste de intrusão, nenhuma auditoria de terceiros e nenhuma certificação. Um comprador ou um fornecedor precisa contratá-los (RG-13).
- **Acessibilidade** — 40%. Uma auditoria automatizada WCAG 2.2 AA é executada em cada rota a cada alteração. Nenhum teste manual, nem com tecnologia assistiva ou com pessoas com deficiência, foi feito (RG-13).
- **Solicitações de titulares de dados** — 50%. Administradores podem exportar e pseudonimizar uma pessoa em todas as referências do esquema, com uma prévia antes (RG-6). Nomes digitados em títulos, comentários ou documentos, ficheiros enviados, instantâneos de auditoria e backups não são encontrados automaticamente.
- **Retenção** — 35%. Uma limpeza, em simulação por padrão, abrange notificações, registos de entrega, chaves de idempotência e instantâneos de tendência e de adoção, com prazos definidos por implantação (RG-4). Planos, tarefas, registos de tempo, documentos e a trilha de auditoria são mantidos até que um operador os apague.
- **Marcas de tratamento** — 60%. Uma classificação configurável oculta planos acima da credencial de quem consulta, administradores inclusive, e rotula cada resposta (RG-7). A marca não é gravada nos ficheiros gerados, e a credencial vale tanto quanto o atributo do provedor de identidade.
- **Segurança de anexos** — 50%. O conteúdo deve corresponder ao tipo declarado, programas são recusados, e uma verificação opcional com `clamd` falha de forma fechada (RG-8). Nenhum verificador é incluído, e o gancho foi testado apenas com um substituto.
- **Revisão de acessos e acesso de emergência** — 45%. Um relatório de concessões vigentes apoia a recertificação (RG-9), e o acesso de emergência tem prazo e exige um motivo declarado (RG-10). O relatório mostra o que foi registado, não o que o provedor de identidade impõe; nenhuma rota regista a decisão de um revisor; e um aviso de acesso de emergência pode citar alguém que não recebe nada enquanto a concessão continua valendo.
- **Desempenho e capacidade** — 40%. Um teste de carga e de resistência em uma só máquina está em [`BENCHMARKS.md`](../../../BENCHMARKS.md): requisições comuns respondem em dezenas de milissegundos, um plano de cerca de 12.700 tarefas demora para ser programado, e o processo cresce até cerca de 1 GB. Não há plano de capacidade, nem teste com muitos utilizadores, nem execução em várias máquinas.
- **Sessões em vários servidores de front-end** — 70%. As sessões ficam na memória, a menos que o Redis esteja configurado (RG-12); com o Redis, o login depende dele.
- **Integridade da trilha de auditoria** — 50%. As gravações são auditadas, e um gatilho do banco de dados recusa alterações e exclusões, com exceções declaradas (RG-3). Um administrador do banco de dados pode remover o gatilho, e não há cadeia de hashes.
- **Backup e recuperação** — 80%. Uma restauração é testada na integração contínua, simples e cifrada, em relação a metas de recuperação declaradas (RG-5). A cifragem é opcional, e o ensaio é executado na CI do próprio projeto, não na hospedagem de quem implanta.
- **Suporte e continuidade** — 10%. Um único mantenedor e nenhum acordo de nível de serviço ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Traduções** — 20%. O texto em idiomas diferentes do inglês é escrito por máquina até que um falante o revise, e deveres legais de idioma exigem texto revisado ([declaração de idioma](../../../governance/language-statement.md)).
- **Uso clínico** — 0%. Nenhum caso de segurança clínica foi escrito, e o uso clínico está fora do escopo.

## O que uma primeira implantação deve fazer

1. Concluir a [lista de verificação de garantia](../../../governance/assurance-checklist.md), começando por um teste de intrusão e uma auditoria de acessibilidade.
2. Preencher cada linha "Buyer to complete" da [lista de verificação do DTAC](../../../governance/dtac-checklist.md) e os marcadores da [DPIA](../../../governance/dpia.md).
3. Combinar os prazos de retenção com o gestor de documentos e agendar a limpeza.
4. Ativar backups cifrados e executar o ensaio de restauração na hospedagem da própria implantação.
5. Manter os dados não clínicos e abaixo do nível de marcação para o qual a implantação tem garantia.
6. Planear as lacunas acima ou financiar a sua eliminação.

As evidências de cada item estão na [avaliação](../../../governance/production-readiness.md).
