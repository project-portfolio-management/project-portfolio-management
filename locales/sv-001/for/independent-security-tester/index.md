# Oberoende säkerhetstestning

*För en köpare, en bedömare eller en testare: vad oberoende säkerhetstestning av den här programvaran bör omfatta, vad projektet tillhandahåller som hjälp, vad som har gjorts och inte gjorts, och vad som lämnas åt dig. Detta är vägledning, inte en testrapport och inte juridisk rådgivning.*

## Läget just nu

**Inget oberoende säkerhetstest har gjorts.** Inget penetrationstest, ingen tredjepartsgranskning, ingen oberoende kodgranskning eller certifiering finns för den här programvaran, och projektet gör inte gällande någon ([Vägen till produktion](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). En passiv skanning körs i projektets kontinuerliga integrering (regler i `.zap/rules.tsv`), och projektets egna tester kontrollerar de kontroller som listas nedan; ingetdera ersätter en testare som inte har skrivit programvaran.

Detta är punkt 1 och punkt 9 i [checklistan för kvalitetssäkring](../../../../governance/assurance-checklist.md). Det tillhör driftsättaren, med en certifierad testare, mot **din** driftsättning: projektet kan inte testa din driftmiljö, din identitetsleverantör, ditt nätverk eller dina nycklar.

## Vad som ska testas

Testa driftsättningen så som den kommer att köras, med inloggning tillämpad och din identitetsleverantör framför. Föreslagen omfattning, utifrån de kontroller som programvaran gör gällande:

| Område | Vad programvaran gör gällande | Vad en testare bör försöka |
|---|---|---|
| Autentisering | Inloggning är på som standard; tjänsten startar inte utan verifieringsnycklar eller, i produktion, med inloggning avstängd; tokens verifieras offline mot publicerade nycklar | Förfalskade, utgångna, återanvända och felaktigt signerade tokens; fel utfärdare eller målgrupp; nyckelrotation; en saknad nyckeluppsättning |
| Auktorisering | Attributbaserad policy; regler på postnivå | Horisontell och vertikal behörighetseskalering på varje rutt; ändrade identifierare i sökvägar och kroppar; attribut som en anropare inte ska kunna lägga till |
| Privata planer och hanteringsmarkeringar | En privat plan, och en plan över en anropares behörighetsnivå, finns i ingen lista, inget antal, ingen summa, inget sökresultat, inget flöde och ingen export för den anroparen, administratörer inräknade | Varje rutt som läcker förekomsten, ett namn, ett antal eller en summa för en plan som anroparen inte kan läsa; felmeddelanden och svarstider som skiljer sig mellan "dold" och "saknas" |
| Nödåtkomst | Endast behöriga personer, en angiven orsak, högst fyra timmar, en andra person underrättas | Användning utan behörighet; förlängning av en behörighet; handlande utan att den andra personen underrättas |
| Frysta poster och poster där man bara lägger till | Granskningsloggen, utfärdade direktionspaket, historiken över leveranssäkerhet, programstatushistoriken och projektbuffertar nekar ändring och radering i databasen | Uppdatering eller radering via API:et, via massimport och via varje rutt som rör samma rader |
| Funktionsseparation | En som begär kan inte godkänna sin egen ändringsbegäran | Godkännande som den som begär, som administratör och genom att byta godkännare efter inlämning |
| Uppladdningar och bilagor | Innehållet måste stämma med den angivna typen; program nekas; en valfri virusskanningskoppling stänger vid fel | Polyglotfiler, omdöpta program, för stora och felformade filer, sökvägstrick i namn, och om din skanner verkligen anropas |
| Utgående anrop | Destinationer som användaren anger är begränsade till offentliga `https`-adresser | Förfalskade serversidiga anrop (SSRF) till privata adresser, länklokala adresser och metadataadresser, omdirigeringar, DNS-rebinding |
| Sessioner och webbläsaren | Tidsgräns för overksamhet, en hård gräns, en `httpOnly`-kaka, en strikt Content Security Policy, en CORS-tillåtelselista | Sessionsfixering och återanvändning, förfalskade förfrågningar mellan webbplatser (CSRF), injektion i gränssnittet, clickjacking |
| Hastighetsbegränsning | Gränser per anropare | Förbigående genom att byta identitet, rubrik eller sökväg; om gränserna består genom din proxy |
| Verktyg för registrerades rättigheter | Export och pseudonymisering, endast administratörer, förhandsvisning först | Användning av en icke-administratör; fullständighet mot [förteckningen över personuppgifter](../../../../governance/personal-data-inventory.csv) |
| Säkerhetskopior | Valfri kryptering, en testad återställning | Åtkomst till säkerhetskopiefiler och nycklar; återställning till en ren miljö |

## Vad projektet tillhandahåller

- Källkoden, specifikationen och testerna, så att en kodgranskning kan utgå från påståendena.
- En maskinläsbar beskrivning av API:et (`/api-docs/openapi.json`), med schemata för begäran och svar för arbetet med direktionspaketet; listan över rutter kontrolleras mot koden av tester.
- En engångskonfiguration med inloggning avstängd (`dev-insecure`, se [README](../../../../README.md)) **endast för testarens egen maskin**, och ett skript som genererar en testnyckeluppsättning (`scripts/smoke-keys.sh`) så att tokens med valda attribut kan skapas mot en testdriftsättning.
- [Dataflödesdiagrammet](../../../../governance/data-flow.md), [driftshandboken](../../../../operations/runbook.md) och [återställningsmålen](../../../../operations/recovery-objectives.md).
- Ett privat sätt att rapportera det som hittas: [SECURITY.md](../../../../SECURITY.md).

## Regler för uppdraget

Kom överens om dessa skriftligt före varje test; projektet kan inte fastställa dem för din miljö.

- **Miljö.** Testa en kopia med realistiska men inte verkliga data. Testa inte produktion om du inte skriftligen har beslutat det.
- **Tillstånd.** Testaren har ditt skriftliga tillstånd, en namngiven kontaktperson och ett angivet tidsfönster.
- **Utanför gränserna.** Överbelastningsattacker med volym, social manipulation av personal och tredjepartstjänster (din identitetsleverantör, din värd) ligger utanför omfattningen om inte ägaren till var och en går med på det. Projektets egen omfattning anges i [SECURITY](../../../../SECURITY.md).
- **Data.** Verkliga personuppgifter används inte, kopieras inte ut och förs inte in i rapporten.
- **Nycklar.** Testnycklar är skilda från produktionsnycklar och förstörs efteråt.

## Rapportering och omtest

- Fynd om den här programvaran går privat till underhållaren, så som [SECURITY](../../../../SECURITY.md) anger, med hur man återskapar felet, versionen eller incheckningen och konsekvensen. Projektets mål är bekräftelse inom 5 arbetsdagar och en plan inom 15; de är mål, inte garantier. Det finns en underhållare och inget servicenivåavtal.
- Fynd om din driftsättning går till din egen ägare.
- Be testaren betygsätta varje fynd, ange vad som **inte** testades och testa om åtgärderna. En rapport som bara listar det som hittades talar inte om vad som missades.
- Behåll rapporten, omfattningen, datumen och omtestet bland dina egna handlingar. Projektet kan inte förvara dem åt dig.

## Det här dokumentets gränser

Det listar områden att täcka, inte en fullständig testplan, och påståendena i tabellen är projektets egna. Det certifierar ingenting, och ett rent test är belägg om en driftsättning på ett datum.

## Beslut som är dina

Vem som testar, när, mot vad, med vilka data och under vilka regler; hur fynd betygsätts och när de måste vara åtgärdade; och om resultatet delas med dina bedömare.

## Läs vidare

[Checklista för kvalitetssäkring](../../../../governance/assurance-checklist.md) · [Vägen till produktion](../../path-to-production/index.md) · [Säkerhetschef](../chief-security-officer/index.md) · [Styrning, risk och efterlevnad](../governance-risk-compliance/index.md)
