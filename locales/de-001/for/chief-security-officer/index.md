# Für die Sicherheitsleitung (CSO)

*Sicherheitsleitung (CSO): was die Software durchsetzt, was das Projekt geprüft hat und was Sie selbst testen und betreiben müssen.*

## Was die Software durchsetzt

| Bereich | Verhalten |
|---|---|
| Authentifizierung | Die Anmeldung ist standardmäßig eingeschaltet. Der Dienst startet nicht ohne Schlüssel zur Tokenprüfung und im Produktivbetrieb nicht bei ausgeschalteter Anmeldung. Anmeldung über einen Identitätsanbieter wie Microsoft Entra ID; die Multifaktor-Authentifizierung bleibt beim Anbieter ([Anleitung zur Anmeldung](../../../../operations/sign-in-with-entra.md)) |
| Autorisierung | Attributbasierte Richtlinie auf Token-Attributen. Private Pläne bleiben für alle aus jeder Aggregation heraus; Einstufungsvermerke blenden Pläne oberhalb der Freigabe des Aufrufers aus, auch vor Administratoren |
| Notfallzugriff (Break-glass) | Der Zugriff auf private Pläne erfordert Berechtigung, eine angegebene Begründung, höchstens vier Stunden und eine zweite Person, die benachrichtigt wird; jede Anfrage wird protokolliert |
| Sitzungen | Zeitlimit bei Inaktivität (standardmäßig 30 Minuten) und eine feste Sitzungsgrenze |
| Transport und Browser | Sicherheits-Header, eine strenge Content Security Policy, eine CORS-Zulassungsliste, Ratenbegrenzung je Aufrufer |
| Ausgehend | Von Nutzenden angegebene Ziele sind auf öffentliche `https`-Adressen beschränkt |
| Uploads | Der Inhalt muss zum angegebenen Typ passen; Programme werden abgewiesen; eine optionale `clamd`-Prüfung schlägt sicher fehl |
| Prüfprotokoll | Das Prüfprotokoll weist in der Datenbank Änderung und Löschung ab |
| Eingefrorene Datensätze | Herausgegebene Leitungspakete und Bewertungen der Lieferzuversicht sind per Datenbank-Trigger unveränderlich oder nur ergänzbar |
| Lieferkette | `cargo deny`, eine Software-Stückliste, Build-Herkunft bei der Veröffentlichung, eine [Sicherheitsrichtlinie](../../../../SECURITY.md) |

## Was nicht getan ist

- Kein Penetrationstest, keine Prüfung durch Dritte, keine unabhängige Codeprüfung und keine Zertifizierung.
- Kein mitgelieferter Malware-Scanner; die Anbindung `clamd` wurde nur gegen einen Platzhalter getestet.
- Das Prüfprotokoll lässt sich durch einen Datenbankadministrator aushebeln; es gibt keine Hash-Kette.
- Einstufungsvermerke blenden aus und kennzeichnen; sie verschlüsseln nicht, und ein Vermerk wird nicht in erzeugte Dateien eingeprägt.
- Sitzungen liegen im Arbeitsspeicher, sofern Redis nicht konfiguriert ist.
- Das Werkzeug für Betroffenenrechte findet Verweise, keine Namen, die in Freitext, hochgeladenen Dateien oder Sicherungen stehen.

## Betrieb

Sicherungen haben eine getestete Wiederherstellung, optionale Verschlüsselung und genannte Wiederherstellungsziele ([Betriebspaket](../../../../operations/README.md)). Das [Betriebshandbuch](../../../../operations/runbook.md) gibt die technischen Schritte bei Vorfällen an; das Verfahren für Vorfälle und Datenpannen liegt bei Ihnen. Melden Sie eine Schwachstelle vertraulich, wie [SECURITY.md](../../../../SECURITY.md) es beschreibt; das Ziel ist eine Bestätigung innerhalb von 5 Arbeitstagen, keine Garantie.

## Entscheidungen, die bei Ihnen liegen

Wer den Penetrationstest durchführt und wann; die Zugriffsbedingungen und die Rollenzuordnung des Identitätsanbieters; für welche Einstufungsstufe dieser Betrieb abgesichert ist; der Scanner hinter der Anbindung für Anhänge; wie oft der Zugriff rezertifiziert wird.

## Weiterlesen

[Prüfliste](../../../../governance/assurance-checklist.md) · [Weg in den Produktivbetrieb](../../path-to-production/index.md) · [Datenfluss](../../../../governance/data-flow.md)
