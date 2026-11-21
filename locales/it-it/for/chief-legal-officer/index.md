# Per il direttore legale

*Direttore legale: licenza, protezione dei dati, accessibilità, obblighi linguistici e ciò che il progetto non garantisce. Questa non è una consulenza legale.*

## Licenza

Offerto con una qualsiasi tra MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, GPL-3.0-only, OGL-UK-3.0 o CC-BY-NC-SA-4.0, o con condizioni personalizzate previo accordo. La scelta e i testi sono in [`LICENSE.md`](../../../../LICENSE.md) e `LICENSES/`; un test mantiene coerenti con essi tutti i manifesti.

## Protezione dei dati (la legge che si applica a te)

Scritto per qualsiasi legge sulla protezione dei dati: la [specifica di conformità §12.2a](../../../../spec/12-compliance.md) mette in corrispondenza le più comuni per regione (GDPR e UK GDPR, LGPD, PIPL, DPDP Act, PIPEDA, CCPA e altre) con ciò che ciascuna comporta qui, in base a conoscenze generali e non verificato sul testo di ciascuna legge. Documenti precompilati da adattare; ogni segnaposto `‹buyer›` è una decisione che spetta a te (titolare del trattamento, base giuridica, periodi di conservazione, responsabili del trattamento, approvazione):

- [Modello di DPIA](../../../../governance/dpia.md) e [registro dei trattamenti](../../../../governance/record-of-processing.md)
- [Inventario dei dati personali](../../../../governance/personal-data-inventory.csv): ogni colonna che può contenere dati personali
- [Piano di conservazione](../../../../governance/retention-schedule.md): proposte, con ciò che il codice applica oggi
- [Diagramma del flusso dei dati](../../../../governance/data-flow.md)

Il software conserva le persone come riferimenti opachi insieme a tempo registrato, disponibilità e tariffe di costo, il che è vicino al monitoraggio e quindi richiede una DPIA. Offre l'esportazione dei dati dell'interessato e la cancellazione tramite pseudonimizzazione o, se l'installazione lo sceglie, eliminazione (solo amministratori, con anteprima prima; WW-1.8). I pacchetti per la direzione congelati e lo storico dell'affidabilità a sola aggiunta vengono svuotati della persona che li ha redatti o valutati, mai eliminati, così una richiesta di cancellazione può essere soddisfatta senza spezzare la registrazione. **Non** trova i nomi digitati in testo libero, e i backup contengono record che l'applicazione ha eliminato.

## Quadri normativi

HIPAA e i regimi statunitensi di certificazione dei dati sanitari **non sono coinvolti** (nessun dato sui pazienti); il regime dei dispositivi clinici non è coinvolto. La [lista di controllo DTAC](../../../../governance/dtac-checklist.md) risponde ai Digital Technology Assessment Criteria del servizio sanitario nazionale britannico, uno schema nazionale conservato come esempio svolto; per gli schemi di altri paesi e settori (ISO/IEC 27001, SOC 2, schemi nazionali per il cloud e per il settore pubblico) parti dalla [lista di controllo della garanzia](../../../../governance/assurance-checklist.md). Nulla è certificato.

## Accessibilità e lingua

WCAG 2.2 AA è verificato automaticamente su ogni percorso; non è stato eseguito alcun test manuale o con tecnologie assistive ([dichiarazione](../../../../governance/accessibility-statement.md)). Le traduzioni in gallese e in altre lingue sono bozze in attesa di un traduttore professionista ([dichiarazione sulle lingue](../../../../governance/language-statement.md)); rilevante se hai obblighi linguistici di legge.

## Verifica del fornitore

Un solo manutentore, nessun supporto contrattuale, nessuna garanzia dichiarata, strumenti di intelligenza artificiale usati nello sviluppo e dichiarati nella [dichiarazione sull'IA](../../../../AI_STATEMENT.md). Il prodotto non contiene alcun modello di IA. Segnalazione delle vulnerabilità: [SECURITY.md](../../../../SECURITY.md).

## Decisioni che spettano a te

Base giuridica e periodi di conservazione; se ti serve una procedura di blocco legale (nessuna è realizzata); condizioni contrattuali e di supporto con il manutentore.

## Da leggere dopo

[Lista di controllo della garanzia](../../../../governance/assurance-checklist.md) · [Indice del pacchetto di governance](../../../../governance/README.md)
