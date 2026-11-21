# Anleitung

*Schritte, um die Demonstration auf einem lokalen Rechner auszuführen und ein Beispielportfolio zu durchsehen.*

Die Demonstration ist eine vollständige Kopie der Software, geladen mit Daten einer fiktiven Organisation, der Northgate Transport Authority. Alle Personen, Pläne, Zahlen und Daten sind erfunden, und die Daten lassen sich frei ändern.

**Sicherheitshinweis.** Die Demonstration vertraut einem veröffentlichten Testschlüssel. Sie darf nicht in einem gemeinsam genutzten Netzwerk laufen und nicht mit echten Daten verwendet werden.

## Voraussetzungen

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) mit Compose

## Schritt 1: den Code herunterladen

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Schritt 2: die Demonstration starten

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Dieser Befehl baut die Software, startet die Datenbank, lädt das fiktive Portfolio und startet die Oberfläche. Der erste Build dauert länger als spätere.

## Schritt 3: anmelden

Öffnen Sie <http://localhost:3000/de-001/signin>, lassen Sie die Adresse bei `example@example.com` und wählen Sie **Sign in to the demonstration**. Es wird keine E-Mail gesendet.

Die Beschriftungen der Demonstrationsoberfläche erscheinen in dieser Anleitung auf Englisch, so wie die Demonstration sie zeigt.

Die erste Seite ist die **Demonstrationstour**, die jede Funktion nach Gruppen mit Beschreibung und Link aufführt. Ein gelbes Banner auf jeder Seite weist darauf hin, dass die Daten erfunden sind.

## Schritt 4: das Portfolio durchsehen

Öffnen Sie in der Tour **Plans**, um das Portfolio, seine Programme und seine Projekte zu sehen. Öffnen Sie dann das **Portfolio dashboard**.

## Schritt 5: ein Projekt durchsehen

Öffnen Sie das Projekt **North Line interlocking** über die Umsetzungsgruppe der Tour. Sehen Sie sich sein **Gantt**-Diagramm, den **Critical path** und den **Earned value** an.

## Schritt 6: persönliche Arbeit durchsehen

Öffnen Sie **My work** und **My kanban board**. Die Sitzung ist als Example Person angemeldet, die offene Aufgaben hat.

## Schritt 7: Kapazität, Startsteuerung und Berichte durchsehen

Öffnen Sie **Capacity heatmap**, um überbuchte Ressourcen zu sehen. Öffnen Sie **Start queue**, wo die Begrenzung der laufenden Arbeit ein Programm zurückgehalten hat. Öffnen Sie **Director packs**, die monatliche Zusammenfassung, die nach der Herausgabe eingefroren ist.

## Schritt 8: die Demonstration beenden

Dieser Befehl stoppt alle Dienste und löscht die Demonstrationsdaten:

```sh
podman compose --profile demo down -v
```

## Nächste Schritte

- [Installationsanleitung](../../../../INSTALL.md), mit Anmeldung und Identitätsanbietern
- [Rollenbriefings](../../for/) für Führungsrollen
- [Anleitung zur Demonstration](../../../../operations/demo.md)
- [Funktionsliste](../funktionen/index.md)

Probleme lassen sich über den [Issue-Tracker](https://github.com/project-portfolio-management/project-portfolio-management/issues) melden.
