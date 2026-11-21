# Vejledning

*Trin til at køre demonstrationen på en lokal computer og gennemgå en eksempelportefølje.*

Demonstrationen er en komplet kopi af softwaren, indlæst med data for en fiktiv organisation, Northgate Transport Authority. Alle personer, planer, tal og datoer er fiktive, og dataene kan ændres frit.

**Sikkerhedsnote.** Demonstrationen stoler på en offentliggjort testnøgle. Den må ikke køres på et delt netværk eller bruges med rigtige data.

## Krav

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) med Compose

## Trin 1: hent koden

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Trin 2: start demonstrationen

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Kommandoen bygger softwaren, starter databasen, indlæser den fiktive portefølje og starter brugerfladen. Den første build tager længere tid end de følgende.

## Trin 3: log ind

Åbn <http://localhost:3000/da-001/signin>, lad adressen stå som `example@example.com`, og tryk på **Sign in to the demonstration**. Demonstrationens knapper og menuer er på engelsk. Der sendes ingen e-mail.

Den første side er **demonstrationsrundturen**, som viser alle funktioner i grupper, med en beskrivelse og et link. Et gult banner på hver side oplyser, at dataene er fiktive.

## Trin 4: gennemgå porteføljen

Åbn **Plans** fra rundturen for at se porteføljen, dens programmer og dens projekter. Åbn derefter **Portfolio dashboard**.

## Trin 5: gennemgå ét projekt

Åbn projektet **North Line interlocking** via rundturens leveringsgruppe. Gennemgå dets **Gantt**-diagram, **Critical path** og **Earned value**.

## Trin 6: gennemgå personligt arbejde

Åbn **My work** og **My kanban board**. Sessionen er logget ind som Example Person, som har åbne opgaver.

## Trin 7: gennemgå kapacitet, startstyring og rapportering

Åbn **Capacity heatmap** for at se overbookede ressourcer. Åbn **Start queue**, hvor grænsen for igangværende arbejde har holdt ét program tilbage. Åbn **Director packs**, den månedlige oversigt, der fryses, når den er udstedt.

## Trin 8: stop demonstrationen

Kommandoen stopper alle tjenester og sletter demonstrationens data:

```sh
podman compose --profile demo down -v
```

## Næste skridt

- [Installationsvejledning](../../../../INSTALL.md), som dækker login og identitetsudbydere
- [Rollebeskrivelser](../../for/) til topledere
- [Vejledning til demonstrationen](../../../../operations/demo.md)
- [Funktionsoversigt](../funktioner/index.md)

Problemer kan meldes gennem [sagshåndteringen](https://github.com/project-portfolio-management/project-portfolio-management/issues).
