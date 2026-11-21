# Fordele

*De vigtigste fordele for en organisation, fordelt på områder.*

## Overblik over porteføljen

Porteføljeoplysninger ligger ofte flere steder. I denne software er hver portefølje, hvert program, projekt og produkt én posttype, kaldet en **plan**. Enhver plan kan indeholde enhver anden, i vilkårlig dybde, så strukturen kan svare til organisationen. En betegnelse som "Projekt" er beskrivende og sætter ingen begrænsning for en plan.

## Færre dubletter

To afdelinger kan oprette den samme indsats under forskellige navne. Matcheren sammenligner de egenskaber, der identificerer en plan: navn, mål, ejer, sponsor, overordnet plan, tidsramme, nøgleord og identifikatorer. Den arbejder på tværs af alle plantyper og viser dokumentationen for hvert match, og et menneske træffer beslutningen. To poster for den samme plan kan lægges sammen.

## Rapporteringens kvalitet

- Et manglende tal vises med en årsag, for eksempel "ingen prognose, fordi …".
- En plan uden vurdering vises som "ikke vurderet".
- Leveringssikkerhed angivet af mennesker vises ved siden af den beregnede status, og de to holdes adskilt.
- Private planer er udeladt fra alle totaler og optællinger for alle brugere. Tests kontrollerer denne regel.
- Hver varighed oplyser, om den er i kalenderdage eller arbejdsdage.

## Beslutningsregistre

Hver ændring bliver revideret, og revisionssporet kan ikke ændres gennem tjenesten. Den, der stiller en ændringsanmodning, kan ikke godkende den. En månedlig direktionspakke udarbejdes én gang, udstedes af en navngiven person og fryses derefter af databasen. En rettelse udstedes som en ny version med en note.

## Sikkerhed

Login er slået til som standard, og tjenesten starter ikke i produktion med login slået fra. Sikkerhedshoveder, begrænsning af antal forespørgsler, en genopretningsplan med oplyste mål og en pakke om informationsstyring er med i repositoriet.

## Kontrol og portabilitet

- **Licens.** Softwaren er open source, med valg mellem MIT, Apache-2.0, BSD-3-Clause, GPL-2.0 eller GPL-3.0.
- **Drift.** Softwaren driftes selv: én Rust-tjeneste oven på PostgreSQL med en SvelteKit-brugerflade.
- **Dataimport.** CSV-filer fra Jira og Asana kan importeres med en forhåndsvisning og en oplyst årsag til alt, der er udeladt. Microsoft Project XML kan importeres og eksporteres.
- **Sprog.** Brugerfladen findes på ti sprogversioner, herunder arabisk, som skrives fra højre mod venstre. Oversættelserne er udkast, der afventer gennemgang af personer, der taler sprogene.

## Relaterede sider

Se også [funktioner](../funktioner/index.md), [sammenligninger](../sammenligninger/index.md) og [vejledningen](../vejledning/index.md).
