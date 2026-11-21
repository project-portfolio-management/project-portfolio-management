# Til sikkerhedsdirektør

*Sikkerhedsdirektør: hvad softwaren håndhæver, hvad projektet har verificeret, og hvad du selv må teste og drive.*

## Hvad softwaren håndhæver

| Område | Adfærd |
|---|---|
| Godkendelse | Login er slået til som standard. Tjenesten starter ikke uden nøgler til verifikation af tokens og starter ikke i produktion med login slået fra. Log ind gennem en identitetsudbyder som Microsoft Entra ID; multifaktorgodkendelse bliver hos udbyderen ([vejledning til login](../../../../operations/sign-in-with-entra.md)) |
| Autorisation | Attributbaseret politik på tokenattributter. Private planer holdes uden for alle aggregater for alle; klassificeringsmærker skjuler planer over en brugers sikkerhedsgodkendelse, også for administratorer |
| Nødadgang (break-glass) | Adgang til private planer kræver berettigelse, en angivet begrundelse, højst fire timer og at en anden person får besked; hver anmodning revideres |
| Sessioner | Timeout ved inaktivitet (som standard 30 minutter) og en fast grænse for sessionen |
| Transport og browser | Sikkerhedshoveder, en streng Content Security Policy, en tilladelsesliste til CORS, begrænsning af antal forespørgsler pr. bruger |
| Udgående | Destinationer angivet af brugeren er begrænset til offentlige `https`-adresser |
| Uploads | Indholdet skal svare til den angivne type; programmer afvises; en valgfri `clamd`-scanning lukker ved fejl |
| Revision | Revisionssporet afviser ændring og sletning i databasen |
| Frosne poster | Udstedte direktionspakker og vurderinger af leveringssikkerhed er uforanderlige eller kan kun tilføjes til, via databasetrigger |
| Forsyningskæde | `cargo deny`, en softwarestykliste, byggeoprindelse ved udgivelse, en [sikkerhedspolitik](../../../../SECURITY.md) |

## Hvad der ikke er gjort

- Ingen penetrationstest, revision fra tredjepart, uafhængig kodegennemgang eller certificering.
- Ingen medfølgende malwarescanner; `clamd`-tilslutningspunktet er kun afprøvet mod en stedfortræder.
- Revisionssporet kan omgås af en databaseadministrator; der er ingen hashkæde.
- Klassificeringsmærker skjuler og mærker; de krypterer ikke, og et mærke stemples ikke ind i genererede filer.
- Sessioner holdes i hukommelsen, medmindre Redis er konfigureret.
- Værktøjet til registrerede finder referencer, ikke navne skrevet i fri tekst, uploadede filer eller sikkerhedskopier.

## Drift

Sikkerhedskopier har en afprøvet gendannelse, valgfri kryptering og oplyste genopretningsmål ([driftspakken](../../../../operations/README.md)). [Driftshåndbogen](../../../../operations/runbook.md) angiver de tekniske trin ved hændelser; processen for hændelser og databrud er din. Indberet en sårbarhed privat, som [SECURITY.md](../../../../SECURITY.md) beskriver; målene er en bekræftelse inden for 5 hverdage, ikke garantier.

## Beslutninger, der er dine

Hvem der udfører penetrationstesten, og hvornår; identitetsudbyderens betingede adgang og tildeling af roller; hvilket klassificeringsniveau denne installation er kvalitetssikret til; scanneren bag tilslutningspunktet til vedhæftninger; hvor ofte adgang recertificeres.

## Læs videre

[Tjekliste til kvalitetssikring](../../../../governance/assurance-checklist.md) · [Vejen til produktion](../../path-to-production/index.md) · [Datastrøm](../../../../governance/data-flow.md)
