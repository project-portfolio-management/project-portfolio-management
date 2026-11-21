# Confronti

*Un riepilogo di come il software si pone rispetto ad altri prodotti.*

I prodotti qui sotto hanno scopi diversi e non ricevono punteggi. Queste note si basano sul materiale pubblico di ciascun progetto. **Nessun altro prodotto è stato installato o eseguito**, e ciascuno può essere cambiato nel frattempo. Prima di prendere una decisione va consultata la documentazione aggiornata del prodotto.

| Progetto | Che cos'è | Dove questo progetto differisce |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Gestione dei progetti per chi non è un project manager | Questo progetto comprende anche le funzioni di pianificazione, costi e governance usate da un ufficio di portafoglio |
| [Redmine](https://www.redmine.org/) | Un sistema di tracciamento generico e configurabile, con tipi di problema, stati e ruoli definiti dall'organizzazione | Tipi di problema, stati e flussi di lavoro sono definiti dall'organizzazione. I ruoli portano permessi sui problemi, assegnati piano per piano, entro ciò che l'accesso di una persona consente. Vedi il [confronto con Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (tramite il suo fork, Platform Collective) | Un framework per applicazioni aziendali basato su molti servizi | Huly è una piattaforma a più servizi. Questo progetto è un unico servizio su PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Un'alternativa desktop a Microsoft Project, per un solo responsabile e un solo progetto | Questo progetto è un servizio ospitato, multiutente, con controllo degli accessi |

## Confronti tra prodotti

Pagine di confronto singole (in inglese): [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interoperabilità

- **Jira e Asana:** importazione CSV, con anteprima e il motivo dichiarato di tutto ciò che viene escluso.
- **Microsoft Project:** importazione ed esportazione di XML MSPDI, che coprono attività, struttura, date, durate, vincoli, dipendenze e calendario. Assegnatari, risorse e linee di base sono esclusi, e il file lo registra.

## Differenze di progettazione

- **Motivi dichiarati.** Una cifra mancante è mostrata con un motivo, per esempio «nessuna previsione, perché…».
- **Rilevamento spiegabile dei duplicati.** La corrispondenza funziona su tutti i tipi di piano e mostra le proprie prove.
- **Impostazioni di sicurezza predefinite.** L'accesso con credenziali è attivo per impostazione predefinita e il materiale sulla sicurezza è incluso nel repository.
- **Durate esplicite.** Ogni durata è indicata in giorni di calendario o in giorni lavorativi.
- **Limiti registrati.** Il backlog registra i limiti di ogni voce.

## Limiti noti

- Nessuna app mobile né applicazione desktop.
- Nessuna modifica collaborativa in tempo reale.
- Un solo manutentore, uno storico di produzione limitato e nessun audit di terzi.
- La prova di carico pubblicata è stata eseguita su una sola macchina. Indica un ordine di grandezza e non è un piano di capacità.
- Le traduzioni sono bozze in attesa di revisione.
- Nessuna schermata per redigere o decidere le richieste di modifica.
- Nessun ecosistema di plugin o marketplace.

Ulteriori dettagli sono nelle [note di confronto](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Le correzioni possono essere inviate come segnalazione con una fonte.
