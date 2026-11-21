# Sammenligninger

*Et resumé af, hvordan softwaren forholder sig til andre produkter.*

Produkterne nedenfor tjener forskellige formål og er ikke givet point. Disse noter bygger på hvert projekts offentlige materiale. **Intet andet produkt er installeret eller kørt**, og hvert af dem kan have ændret sig siden. Den aktuelle produktdokumentation bør tjekkes, før en beslutning træffes.

| Projekt | Hvad det er | Hvor dette projekt adskiller sig |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Projektledelse for folk, der ikke er projektledere | Dette projekt omfatter også de funktioner til planlægning, økonomi og governance, som et porteføljekontor bruger |
| [Redmine](https://www.redmine.org/) | En konfigurerbar generel sagshåndtering med sagstyper, statusser og roller, som organisationen selv definerer | Sagstyper, statusser og arbejdsgange defineres af organisationen. Roller bærer sagsrettigheder, der tildeles plan for plan, inden for hvad en persons login tillader. Se [sammenligningen med Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (via forken Platform Collective) | Et framework til forretningsapplikationer baseret på mange tjenester | Huly er en platform med flere tjenester. Dette projekt er én tjeneste oven på PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Et skrivebordsalternativ til Microsoft Project, til én leder og ét projekt | Dette projekt er en hostet tjeneste med flere brugere og adgangskontrol |

## Produktsammenligninger

Enkeltstående sammenligningssider (på engelsk): [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interoperabilitet

- **Jira og Asana:** CSV-import med en forhåndsvisning og en oplyst årsag til alt, der er udeladt.
- **Microsoft Project:** import og eksport af MSPDI XML, som omfatter opgaver, disposition, datoer, varigheder, begrænsninger, afhængigheder og kalenderen. Tildelte personer, ressourcer og baselines er udeladt, og filen oplyser dette.

## Forskelle i design

- **Oplyste årsager.** Et manglende tal vises med en årsag, for eksempel "ingen prognose, fordi …".
- **Forklarlig registrering af dubletter.** Matchningen virker på tværs af alle plantyper og viser sin dokumentation.
- **Sikre standardindstillinger.** Login er slået til som standard, og sikkerhedsmateriale er med i repositoriet.
- **Eksplicitte varigheder.** Hver varighed er oplyst i kalenderdage eller arbejdsdage.
- **Registrerede begrænsninger.** Backloggen registrerer begrænsningerne for hvert punkt.

## Kendte begrænsninger

- Ingen mobilapp og intet skrivebordsprogram.
- Ingen samtidig redigering i realtid.
- Én vedligeholder, en begrænset produktionshistorik og ingen revision fra tredjepart.
- Den offentliggjorte belastningstest kørte på én maskine. Den angiver en størrelsesorden og er ikke en kapacitetsplan.
- Oversættelserne er udkast, der afventer gennemgang.
- Ingen skærm til at udarbejde eller afgøre ændringsanmodninger.
- Intet økosystem af plugins eller marked.

Yderligere oplysninger findes i [sammenligningsnoterne](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Rettelser kan indsendes som en sag med en kilde.
