# Styrning, risk och efterlevnad

*För dem som driver styrning, risk och efterlevnad (GRC): vad programvaran hjälper dig belägga, vad den tillämpar och vad du själv måste bidra med. Inte juridisk rådgivning.*

## Styrning

- **Beslut registreras.** Ändringsbegäranden kräver en namngiven godkännare som inte är den som begär, en angiven orsak på minst 10 tecken, och granskas per aktör; databasen nekar ett godkännande utan beslutsfattare och orsak.
- **Ett fruset månatligt direktionspaket.** Skrivs en gång, utfärdas av en person, oföränderligt när det utfärdats; rättelser är en återutfärdning med en anmärkning. Läsåtkomst sker per målgrupp.
- **Leveranssäkerhet och undantag.** Angiven säkerhet är bara tillägg och visas bredvid beräknad status, aldrig blandad. Ett omdöme i rött eller bärnstensrött öppnar ett undantag med en klocka på 48 timmar (kan konfigureras) som bara ett registrerat beslut stänger.
- **Projektstyrning av själva programvaran:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [ändringskontroll](../../../../spec/18-change-control.md), [AI-uttalande](../../../../AI_STATEMENT.md).

## Risk

- Risker, problem och ändringsbegäranden per plan; ett problemregister över läsbara planer.
- Saknade data visas som "inte bedömd", aldrig som grönt.
- Projektets egna öppna risker (inget penetrationstest, delvis gallring, en enda underhållare, översättningar som är utkast) finns på sidan [Vägen till produktion](../../path-to-production/index.md), punkt för punkt med en uppskattning för var och en.

## Efterlevnad

| Område | Var | Läge |
|---|---|---|
| Dataskydd (tillämplig lag; [§12.2a](../../../../spec/12-compliance.md) kartlägger GDPR, brittisk GDPR, LGPD, PIPL, DPDP, PIPEDA, CCPA med flera) | [DPIA](../../../../governance/dpia.md), [register över behandling](../../../../governance/record-of-processing.md), [förteckning över personuppgifter](../../../../governance/personal-data-inventory.csv), [gallringsschema](../../../../governance/retention-schedule.md), [dataflöde](../../../../governance/data-flow.md) | Förifyllt; `‹buyer›`-markeringar är dina; gallring tillämpas bara delvis |
| Brittiska NHS DTAC (ett nationellt system, ett arbetsexempel; andra system utgår från [checklistan för kvalitetssäkring](../../../../governance/assurance-checklist.md)) | [DTAC-checklista](../../../../governance/dtac-checklist.md) | Besvarad avsnitt för avsnitt; rader för köpare och leverantör är öppna |
| Tillgänglighet | [uttalande](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA automatiskt; inget manuellt test |
| Språkskyldigheter | [språkuttalande](../../../../governance/language-statement.md) | Översättningar är utkast |
| ISO/IEC 27001 | [efterlevnadsspecifikation](../../../../spec/12-compliance.md) | Kontroller på driftsättningssidan; inte certifierat |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [efterlevnadsspecifikation](../../../../spec/12-compliance.md) §12.4 | Inte tillämpligt (inga patientuppgifter); angivet, inte konstruerat |

## Underlag och kvalitetssäkring

[Checklistan för kvalitetssäkring](../../../../governance/assurance-checklist.md) listar 14 punkter som en bedömare kommer att fråga efter, med ägare och läge. De flesta är öppna och tillhör driftsättaren: penetrationstest, tillgänglighetsgranskning, katastrofåterställningsövning, säkring av driftmiljön, konfiguration av identitetsleverantören, omcertifiering av åtkomst, incidentprocess, arkivhantering. Granskningsloggen nekar ändring och radering i databasen, men en databasadministratör kan sätta den ur spel; det finns ingen hashkedja.

## Beslut som är dina

Rättslig grund och gallringsperioder; undantagsfönster; markeringsnivå för driftsättningen; vem som granskar åtkomst och hur ofta; om ett förfarande för rättsligt bevarande behövs (inget är byggt).

## Läs vidare

[Index över styrningspaketet](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Efterlevnadsspecifikation](../../../../spec/12-compliance.md)
