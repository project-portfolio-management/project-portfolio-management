# Vergleiche

*Eine Übersicht, wie sich die Software zu anderen Produkten verhält.*

Die folgenden Produkte dienen unterschiedlichen Zwecken und werden nicht bewertet. Diese Anmerkungen beruhen auf dem öffentlichen Material der jeweiligen Projekte. **Kein anderes Produkt wurde installiert oder ausgeführt**, und jedes kann sich seither geändert haben. Vor einer Entscheidung ist die aktuelle Produktdokumentation zu prüfen.

| Projekt | Was es ist | Worin sich dieses Projekt unterscheidet |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Projektmanagement für Menschen, die keine Projektleiter sind | Dieses Projekt enthält zusätzlich die Funktionen für Terminplanung, Kosten und Governance, die ein Portfoliobüro nutzt |
| [Redmine](https://www.redmine.org/) | Ein konfigurierbares Allzweck-Tracking-System mit von der Organisation definierten Problemtypen, Status und Rollen | Problemtypen, Status und Workflows werden von der Organisation definiert. Rollen tragen Problemberechtigungen, die Plan für Plan gelten, innerhalb dessen, was die Anmeldung einer Person erlaubt. Siehe den [Vergleich mit Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (über ihren Fork Platform Collective) | Ein Framework für Geschäftsanwendungen auf vielen Diensten | Huly ist eine Plattform aus mehreren Diensten. Dieses Projekt ist ein Dienst über PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Eine Desktop-Alternative zu Microsoft Project, für eine Leitung und ein Projekt | Dieses Projekt ist ein gehosteter Mehrbenutzerdienst mit Zugriffskontrolle |

## Produktvergleiche

Einzelne Vergleichsseiten (nur auf Englisch): [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interoperabilität

- **Jira und Asana:** CSV-Import, mit einer Vorschau und einem genannten Grund für alles, was ausgelassen wird.
- **Microsoft Project:** Import und Export von MSPDI-XML mit Aufgaben, Gliederung, Terminen, Dauern, Einschränkungen, Abhängigkeiten und Kalender. Zugewiesene Personen, Ressourcen und Basispläne sind ausgeschlossen, und die Datei hält das fest.

## Unterschiede im Entwurf

- **Genannte Gründe.** Eine fehlende Zahl wird mit einem Grund angezeigt, zum Beispiel „keine Prognose, weil …“.
- **Nachvollziehbare Dublettenerkennung.** Der Abgleich arbeitet über alle Plantypen hinweg und zeigt seine Belege.
- **Sicherheitsvorgaben.** Die Anmeldung ist standardmäßig eingeschaltet, und Sicherheitsmaterial gehört zum Repository.
- **Ausdrückliche Dauern.** Jede Dauer wird in Kalendertagen oder Arbeitstagen angegeben.
- **Festgehaltene Grenzen.** Der Backlog hält die Grenzen jedes Eintrags fest.

## Bekannte Einschränkungen

- Keine mobile App und keine Desktopanwendung.
- Kein gemeinsames Bearbeiten in Echtzeit.
- Eine Betreuerin oder ein Betreuer, eine begrenzte Praxis im Produktivbetrieb und keine Prüfung durch Dritte.
- Der veröffentlichte Lasttest lief auf einem Rechner. Er gibt eine Größenordnung an und ist keine Kapazitätsplanung.
- Die Übersetzungen sind Entwürfe, die auf Prüfung warten.
- Keine Maske zum Entwerfen oder Entscheiden von Änderungsanträgen.
- Kein Ökosystem aus Plugins oder einem Marktplatz.

Weitere Einzelheiten stehen in den [Vergleichsnotizen](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Korrekturen lassen sich als Issue mit einer Quelle einreichen.
