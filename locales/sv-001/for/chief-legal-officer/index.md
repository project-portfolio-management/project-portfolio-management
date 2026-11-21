# För chefsjuristen

*Chefsjurist: licens, dataskydd, tillgänglighet, språkskyldigheter och vad projektet inte garanterar. Detta är inte juridisk rådgivning.*

## Licens

Erbjuds under någon av MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only eller GPL-3.0-only, eller enligt särskilda villkor efter överenskommelse. Valet och texterna finns i [`LICENSE.md`](../../../../LICENSE.md) och `LICENSE/`; ett test håller varje manifest förenligt med dem.

## Dataskydd (den lag som gäller för dig)

Skrivet för vilken dataskyddslag som helst: [efterlevnadsspecifikationen §12.2a](../../../../spec/12-compliance.md) kartlägger de vanligaste efter region (GDPR och brittisk GDPR, LGPD, PIPL, DPDP Act, PIPEDA, CCPA med flera) till vad var och en leder till här, utifrån allmän kunskap och inte kontrollerat mot varje lags text. Förifyllda dokument som du anpassar; varje `‹buyer›`-markering är ett beslut som är ditt (personuppgiftsansvarig, rättslig grund, gallringsperioder, personuppgiftsbiträden, godkännande):

- [Mall för konsekvensbedömning avseende dataskydd (DPIA)](../../../../governance/dpia.md) och [register över behandling](../../../../governance/record-of-processing.md)
- [Förteckning över personuppgifter](../../../../governance/personal-data-inventory.csv): varje kolumn som kan innehålla personuppgifter
- [Gallringsschema](../../../../governance/retention-schedule.md): förslag, med vad koden tillämpar i dag
- [Dataflödesdiagram](../../../../governance/data-flow.md)

Programvaran lagrar personer som ogenomskinliga referenser samt registrerad tid, tillgänglighet och kostnadssatser, vilket ligger nära övervakning och därför motiverar en DPIA. Den erbjuder export av registrerades data och radering genom pseudonymisering eller, om driftsättningen väljer det, borttagning (endast administratörer, förhandsvisning först; WW-1.8). Frysta direktionspaket och endast tillägg-historik över säkerhet töms på uppgift om vem som skrev eller bedömde, men raderas aldrig, så att en begäran om radering kan uppfyllas utan att posten bryts. Den hittar **inte** namn som skrivits i fritext, och säkerhetskopior innehåller poster som applikationen har raderat.

## Ramverk

HIPAA och de amerikanska certifieringsordningarna för hälsodata är **inte tillämpliga** (inga patientuppgifter); regelverket för kliniska enheter är inte tillämpligt. [DTAC-checklistan](../../../../governance/dtac-checklist.md) besvarar det brittiska NHS:s Digital Technology Assessment Criteria, ett nationellt system som behålls som ett arbetsexempel; för andra länders och sektorers system (ISO/IEC 27001, SOC 2, nationella moln- och offentliga system) utgår du från [checklistan för kvalitetssäkring](../../../../governance/assurance-checklist.md). Ingenting är certifierat.

## Tillgänglighet och språk

WCAG 2.2 AA testas automatiskt på varje rutt; inget manuellt test eller test med hjälpmedel har gjorts ([uttalande](../../../../governance/accessibility-statement.md)). Walesiska och andra översättningar är utkast som väntar på en professionell översättare ([språkuttalande](../../../../governance/language-statement.md)); relevant om du har lagstadgade språkskyldigheter.

## Leverantörsgranskning

En enda underhållare, ingen avtalad support, ingen garanti utfärdad, AI-verktyg används vid utvecklingen och redovisas i [AI-uttalandet](../../../../AI_STATEMENT.md). Produkten innehåller ingen AI-modell. Rapportering av sårbarheter: [SECURITY.md](../../../../SECURITY.md).

## Beslut som är dina

Rättslig grund och gallringsperioder; om du behöver ett förfarande för rättsligt bevarande (inget är byggt); avtals- och supportvillkor med underhållaren.

## Läs vidare

[Checklista för kvalitetssäkring](../../../../governance/assurance-checklist.md) · [Index över styrningspaketet](../../../../governance/README.md)
