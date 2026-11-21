# Per il direttore della strategia

*Direttore della strategia: come il software collega la strategia al lavoro, che cosa ti mostra e che cosa non può ancora.*

## Che cosa fa per la strategia

- **Un unico registro di tutto ciò che è finanziato.** Portafogli, programmi, progetti e prodotti sono un unico record ricorsivo, così l'intero patrimonio si legge come un albero ([scopo e visione](../../../../spec/01-purpose-and-vision.md)).
- **Rilevamento dei duplicati.** La corrispondenza spiegabile mostra quando due parti dell'organizzazione hanno avviato la stessa iniziativa, prima che entrambe siano finanziate.
- **Obiettivi strategici e mappa del portafoglio.** Contrassegni gli obiettivi come strategici e indichi un obiettivo primario per ogni programma. `GET /api/portfolio-map` e il pacchetto mensile per la direzione raggruppano i programmi per obiettivo primario. Un programma compare una sola volta, con gli altri obiettivi indicati; un programma senza obiettivo primario è elencato sotto «nessun obiettivo strategico», e i totali non sono mai contati due volte.
- **Affidabilità di consegna accanto allo stato calcolato.** Le persone dichiarano la propria fiducia nella consegna; il software la mostra accanto al rosso/ambra/verde calcolato e segnala uno scarto di due livelli. Un piano che nessuno ha valutato mostra «non valutato», mai verde.
- **Un pacchetto mensile congelato.** Ciò che è stato comunicato alla direzione è conservato esattamente come emesso, con la tendenza misurata rispetto al pacchetto emesso precedente.

## Che cosa non fa ancora

- Il pacchetto non contiene ancora le decisioni di passaggio, i finanziamenti per esercizio finanziario, il grafico a febbre dei buffer, il valore guadagnato, i benefici convalidati né la mappa termica della capacità (da `DP-5` a `DP-10` in [`tasks.md`](../../../../tasks.md)). Il pacchetto dice quali sezioni non sono realizzate.
- «Programma» indica un piano il cui tipo è `Program`, perché i dati non ne hanno un'altra definizione; gli obiettivi «di vertice» sono quelli che contrassegni tu, poiché gli obiettivi non hanno gerarchia.
- Non c'è pianificazione per scenari e non c'è ancora una schermata per la mappa o per il pacchetto: sono disponibili come dati.
- Non è certificato e ha un solo manutentore ([percorso verso la produzione](../../path-to-production/index.md)).

## Decisioni che spettano a te

Quali obiettivi sono strategici; l'obiettivo primario di ogni programma; chi dichiara e chi valuta in modo indipendente l'affidabilità di consegna; quanti mesi di tendenza deve vedere la direzione.

## Da leggere dopo

[Tabella di marcia](../../../../spec/15-roadmap.md) · [Questioni aperte](../../../../spec/16-open-questions.md) · [Ambito](../../../../spec/02-scope.md)
