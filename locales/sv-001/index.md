# Projektportföljhantering

*Ett register över finansierat arbete, med dubblettsökning och en arbetsyta för projektledning.*

Projektportföljhantering (Project Portfolio Management) är programvara med öppen källkod för att registrera de planer som en organisation finansierar och för att styra arbetet inom dem. Den omfattar uppgifter, scheman, kostnader, resurser, risker och styrning. Den jämför planer för att hitta möjliga dubbletter och visar underlaget för varje träff. Där ett värde inte går att beräkna anger programvaran orsaken.

## Huvudsakliga fördelar

- **En enda portföljpost.** Portföljer, program, projekt och produkter använder samma posttyp, och varje plan kan innehålla en annan. Rapporteringen utgår från en enda lista för alla avdelningar.
- **Dubblettsökning.** Matchningen jämför planer som registrerats under olika namn och visar underlaget för varje resultat, så att granskare kan agera innan båda finansieras.
- **Spårbara värden.** Ett saknat värde visas tillsammans med orsaken till att det saknas. En plan utan bedömning visas som "inte bedömd".
- **Granskningslogg.** Varje ändring loggas. Den som väcker en ändringsbegäran kan inte godkänna den.
- **Åtkomstkontroll.** Inloggning är på som standard, och tjänsten körs inte i produktion med inloggningen avstängd.
- **Driftsättning och licensiering.** Programvaran driftas av dig själv och erbjuds under fem licenser med öppen källkod. Arbete kan importeras från Jira, Asana och Microsoft Project.

[Visa alla fördelar](topics/fördelar/index.md)

## Funktionsområden

- **Planer och matchning.** Rekursivt register, förklarbar dubblettsökning och sammanslagning.
- **Leverans.** Kanbantavlor, Gantt-scheman, kalendrar, milstolpar, kritisk linje och kritisk kedja.
- **Ekonomi.** Budgetar, finansieringsliggare, intjänat värde samt kostnads- och tidsrisk.
- **Resurser.** Kompetenspooler, kapacitetsvärmekarta och nyckelpersoner.
- **Strategi.** Intag av arbete, idéer, mål och nyckelresultat samt scenarier.
- **Styrning.** Risker, problem, grindgranskningar, eskalering och månatliga direktionspaket.

[Visa alla funktioner](topics/funktioner/index.md)

## Avsedda användare

- **Portföljkontor.** Rapportering över program och projekt.
- **Projekt- och leveranschefer.** Schemaläggning samt kostnads- och riskhantering.
- **Teammedlemmar.** En personlig vy över tilldelat arbete.
- **Ledande befattningshavare.** [Rollsammanfattningarna](for/) beskriver vad programvaran ger för varje roll, dess begränsningar och de beslut som krävs.

## Nuläge

Programvaran är i ett tidigt skede. Den har en enda underhållare, ingen tredjepartsgranskning och inget supportavtal. Vad produktionsanvändning skulle kräva, punkt för punkt med en uppskattning för var och en, finns på sidan [Vägen till produktion](path-to-production/index.md). Kända begränsningar anges på [jämförelsesidan](topics/jämförelser/index.md).

## Demonstration

En lokal demonstration använder en fiktiv transportmyndighet och kräver Git och Podman. Stegen finns i [handledningen](topics/handledning/index.md).
