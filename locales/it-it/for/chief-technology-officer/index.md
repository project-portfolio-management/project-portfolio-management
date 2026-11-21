# Per il direttore tecnico

*Direttore tecnico: architettura, qualità, come estenderlo e dove l'ingegneria è debole.*

## Architettura

Tre sottoprogetti, un'unica entità ([architettura](../../../../spec/08-architecture.md)):

| Parte | Tecnologia |
|---|---|
| Libreria di corrispondenza | Rust; confronto a coppie deterministico e probabilistico, spiegabile; incorporata nel servizio |
| Servizio | Rust, axa su axum, sqlx, PostgreSQL 18; REST sotto `/api`, OpenAPI derivato dai percorsi |
| Interfaccia | SvelteKit 3, SPA Svelte 5 con un backend-for-frontend; cataloghi i18n |

La toolchain è fissata in [`rust-toolchain.toml`](../../../../rust-toolchain.toml). La persistenza è nella [specifica 10](../../../../spec/10-persistence.md); l'API nella [specifica 09](../../../../spec/09-api-surface.md).

## Punti di forza ingegneristici da conoscere

- **Gli invarianti vivono nel database, dove contano:** il registro di audit rifiuta modifiche e cancellazioni; i pacchetti per la direzione emessi e le valutazioni di affidabilità di consegna sono immutabili o a sola aggiunta tramite trigger; i vincoli CHECK sostengono le regole del servizio.
- **Test di guardia:** i test di censimento fanno fallire la compilazione quando un nuovo percorso aggregato salta il filtro dei piani privati, un percorso di elenco non è paginato, una colonna di dati personali manca dall'inventario o un percorso manca da OpenAPI. Si correggono classificando la novità, non indebolendo il test.
- **Test con token reali:** l'autorizzazione è provata da un capo all'altro con token PASETO firmati (ogni test in un processo a sé, perché le impostazioni di autenticazione si leggono una volta per processo).
- **Catena di fornitura:** `cargo deny`, una distinta dei componenti software, provenienza della compilazione al rilascio, una politica di sicurezza.
- **La CI** compila e prova tutto a ogni push; vedi `.github/workflows/`.

## Dove è debole

- Un solo manutentore, quindi il bus factor è uno ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- Le prestazioni si conoscono solo approssimativamente; la cache delle risposte copre solo il percorso del percorso critico; i piani molto grandi sono lenti.
- Le sessioni sono in memoria a meno che Redis sia configurato.
- Diverse sezioni del pacchetto per la direzione e tutte le schermate del pacchetto non sono realizzate ([`tasks.md`](../../../../tasks.md)).
- Per costruirlo si usano strumenti di IA; il prodotto non contiene alcun modello. Vedi la [dichiarazione sull'IA](../../../../AI_STATEMENT.md).

## Decisioni che spettano a te

Hosting e topologia di Redis, se creare un fork o seguire l'upstream, chi rivede il codice in modo indipendente e come il servizio si inserisce nel tuo provider di identità (insieme di chiavi PASETO, oppure OIDC tramite Keycloak o Entra).

## Da leggere dopo

[Architettura](../../../../spec/08-architecture.md) · [Strategia di test](../../../../spec/11-testing-strategy.md) · [Stato dell'implementazione](../../../../spec/14-implementation-status.md) · [Benchmark](../../../../BENCHMARKS.md)
