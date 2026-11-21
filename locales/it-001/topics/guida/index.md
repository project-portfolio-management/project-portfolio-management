# Guida

*I passaggi per eseguire la dimostrazione su un computer locale ed esaminare un portafoglio di esempio.*

La dimostrazione è una copia completa del software, caricata con i dati di un'organizzazione fittizia, la Northgate Transport Authority. Tutte le persone, i piani, le cifre e le date sono fittizi, e i dati si possono modificare liberamente.

**Nota sulla sicurezza.** La dimostrazione si fida di una chiave di prova pubblicata. Non deve essere eseguita su una rete condivisa né usata con dati reali.

## Requisiti

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) con Compose

## Passaggio 1: scaricare il codice

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Passaggio 2: avviare la dimostrazione

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Questo comando compila il software, avvia il database, carica il portafoglio fittizio e avvia l'interfaccia. La prima compilazione richiede più tempo delle successive.

## Passaggio 3: accedere

Apri <http://localhost:3000/it-001/signin>, lascia l'indirizzo `example@example.com` e premi **Sign in to the demonstration**. Non viene inviata alcuna email. Questa guida mantiene le etichette inglesi dell'interfaccia della dimostrazione, perché è così che appaiono a schermo.

La prima pagina è il **demonstration tour**, che elenca ogni funzionalità per gruppo, con una descrizione e un collegamento. Un banner giallo su ogni pagina indica che i dati sono fittizi.

## Passaggio 4: esaminare il portafoglio

Dal tour apri **Plans** per vedere il portafoglio, i suoi programmi e i suoi progetti. Poi apri la **Portfolio dashboard**.

## Passaggio 5: esaminare un progetto

Apri il progetto **North Line interlocking** dal gruppo di consegna del tour. Esamina il suo **Gantt**, il **Critical path** e l'**Earned value**.

## Passaggio 6: esaminare il lavoro personale

Apri **My work** e **My kanban board**. La sessione è avviata come Example Person, che ha attività aperte.

## Passaggio 7: esaminare capacità, controllo degli avvii e reporting

Apri **Capacity heatmap** per vedere le risorse sovraccariche. Apri **Start queue**, dove il limite di lavoro in corso ha trattenuto un programma. Apri **Director packs**, il riepilogo mensile che viene congelato una volta emesso.

## Passaggio 8: fermare la dimostrazione

Questo comando ferma tutti i servizi ed elimina i dati della dimostrazione:

```sh
podman compose --profile demo down -v
```

## Prossimi passi

- [Guida all'installazione](../../../../INSTALL.md), che tratta l'accesso e i provider di identità
- [Schede per ruolo](../../for/) per i ruoli dirigenziali
- [Guida alla dimostrazione](../../../../operations/demo.md)
- [Elenco delle funzionalità](../funzionalità/index.md)

I problemi possono essere segnalati tramite il [tracker delle segnalazioni](https://github.com/project-portfolio-management/project-portfolio-management/issues).
