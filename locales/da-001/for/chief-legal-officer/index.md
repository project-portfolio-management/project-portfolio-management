# Til juridisk direktør

*Juridisk direktør: licens, databeskyttelse, tilgængelighed, sprogkrav og hvad projektet ikke garanterer. Dette er ikke juridisk rådgivning.*

## Licens

Tilbydes under en af MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, GPL-3.0-only, OGL-UK-3.0 eller CC-BY-NC-SA-4.0, eller på særlige vilkår efter aftale. Valget og teksterne findes i [`LICENSE.md`](../../../../LICENSE.md) og `LICENSES/`; en test sikrer, at alle manifester stemmer overens med dem.

## Databeskyttelse (den lov, der gælder for dig)

Skrevet til enhver databeskyttelseslov: [overholdelsesspecifikationen §12.2a](../../../../spec/12-compliance.md) kortlægger de almindelige efter region (GDPR og UK GDPR, LGPD, PIPL, DPDP Act, PIPEDA, CCPA og andre) til det, hver af dem medfører her, ud fra almen viden og ikke kontrolleret mod hver lovs tekst. Udfyldte dokumenter, du tilpasser; hver `‹buyer›`-markering er en beslutning, der er din (dataansvarlig, behandlingsgrundlag, opbevaringsperioder, databehandlere, godkendelse):

- [DPIA-skabelon](../../../../governance/dpia.md) og [fortegnelse over behandlingsaktiviteter](../../../../governance/record-of-processing.md)
- [Fortegnelse over personoplysninger](../../../../governance/personal-data-inventory.csv): hver kolonne, der kan indeholde personoplysninger
- [Opbevaringsplan](../../../../governance/retention-schedule.md): forslag, med hvad koden håndhæver i dag
- [Datastrømsdiagram](../../../../governance/data-flow.md)

Softwaren opbevarer personer som uigennemsigtige referencer samt registreret tid, tilgængelighed og omkostningssatser, hvilket grænser til overvågning og derfor kræver en DPIA. Den tilbyder eksport af den registreredes data og sletning ved pseudonymisering eller, hvis installationen vælger det, ved egentlig sletning (kun administratorer, forhåndsvisning først; WW-1.8). Frosne direktionspakker og en tilføjelsesbaseret historik over sikkerhed får fjernet den person, der har udarbejdet eller vurderet, og slettes aldrig, så en anmodning om sletning kan opfyldes uden at bryde registreringen. Den finder **ikke** navne, der er skrevet i fri tekst, og sikkerhedskopier indeholder poster, som programmet har slettet.

## Rammer

HIPAA og de amerikanske certificeringsordninger for sundhedsdata er **ikke berørt** (ingen patientdata); ordningen for klinisk udstyr er ikke berørt. [DTAC-tjeklisten](../../../../governance/dtac-checklist.md) besvarer den britiske NHS' Digital Technology Assessment Criteria, én national ordning, der er bevaret som et gennemarbejdet eksempel; for andre landes og sektorers ordninger (ISO/IEC 27001, SOC 2, nationale cloud- og offentlige ordninger) start fra [tjeklisten til kvalitetssikring](../../../../governance/assurance-checklist.md). Intet er certificeret.

## Tilgængelighed og sprog

WCAG 2.2 AA testes automatisk på hver rute; der er ikke udført nogen manuel test eller test med hjælpeteknologi ([erklæring](../../../../governance/accessibility-statement.md)). Walisiske og andre oversættelser er udkast, der afventer en professionel oversætter ([sprogerklæring](../../../../governance/language-statement.md)); relevant, hvis du har lovpligtige sprogkrav.

## Leverandørundersøgelse

Én vedligeholder, ingen kontraktlig support, ingen garanti hævdet, og AI-værktøjer er brugt i udviklingen og oplyst i [AI-erklæringen](../../../../AI_STATEMENT.md). Produktet indeholder ingen AI-model. Indberetning af sårbarheder: [SECURITY.md](../../../../SECURITY.md).

## Beslutninger, der er dine

Behandlingsgrundlag og opbevaringsperioder; om du har brug for en procedure for juridisk tilbageholdelse (ingen er bygget); kontrakt- og supportvilkår med vedligeholderen.

## Læs videre

[Tjekliste til kvalitetssikring](../../../../governance/assurance-checklist.md) · [Oversigt over governance-pakken](../../../../governance/README.md)
