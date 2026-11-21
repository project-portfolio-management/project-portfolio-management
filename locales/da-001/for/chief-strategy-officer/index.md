# Til strategidirektør

*Strategidirektør: hvordan softwaren forbinder strategi med arbejdet, hvad den viser dig, og hvad den endnu ikke kan.*

## Hvad den gør for strategien

- **Ét register over alt, der er finansieret.** Porteføljer, programmer, projekter og produkter er én rekursiv post, så hele porteføljen kan læses som et træ ([formål og vision](../../../../spec/01-purpose-and-vision.md)).
- **Registrering af dubletter.** Forklarlig matchning viser, når to dele af organisationen har oprettet den samme indsats, før begge er finansieret.
- **Strategiske mål og porteføljekortet.** Du markerer mål som strategiske mål og angiver ét primært mål for hvert program. `GET /api/portfolio-map` og den månedlige direktionspakke grupperer programmer efter primært mål. Et program optræder én gang, med dets øvrige mål nævnt; et program uden primært mål opføres under "intet strategisk mål", og totaler tælles aldrig dobbelt.
- **Leveringssikkerhed ved siden af beregnet status.** Mennesker angiver deres sikkerhed for leveringen; softwaren viser den ved siden af den beregnede rød/ravgul/grøn og markerer et hul på to trin. En plan, ingen har vurderet, viser "ikke vurderet", aldrig grøn.
- **En frossen månedlig pakke.** Det, direktionen fik at vide, opbevares præcis som udstedt, med tendensen målt mod den forrige udstedte pakke.

## Hvad den endnu ikke gør

- Pakken indeholder endnu ikke gate-beslutninger, finansiering fordelt på regnskabsår, bufferfeberdiagrammet, earned value, validerede gevinster eller kapacitetsheatmappet (`DP-5` til `DP-10` i [`tasks.md`](../../../../tasks.md)). Pakken oplyser, hvilke afsnit der ikke er bygget.
- "Program" betyder en plan, hvis art er `Program`, fordi dataene ikke har nogen anden definition; mål på "øverste niveau" er de, du markerer, da mål ikke har noget hierarki.
- Der er ingen scenarieplanlægning og endnu ingen skærm til kortet eller pakken: de findes som data.
- Den er ikke certificeret, og den har én vedligeholder ([vejen til produktion](../../path-to-production/index.md)).

## Beslutninger, der er dine

Hvilke mål der er strategiske mål; det primære mål for hvert program; hvem der angiver, og hvem der uafhængigt vurderer leveringssikkerheden; hvor mange måneders tendens direktionen skal se.

## Læs videre

[Køreplan](../../../../spec/15-roadmap.md) · [Åbne spørgsmål](../../../../spec/16-open-questions.md) · [Omfang](../../../../spec/02-scope.md)
