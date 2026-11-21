# Governance, Risiko, Compliance

*Für alle, die Governance, Risiko und Compliance verantworten: was die Software Ihnen zu belegen hilft, was sie durchsetzt und was Sie selbst beisteuern müssen. Keine Rechtsberatung.*

## Governance

- **Entscheidungen werden festgehalten.** Änderungsanträge brauchen eine benannte genehmigende Person, die nicht die antragstellende ist, eine angegebene Begründung von mindestens 10 Zeichen, und werden nach Handelnden protokolliert; die Datenbank weist eine Genehmigung ohne entscheidende Person und Begründung ab.
- **Ein eingefrorenes monatliches Leitungspaket.** Einmal entworfen, von einer Person herausgegeben, nach der Herausgabe unveränderlich; Korrekturen sind eine erneute Herausgabe mit einem Hinweis. Der Lesezugriff richtet sich nach der Zielgruppe.
- **Lieferzuversicht und Ausnahmen.** Die angegebene Zuversicht ist nur ergänzbar und wird neben dem berechneten Status gezeigt, nie mit ihm vermischt. Eine rote oder bernsteinrote Bewertung eröffnet eine Ausnahme mit einer 48-Stunden-Uhr (konfigurierbar), die nur eine erfasste Entscheidung schließt.
- **Projekt-Governance der Software selbst:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [Änderungskontrolle](../../../../spec/18-change-control.md), [KI-Erklärung](../../../../AI_STATEMENT.md).

## Risiko

- Risiken, Probleme und Änderungsanträge je Plan; ein Problemregister über alle lesbaren Pläne.
- Fehlende Daten erscheinen als „nicht bewertet“, nie als Grün.
- Die eigenen offenen Risiken des Projekts (kein Penetrationstest, teilweise Aufbewahrung, eine Betreuerin oder ein Betreuer, Übersetzungsentwürfe) stehen auf der Seite [Weg in den Produktivbetrieb](../../path-to-production/index.md), Punkt für Punkt mit je einer Schätzung.

## Compliance

| Bereich | Wo | Stand |
|---|---|---|
| Datenschutz (das geltende Recht; [§12.2a](../../../../spec/12-compliance.md) ordnet DSGVO, UK GDPR, LGPD, PIPL, DPDP, PIPEDA, CCPA und andere zu) | [DPIA](../../../../governance/dpia.md), [Verzeichnis von Verarbeitungstätigkeiten](../../../../governance/record-of-processing.md), [Verzeichnis personenbezogener Daten](../../../../governance/personal-data-inventory.csv), [Aufbewahrungsplan](../../../../governance/retention-schedule.md), [Datenfluss](../../../../governance/data-flow.md) | Vorausgefüllt; die Markierungen `‹buyer›` liegen bei Ihnen; Aufbewahrung nur teilweise durchgesetzt |
| Britischer NHS DTAC (ein nationales Schema, ein durchgearbeitetes Beispiel; andere Schemata beginnen bei der [Prüfliste](../../../../governance/assurance-checklist.md)) | [DTAC-Checkliste](../../../../governance/dtac-checklist.md) | Abschnitt für Abschnitt beantwortet; Zeilen für Käufer und Lieferanten offen |
| Barrierefreiheit | [Erklärung](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA automatisch; kein manueller Test |
| Sprachpflichten | [Sprachenerklärung](../../../../governance/language-statement.md) | Die Übersetzungen sind Entwürfe |
| ISO/IEC 27001 | [Compliance-Spezifikation](../../../../spec/12-compliance.md) | Kontrollen auf der Betriebsseite; nicht zertifiziert |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [Compliance-Spezifikation](../../../../spec/12-compliance.md) §12.4 | Nicht berührt (keine Patientendaten); angegeben, nicht konstruiert |

## Nachweise und Absicherung

Die [Prüfliste](../../../../governance/assurance-checklist.md) führt 14 Punkte auf, nach denen eine prüfende Stelle fragen wird, mit Verantwortlichen und Stand. Die meisten sind offen und gehören dem Betreiber: Penetrationstest, Barrierefreiheitsprüfung, Notfallübung, Absicherung des Hostings, Konfiguration des Identitätsanbieters, Rezertifizierung des Zugriffs, Vorfallprozess, Schriftgutverwaltung. Das Prüfprotokoll weist in der Datenbank Änderung und Löschung ab, aber ein Datenbankadministrator kann es aushebeln; es gibt keine Hash-Kette.

## Entscheidungen, die bei Ihnen liegen

Rechtsgrundlage und Aufbewahrungsfristen; Fenster für Ausnahmen; Einstufungsstufe für den Betrieb; wer den Zugriff prüft und wie oft; ob ein Verfahren für rechtliche Aufbewahrungspflichten nötig ist (keines ist gebaut).

## Weiterlesen

[Verzeichnis des Governance-Pakets](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Compliance-Spezifikation](../../../../spec/12-compliance.md)
