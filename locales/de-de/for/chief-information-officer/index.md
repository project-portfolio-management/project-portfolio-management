# Für die Informationsleitung (CIO)

*Informationsleitung (CIO): Integration, Identität, Daten, Betrieb und die Informationen, die man von Ihnen verlangen wird.*

## Identität und Zugriff

- Die Anmeldung wird **standardmäßig erzwungen**; der Dienst startet nicht ohne Schlüssel zur Tokenprüfung und im Produktivbetrieb nicht bei ausgeschalteter Anmeldung.
- Anmeldung über einen Identitätsanbieter (auch Microsoft Entra ID); die Multifaktor-Authentifizierung bleibt beim Anbieter ([Anleitung zur Anmeldung](../../../../operations/sign-in-with-entra.md)).
- Die Autorisierung verwendet Token-Attribute (zum Beispiel `access`, `directorpack`, `confidence`) mit einer Attributrichtlinie. Private Pläne sind aus jeder Aggregation ausgeblendet; Einstufungsvermerke blenden Pläne oberhalb der Freigabe des Aufrufers aus, auch vor Administratoren.
- Der Notfallzugriff (Break-glass) auf private Pläne erfordert Berechtigung, eine angegebene Begründung, höchstens vier Stunden und eine zweite Person, die benachrichtigt wird.
- Administratoren können bestehende Zugriffsberechtigungen auflisten (eine Hilfe bei der Rezertifizierung) und die Daten einer Person exportieren oder pseudonymisieren.

## Daten und Integration

| Thema | Stand |
|---|---|
| Gehaltene Daten | Datensätze zu Plänen, Aufgaben, Risiken, Budgets, Meilensteinen, Zeit und Kosten; Personen sind undurchsichtige Verweise, keine Namen. Keine Patientendaten und von der Konzeption her keine besonderen Kategorien von Daten |
| Verzeichnis | Jede Spalte mit personenbezogenen Daten steht im [Verzeichnis personenbezogener Daten](../../../../governance/personal-data-inventory.csv); ein Test hält es mit dem Schema in Übereinstimmung |
| Schnittstellen | REST mit einem abgeleiteten OpenAPI-Dokument; Massenimport und -export; Kalender-Feeds; ausgehende Ereignisse und Webhooks nur an öffentliche `https`-Ziele |
| Sicherungen | Skriptgesteuerte Sicherung mit getesteter Wiederherstellung, optionaler Verschlüsselung und genannten Wiederherstellungszielen ([Sicherung und Wiederherstellung](../../../../operations/backup-and-restore.md)) |
| Betrieb | Container-Images und `compose.yaml`; die Software nimmt selbst keine Übermittlungen vor, daher bestimmt Ihre Wahl des Hostings den Speicherort der Daten |

## Lücken, die Sie schließen müssen

- Penetrationstest, unabhängige Codeprüfung und Zertifizierung: nichts davon durchgeführt.
- Die Aufbewahrung wird nur teilweise durchgesetzt; die Bereinigung muss eingeplant werden (siehe den [Aufbewahrungsplan](../../../../governance/retention-schedule.md)).
- Das Werkzeug für Betroffenenrechte findet Verweise, keine Namen, die in Freitext oder hochgeladenen Dateien stehen.
- Das Prüfprotokoll ist gegenüber einem Datenbankadministrator nicht manipulationssicher.

## Weiterlesen

[Datenflussdiagramm](../../../../governance/data-flow.md) · [Prüfliste](../../../../governance/assurance-checklist.md) · [Betrieb](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
