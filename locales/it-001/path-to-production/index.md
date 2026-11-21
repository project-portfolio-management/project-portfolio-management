# Percorso verso la produzione

*Che cosa significa «produzione» per questo software e quanto ne è distante, voce per voce.*

«Produzione» qui significa che un'organizzazione usa il software per lavoro reale. Piani reali, nomi di persone reali
e decisioni reali dipendono da esso, un'interruzione o una fuga di dati ha conseguenze e qualcuno ne risponde. In un
grande ente sanitario o in un ministero, produzione significa anche aver superato le verifiche di garanzia della
propria organizzazione: un test di sicurezza, una valutazione d'impatto sulla protezione dei dati, un audit di
accessibilità e, nel Servizio sanitario nazionale britannico, i Digital Technology Assessment Criteria (DTAC).

## A che punto siamo

- Non si dichiara alcuna certificazione, approvazione o audit. Questa pagina è la stima del progetto stesso.
- «Medico» indica uno strumento **non clinico** per portafogli e progetti: nessuna cartella clinica e nessun supporto alle decisioni cliniche.
- Il progetto suggerisce di iniziare con un pilota su dati non clinici e non riservati, gestito da un'organizzazione che completi
  le proprie verifiche di garanzia.

## Prontezza, voce per voce

Ogni cifra è la stima del progetto al 13 novembre 2026, non una misurazione. Ogni cursore parte da quella stima.
Spostare un cursore registra il tuo parere mentre leggi; nulla viene salvato o inviato.

- **Garanzia di sicurezza indipendente** — 0%. Nessun test di penetrazione, nessun audit di terzi e nessuna certificazione. Un acquirente o un fornitore deve commissionarli (RG-13).
- **Accessibilità** — 40%. Un audit automatico WCAG 2.2 AA viene eseguito su ogni percorso a ogni modifica. Non è stato fatto alcun test manuale, né alcun test con tecnologie assistive o con persone con disabilità (RG-13).
- **Richieste degli interessati** — 50%. Gli amministratori possono esportare e pseudonimizzare una persona in ogni riferimento dello schema, con un'anteprima prima (RG-6). I nomi digitati in titoli, commenti o documenti, i file caricati, le istantanee di audit e i backup non vengono trovati automaticamente.
- **Conservazione** — 35%. Una pulizia, in prova per impostazione predefinita, copre notifiche, registri di consegna, chiavi di idempotenza e istantanee di tendenza e di adozione, con periodi fissati per ogni installazione (RG-4). Piani, attività, registrazioni di tempo, documenti e traccia di audit vengono conservati finché un operatore non li elimina.
- **Contrassegni di trattamento** — 60%. Una classificazione configurabile nasconde i piani oltre il nulla osta di chi chiama, amministratori compresi, ed etichetta ogni risposta (RG-7). Il contrassegno non è impresso nei file generati, e il nulla osta vale quanto l'attributo del provider di identità.
- **Sicurezza degli allegati** — 50%. Il contenuto deve corrispondere al tipo dichiarato, i programmi sono rifiutati e una scansione `clamd` facoltativa si chiude in caso di errore (RG-8). Non è incluso alcuno scanner e il punto di aggancio è stato provato solo con un sostituto.
- **Revisione degli accessi e accesso di emergenza** — 45%. Un rapporto sulle concessioni permanenti supporta la ricertificazione (RG-9), e l'accesso di emergenza ha una durata limitata e richiede un motivo dichiarato (RG-10). Il rapporto mostra ciò che è stato registrato, non ciò che il provider di identità applica; nessun percorso registra la decisione di un revisore; e una notifica di accesso di emergenza può indicare qualcuno che non riceve nulla mentre la concessione resta valida.
- **Prestazioni e capacità** — 40%. Una prova di carico e di resistenza su una sola macchina è in [`BENCHMARKS.md`](../../../BENCHMARKS.md): le richieste ordinarie rispondono in decine di millisecondi, un piano di circa 12.700 attività è lento da pianificare e il processo cresce fino a circa 1 GB. Non c'è un piano di capacità, né un test con molti utenti, né un'esecuzione su più macchine.
- **Sessioni su più server front-end** — 70%. Le sessioni sono tenute in memoria a meno che Redis sia configurato (RG-12); con Redis, l'accesso dipende da esso.
- **Integrità della traccia di audit** — 50%. Le scritture sono registrate e un trigger del database rifiuta modifiche e cancellazioni, con eccezioni dichiarate (RG-3). Un amministratore del database può eliminare il trigger, e non c'è una catena di hash.
- **Backup e ripristino** — 80%. Un ripristino è provato nell'integrazione continua, in chiaro e cifrato, rispetto a obiettivi di ripristino dichiarati (RG-5). La cifratura è facoltativa, e la prova gira sulla CI del progetto, non sull'hosting di chi installa.
- **Supporto e continuità** — 10%. Un solo manutentore e nessun accordo sul livello di servizio ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Traduzioni** — 20%. Il testo in lingue diverse dall'inglese è scritto da una macchina finché un parlante non lo rivede, e gli obblighi linguistici di legge richiedono un testo rivisto ([dichiarazione sulle lingue](../../../governance/language-statement.md)).
- **Uso clinico** — 0%. Non è stato scritto alcun caso di sicurezza clinica, e l'uso clinico è fuori ambito.

## Che cosa dovrebbe fare una prima installazione

1. Completare la [lista di controllo della garanzia](../../../governance/assurance-checklist.md), cominciando da un test di penetrazione e da un audit di accessibilità.
2. Compilare ogni riga «Buyer to complete» nella [lista di controllo DTAC](../../../governance/dtac-checklist.md) e i segnaposto nella [DPIA](../../../governance/dpia.md).
3. Concordare i periodi di conservazione con il responsabile dei registri e pianificare la pulizia.
4. Attivare i backup cifrati ed eseguire la prova di ripristino sull'hosting dell'installazione.
5. Mantenere i dati non clinici e al di sotto del livello di classificazione per cui l'installazione ha ottenuto la garanzia.
6. Pianificare le lacune sopra indicate, o finanziarne la risoluzione.

Le prove per ogni voce sono nella [valutazione](../../../governance/production-readiness.md).
