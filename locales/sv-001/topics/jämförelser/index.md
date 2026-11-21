# Jämförelser

*En sammanfattning av hur programvaran förhåller sig till andra produkter.*

Produkterna nedan tjänar olika syften och poängsätts inte. Anteckningarna bygger på varje projekts offentliga material. **Ingen annan produkt har installerats eller körts**, och var och en kan ha ändrats sedan dess. Aktuell produktdokumentation bör kontrolleras innan ett beslut fattas.

| Projekt | Vad det är | Hur det här projektet skiljer sig |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Projektledning för personer som inte är projektledare | Det här projektet har också de funktioner för schemaläggning, kostnader och styrning som ett portföljkontor använder |
| [Redmine](https://www.redmine.org/) | Ett konfigurerbart allmänt ärendehanteringssystem, med problemtyper, statusar och roller som organisationen själv definierar | Problemtyper, statusar och arbetsflöden definieras av organisationen. Roller bär behörigheter för problem, som hålls plan för plan, inom ramen för vad en persons inloggning tillåter. Se [jämförelsen med Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (via dess avknoppning Platform Collective) | Ett ramverk för affärsapplikationer som bygger på många tjänster | Huly är en plattform med flera tjänster. Det här projektet är en enda tjänst ovanpå PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Ett skrivbordsalternativ till Microsoft Project, för en projektledare och ett projekt | Det här projektet är en driftad fleranvändartjänst med åtkomstkontroll |

## Produktjämförelser

Enskilda jämförelsesidor (på engelska): [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interoperabilitet

- **Jira och Asana:** CSV-import, med förhandsvisning och en angiven orsak till allt som utelämnas.
- **Microsoft Project:** import och export av MSPDI XML, som omfattar uppgifter, disposition, datum, varaktigheter, begränsningar, beroenden och kalendern. Tilldelade personer, resurser och baslinjer utelämnas, och filen anger detta.

## Skillnader i utformning

- **Angivna orsaker.** Ett saknat värde visas med en orsak, till exempel "ingen prognos, eftersom …".
- **Förklarbar dubblettsökning.** Matchningen fungerar för alla plantyper och visar sitt underlag.
- **Säkerhetsstandarder.** Inloggning är på som standard, och säkerhetsmaterial ingår i förvaret.
- **Uttryckliga varaktigheter.** Varje varaktighet anges i kalenderdagar eller arbetsdagar.
- **Dokumenterade begränsningar.** Backloggen anger begränsningarna för varje punkt.

## Kända begränsningar

- Ingen mobilapp eller skrivbordsapplikation.
- Ingen samredigering i realtid.
- En underhållare, begränsad produktionshistorik och ingen tredjepartsgranskning.
- Det publicerade belastningstestet kördes på en enda maskin. Det ger en storleksordning och är ingen kapacitetsplan.
- Översättningarna är utkast som väntar på granskning.
- Ingen skärm för att skriva eller avgöra ändringsbegäranden.
- Inget ekosystem av tillägg eller marknadsplats.

Mer information finns i [jämförelseanteckningarna](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Rättelser kan lämnas som ett ärende med en källa.
