# Per il direttore operativo

*Direttore operativo: che cosa gestisci ogni giorno, che cosa fa il software per la consegna e che cosa richiede farlo funzionare.*

## Che cosa fa per le operazioni

- **Pianificazione e controllo della consegna:** piani, attività, dipendenze, calcolo del percorso critico, tappe tra piani, sprint e fasi, burndown e metriche di flusso.
- **Capacità:** assegnazioni, disponibilità, ferie e periodi non lavorativi, ruoli delle risorse, carico di lavoro e utilizzo derivati. (Una mappa termica della capacità per gruppo di competenze e mese per il pacchetto della direzione non è realizzata: `DP-10`.)
- **Tempo e costi:** fogli presenze, tempo registrato, tariffe di costo (riservate alla finanza), budget e costi derivati.
- **Flusso di governance:** registro dei problemi, richieste di modifica con separazione dei compiti (chi le richiede non può approvarle), approvazioni, revisioni, casi di ingresso, trasferimento e uscita.
- **Eccezioni:** un'affidabilità di consegna rossa o ambra-rosso apre un'eccezione che richiede una causa, la decisione necessaria, un responsabile indicato e una data di decisione entro 48 ore (finestra configurabile, da 1 a 720 ore). Le violazioni compaiono nel pacchetto per la direzione e in `GET /api/exceptions`.

## Che cosa richiede farlo funzionare

| Necessità | Dettaglio |
|---|---|
| Servizio | Un servizio Rust, PostgreSQL 18, Redis facoltativo (cache di produzione, sessioni condivise facoltative), un'interfaccia SvelteKit |
| Manuale operativo | [pacchetto operativo](../../../../operations/README.md): installazione, manuale operativo, backup e ripristino, obiettivi di ripristino, aggiornamento e migrazione |
| Lavori pianificati | Nulla si pianifica da sé. La pulizia per la conservazione, gli avvisi di violazione delle eccezioni e simili vanno messi in un pianificatore dai tuoi operatori |
| Supporto | Un solo manutentore, nessun accordo sul livello di servizio; prevedi un budget per il tuo supporto di primo livello |

## Conosci i limiti

- Un piano di diverse migliaia di attività è lento da pianificare; la risposta completa è rifiutata oltre 5.000 attività se non richiesta (`?full=true` o `?only=critical`). Cifre e avvertenze sono in [`BENCHMARKS.md`](../../../../BENCHMARKS.md); sono un ordine di grandezza da una sola macchina, non una capacità.
- Esistono schermate per gran parte dello spazio di lavoro, ma **non** ancora per richieste di modifica, problemi, pacchetti o affidabilità di consegna; oggi questi sono disponibili solo tramite API.

## Decisioni che spettano a te

Chi è responsabile di ogni eccezione, chi emette il pacchetto mensile (cadenza e promemoria sono `DP-12`, non realizzati) e con quale cadenza di revisione si alimenta la regola delle eccezioni.

## Da leggere dopo

[Pacchetto operativo](../../../../operations/README.md) · [Manuale operativo](../../../../operations/runbook.md) · [Obiettivi di ripristino](../../../../operations/recovery-objectives.md)
