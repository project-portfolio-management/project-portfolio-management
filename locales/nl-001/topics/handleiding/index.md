# Handleiding

*Stappen om de demonstratie op een lokale computer uit te voeren en een voorbeeldportfolio te bekijken.*

De demonstratie is een volledige kopie van de software, gevuld met gegevens van een fictieve organisatie, de Northgate Transport Authority. Alle personen, plannen, cijfers en datums zijn fictief, en de gegevens mogen vrij worden gewijzigd.

**Beveiligingsopmerking.** De demonstratie vertrouwt op een gepubliceerde testsleutel. Voer haar niet uit op een gedeeld netwerk en gebruik haar niet met echte gegevens.

## Vereisten

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) met Compose

## Stap 1: de code downloaden

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Stap 2: de demonstratie starten

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Dit commando bouwt de software, start de database, laadt het fictieve portfolio en start de interface. De eerste build duurt langer dan latere.

## Stap 3: inloggen

Open <http://localhost:3000/nl-001/signin>, laat het adres staan op `example@example.com` en druk op **Sign in to the demonstration**. Er wordt geen e-mail verzonden. De demonstratie toont haar eigen interfaceteksten in het Engels; die namen staan hieronder zoals ze op het scherm staan.

De eerste pagina is de **demonstratietour**, die elke functie per groep opsomt, met een beschrijving en een koppeling. Een gele banner op elke pagina meldt dat de gegevens fictief zijn.

## Stap 4: het portfolio bekijken

Open vanuit de tour **Plans** om het portfolio, de programma's en de projecten te bekijken. Open daarna het **Portfolio dashboard**.

## Stap 5: één project bekijken

Open het project **North Line interlocking** via de leveringsgroep van de tour. Bekijk het **Gantt**-diagram, het **Critical path** en de **Earned value**.

## Stap 6: persoonlijk werk bekijken

Open **My work** en **My kanban board**. De sessie is aangemeld als Example Person, die openstaande taken heeft.

## Stap 7: capaciteit, startbeheersing en rapportage bekijken

Open **Capacity heatmap** om overboekte middelen te zien. Open **Start queue**, waar de limiet voor werk in uitvoering één programma heeft tegengehouden. Open **Director packs**, de maandelijkse samenvatting die wordt bevroren zodra ze is uitgegeven.

## Stap 8: de demonstratie stoppen

Dit commando stopt alle diensten en verwijdert de demonstratiegegevens:

```sh
podman compose --profile demo down -v
```

## Vervolgstappen

- [Installatiegids](../../../../INSTALL.md), met inloggen en identiteitsproviders
- [Rolbeschrijvingen](../../for/) voor senior rollen
- [Demonstratiegids](../../../../operations/demo.md)
- [Functieoverzicht](../functies/index.md)

Problemen kunnen worden gemeld via de [issuetracker](https://github.com/project-portfolio-management/project-portfolio-management/issues).
