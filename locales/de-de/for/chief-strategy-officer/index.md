# Für die Strategieleitung (CSO)

*Strategieleitung: wie die Software Strategie mit der Arbeit verbindet, was sie Ihnen zeigt und was sie noch nicht kann.*

## Was sie für die Strategie leistet

- **Ein Register für alles Finanzierte.** Portfolios, Programme, Projekte und Produkte sind ein rekursiver Datensatz, sodass sich der gesamte Bestand als Baum lesen lässt ([Zweck und Vision](../../../../spec/01-purpose-and-vision.md)).
- **Dublettenerkennung.** Nachvollziehbarer Abgleich zeigt, wenn zwei Teile der Organisation dieselbe Initiative beauftragt haben, bevor beide finanziert werden.
- **Strategische Ziele und die Portfoliokarte.** Sie markieren Ziele als strategische Ziele und benennen für jedes Programm ein primäres Ziel. `GET /api/portfolio-map` und das monatliche Leitungspaket gruppieren Programme nach dem primären Ziel. Ein Programm erscheint einmal, mit seinen weiteren Zielen; ein Programm ohne primäres Ziel steht unter „kein strategisches Ziel“, und Summen werden nie doppelt gezählt.
- **Lieferzuversicht neben dem berechneten Status.** Menschen geben ihre Zuversicht in die Lieferung an; die Software zeigt sie neben dem berechneten Rot/Bernstein/Grün und markiert eine Abweichung um zwei Stufen. Ein Plan, den niemand bewertet hat, zeigt „nicht bewertet“, nie Grün.
- **Ein eingefrorenes Monatspaket.** Was der Leitung mitgeteilt wurde, bleibt genau so erhalten, wie es herausgegeben wurde, mit dem Trend gegenüber dem zuvor herausgegebenen Paket.

## Was sie noch nicht leistet

- Das Paket enthält noch nicht Gate-Entscheidungen, Finanzierung nach Geschäftsjahr, das Puffer-Fieberdiagramm, Earned Value, validierte Nutzen oder die Kapazitäts-Heatmap (`DP-5` bis `DP-10` in [`tasks.md`](../../../../tasks.md)). Das Paket nennt die noch nicht gebauten Abschnitte.
- „Programm“ bezeichnet einen Plan der Art `Program`, weil die Daten keine andere Definition kennen; „oberste“ Ziele sind, was Sie markieren, da Ziele keine Hierarchie haben.
- Es gibt keine Szenarioplanung, und noch keine Maske für die Karte oder das Paket: Sie stehen als Daten zur Verfügung.
- Sie ist nicht zertifiziert, und es gibt eine Betreuerin oder einen Betreuer ([Weg in den Produktivbetrieb](../../path-to-production/index.md)).

## Entscheidungen, die bei Ihnen liegen

Welche Ziele strategische Ziele sind; das primäre Ziel jedes Programms; wer die Lieferzuversicht angibt und wer sie unabhängig bewertet; wie viele Monate Trend die Leitung sehen soll.

## Weiterlesen

[Fahrplan](../../../../spec/15-roadmap.md) · [Offene Fragen](../../../../spec/16-open-questions.md) · [Umfang](../../../../spec/02-scope.md)
