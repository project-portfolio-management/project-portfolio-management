# Handledning

*Steg för att köra demonstrationen på en lokal dator och granska en exempelportfölj.*

Demonstrationen är en fullständig kopia av programvaran, laddad med data för en fiktiv organisation, Northgate Transport Authority. Alla personer, planer, värden och datum är fiktiva, och uppgifterna kan ändras fritt.

**Säkerhetsanmärkning.** Demonstrationen litar på en publicerad testnyckel. Den får inte köras på ett delat nätverk eller användas med verkliga data.

## Krav

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) med Compose

## Steg 1: ladda ned koden

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Steg 2: starta demonstrationen

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Kommandot bygger programvaran, startar databasen, laddar den fiktiva portföljen och startar gränssnittet. Det första bygget tar längre tid än de följande.

## Steg 3: logga in

Öppna <http://localhost:3000/sv-se/signin>, låt adressen vara `example@example.com` och tryck på **Sign in to the demonstration**. Inget e-postmeddelande skickas.

Den första sidan är **demonstrationsrundan**, som listar varje funktion efter grupp, med en beskrivning och en länk. Ett gult fält på varje sida anger att uppgifterna är fiktiva.

Demonstrationens gränssnittsetiketter är på engelska och anges nedan som de visas.

## Steg 4: granska portföljen

Från rundan öppnar du **Plans** för att se portföljen, dess program och dess projekt. Öppna sedan **Portfolio dashboard**.

## Steg 5: granska ett projekt

Öppna projektet **North Line interlocking** via rundans leveransgrupp. Granska dess **Gantt**-schema, **Critical path** och **Earned value**.

## Steg 6: granska personligt arbete

Öppna **My work** och **My kanban board**. Sessionen är inloggad som Example Person, som har öppna uppgifter.

## Steg 7: granska kapacitet, startkontroll och rapportering

Öppna **Capacity heatmap** för att se överbokade resurser. Öppna **Start queue**, där gränsen för pågående arbete har hållit tillbaka ett program. Öppna **Director packs**, den månatliga sammanfattningen som fryses när den har utfärdats.

## Steg 8: stoppa demonstrationen

Det här kommandot stoppar alla tjänster och raderar demonstrationsdata:

```sh
podman compose --profile demo down -v
```

## Nästa steg

- [Installationsguide](../../../../INSTALL.md), som omfattar inloggning och identitetsleverantörer
- [Rollsammanfattningar](../../for/) för ledande roller
- [Demonstrationsguide](../../../../operations/demo.md)
- [Funktionslista](../funktioner/index.md)

Problem kan rapporteras via [ärendehanteringen](https://github.com/project-portfolio-management/project-portfolio-management/issues).
