# Vejen til produktion

*Hvad "produktion" betyder for denne software, og hvor langt den er fra det, punkt for punkt.*

"Produktion" betyder her, at en organisation bruger softwaren til rigtigt arbejde. Rigtige planer, rigtige menneskers navne og rigtige beslutninger afhænger af den, et nedbrud eller en lækage har konsekvenser, og nogen er ansvarlig. Hos et stort sundhedsorgan eller et ministerium betyder produktion også, at organisationens egen kvalitetssikring er bestået: en sikkerhedstest, en konsekvensanalyse for databeskyttelse, en tilgængelighedsrevision og, i det britiske sundhedsvæsen NHS, Digital Technology Assessment Criteria (DTAC).

## Hvor den står

- Der hævdes ingen certificering, godkendelse eller revision. Denne side er projektets eget estimat.
- "Medicinsk" betyder et **ikke-klinisk** værktøj til porteføljer og projekter: ingen patientjournaler og ingen klinisk beslutningsstøtte.
- Projektet foreslår at starte med en pilot på ikke-kliniske, ikke-hemmelige data, kørt af en organisation, der selv gennemfører sin kvalitetssikring.

## Parathed, punkt for punkt

Hvert tal er projektets estimat den 13. november 2026, ikke en måling. Hver skyder starter ved dette estimat. Når du flytter en skyder, registreres dit eget syn, mens du læser; intet bliver gemt eller sendt.

- **Uafhængig sikkerhedssikring** — 0%. Ingen penetrationstest, ingen revision fra tredjepart og ingen certificering. En køber eller en leverandør må bestille dem (RG-13).
- **Tilgængelighed** — 40%. En automatisk WCAG 2.2 AA-revision kører på hver rute ved hver ændring. Der er ikke udført nogen manuel test og ingen test med hjælpeteknologi eller med personer med handicap (RG-13).
- **Anmodninger fra registrerede** — 50%. Administratorer kan eksportere og pseudonymisere en person på tværs af alle referencer i skemaet, med en forhåndsvisning først (RG-6). Navne, der er skrevet i titler, kommentarer eller dokumenter, uploadede filer, snapshots af revisionssporet og sikkerhedskopier findes ikke automatisk.
- **Opbevaring** — 35%. En sletning, som som standard er en prøvekørsel, dækker notifikationer, leveringslogge, idempotensnøgler samt snapshots af tendenser og brug, med perioder fastsat for hver installation (RG-4). Planer, opgaver, tidsregistreringer, dokumenter og revisionssporet opbevares, indtil en driftsansvarlig sletter dem.
- **Klassificeringsmærker** — 60%. En konfigurerbar klassificering skjuler planer over en brugers sikkerhedsgodkendelse, også for administratorer, og mærker hvert svar (RG-7). Mærket stemples ikke ind i genererede filer, og godkendelsen er kun så god som identitetsudbyderens attribut.
- **Sikkerhed for vedhæftninger** — 50%. Indholdet skal svare til den angivne type, programmer afvises, og en valgfri `clamd`-scanning lukker ved fejl (RG-8). Der følger ingen scanner med, og tilslutningspunktet er kun afprøvet mod en stedfortræder.
- **Adgangsgennemgang og nødadgang** — 45%. En rapport over faste rettigheder understøtter recertificering (RG-9), og nødadgang er tidsbegrænset og kræver en angivet begrundelse (RG-10). Rapporten viser, hvad der er registreret, ikke hvad identitetsudbyderen håndhæver; ingen rute registrerer en gennemgangs beslutning; og en meddelelse om nødadgang kan nævne en person, der ikke modtager noget, mens rettigheden stadig gælder.
- **Ydeevne og kapacitet** — 40%. En belastnings- og udholdenhedstest på én maskine står i [`BENCHMARKS.md`](../../../BENCHMARKS.md): almindelige forespørgsler besvares på ti-talsvis af millisekunder, en plan med omkring 12.700 opgaver er langsom at planlægge, og processen vokser til omkring 1 GB. Der er ingen kapacitetsplan, ingen test med mange brugere og ingen kørsel på flere maskiner.
- **Sessioner på tværs af flere frontend-servere** — 70%. Sessioner holdes i hukommelsen, medmindre Redis er konfigureret (RG-12); med Redis afhænger login af det.
- **Revisionssporets integritet** — 50%. Skrivninger revideres, og en databasetrigger afviser ændringer og sletninger, med oplyste undtagelser (RG-3). En databaseadministrator kan fjerne triggeren, og der er ingen hashkæde.
- **Sikkerhedskopiering og genopretning** — 80%. En gendannelse afprøves i kontinuerlig integration, både ukrypteret og krypteret, mod oplyste genopretningsmål (RG-5). Kryptering er valgfri, og øvelsen kører på projektets egen CI, ikke på driftsansvarliges hosting.
- **Support og kontinuitet** — 10%. Én vedligeholder og ingen serviceaftale ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Oversættelser** — 20%. Tekst på andre sprog end engelsk er maskinskrevet, indtil en person, der taler sproget, har gennemgået den, og lovpligtige sprogkrav kræver gennemgået tekst ([sprogerklæring](../../../governance/language-statement.md)).
- **Klinisk brug** — 0%. Der er ikke skrevet nogen klinisk sikkerhedsvurdering, og klinisk brug ligger uden for omfanget.

## Hvad en første installation bør gøre

1. Gennemfør [kvalitetssikringens tjekliste](../../../governance/assurance-checklist.md), og start med en penetrationstest og en tilgængelighedsrevision.
2. Udfyld hver række, der er mærket "Buyer to complete", i [DTAC-tjeklisten](../../../governance/dtac-checklist.md) og markeringerne i [DPIA'en](../../../governance/dpia.md).
3. Aftal opbevaringsperioder med arkivansvarlig, og planlæg sletningen.
4. Slå krypterede sikkerhedskopier til, og kør gendannelsesøvelsen på installationens egen hosting.
5. Hold dataene ikke-kliniske og under det klassificeringsniveau, som installationen er kvalitetssikret til.
6. Planlæg for hullerne ovenfor, eller finansier, at de bliver lukket.

Dokumentationen for hvert punkt findes i [vurderingen](../../../governance/production-readiness.md).
