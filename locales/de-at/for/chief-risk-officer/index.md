# Für die Risikoleitung (CRO)

*Risikoleitung (CRO): die Risiken, die die Software sichtbar machen hilft, und die Risiken ihrer Nutzung.*

## Risiken, die sie steuern hilft

- **Risiken, Probleme und Änderungsanträge** je Plan erfasst; ein Problemregister über alle lesbaren Pläne; Änderungsanträge mit Funktionstrennung.
- **Lieferzuversicht und Ausnahmen:** angegebene Zuversicht getrennt vom berechneten RAG-Status, markiert, wenn beide um zwei Stufen auseinanderliegen, mit einer nur ergänzbaren Historie, und eine 48-Stunden-Uhr für Ausnahmen, die sich bei einer schlechteren Bewertung nicht neu startet und nur durch eine erfasste Entscheidung geschlossen wird.
- **Ehrliche Abwesenheit:** Fehlende Daten erscheinen als „nicht bewertet“ oder als Null-Wert mit einem Grund, nie als Grün oder Null.
- **Ein eingefrorenes, versioniertes Monatspaket,** damit sich später genau so zeigen lässt, was der Leitung mitgeteilt wurde.

## Risiken der Nutzung (das eigene Register des Projekts)

| Risiko | Stand |
|---|---|
| Kein Penetrationstest, keine Prüfung durch Dritte und keine Zertifizierung | Offen: vom Käufer zu beauftragen ([Prüfliste](../../../../governance/assurance-checklist.md)) |
| Aufbewahrung nur für einen Teil der Daten durchgesetzt | Offen; die Bereinigung muss eingeplant werden |
| Das Werkzeug für Betroffenenrechte durchsucht weder Freitext noch Anhänge noch Sicherungen | Offen |
| Das Prüfprotokoll lässt sich durch einen Datenbankadministrator aushebeln; keine Hash-Kette | In der [DPIA](../../../../governance/dpia.md) angegeben |
| Einstufungsvermerke blenden aus und kennzeichnen, sie verschlüsseln nicht | Angegeben |
| Die Prüfung von Anhängen braucht einen vom Betreiber geführten Scanner | Offen |
| Eine Betreuerin oder ein Betreuer, keine SLA | Offen |
| Übersetzungen sind Entwürfe | Offen |
| Nur nichtklinisch; kein klinischer Sicherheitsnachweis | Keine klinischen Inhalte eingeben |

## Kontrollen, die die Software durchsetzt

Anmeldung standardmäßig, Ratenbegrenzung, strenge Sicherheits-Header, ein Zeitlimit bei Inaktivität der Sitzung (30 Minuten) und eine feste Sitzungsgrenze, ausgehende Ziele auf öffentliches `https` beschränkt, an der Signatur geprüfte Anhänge, unveränderliche Zeilen im Prüfprotokoll, Notfallzugriff mit Protokollierung. Jede davon ist durch einen Test abgedeckt, der im [Weg in den Produktivbetrieb](../../path-to-production/index.md) genannt wird.

## Entscheidungen, die bei Ihnen liegen

Risikobereitschaft für einen Pilot; das Fenster für Ausnahmen (Standard 48 Stunden); für welche Einstufungsstufe der Betrieb abgesichert ist; wer die Zugriffsliste prüft und wie oft.

## Weiterlesen

[Weg in den Produktivbetrieb](../../path-to-production/index.md) · [DPIA](../../../../governance/dpia.md) · [Compliance](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
