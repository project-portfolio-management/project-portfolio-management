# Für die Betriebsleitung (COO)

*Betriebsleitung (COO): was Sie täglich steuern, was die Software für die Umsetzung leistet und was ihr Betrieb erfordert.*

## Was sie für den Betrieb leistet

- **Planung und Steuerung der Umsetzung:** Pläne, Aufgaben, Abhängigkeiten, eine Berechnung des kritischen Pfads, planübergreifende Meilensteine, Sprints und Phasen, Burndown und Flusskennzahlen.
- **Kapazität:** Zuteilungen, Verfügbarkeit, Urlaub und arbeitsfreie Zeiten, Ressourcenrollen, abgeleitete Arbeitslast und Auslastung. (Eine Kapazitäts-Heatmap nach Kompetenzpool und Monat für das Leitungspaket ist nicht gebaut: `DP-10`.)
- **Zeit und Kosten:** Zeitnachweise, erfasste Zeit, Kostensätze (nur für das Finanzwesen), Budgets und abgeleitete Kosten.
- **Governance-Ablauf:** Problemregister, Änderungsanträge mit Funktionstrennung (die antragstellende Person kann nicht genehmigen), Genehmigungen, Reviews, Vorgänge für Eintritt, Wechsel und Austritt.
- **Ausnahmen:** Eine rote oder bernsteinrote Lieferzuversicht eröffnet eine Ausnahme, die eine Ursache, die nötige Entscheidung, eine benannte verantwortliche Person und einen Entscheidungstermin innerhalb von 48 Stunden braucht (Fenster konfigurierbar, 1 bis 720 Stunden). Überschreitungen erscheinen im Leitungspaket und in `GET /api/exceptions`.

## Was der Betrieb erfordert

| Bedarf | Einzelheiten |
|---|---|
| Dienst | Ein Rust-Dienst, PostgreSQL 18, optional Redis (Cache im Produktivbetrieb, optional gemeinsame Sitzungen), eine SvelteKit-Oberfläche |
| Betriebshandbuch | [Betriebspaket](../../../../operations/README.md): Betrieb, Betriebshandbuch, Sicherung und Wiederherstellung, Wiederherstellungsziele, Upgrade und Migration |
| Geplante Aufträge | Nichts plant sich selbst. Die Bereinigung nach Aufbewahrungsfristen, Meldungen über Fristüberschreitungen bei Ausnahmen und ähnliche Aufgaben muss Ihr Betriebsteam in einen Scheduler legen |
| Support | Eine Betreuerin oder ein Betreuer, keine SLA; planen Sie Ihren eigenen Ersten-Level-Support ein |

## Die Grenzen kennen

- Ein Plan mit mehreren tausend Aufgaben lässt sich nur langsam terminieren; die vollständige Antwort wird ab 5.000 Aufgaben verweigert, sofern sie nicht verlangt wird (`?full=true` oder `?only=critical`). Zahlen und Vorbehalte stehen in [`BENCHMARKS.md`](../../../../BENCHMARKS.md); sie sind eine Größenordnung von einem Rechner, keine Kapazität.
- Für einen großen Teil des Arbeitsbereichs gibt es Masken, **aber noch nicht** für Änderungsanträge, Probleme, Pakete oder Lieferzuversicht; diese sind heute nur über die API erreichbar.

## Entscheidungen, die bei Ihnen liegen

Wer jede Ausnahme verantwortet, wer das Monatspaket herausgibt (Rhythmus und Erinnerungen sind `DP-12`, nicht gebaut) und welchen Prüfrhythmus die Ausnahmeregel speist.

## Weiterlesen

[Betriebspaket](../../../../operations/README.md) · [Betriebshandbuch](../../../../operations/runbook.md) · [Wiederherstellungsziele](../../../../operations/recovery-objectives.md)
