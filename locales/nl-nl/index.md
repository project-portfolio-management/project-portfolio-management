# Projectportfoliomanagement

*Een register van gefinancierd werk, met detectie van dubbele plannen en een werkruimte voor projectmanagement.*

Projectportfoliomanagement is opensourcesoftware om de plannen vast te leggen die een organisatie financiert en om het werk daarbinnen te beheren. Het omvat taken, planningen, kosten, middelen, risico's en governance. De software vergelijkt plannen om mogelijke dubbelingen op te sporen en toont het bewijs voor elke overeenkomst. Waar een cijfer niet kan worden berekend, noemt de software de reden.

## Belangrijkste voordelen

- **Eén portfolioregister.** Portfolio's, programma's, projecten en producten gebruiken één recordtype, en elk plan kan een ander plan bevatten. Rapportages putten uit één lijst voor alle afdelingen.
- **Detectie van dubbelingen.** Matching vergelijkt plannen die onder verschillende namen zijn vastgelegd en toont het bewijs voor elk resultaat, zodat beoordelaars kunnen ingrijpen voordat beide worden gefinancierd.
- **Herleidbare cijfers.** Een ontbrekend cijfer wordt getoond met de reden waarom het ontbreekt. Een plan zonder beoordeling wordt weergegeven als "niet beoordeeld".
- **Auditspoor.** Elke wijziging wordt vastgelegd. Wie een wijzigingsverzoek indient, kan het niet goedkeuren.
- **Toegangsbeheer.** Inloggen staat standaard aan, en de dienst draait in productie niet met uitgeschakelde aanmelding.
- **Implementatie en licenties.** De software is zelf te hosten en wordt aangeboden onder vijf opensourcelicenties. Werk kan worden geïmporteerd uit Jira, Asana en Microsoft Project.

[Alle voordelen bekijken](topics/voordelen/index.md)

## Functionele gebieden

- **Plannen en matching.** Recursief register, verklaarbare detectie van dubbelingen en samenvoegen.
- **Uitvoering.** Kanbanborden, gantt-diagrammen, agenda's, mijlpalen, kritiek pad en kritieke keten.
- **Financiën.** Budgetten, financieringsboek, verdiende waarde, en kosten- en planningsrisico.
- **Middelen.** Vaardighedenpools, capaciteitsheatmap en sleutelpersonen.
- **Strategie.** Werkintake, ideeën, doelstellingen, doelen en kernresultaten, en scenario's.
- **Governance.** Risico's, issues, poortbeoordelingen, escalatie en maandelijkse bestuurspakketten.

[Alle functies bekijken](topics/functies/index.md)

## Beoogde gebruikers

- **Portfoliokantoren.** Rapportage over programma's en projecten.
- **Project- en leveringsmanagers.** Planning, kosten- en risicobeheer.
- **Teamleden.** Een persoonlijk overzicht van toegewezen werk.
- **Senior leiders.** De [rolbeschrijvingen](for/) geven per rol aan wat de software biedt, wat de grenzen zijn en welke beslissingen nodig zijn.

## Huidige status

De software bevindt zich in een vroeg stadium. Er is één beheerder, geen audit door derden en geen ondersteuningsovereenkomst. Wat productiegebruik zou vergen, punt voor punt met een schatting per punt, staat op de pagina [weg naar productie](path-to-production/index.md). Bekende beperkingen staan op de [vergelijkingspagina](topics/vergelijkingen/index.md).

## Demonstratie

Een lokale demonstratie gebruikt een fictieve vervoersautoriteit en vereist Git en Podman. De stappen staan in de [handleiding](topics/handleiding/index.md).
