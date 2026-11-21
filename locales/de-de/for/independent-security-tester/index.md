# Unabhängige Sicherheitsprüfung

*Für eine Käuferin, eine Prüferin oder einen Tester: was eine unabhängige Sicherheitsprüfung dieser Software abdecken sollte, was das Projekt dafür bereitstellt, was getan und nicht getan ist und was Ihnen überlassen bleibt. Dies ist eine Anleitung, kein Prüfbericht und keine Rechtsberatung.*

## Wo es steht

**Es wurde keine unabhängige Sicherheitsprüfung durchgeführt.** Für diese Software gibt es keinen Penetrationstest, keine Prüfung durch Dritte, keine unabhängige Codeprüfung und keine Zertifizierung, und das Projekt behauptet keine ([Weg in den Produktivbetrieb](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). Eine passive Prüfung läuft in der kontinuierlichen Integration des Projekts (Regeln in `.zap/rules.tsv`), und die eigenen Tests des Projekts prüfen die unten aufgeführten Kontrollen; keines von beiden ersetzt jemanden, der die Software nicht selbst geschrieben hat.

Dies sind Punkt 1 und Punkt 9 der [Prüfliste](../../../../governance/assurance-checklist.md). Sie gehören dem Betreiber, mit einer zertifizierten Prüfstelle, gegen **Ihren** Betrieb: Das Projekt kann Ihr Hosting, Ihren Identitätsanbieter, Ihr Netzwerk oder Ihre Schlüssel nicht testen.

## Was zu testen ist

Testen Sie den Betrieb so, wie er laufen wird, mit erzwungener Anmeldung und Ihrem Identitätsanbieter davor. Vorgeschlagener Umfang, aus den Kontrollen, die die Software für sich beansprucht:

| Bereich | Was die Software beansprucht | Was eine Prüfstelle versuchen sollte |
|---|---|---|
| Authentifizierung | Die Anmeldung ist standardmäßig eingeschaltet; der Dienst startet nicht ohne Prüfschlüssel oder im Produktivbetrieb bei ausgeschalteter Anmeldung; Tokens werden offline gegen veröffentlichte Schlüssel geprüft | Gefälschte, abgelaufene, wiedereingespielte und falsch signierte Tokens; falscher Aussteller oder falsche Zielgruppe; Schlüsselrotation; ein fehlender Schlüsselsatz |
| Autorisierung | Attributbasierte Richtlinie; Regeln auf Ebene der Datensätze | Horizontale und vertikale Rechteausweitung auf jeder Route; Kennungen in Pfaden und Inhalten ändern; Attribute, die ein Aufrufer nicht hinzufügen können sollte |
| Private Pläne und Einstufungsvermerke | Ein privater Plan und ein Plan oberhalb der Freigabe eines Aufrufers steht für diesen Aufrufer in keiner Liste, Zählung, Summe, Suchergebnis, Feed oder Export, auch nicht für Administratoren | Jede Route, die Existenz, Namen, Anzahl oder Summe eines Plans preisgibt, den der Aufrufer nicht lesen darf; Fehlermeldungen und Antwortzeiten, die sich zwischen „verborgen“ und „nicht vorhanden“ unterscheiden |
| Notfallzugriff (Break-glass) | Nur berechtigte Personen, eine angegebene Begründung, höchstens vier Stunden, eine zweite Person wird benachrichtigt | Nutzung ohne Berechtigung; Verlängerung einer Berechtigung; Handeln, ohne dass die zweite Person benachrichtigt wird |
| Eingefrorene und nur ergänzbare Datensätze | Das Prüfprotokoll, herausgegebene Leitungspakete, die Historie der Lieferzuversicht, die Statushistorie von Programmen und Projektpuffer weisen in der Datenbank Änderung und Löschung ab | Ändern oder Löschen über die API, über den Massenimport und über jede Route, die dieselben Zeilen berührt |
| Funktionstrennung | Wer einen Antrag stellt, kann den eigenen Änderungsantrag nicht genehmigen | Genehmigen als antragstellende Person, als Administrator und durch Ändern der genehmigenden Person nach dem Einreichen |
| Uploads und Anhänge | Der Inhalt muss zum angegebenen Typ passen; Programme werden abgewiesen; eine optionale Anbindung zur Virenprüfung schlägt sicher fehl | Polyglot-Dateien, umbenannte Programme, zu große und fehlerhafte Dateien, Pfadtricks in Namen, und ob Ihr Scanner wirklich aufgerufen wird |
| Ausgehende Anfragen | Von Nutzenden angegebene Ziele sind auf öffentliche `https`-Adressen beschränkt | Server-Side Request Forgery gegen private Adressen, Link-Local-Adressen und Metadaten-Adressen, Weiterleitungen, DNS-Rebinding |
| Sitzungen und Browser | Zeitlimit bei Inaktivität, eine feste Grenze, ein `httpOnly`-Cookie, eine strenge Content Security Policy, eine CORS-Zulassungsliste | Session Fixation und Wiederverwendung, Cross-Site Request Forgery, Einschleusen in die Oberfläche, Clickjacking |
| Ratenbegrenzung | Grenzen je Aufrufer | Umgehung durch Wechsel von Identität, Header oder Pfad; ob die Grenzen Ihren Proxy überstehen |
| Werkzeuge für Betroffenenrechte | Export und Pseudonymisierung, nur Administratoren, zuerst mit Vorschau | Nutzung durch Nicht-Administratoren; Vollständigkeit gegenüber dem [Verzeichnis personenbezogener Daten](../../../../governance/personal-data-inventory.csv) |
| Sicherungen | Optionale Verschlüsselung, eine getestete Wiederherstellung | Zugriff auf Sicherungsdateien und Schlüssel; Wiederherstellung in eine saubere Umgebung |

## Was das Projekt bereitstellt

- Den Quellcode, die Spezifikation und die Tests, damit eine Codeprüfung bei den Behauptungen beginnen kann.
- Eine maschinenlesbare Beschreibung der API (`/api-docs/openapi.json`), mit Schemata für Anfragen und Antworten zur Arbeit am Leitungspaket; die Liste der Routen wird von Tests gegen den Code geprüft.
- Eine Wegwerf-Konfiguration mit ausgeschalteter Anmeldung (`dev-insecure`, siehe die [README](../../../../README.md)) **nur für den eigenen Rechner der Prüfstelle** und ein Skript, das einen Testschlüsselsatz erzeugt (`scripts/smoke-keys.sh`), damit Tokens mit gewählten Attributen gegen einen Testbetrieb ausgestellt werden können.
- Das [Datenflussdiagramm](../../../../governance/data-flow.md), das [Betriebshandbuch](../../../../operations/runbook.md) und die [Wiederherstellungsziele](../../../../operations/recovery-objectives.md).
- Einen vertraulichen Weg, um zu melden, was gefunden wird: [SECURITY.md](../../../../SECURITY.md).

## Spielregeln

Vereinbaren Sie diese schriftlich vor jedem Test; das Projekt kann sie für Ihre Umgebung nicht festlegen.

- **Umgebung.** Testen Sie eine Kopie mit realistischen, aber nicht echten Daten. Testen Sie nicht den Produktivbetrieb, außer Sie haben es schriftlich entschieden.
- **Genehmigung.** Die Prüfstelle hat Ihre schriftliche Erlaubnis, eine benannte Ansprechperson und einen angegebenen Zeitraum.
- **Außerhalb des Rahmens.** Denial of Service durch Menge, Social Engineering von Beschäftigten und Dienste Dritter (Ihr Identitätsanbieter, Ihr Hoster) sind nicht Gegenstand, sofern deren jeweilige Inhaber nicht zustimmen. Der eigene Umfang des Projekts steht in [SECURITY](../../../../SECURITY.md).
- **Daten.** Echte personenbezogene Daten werden nicht verwendet, nicht herauskopiert und nicht in den Bericht aufgenommen.
- **Schlüssel.** Testschlüssel sind von Produktionsschlüsseln getrennt und werden danach vernichtet.

## Berichten und erneutes Testen

- Befunde zu dieser Software gehen vertraulich an die Betreuung, wie [SECURITY](../../../../SECURITY.md) es vorsieht, mit einer Anleitung zur Reproduktion, der Version oder dem Commit und der Auswirkung. Die Ziele des Projekts sind eine Bestätigung innerhalb von 5 Arbeitstagen und ein Plan innerhalb von 15; sie sind Ziele, keine Garantien. Es gibt eine Betreuerin oder einen Betreuer und keine Dienstgütevereinbarung.
- Befunde zu Ihrem Betrieb gehen an Ihre eigene verantwortliche Stelle.
- Bitten Sie die Prüfstelle, jeden Befund zu bewerten, anzugeben, was **nicht** getestet wurde, und die Korrekturen erneut zu testen. Ein Bericht, der nur auflistet, was gefunden wurde, sagt Ihnen nicht, was übersehen wurde.
- Bewahren Sie den Bericht, den Umfang, die Daten und den erneuten Test bei Ihren eigenen Unterlagen auf. Das Projekt kann sie nicht für Sie aufbewahren.

## Grenzen dieses Dokuments

Es führt Bereiche auf, die abzudecken sind, keinen vollständigen Testplan, und die Behauptungen in der Tabelle sind die des Projekts selbst. Es zertifiziert nichts, und ein sauberer Test ist ein Beleg für einen Betrieb an einem Datum.

## Entscheidungen, die bei Ihnen liegen

Wer testet, wann, wogegen, mit welchen Daten und nach welchen Regeln; wie Befunde bewertet werden und bis wann sie behoben sein müssen; und ob das Ergebnis mit Ihren Prüfern geteilt wird.

## Weiterlesen

[Prüfliste](../../../../governance/assurance-checklist.md) · [Weg in den Produktivbetrieb](../../path-to-production/index.md) · [Sicherheitsleitung](../chief-security-officer/index.md) · [Governance, Risiko, Compliance](../governance-risk-compliance/index.md)
