# Governance, rischio e conformità

*Per chi gestisce governance, rischio e conformità: che cosa il software ti aiuta a documentare, che cosa applica e che cosa devi fornire tu. Non è una consulenza legale.*

## Governance

- **Le decisioni sono registrate.** Le richieste di modifica richiedono un approvatore indicato che non sia chi le richiede, un motivo dichiarato di almeno 10 caratteri, e sono registrate per autore; il database rifiuta un'approvazione senza chi decide e senza motivo.
- **Un pacchetto mensile congelato per la direzione.** Redatto una volta, emesso da una persona, immutabile una volta emesso; le correzioni sono una riemissione con una nota. L'accesso in lettura è per destinatari.
- **Affidabilità di consegna ed eccezioni.** L'affidabilità dichiarata è a sola aggiunta e mostrata accanto allo stato calcolato, mai mescolata. Una valutazione rossa o ambra-rosso apre un'eccezione con un orologio di 48 ore (configurabile) che solo una decisione registrata chiude.
- **Governance del progetto del software stesso:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [controllo delle modifiche](../../../../spec/18-change-control.md), [dichiarazione sull'IA](../../../../AI_STATEMENT.md).

## Rischio

- Rischi, problemi e richieste di modifica per piano; un registro dei problemi tra i piani leggibili.
- I dati mancanti sono mostrati come «non valutato», mai come verde.
- I rischi aperti del progetto stesso (nessun test di penetrazione, conservazione parziale, un solo manutentore, traduzioni in bozza) sono nella pagina [percorso verso la produzione](../../path-to-production/index.md), voce per voce con una stima per ciascuna.

## Conformità

| Area | Dove | Stato |
|---|---|---|
| Protezione dei dati (la legge applicabile; la [§12.2a](../../../../spec/12-compliance.md) mette in corrispondenza GDPR, UK GDPR, LGPD, PIPL, DPDP, PIPEDA, CCPA e altre) | [DPIA](../../../../governance/dpia.md), [registro dei trattamenti](../../../../governance/record-of-processing.md), [inventario dei dati personali](../../../../governance/personal-data-inventory.csv), [piano di conservazione](../../../../governance/retention-schedule.md), [flusso dei dati](../../../../governance/data-flow.md) | Precompilato; i segnaposto `‹buyer›` sono tuoi; conservazione applicata solo in parte |
| DTAC del servizio sanitario nazionale britannico (uno schema nazionale, un esempio svolto; gli altri schemi partono dalla [lista di controllo della garanzia](../../../../governance/assurance-checklist.md)) | [Lista di controllo DTAC](../../../../governance/dtac-checklist.md) | Risposto sezione per sezione; righe dell'acquirente e del fornitore aperte |
| Accessibilità | [dichiarazione](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA automatico; nessun test manuale |
| Obblighi linguistici | [dichiarazione sulle lingue](../../../../governance/language-statement.md) | Le traduzioni sono bozze |
| ISO/IEC 27001 | [specifica di conformità](../../../../spec/12-compliance.md) | Controlli lato installazione; non certificato |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [specifica di conformità](../../../../spec/12-compliance.md) §12.4 | Non coinvolti (nessun dato sui pazienti); dichiarato, non fabbricato |

## Prove e garanzia

La [lista di controllo della garanzia](../../../../governance/assurance-checklist.md) elenca 14 voci che un valutatore richiederà, con responsabile e stato. La maggior parte è aperta e spetta a chi installa: test di penetrazione, audit di accessibilità, esercitazione di ripristino di emergenza, garanzia sull'hosting, configurazione del provider di identità, ricertificazione degli accessi, processo per gli incidenti, gestione dei registri. La traccia di audit rifiuta modifiche e cancellazioni nel database, ma un amministratore del database può aggirarla; non c'è una catena di hash.

## Decisioni che spettano a te

Base giuridica e periodi di conservazione; finestra delle eccezioni; livello di classificazione dell'installazione; chi esamina gli accessi e con quale frequenza; se serve una procedura di blocco legale (nessuna è realizzata).

## Da leggere dopo

[Indice del pacchetto di governance](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Specifica di conformità](../../../../spec/12-compliance.md)
