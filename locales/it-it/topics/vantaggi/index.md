# Vantaggi

*I principali vantaggi per un'organizzazione, per area.*

## Visibilità del portafoglio

Le informazioni sul portafoglio sono spesso sparse in più luoghi. In questo software ogni portafoglio, programma, progetto e prodotto è un unico tipo di record, chiamato **piano**. Ogni piano può contenere qualsiasi altro, a qualunque profondità, così la struttura può rispecchiare l'organizzazione. Un'etichetta come «Progetto» è solo descrittiva e non pone alcun limite a un piano.

## Meno duplicati

Due reparti possono avviare la stessa iniziativa con nomi diversi. Il confronto mette a paragone gli attributi che identificano un piano: nome, obiettivi, responsabile, promotore, piano superiore, periodo, parole chiave e identificativi. Funziona su tutti i tipi di piano, mostra le prove di ogni corrispondenza e la decisione spetta a una persona. Due record dello stesso piano possono essere uniti.

## Qualità del reporting

- Una cifra mancante è mostrata con un motivo, per esempio «nessuna previsione, perché…».
- Un piano senza valutazione è indicato come «non valutato».
- L'affidabilità di consegna indicata dalle persone è mostrata accanto allo stato calcolato, e i due valori restano separati.
- I piani privati sono esclusi da tutti i totali e conteggi per tutti gli utenti. Dei test verificano questa regola.
- Ogni durata indica se è in giorni di calendario o in giorni lavorativi.

## Registro delle decisioni

Ogni modifica è registrata e la traccia di audit non può essere alterata tramite il servizio. Chi presenta una richiesta di modifica non può approvarla. Un pacchetto mensile per la direzione viene redatto una volta, emesso da una persona indicata e poi congelato dal database. Una correzione viene emessa come nuova versione con una nota.

## Sicurezza

L'accesso con credenziali è attivo per impostazione predefinita, e il servizio non si avvia in produzione con l'accesso disattivato. Intestazioni di sicurezza, limitazione della frequenza delle richieste, un piano di ripristino con obiettivi dichiarati e un pacchetto di governance delle informazioni sono inclusi nel repository.

## Controllo e portabilità

- **Licenze.** Il software è open source, a scelta tra MIT, Apache-2.0, BSD-3-Clause, GPL-2.0 o GPL-3.0.
- **Hosting.** Il software è ospitato in proprio: un servizio Rust su PostgreSQL, con un'interfaccia SvelteKit.
- **Importazione dei dati.** I file CSV di Jira e Asana possono essere importati, con anteprima e il motivo dichiarato di tutto ciò che viene escluso. Il formato XML di Microsoft Project può essere importato ed esportato.
- **Lingue.** L'interfaccia è disponibile in dieci impostazioni locali, compreso l'arabo da destra a sinistra. Le traduzioni sono bozze in attesa di revisione da parte di parlanti.

## Pagine correlate

Vedi anche le [funzionalità](../funzionalità/index.md), i [confronti](../confronti/index.md) e la [guida](../guida/index.md).
