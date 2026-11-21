# Vergelijkingen

*Een overzicht van hoe de software zich verhoudt tot andere producten.*

De onderstaande producten dienen verschillende doelen en worden niet gescoord. Deze aantekeningen zijn gebaseerd op het openbare materiaal van elk project. **Geen enkel ander product is geïnstalleerd of uitgevoerd**, en elk product kan sindsdien zijn veranderd. Raadpleeg de actuele productdocumentatie voordat u een besluit neemt.

| Project | Wat het is | Waarin dit project verschilt |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Projectmanagement voor mensen die geen projectmanager zijn | Dit project bevat ook de planning-, kosten- en governancefuncties die een portfoliokantoor gebruikt |
| [Redmine](https://www.redmine.org/) | Een configureerbare algemene tracker, met door de organisatie gedefinieerde issuetypen, statussen en rollen | Issuetypen, statussen en workflows worden door de organisatie gedefinieerd. Rollen bepalen issuerechten, per plan toegekend, binnen wat de aanmelding van een persoon toestaat. Zie de [vergelijking met Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (via de fork Platform Collective) | Een raamwerk voor bedrijfsapplicaties op veel diensten | Huly is een platform met meerdere diensten. Dit project is één dienst op PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Een desktopalternatief voor Microsoft Project, voor één manager en één project | Dit project is een gehoste dienst voor meerdere gebruikers met toegangsbeheer |

## Productvergelijkingen

Afzonderlijke vergelijkingspagina's (in het Engels): [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interoperabiliteit

- **Jira en Asana:** CSV-import, met een voorbeeldweergave en een vermelde reden voor alles wat wordt weggelaten.
- **Microsoft Project:** import en export van MSPDI-XML, met taken, outline, datums, doorlooptijden, beperkingen, afhankelijkheden en de kalender. Toegewezen personen, middelen en basislijnen worden uitgesloten, en het bestand legt dit vast.

## Ontwerpverschillen

- **Vermelde redenen.** Een ontbrekend cijfer wordt getoond met een reden, bijvoorbeeld "geen prognose, omdat…".
- **Verklaarbare detectie van dubbelingen.** Matching werkt voor alle plantypen en toont het bewijs.
- **Veilige standaardinstellingen.** Inloggen staat standaard aan, en beveiligingsmateriaal maakt deel uit van de repository.
- **Expliciete doorlooptijden.** Elke doorlooptijd is vermeld in kalenderdagen of werkdagen.
- **Vastgelegde grenzen.** De backlog legt de beperkingen van elk onderdeel vast.

## Bekende beperkingen

- Geen mobiele app of desktoptoepassing.
- Geen realtime gezamenlijk bewerken.
- Eén beheerder, een beperkte productiegeschiedenis en geen audit door derden.
- De gepubliceerde belastingtest draaide op één machine. Hij geeft een ordegrootte aan en is geen capaciteitsplan.
- De vertalingen zijn concepten die nog moeten worden beoordeeld.
- Geen scherm om wijzigingsverzoeken op te stellen of te beslissen.
- Geen ecosysteem van plug-ins of marktplaats.

Meer details staan in de [vergelijkingsaantekeningen](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Correcties kunnen met een bron als issue worden ingediend.
