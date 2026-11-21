# Per il responsabile della sicurezza

*Responsabile della sicurezza: che cosa il software applica, che cosa il progetto ha verificato e che cosa devi provare e gestire tu.*

## Che cosa il software applica

| Area | Comportamento |
|---|---|
| Autenticazione | L'accesso con credenziali è attivo per impostazione predefinita. Il servizio non si avvia senza le chiavi di verifica dei token, e non si avvia in produzione con l'accesso disattivato. Autenticazione tramite un provider di identità come Microsoft Entra ID; l'autenticazione a più fattori resta al provider ([guida all'accesso](../../../../operations/sign-in-with-entra.md)) |
| Autorizzazione | Politica basata sugli attributi del token. I piani privati restano fuori da ogni aggregato per tutti; i contrassegni di trattamento nascondono i piani oltre il nulla osta di chi chiama, amministratori compresi |
| Accesso di emergenza (break-glass) | L'accesso ai piani privati richiede idoneità, un motivo dichiarato, al massimo quattro ore e una seconda persona avvisata; ogni richiesta è registrata |
| Sessioni | Timeout per inattività (30 minuti per impostazione predefinita) e un limite massimo di sessione |
| Trasporto e browser | Intestazioni di sicurezza, una Content Security Policy rigorosa, un elenco consentito per CORS, limitazione della frequenza per chiamante |
| In uscita | Le destinazioni fornite dall'utente sono limitate a indirizzi `https` pubblici |
| Caricamenti | Il contenuto deve corrispondere al tipo dichiarato; i programmi sono rifiutati; una scansione `clamd` facoltativa si chiude in caso di errore |
| Audit | La traccia di audit rifiuta modifiche e cancellazioni nel database |
| Record congelati | I pacchetti per la direzione emessi e le valutazioni di affidabilità di consegna sono immutabili o a sola aggiunta tramite trigger del database |
| Catena di fornitura | `cargo deny`, una distinta dei componenti software, provenienza della compilazione al rilascio, una [politica di sicurezza](../../../../SECURITY.md) |

## Che cosa non è stato fatto

- Nessun test di penetrazione, audit di terzi, revisione indipendente del codice o certificazione.
- Nessuno scanner antimalware incluso; il punto di aggancio `clamd` è stato provato solo con un sostituto.
- La traccia di audit può essere aggirata da un amministratore del database; non c'è una catena di hash.
- I contrassegni di trattamento nascondono ed etichettano; non cifrano, e un contrassegno non è impresso nei file generati.
- Le sessioni sono tenute in memoria a meno che Redis sia configurato.
- Lo strumento per gli interessati trova riferimenti, non nomi digitati in testo libero, file caricati o backup.

## Farlo funzionare

I backup hanno un ripristino collaudato, una cifratura facoltativa e obiettivi di ripristino dichiarati ([pacchetto operativo](../../../../operations/README.md)). Il [manuale operativo](../../../../operations/runbook.md) indica i passaggi tecnici per gli incidenti; il processo per incidenti e violazioni dei dati è tuo. Segnala una vulnerabilità in privato come descrive [SECURITY.md](../../../../SECURITY.md); gli obiettivi sono un riscontro entro 5 giorni lavorativi, non garanzie.

## Decisioni che spettano a te

Chi esegue il test di penetrazione e quando; l'accesso condizionale e la mappatura dei ruoli del provider di identità; il livello di classificazione per cui questa installazione ha ottenuto la garanzia; lo scanner dietro il punto di aggancio degli allegati; con quale frequenza gli accessi vengono ricertificati.

## Da leggere dopo

[Lista di controllo della garanzia](../../../../governance/assurance-checklist.md) · [Percorso verso la produzione](../../path-to-production/index.md) · [Flusso dei dati](../../../../governance/data-flow.md)
