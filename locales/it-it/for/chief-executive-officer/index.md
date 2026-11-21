# Per l'amministratore delegato

*Amministratore delegato: che cos'è questo software, che cosa ti offre, che cosa non offre e le decisioni che spettano a te.*

## Che cos'è, in un paragrafo

Un registro di ogni **piano** che l'organizzazione finanzia (portafogli, programmi, progetti, prodotti e così via, un unico record ricorsivo), con sotto uno spazio di lavoro per la gestione dei progetti (attività, pianificazione, costi, risorse, rischi, problemi, richieste di modifica, governance). Trova le iniziative duplicate con una corrispondenza spiegabile, così due reparti che avviano lo stesso lavoro emergono prima che entrambi siano finanziati. Vedi [scopo e visione](../../../../spec/01-purpose-and-vision.md).

## Su che cosa puoi contare

- **Un unico posto dove chiedere «che cosa stiamo facendo?»** I piani si annidano a qualsiasi profondità; i piani privati restano fuori da ogni totale e conteggio, per tutti, secondo una regola che i test verificano.
- **Un pacchetto mensile per la direzione** redatto una volta, emesso da una persona indicata e poi **congelato dal database**: un pacchetto emesso non si può modificare; una correzione è una nuova versione riemessa con una nota. Oggi contiene la mappa del portafoglio per obiettivo strategico, l'affidabilità di consegna, le eccezioni, i problemi e le richieste di modifica.
- **L'affidabilità di consegna indicata dalle persone, mostrata accanto allo stato calcolato, mai mescolata con esso.** Un piano che nessuno ha valutato mostra «non valutato», mai verde. Una valutazione ambra-rosso o rosso apre un'eccezione con un orologio di 48 ore (configurabile); un orologio scaduto è mostrato nel pacchetto.
- **Le decisioni lasciano una traccia.** Ogni modifica è registrata, la traccia di audit non può essere alterata tramite il servizio, e una richiesta di modifica non può essere approvata da chi l'ha presentata.

## Che cosa non ti offre ancora

- Il pacchetto **non** comprende ancora le decisioni di passaggio, i finanziamenti e il denaro per esercizio finanziario, il grafico a febbre dei buffer, il valore guadagnato, i benefici convalidati né la mappa termica della capacità. Sono elencati da `DP-5` a `DP-10` in [`tasks.md`](../../../../tasks.md), e il pacchetto dice quali sezioni non sono ancora realizzate invece di lasciare uno spazio vuoto.
- Nessun PDF né schermata per il pacchetto: è fornito come dati (`DP-11`).
- Non è **certificato né sottoposto ad audit** da nessuno. Che cosa servirebbe per l'uso in produzione, voce per voce e con una stima per ciascuna, è nella pagina [percorso verso la produzione](../../path-to-production/index.md).
- Un solo manutentore, nessun accordo sul livello di servizio ([governance](../../../../GOVERNANCE.md)).

## Decisioni che spettano a te

1. Se avviare un pilota, e su quale parte del portafoglio.
2. Chi è il **promotore** per la regola delle eccezioni a 48 ore e chi può emettere il pacchetto mensile.
3. Se finanziare le lacune sopra indicate (valutazione, test di penetrazione, accordo di supporto) o accettarle.

## Da leggere dopo

[Percorso verso la produzione](../../path-to-production/index.md) · [Tabella di marcia](../../../../spec/15-roadmap.md) · [Questioni aperte](../../../../spec/16-open-questions.md)
