# Vägen till produktion

*Vad "produktion" betyder för den här programvaran, och hur långt den är från det, punkt för punkt.*

"Produktion" betyder här att en organisation kör programvaran för verkligt arbete. Verkliga planer, verkliga personers namn och
verkliga beslut är beroende av den, ett avbrott eller ett läckage får konsekvenser, och någon är ansvarig för det. Vid en stor
hälsomyndighet eller ett statligt departement betyder produktion också att organisationens egen kvalitetssäkring har klarats: ett
säkerhetstest, en konsekvensbedömning avseende dataskydd, en tillgänglighetsgranskning och, inom brittiska National Health Service,
Digital Technology Assessment Criteria (DTAC).

## Läget just nu

- Inga certifieringar, godkännanden eller granskningar görs gällande. Den här sidan är projektets egen uppskattning.
- "Medicinsk" betyder ett **icke-kliniskt** verktyg för portfölj och projekt: inga patientjournaler och inget kliniskt beslutsstöd.
- Projektet föreslår att man börjar med en pilot på icke-kliniska, icke-hemliga data, som drivs av en organisation som genomför
  sin egen kvalitetssäkring.

## Beredskap, punkt för punkt

Varje siffra är projektets uppskattning den 13 november 2026, inte en mätning. Varje reglage börjar vid den uppskattningen.
När du flyttar ett reglage noteras din egen bedömning medan du läser; inget sparas eller skickas.

- **Oberoende säkerhetsgranskning** — 0%. Inget penetrationstest, ingen tredjepartsgranskning och ingen certifiering. En köpare eller en leverantör måste beställa dem (RG-13).
- **Tillgänglighet** — 40%. En automatisk WCAG 2.2 AA-granskning körs på varje rutt vid varje ändring. Inget manuellt test, och inget test med hjälpmedel eller med personer med funktionsnedsättning, har gjorts (RG-13).
- **Begäranden från registrerade** — 50%. Administratörer kan exportera och pseudonymisera en person i alla referenser i schemat, med förhandsvisning först (RG-6). Namn som skrivits in i titlar, kommentarer eller dokument, uppladdade filer, granskningsögonblicksbilder och säkerhetskopior hittas inte automatiskt.
- **Gallring** — 35%. En rensning, som som standard är en provkörning, omfattar notiser, leveransloggar, idempotensnycklar samt trend- och användningsögonblicksbilder, med perioder som ställs in per driftsättning (RG-4). Planer, uppgifter, tidsregistreringar, dokument och granskningsloggen sparas tills en operatör raderar dem.
- **Hanteringsmarkeringar** — 60%. En konfigurerbar klassificering döljer planer över en anropares behörighetsnivå, administratörer inräknade, och märker varje svar (RG-7). Markeringen stämplas inte in i genererade filer, och behörighetsnivån är bara så pålitlig som identitetsleverantörens attribut.
- **Bilagors säkerhet** — 50%. Innehållet måste stämma med den angivna typen, program nekas, och en valfri `clamd`-skanning stänger vid fel (RG-8). Ingen skanner medföljer, och kopplingen har bara testats mot en ersättare.
- **Behörighetsgranskning och nödåtkomst** — 45%. En rapport över stående behörigheter stöder omcertifiering (RG-9), och nödåtkomst är tidsbegränsad och kräver en angiven orsak (RG-10). Rapporten visar vad som registrerats, inte vad identitetsleverantören tillämpar; ingen rutt registrerar en granskares beslut; och ett nödåtkomstmeddelande kan namnge någon som inte får något medan behörigheten ändå gäller.
- **Prestanda och kapacitet** — 40%. Ett belastnings- och uthållighetstest på en maskin finns i [`BENCHMARKS.md`](../../../BENCHMARKS.md): vanliga anrop besvaras på några tiotals millisekunder, en plan med omkring 12 700 uppgifter är långsam att schemalägga, och processen växer till omkring 1 GB. Det finns ingen kapacitetsplan, inget test med många användare och ingen körning över flera maskiner.
- **Sessioner över flera frontservrar** — 70%. Sessioner hålls i minnet om inte Redis har konfigurerats (RG-12); med Redis är inloggningen beroende av det.
- **Granskningsloggens integritet** — 50%. Skrivningar loggas, och en databastrigger nekar ändringar och radering, med angivna undantag (RG-3). En databasadministratör kan ta bort triggern, och det finns ingen hashkedja.
- **Säkerhetskopiering och återställning** — 80%. En återställning testas i kontinuerlig integrering, både okrypterad och krypterad, mot angivna återställningsmål (RG-5). Kryptering är valfri, och övningen körs på projektets egen CI, inte på driftsättarens värdmiljö.
- **Support och kontinuitet** — 10%. En underhållare och inget servicenivåavtal ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Översättningar** — 20%. Text på andra språk än engelska är maskinskriven tills en person som talar språket har granskat den, och lagstadgade språkskyldigheter kräver granskad text ([språkuttalande](../../../governance/language-statement.md)).
- **Klinisk användning** — 0%. Inget kliniskt säkerhetsunderlag har skrivits, och klinisk användning ligger utanför omfattningen.

## Vad en första driftsättning bör göra

1. Slutför [kvalitetssäkringschecklistan](../../../governance/assurance-checklist.md), med början i ett penetrationstest och en tillgänglighetsgranskning.
2. Fyll i varje rad "Buyer to complete" i [DTAC-checklistan](../../../governance/dtac-checklist.md) och markeringarna i [konsekvensbedömningen avseende dataskydd (DPIA)](../../../governance/dpia.md).
3. Kom överens med arkivansvarig om gallringsperioder och schemalägg rensningen.
4. Slå på krypterade säkerhetskopior och kör återställningsövningen på driftsättningens egen värdmiljö.
5. Håll data icke-kliniska och under den markeringsnivå som driftsättningen är kvalitetssäkrad för.
6. Planera för bristerna ovan, eller finansiera att de åtgärdas.

Underlaget för varje punkt finns i [bedömningen](../../../governance/production-readiness.md).
