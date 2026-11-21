# Uafhængig sikkerhedstest

*Til en køber, en vurderingsmand eller en tester: hvad en uafhængig sikkerhedstest af denne software bør dække, hvad projektet leverer til at hjælpe, hvad der er og ikke er gjort, og hvad der er overladt til dig. Dette er vejledning, ikke en testrapport og ikke juridisk rådgivning.*

## Hvor det står

**Der er ikke udført nogen uafhængig sikkerhedstest.** Der findes ingen penetrationstest, revision fra tredjepart, uafhængig kodegennemgang eller certificering for denne software, og projektet hævder ingen ([vejen til produktion](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). En passiv scanning kører i projektets kontinuerlige integration (regler i `.zap/rules.tsv`), og projektets egne tests kontrollerer de kontroller, der er opført nedenfor; ingen af delene erstatter en tester, der ikke har skrevet softwaren.

Dette er punkt 1 og punkt 9 i [tjeklisten til kvalitetssikring](../../../../governance/assurance-checklist.md). Det tilhører den, der installerer, med en certificeret tester, mod **din** installation: projektet kan ikke teste din hosting, din identitetsudbyder, dit netværk eller dine nøgler.

## Hvad der skal testes

Test installationen, som den kommer til at køre, med login påkrævet og din identitetsudbyder foran. Foreslået omfang, ud fra de kontroller, softwaren hævder at have:

| Område | Hvad softwaren hævder | Hvad en tester bør prøve |
|---|---|---|
| Godkendelse | Login er slået til som standard; tjenesten starter ikke uden verifikationsnøgler eller, i produktion, med login slået fra; tokens verificeres offline mod offentliggjorte nøgler | Forfalskede, udløbne, genafspillede og forkert signerede tokens; forkert udsteder eller målgruppe; nøglerotation; et manglende nøglesæt |
| Autorisation | Attributbaseret politik; regler på postniveau | Vandret og lodret rettighedseskalering på hver rute; ændring af identifikatorer i stier og indhold; attributter, en bruger ikke bør kunne tilføje |
| Private planer og klassificeringsmærker | En privat plan og en plan over en brugers sikkerhedsgodkendelse findes ikke i nogen liste, optælling, total, søgeresultat, feed eller eksport for den bruger, heller ikke for administratorer | Enhver rute, der afslører eksistensen, et navn, en optælling eller en total for en plan, brugeren ikke kan læse; fejlmeddelelser og svartider, der adskiller sig mellem "skjult" og "findes ikke" |
| Nødadgang (break-glass) | Kun berettigede personer, en angivet begrundelse, højst fire timer, en anden person får besked | Brug uden berettigelse; forlængelse af en rettighed; handling uden at den anden person får besked |
| Frosne poster og poster, der kun kan tilføjes til | Revisionssporet, udstedte direktionspakker, historikken over leveringssikkerhed, historikken over programmets status og projektbuffere afviser ændring og sletning i databasen | Opdatering eller sletning gennem API'et, gennem masseimport og gennem enhver rute, der rører de samme rækker |
| Funktionsadskillelse | En anmoder kan ikke godkende sin egen ændringsanmodning | Godkendelse som anmoderen, som administrator og ved at ændre godkenderen efter indsendelsen |
| Uploads og vedhæftninger | Indholdet skal svare til den angivne type; programmer afvises; et valgfrit tilslutningspunkt til virusscanning lukker ved fejl | Polyglot-filer, omdøbte programmer, for store og ugyldige filer, tricks med stier i navne, og om din scanner virkelig bliver kaldt |
| Udgående forespørgsler | Destinationer angivet af brugeren er begrænset til offentlige `https`-adresser | Server-side request forgery mod private adresser, link-lokale adresser og metadataadresser, omdirigeringer, DNS rebinding |
| Sessioner og browseren | Timeout ved inaktivitet, en fast grænse, en `httpOnly`-cookie, en streng Content Security Policy, en tilladelsesliste til CORS | Session fixation og genbrug, cross-site request forgery, indsprøjtning i brugerfladen, clickjacking |
| Begrænsning af antal forespørgsler | Grænser pr. bruger | Omgåelse ved at skifte identitet, header eller sti; om grænserne overlever din proxy |
| Værktøjer til registrerede | Eksport og pseudonymisering, kun administratorer, forhåndsvisning først | Brug af en ikke-administrator; fuldstændighed i forhold til [fortegnelsen over personoplysninger](../../../../governance/personal-data-inventory.csv) |
| Sikkerhedskopier | Valgfri kryptering, en afprøvet gendannelse | Adgang til sikkerhedskopier og nøgler; gendannelse i et rent miljø |

## Hvad projektet leverer

- Kildekoden, specifikationen og testene, så en kodegennemgang kan starte fra påstandene.
- En maskinlæsbar beskrivelse af API'et (`/api-docs/openapi.json`), med skemaer for forespørgsler og svar for arbejdet med direktionspakken; listen over ruter kontrolleres mod koden af tests.
- En engangskonfiguration med login slået fra (`dev-insecure`, se [README](../../../../README.md)) **kun til testerens egen maskine**, og et script, der genererer et testnøglesæt (`scripts/smoke-keys.sh`), så tokens med valgte attributter kan udstedes mod en testinstallation.
- [Datastrømsdiagrammet](../../../../governance/data-flow.md), [driftshåndbogen](../../../../operations/runbook.md) og [genopretningsmålene](../../../../operations/recovery-objectives.md).
- En privat vej til at indberette det, der findes: [SECURITY.md](../../../../SECURITY.md).

## Spilleregler

Aftal disse skriftligt før enhver test; projektet kan ikke fastsætte dem for dit miljø.

- **Miljø.** Test en kopi med realistiske men ikke rigtige data. Test ikke produktion, medmindre du skriftligt har besluttet det.
- **Tilladelse.** Testeren har din skriftlige tilladelse, en navngiven kontaktperson og et angivet tidsrum.
- **Uden for rammerne.** Denial of service ved mængde, social manipulation af medarbejdere og tredjepartstjenester (din identitetsudbyder, din hosting) er uden for omfanget, medmindre ejeren af hver af dem er indforstået. Projektets eget omfang står i [SECURITY](../../../../SECURITY.md).
- **Data.** Rigtige personoplysninger bruges ikke, kopieres ikke ud og indgår ikke i rapporten.
- **Nøgler.** Testnøgler holdes adskilt fra produktionsnøgler og destrueres bagefter.

## Rapportering og gentest

- Fund om denne software går privat til vedligeholderen, som [SECURITY](../../../../SECURITY.md) beskriver, med en vejledning til at gengive dem, versionen eller committen og konsekvensen. Projektets mål er en bekræftelse inden for 5 hverdage og en plan inden for 15; de er mål, ikke garantier. Der er én vedligeholder og ingen serviceaftale.
- Fund om din installation går til din egen ejer.
- Bed testeren om at vurdere hvert fund, om at oplyse, hvad der **ikke** blev testet, og om at gentest rettelserne. En rapport, der kun opregner det, der blev fundet, fortæller dig ikke, hvad der blev overset.
- Opbevar rapporten, omfanget, datoerne og gentesten sammen med dine egne optegnelser. Projektet kan ikke opbevare dem for dig.

## Dette dokuments grænser

Det opregner områder, der skal dækkes, ikke en fuldstændig testplan, og påstandene i tabellen er projektets egne. Det certificerer intet, og en ren test er dokumentation for én installation på én dato.

## Beslutninger, der er dine

Hvem der tester, hvornår, mod hvad, med hvilke data og efter hvilke regler; hvordan fund vurderes, og hvornår de senest skal være rettet; og om resultatet deles med dine vurderingsmænd.

## Læs videre

[Tjekliste til kvalitetssikring](../../../../governance/assurance-checklist.md) · [Vejen til produktion](../../path-to-production/index.md) · [Sikkerhedsdirektør](../chief-security-officer/index.md) · [Governance, risiko og overholdelse](../governance-risk-compliance/index.md)
