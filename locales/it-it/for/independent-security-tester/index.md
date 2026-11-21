# Test di sicurezza indipendente

*Per un acquirente, un valutatore o un tester: che cosa dovrebbe coprire un test di sicurezza indipendente di questo software, che cosa il progetto fornisce per aiutare, che cosa è stato fatto e che cosa no, e che cosa resta a te. Questa è una guida, non un rapporto di test e non una consulenza legale.*

## A che punto siamo

**Non è stato eseguito alcun test di sicurezza indipendente.** Non esiste alcun test di penetrazione, audit di terzi, revisione indipendente del codice o certificazione per questo software, e il progetto non ne dichiara alcuno ([percorso verso la produzione](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). Una scansione passiva gira nell'integrazione continua del progetto (regole in `.zap/rules.tsv`), e i test del progetto verificano i controlli elencati sotto; nessuna delle due cose sostituisce un tester che non ha scritto il software.

Questa è la voce 1 e la voce 9 della [lista di controllo della garanzia](../../../../governance/assurance-checklist.md). Spetta a chi installa, con un tester certificato, sulla **tua** installazione: il progetto non può provare il tuo hosting, il tuo provider di identità, la tua rete o le tue chiavi.

## Che cosa provare

Prova l'installazione così come funzionerà, con l'accesso imposto e il tuo provider di identità davanti. Ambito suggerito, in base ai controlli che il software dichiara:

| Area | Che cosa dichiara il software | Che cosa un tester dovrebbe tentare |
|---|---|---|
| Autenticazione | L'accesso è attivo per impostazione predefinita; il servizio non si avvia senza chiavi di verifica o, in produzione, con l'accesso disattivato; i token sono verificati offline rispetto a chiavi pubblicate | Token contraffatti, scaduti, riprodotti e firmati in modo errato; emittente o destinatario sbagliato; rotazione delle chiavi; un insieme di chiavi mancante |
| Autorizzazione | Politica basata sugli attributi; regole a livello di record | Escalation di privilegi orizzontale e verticale su ogni percorso; modifica degli identificativi nei percorsi e nei corpi; attributi che un chiamante non dovrebbe poter aggiungere |
| Piani privati e contrassegni di trattamento | Un piano privato, e un piano oltre il nulla osta di chi chiama, non è in alcun elenco, conteggio, totale, risultato di ricerca, feed o esportazione per quel chiamante, amministratori compresi | Qualsiasi percorso che riveli l'esistenza, un nome, un conteggio o un totale di un piano che il chiamante non può leggere; messaggi di errore e tempi che differiscono tra «nascosto» e «assente» |
| Accesso di emergenza (break-glass) | Solo persone idonee, un motivo dichiarato, al massimo quattro ore, una seconda persona avvisata | Uso senza idoneità; estensione di una concessione; azione senza che la seconda persona sia avvisata |
| Record congelati e a sola aggiunta | La traccia di audit, i pacchetti per la direzione emessi, lo storico dell'affidabilità di consegna, lo storico dello stato dei programmi e i buffer di progetto rifiutano modifiche e cancellazioni nel database | Aggiornamento o cancellazione tramite l'API, tramite importazione in blocco e tramite qualsiasi percorso che tocchi le stesse righe |
| Separazione dei compiti | Chi richiede non può approvare la propria richiesta di modifica | Approvazione come richiedente, come amministratore e cambiando l'approvatore dopo l'invio |
| Caricamenti e allegati | Il contenuto deve corrispondere al tipo dichiarato; i programmi sono rifiutati; un punto di aggancio facoltativo per la scansione antivirus si chiude in caso di errore | File poliglotti, programmi rinominati, file troppo grandi o malformati, trucchi coi percorsi nei nomi e se il tuo scanner viene davvero chiamato |
| Richieste in uscita | Le destinazioni fornite dall'utente sono limitate a indirizzi `https` pubblici | Server-side request forgery verso indirizzi privati, link-local e di metadati, reindirizzamenti, DNS rebinding |
| Sessioni e browser | Timeout per inattività, un limite massimo, un cookie `httpOnly`, una Content Security Policy rigorosa, un elenco consentito per CORS | Fissazione e riuso della sessione, cross-site request forgery, iniezione nell'interfaccia, clickjacking |
| Limitazione della frequenza | Limiti per chiamante | Aggiramento cambiando identità, intestazione o percorso; se i limiti sopravvivono al tuo proxy |
| Strumenti per gli interessati | Esportazione e pseudonimizzazione, solo amministratori, con anteprima prima | Uso da parte di chi non è amministratore; completezza rispetto all'[inventario dei dati personali](../../../../governance/personal-data-inventory.csv) |
| Backup | Cifratura facoltativa, ripristino collaudato | Accesso ai file di backup e alle chiavi; ripristino in un ambiente pulito |

## Che cosa fornisce il progetto

- Il codice sorgente, la specifica e i test, così una revisione del codice può partire dalle dichiarazioni.
- Una descrizione leggibile da macchina dell'API (`/api-docs/openapi.json`), con gli schemi di richiesta e risposta per il lavoro sul pacchetto per la direzione; l'elenco dei percorsi è verificato rispetto al codice da test.
- Una configurazione usa e getta con l'accesso disattivato (`dev-insecure`, vedi il [README](../../../../README.md)) **solo per la macchina del tester**, e uno script che genera un insieme di chiavi di prova (`scripts/smoke-keys.sh`) così da poter emettere token con attributi scelti per un'installazione di prova.
- Il [diagramma del flusso dei dati](../../../../governance/data-flow.md), il [manuale operativo](../../../../operations/runbook.md) e gli [obiettivi di ripristino](../../../../operations/recovery-objectives.md).
- Un modo privato per segnalare ciò che si trova: [SECURITY.md](../../../../SECURITY.md).

## Regole di ingaggio

Concordale per iscritto prima di qualsiasi test; il progetto non può fissarle per il tuo ambiente.

- **Ambiente.** Prova una copia con dati realistici ma non reali. Non provare la produzione a meno che tu non l'abbia deciso per iscritto.
- **Autorizzazione.** Il tester ha il tuo permesso scritto, un referente indicato e una finestra di tempo dichiarata.
- **Fuori ambito.** Denial of service per volume, ingegneria sociale sul personale e servizi di terzi (il tuo provider di identità, il tuo host) sono fuori ambito a meno che il proprietario di ciascuno acconsenta. L'ambito del progetto stesso è in [SECURITY](../../../../SECURITY.md).
- **Dati.** I dati personali reali non si usano, non si copiano all'esterno e non entrano nel rapporto.
- **Chiavi.** Le chiavi di prova sono separate da quelle di produzione e vengono distrutte dopo.

## Rapporto e nuova prova

- I rilievi su questo software vanno al manutentore in privato, come dice [SECURITY](../../../../SECURITY.md), con come riprodurli, la versione o il commit e l'impatto. Gli obiettivi del progetto sono un riscontro entro 5 giorni lavorativi e un piano entro 15; sono obiettivi, non garanzie. C'è un solo manutentore e nessun accordo sul livello di servizio.
- I rilievi sulla tua installazione vanno al tuo responsabile.
- Chiedi al tester di valutare ogni rilievo, di dire che cosa **non** è stato provato e di ripetere la prova delle correzioni. Un rapporto che elenca solo ciò che è stato trovato non ti dice che cosa è sfuggito.
- Conserva il rapporto, l'ambito, le date e la nuova prova insieme ai tuoi registri. Il progetto non può conservarli per te.

## Limiti di questo documento

Elenca aree da coprire, non un piano di test completo, e le dichiarazioni nella tabella sono quelle del progetto. Non certifica nulla, e un test pulito è una prova su una sola installazione in una sola data.

## Decisioni che spettano a te

Chi esegue il test, quando, su che cosa, con quali dati e con quali regole; come si valutano i rilievi e entro quando vanno corretti; e se il risultato è condiviso con i tuoi valutatori.

## Da leggere dopo

[Lista di controllo della garanzia](../../../../governance/assurance-checklist.md) · [Percorso verso la produzione](../../path-to-production/index.md) · [Responsabile della sicurezza](../chief-security-officer/index.md) · [Governance, rischio e conformità](../governance-risk-compliance/index.md)
