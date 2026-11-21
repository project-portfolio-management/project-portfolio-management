# Per il direttore dei sistemi informativi

*Direttore dei sistemi informativi: integrazione, identità, dati, hosting e le informazioni che ti verrà chiesto di fornire.*

## Identità e accesso

- L'accesso con credenziali è **imposto per impostazione predefinita**; il servizio rifiuta di avviarsi senza le chiavi di verifica dei token, e in produzione con l'accesso disattivato.
- Autenticazione tramite un provider di identità (compreso Microsoft Entra ID); l'autenticazione a più fattori resta al provider ([guida all'accesso](../../../../operations/sign-in-with-entra.md)).
- L'autorizzazione usa gli attributi del token (per esempio `access`, `directorpack`, `confidence`) con una politica basata sugli attributi. I piani privati sono nascosti da ogni aggregato; i contrassegni di trattamento nascondono i piani oltre il nulla osta di chi chiama, amministratori compresi.
- L'accesso di emergenza (break-glass) ai piani privati richiede idoneità, un motivo dichiarato, al massimo quattro ore e una seconda persona avvisata.
- Gli amministratori possono elencare le concessioni di accesso permanenti (un aiuto alla ricertificazione) ed esportare o pseudonimizzare i dati di una persona.

## Dati e integrazione

| Argomento | Posizione |
|---|---|
| Dati conservati | Registrazioni di piani, attività, rischi, budget, tappe, tempo e costi; le persone sono riferimenti opachi, non nomi. Nessun dato sui pazienti e nessuna categoria particolare di dati, per progetto |
| Inventario | Ogni colonna di dati personali è elencata nell'[inventario dei dati personali](../../../../governance/personal-data-inventory.csv); un test lo mantiene fedele allo schema |
| Interfacce | REST con un documento OpenAPI derivato; importazione ed esportazione in blocco; feed di calendario; eventi in uscita e webhook solo verso destinazioni `https` pubbliche |
| Backup | Backup da script con ripristino collaudato, cifratura facoltativa, obiettivi di ripristino dichiarati ([backup e ripristino](../../../../operations/backup-and-restore.md)) |
| Hosting | Immagini di container e `compose.yaml`; il software non effettua trasferimenti propri, quindi la sede dei dati dipende dalla tua scelta di hosting |

## Lacune che devi colmare

- Test di penetrazione, revisione indipendente del codice e certificazione: nessuno eseguito.
- La conservazione è applicata solo in parte; la pulizia va pianificata (vedi il [piano di conservazione](../../../../governance/retention-schedule.md)).
- Lo strumento per gli interessati trova riferimenti, non nomi digitati in testo libero o in file caricati.
- La traccia di audit non è a prova di manomissione da parte di un amministratore del database.

## Da leggere dopo

[Diagramma del flusso dei dati](../../../../governance/data-flow.md) · [Lista di controllo della garanzia](../../../../governance/assurance-checklist.md) · [Installazione](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
