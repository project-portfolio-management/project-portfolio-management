# Per il direttore dei rischi

*Direttore dei rischi: i rischi che il software ti aiuta a vedere e i rischi legati al suo uso.*

## Rischi che ti aiuta a gestire

- **Rischi, problemi e richieste di modifica** registrati per piano; un registro dei problemi tra i piani leggibili; richieste di modifica con separazione dei compiti.
- **Affidabilità di consegna ed eccezioni:** l'affidabilità dichiarata è tenuta separata dal RAG calcolato, segnalata quando divergono di due livelli, con uno storico a sola aggiunta, e un orologio delle eccezioni di 48 ore che non riparte quando una valutazione peggiora e si chiude solo con una decisione registrata.
- **Assenza onesta:** i dati mancanti compaiono come «non valutato» o nulli con un motivo, mai come verde o zero.
- **Un pacchetto mensile congelato e versionato,** così ciò che è stato comunicato alla direzione si può mostrare in seguito esattamente come è stato emesso.

## Rischi legati all'uso (il registro del progetto stesso)

| Rischio | Stato |
|---|---|
| Nessun test di penetrazione, audit di terzi o certificazione | Aperto: spetta all'acquirente commissionarli ([lista di controllo della garanzia](../../../../governance/assurance-checklist.md)) |
| Conservazione applicata solo a una parte dei dati | Aperto; la pulizia va pianificata |
| Lo strumento per gli interessati non cerca nel testo libero, negli allegati o nei backup | Aperto |
| La traccia di audit può essere aggirata da un amministratore del database; nessuna catena di hash | Dichiarato nella [DPIA](../../../../governance/dpia.md) |
| I contrassegni di trattamento nascondono ed etichettano, non cifrano | Dichiarato |
| La scansione degli allegati richiede uno scanner gestito da chi installa | Aperto |
| Un solo manutentore, nessun accordo sul livello di servizio | Aperto |
| Le traduzioni sono bozze | Aperto |
| Solo uso non clinico; nessun caso di sicurezza clinica | Non inserire contenuti clinici |

## Controlli che il software applica

Accesso con credenziali per impostazione predefinita, limitazione della frequenza delle richieste, intestazioni di sicurezza rigorose, un timeout di sessione per inattività (30 minuti) e un limite massimo di sessione, destinazioni in uscita limitate a `https` pubblico, allegati con firma verificata, righe di audit immutabili, accesso di emergenza con audit. Ciascuno è coperto da un test citato nel [percorso verso la produzione](../../path-to-production/index.md).

## Decisioni che spettano a te

La propensione al rischio per un pilota; la finestra delle eccezioni (predefinita 48 ore); per quale livello di classificazione l'installazione ha ottenuto la garanzia; chi esamina l'elenco degli accessi e con quale frequenza.

## Da leggere dopo

[Percorso verso la produzione](../../path-to-production/index.md) · [DPIA](../../../../governance/dpia.md) · [Conformità](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
