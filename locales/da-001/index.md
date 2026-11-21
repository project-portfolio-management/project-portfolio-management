# Project Portfolio Management

*Et register over finansieret arbejde med registrering af dubletter og et arbejdsområde til projektledelse.*

Project Portfolio Management er open source-software til at registrere de planer, en organisation finansierer, og til at styre arbejdet i dem. Det dækker opgaver, tidsplaner, omkostninger, ressourcer, risici og governance. Softwaren sammenligner planer for at finde mulige dubletter og viser dokumentationen for hvert match. Hvor et tal ikke kan beregnes, oplyser softwaren årsagen.

## Vigtigste fordele

- **Én samlet porteføljepost.** Porteføljer, programmer, projekter og produkter bruger samme posttype, og enhver plan kan indeholde en anden. Rapporteringen bygger på én liste på tværs af alle afdelinger.
- **Registrering af dubletter.** Matchningen sammenligner planer, der er registreret under forskellige navne, og viser dokumentationen for hvert resultat, så de, der gennemgår dem, kan handle, før begge bliver finansieret.
- **Sporbare tal.** Et manglende tal vises med årsagen til, at det mangler. En plan uden vurdering vises som "ikke vurderet".
- **Revisionsspor.** Hver ændring bliver revideret. Den, der stiller en ændringsanmodning, kan ikke godkende den.
- **Adgangskontrol.** Login er slået til som standard, og tjenesten kører ikke i produktion med login slået fra.
- **Drift og licenser.** Softwaren driftes selv og tilbydes under fem open source-licenser. Arbejde kan importeres fra Jira, Asana og Microsoft Project.

[Se alle fordele](topics/fordele/index.md)

## Funktionsområder

- **Planer og matchning.** Rekursivt register, forklarlig registrering af dubletter og sammenlægning.
- **Levering.** Kanban-tavler, Gantt-diagrammer, kalendere, milepæle, kritisk vej og kritisk kæde.
- **Økonomi.** Budgetter, finansieringsjournal, earned value samt omkostnings- og tidsplanrisiko.
- **Ressourcer.** Kompetencepuljer, kapacitetsheatmap og nøglepersoner.
- **Strategi.** Modtagelse af arbejde, idéer, mål og nøgleresultater samt scenarier.
- **Governance.** Risici, problemer, gate-gennemgange, eskalering og månedlige direktionspakker.

[Se alle funktioner](topics/funktioner/index.md)

## Tiltænkte brugere

- **Porteføljekontorer.** Rapportering på tværs af programmer og projekter.
- **Projekt- og leveranceledere.** Planlægning samt styring af omkostninger og risici.
- **Teammedlemmer.** En personlig oversigt over tildelt arbejde.
- **Topledere.** [Rollebeskrivelserne](for/) viser, hvad softwaren giver for hver rolle, dens begrænsninger og de beslutninger, der skal træffes.

## Aktuel status

Softwaren er på et tidligt stadie. Den har én vedligeholder, ingen revision fra tredjepart og ingen supportaftale. Hvad det vil kræve at tage den i produktion, punkt for punkt med et estimat for hvert, står på siden [vejen til produktion](path-to-production/index.md). Kendte begrænsninger er opført på [siden med sammenligninger](topics/sammenligninger/index.md).

## Demonstration

En lokal demonstration bruger en fiktiv transportmyndighed og kræver Git og Podman. Trinnene står i [vejledningen](topics/vejledning/index.md).
