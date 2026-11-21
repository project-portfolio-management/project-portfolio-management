# Til it-direktør

*It-direktør: integration, identitet, data, drift og de oplysninger, du vil blive bedt om at udlevere.*

## Identitet og adgang

- Login er **påkrævet som standard**; tjenesten nægter at starte uden nøgler til verifikation af tokens, og i produktion med login slået fra.
- Log ind gennem en identitetsudbyder (herunder Microsoft Entra ID); multifaktorgodkendelse bliver hos udbyderen ([vejledning til login](../../../../operations/sign-in-with-entra.md)).
- Autorisation bruger tokenattributter (for eksempel `access`, `directorpack`, `confidence`) med en attributpolitik. Private planer er skjult fra alle aggregater; klassificeringsmærker skjuler planer over en brugers sikkerhedsgodkendelse, også for administratorer.
- Nødadgang (break-glass) til private planer kræver berettigelse, en angivet begrundelse, højst fire timer og at en anden person får besked.
- Administratorer kan vise faste adgangsrettigheder (en hjælp til recertificering) og eksportere eller pseudonymisere én persons data.

## Data og integration

| Emne | Stilling |
|---|---|
| Opbevarede data | Poster om planer, opgaver, risici, budgetter, milepæle, tid og omkostninger; personer er uigennemsigtige referencer, ikke navne. Ingen patientdata og ingen særlige kategorier af data efter design |
| Fortegnelse | Hver kolonne med personoplysninger er opført i [fortegnelsen over personoplysninger](../../../../governance/personal-data-inventory.csv); en test sikrer, at den passer til skemaet |
| Grænseflader | REST med et afledt OpenAPI-dokument; masseimport og -eksport; kalenderfeeds; udgående hændelser og webhooks kun til offentlige `https`-destinationer |
| Sikkerhedskopier | Scriptet sikkerhedskopiering med afprøvet gendannelse, valgfri kryptering og oplyste genopretningsmål ([sikkerhedskopiering og gendannelse](../../../../operations/backup-and-restore.md)) |
| Drift | Containerbilleder og `compose.yaml`; softwaren foretager ingen overførsler af egen drift, så placeringen af dataene er dit valg af hosting |

## Huller, du må lukke

- Penetrationstest, uafhængig kodegennemgang og certificering: ingen udført.
- Opbevaring håndhæves kun delvist; sletningen skal planlægges (se [opbevaringsplanen](../../../../governance/retention-schedule.md)).
- Værktøjet til registrerede finder referencer, ikke navne skrevet i fri tekst eller uploadede filer.
- Revisionssporet er ikke manipulationssikkert over for en databaseadministrator.

## Læs videre

[Datastrømsdiagram](../../../../governance/data-flow.md) · [Tjekliste til kvalitetssikring](../../../../governance/assurance-checklist.md) · [Drift](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
