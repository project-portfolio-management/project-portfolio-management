# Til driftsdirektør

*Driftsdirektør: hvad du driver til daglig, hvad softwaren gør for leveringen, og hvad det kræver at drive den.*

## Hvad den gør for driften

- **Leveranceplanlægning og -styring:** planer, opgaver, afhængigheder, en beregning af den kritiske vej, milepæle på tværs af planer, sprints og faser, burndown og flowmål.
- **Kapacitet:** tildelinger, tilgængelighed, fravær og perioder uden arbejde, ressourceroller, afledt arbejdsbyrde og udnyttelse. (Et kapacitetsheatmap efter kompetencepulje og måned til direktionspakken er ikke bygget: `DP-10`.)
- **Tid og omkostninger:** timesedler, registreret tid, omkostningssatser (kun for økonomifunktionen), budgetter og afledte omkostninger.
- **Governance-flow:** register over problemer, ændringsanmodninger med funktionsadskillelse (den, der anmoder, kan ikke godkende), godkendelser, gennemgange, sager om nye, skiftende og fratrædende medarbejdere.
- **Undtagelser:** en rød eller ravgul-rød leveringssikkerhed åbner en undtagelse, der kræver en årsag, den nødvendige beslutning, en navngiven ejer og en frist for beslutningen inden for 48 timer (vinduet kan konfigureres, 1 til 720 timer). Overskridelser vises i direktionspakken og i `GET /api/exceptions`.

## Hvad det kræver at drive den

| Behov | Detaljer |
|---|---|
| Tjeneste | Én Rust-tjeneste, PostgreSQL 18, valgfri Redis (produktionscache, valgfri delte sessioner), en SvelteKit-brugerflade |
| Driftshåndbog | [driftspakken](../../../../operations/README.md): installation, driftshåndbog, sikkerhedskopiering og gendannelse, genopretningsmål, opgradering og migrering |
| Planlagte job | Intet planlægger sig selv. Sletning efter opbevaringsreglerne, meddelelser om overskredne undtagelser og lignende opgaver skal sættes på en planlægger af dine driftsfolk |
| Support | Én vedligeholder, ingen serviceaftale; afsæt budget til din egen førstelinjesupport |

## Kend grænserne

- En plan med flere tusinde opgaver er langsom at planlægge; det fulde svar afvises over 5.000 opgaver, medmindre det udtrykkeligt bedes om (`?full=true` eller `?only=critical`). Tal og forbehold står i [`BENCHMARKS.md`](../../../../BENCHMARKS.md); de er en størrelsesorden fra én maskine, ikke kapacitet.
- Der findes skærme til meget af arbejdsområdet, men **endnu ikke** til ændringsanmodninger, problemer, pakker eller leveringssikkerhed; de findes i dag kun som API.

## Beslutninger, der er dine

Hvem der ejer hver undtagelse, hvem der udsteder den månedlige pakke (rytme og påmindelser er `DP-12`, ikke bygget), og hvilken gennemgangsrytme undtagelsesreglen skal give input til.

## Læs videre

[Driftspakke](../../../../operations/README.md) · [Driftshåndbog](../../../../operations/runbook.md) · [Genopretningsmål](../../../../operations/recovery-objectives.md)
