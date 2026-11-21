# Vorteile

*Die wichtigsten Vorteile für eine Organisation, nach Bereichen.*

## Transparenz im Portfolio

Portfolioinformationen liegen oft an mehreren Orten. In dieser Software ist jedes Portfolio, Programm, Projekt und Produkt ein einziger Datensatztyp, ein **Plan**. Jeder Plan kann jeden anderen in beliebiger Tiefe enthalten, sodass die Struktur zur Organisation passen kann. Eine Bezeichnung wie „Projekt“ beschreibt nur und schränkt einen Plan nicht ein.

## Weniger Dubletten

Zwei Abteilungen können dieselbe Initiative unter verschiedenen Namen beauftragen. Der Abgleich vergleicht die Merkmale, die einen Plan kennzeichnen: Name, Ziele, Verantwortliche, Sponsor, übergeordneten Plan, Zeitrahmen, Schlüsselwörter und Kennungen. Er arbeitet über alle Plantypen hinweg und zeigt die Belege für jeden Treffer; die Entscheidung trifft ein Mensch. Zwei Datensätze desselben Plans lassen sich zusammenführen.

## Qualität der Berichte

- Eine fehlende Zahl wird mit einem Grund angezeigt, zum Beispiel „keine Prognose, weil …“.
- Ein Plan ohne Bewertung erscheint als „nicht bewertet“.
- Die von Menschen erfasste Lieferzuversicht erscheint neben dem berechneten Status, und beide bleiben getrennt.
- Private Pläne sind für alle Nutzenden aus allen Summen und Zählungen ausgeschlossen. Tests prüfen diese Regel.
- Jede Dauer gibt an, ob sie in Kalendertagen oder Arbeitstagen gemessen wird.

## Entscheidungsnachweise

Jede Änderung wird protokolliert, und das Prüfprotokoll lässt sich über den Dienst nicht verändern. Wer einen Änderungsantrag stellt, kann ihn nicht genehmigen. Ein monatliches Leitungspaket wird einmal entworfen, von einer benannten Person herausgegeben und dann von der Datenbank eingefroren. Eine Korrektur erscheint als neue Fassung mit einem Hinweis.

## Sicherheit

Die Anmeldung ist standardmäßig eingeschaltet, und der Dienst startet im Produktivbetrieb nicht mit ausgeschalteter Anmeldung. Sicherheits-Header, Ratenbegrenzung, ein Wiederherstellungsplan mit genannten Zielen und ein Paket zur Informationsgovernance gehören zum Repository.

## Kontrolle und Übertragbarkeit

- **Lizenzierung.** Die Software ist Open Source, wahlweise unter MIT, Apache-2.0, BSD-3-Clause, GPL-2.0, GPL-3.0, OGL-UK-3.0 oder CC-BY-NC-SA-4.0.
- **Betrieb.** Die Software wird selbst betrieben: ein Rust-Dienst über PostgreSQL mit einer SvelteKit-Oberfläche.
- **Datenimport.** CSV-Dateien aus Jira und Asana lassen sich importieren, mit einer Vorschau und einem genannten Grund für alles Ausgeschlossene. Microsoft-Project-XML lässt sich importieren und exportieren.
- **Sprachen.** Die Oberfläche gibt es in zehn Sprachfassungen, darunter das von rechts nach links geschriebene Arabisch. Die Übersetzungen sind Entwürfe, die auf die Prüfung durch Sprachkundige warten.

## Verwandte Seiten

Siehe auch [Funktionen](../funktionen/index.md), [Vergleiche](../vergleiche/index.md) und die [Anleitung](../anleitung/index.md).
