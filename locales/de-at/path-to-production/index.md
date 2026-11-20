# Weg in den Produktivbetrieb

*Was „Produktivbetrieb“ für diese Software bedeutet und wie weit sie davon entfernt ist, Punkt für Punkt.*

„Produktivbetrieb“ bedeutet hier, dass eine Organisation die Software für echte Arbeit einsetzt. Echte Pläne, die Namen
echter Menschen und echte Entscheidungen hängen davon ab, ein Ausfall oder ein Datenabfluss hat Folgen, und jemand ist
dafür verantwortlich. In einer großen Gesundheitseinrichtung oder einer Behörde bedeutet Produktivbetrieb außerdem, dass
die eigene Prüfung der Organisation bestanden ist: ein Sicherheitstest, eine Datenschutz-Folgenabschätzung, eine
Barrierefreiheitsprüfung und im britischen National Health Service die Digital Technology Assessment Criteria (DTAC).

## Wo sie steht

- Es wird keine Zertifizierung, Freigabe oder Prüfung behauptet. Diese Seite ist die eigene Schätzung des Projekts.
- „Medizinisch“ bedeutet ein **nichtklinisches** Portfolio- und Projektwerkzeug: keine Patientenakten und keine
  Unterstützung klinischer Entscheidungen.
- Das Projekt empfiehlt, mit einem Pilotbetrieb auf nichtklinischen, nicht geheimen Daten zu beginnen, durchgeführt von
  einer Organisation, die ihre eigene Prüfung abschließt.

## Reife, Punkt für Punkt

Jede Zahl ist die Schätzung des Projekts vom 13. November 2026, keine Messung. Jeder Schieberegler beginnt bei dieser
Schätzung. Wer einen Schieberegler bewegt, hält beim Lesen die eigene Einschätzung fest; nichts wird gespeichert oder
gesendet.

- **Unabhängige Sicherheitsprüfung** — 0%. Kein Penetrationstest, keine Prüfung durch Dritte und keine Zertifizierung. Ein Käufer oder ein Lieferant muss sie in Auftrag geben (RG-13).
- **Barrierefreiheit** — 40%. Bei jeder Änderung läuft für jede Route eine automatische Prüfung nach WCAG 2.2 AA. Ein manueller Test und ein Test mit assistiven Technologien oder mit Menschen mit Behinderungen wurden nicht durchgeführt (RG-13).
- **Anfragen betroffener Personen** — 50%. Administratoren können eine Person über jeden Verweis im Schema exportieren und pseudonymisieren, zuerst mit einer Vorschau (RG-6). Namen in Titeln, Kommentaren oder Dokumenten, hochgeladene Dateien, Prüfprotokoll-Momentaufnahmen und Sicherungen werden nicht automatisch gefunden.
- **Aufbewahrung** — 35%. Eine Bereinigung, standardmäßig als Probelauf, umfasst Benachrichtigungen, Zustellprotokolle, Idempotenzschlüssel sowie Trend- und Nutzungsmomentaufnahmen, mit Fristen je Installation (RG-4). Pläne, Aufgaben, Zeiterfassungen, Dokumente und das Prüfprotokoll bleiben erhalten, bis ein Betreiber sie löscht.
- **Einstufungsvermerke** — 60%. Eine konfigurierbare Einstufung verbirgt Pläne oberhalb der Freigabe des Aufrufers, auch vor Administratoren, und kennzeichnet jede Antwort (RG-7). Der Vermerk wird nicht in erzeugte Dateien eingeprägt, und die Freigabe ist nur so verlässlich wie das Attribut des Identitätsanbieters.
- **Sicherheit von Anhängen** — 50%. Der Inhalt muss zum angegebenen Typ passen, Programme werden abgewiesen, und eine optionale `clamd`-Prüfung schlägt sicher fehl (RG-8). Ein Virenscanner wird nicht mitgeliefert, und die Anbindung wurde nur gegen einen Platzhalter getestet.
- **Zugriffsüberprüfung und Notfallzugriff** — 45%. Ein Bericht über bestehende Berechtigungen unterstützt die Rezertifizierung (RG-9), und der Notfallzugriff ist zeitlich begrenzt und verlangt eine angegebene Begründung (RG-10). Der Bericht zeigt, was erfasst wurde, nicht was der Identitätsanbieter durchsetzt; es gibt keine Route, um die Entscheidung eines Prüfers festzuhalten; und eine Notfallzugriffsmeldung kann eine Person nennen, die nichts erhält, während die Berechtigung weiter besteht.
- **Leistung und Kapazität** — 40%. Ein Last- und Dauertest auf einem Rechner steht in [`BENCHMARKS.md`](../../../BENCHMARKS.md): Gewöhnliche Anfragen antworten in einigen zehn Millisekunden, ein Plan mit etwa 12.700 Aufgaben wird nur langsam terminiert, und der Prozess wächst auf etwa 1 GB. Es gibt keine Kapazitätsplanung, keinen Test mit vielen Nutzern und keinen Lauf über mehrere Rechner.
- **Sitzungen über mehrere Frontend-Server** — 70%. Sitzungen liegen im Arbeitsspeicher, sofern Redis nicht konfiguriert ist (RG-12); mit Redis hängt die Anmeldung davon ab.
- **Integrität des Prüfprotokolls** — 50%. Schreibvorgänge werden protokolliert, und ein Datenbank-Trigger weist Änderungen und Löschungen ab, mit angegebenen Ausnahmen (RG-3). Ein Datenbankadministrator kann den Trigger entfernen, und es gibt keine Hash-Kette.
- **Sicherung und Wiederherstellung** — 80%. Eine Wiederherstellung wird in der kontinuierlichen Integration getestet, unverschlüsselt und verschlüsselt, gegen angegebene Wiederherstellungsziele (RG-5). Die Verschlüsselung ist optional, und die Übung läuft auf der CI des Projekts, nicht beim Hosting des Betreibers.
- **Unterstützung und Fortbestand** — 10%. Ein Betreuer und keine Dienstgütevereinbarung ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Übersetzungen** — 20%. Text in anderen Sprachen als Englisch ist maschinell geschrieben, bis eine sprachkundige Person ihn prüft, und gesetzliche Sprachpflichten verlangen geprüften Text ([Sprachenerklärung](../../../governance/language-statement.md)).
- **Klinische Nutzung** — 0%. Es wurde kein klinischer Sicherheitsnachweis geschrieben, und klinische Nutzung liegt außerhalb des Umfangs.

## Was ein erster Einsatz tun sollte

1. Die [Prüfliste](../../../governance/assurance-checklist.md) abschließen, beginnend mit einem Penetrationstest und einer Barrierefreiheitsprüfung.
2. Jede Zeile „Buyer to complete“ in der [DTAC-Checkliste](../../../governance/dtac-checklist.md) und die Markierungen in der [DPIA](../../../governance/dpia.md) ausfüllen.
3. Aufbewahrungsfristen mit der Schriftgutverwaltung vereinbaren und die Bereinigung einplanen.
4. Verschlüsselte Sicherungen einschalten und die Wiederherstellungsübung beim eigenen Hosting durchführen.
5. Die Daten nichtklinisch und unterhalb der Einstufung halten, für die der Einsatz geprüft ist.
6. Die oben genannten Lücken einplanen oder ihre Beseitigung finanzieren.

Die Belege zu jedem Punkt stehen in der [Bewertung](../../../governance/production-readiness.md).
