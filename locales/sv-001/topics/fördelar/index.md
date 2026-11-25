# Fördelar

*De viktigaste fördelarna för en organisation, uppdelade efter område.*

## Överblick över portföljen

Portföljinformation finns ofta på flera ställen. I den här programvaran är varje portfölj, varje program, projekt och produkt samma posttyp, kallad en **plan**. Varje plan kan innehålla vilken annan plan som helst, på valfritt djup, så att strukturen kan följa organisationen. En etikett som "Projekt" är beskrivande och innebär ingen begränsning av planen.

## Färre dubbletter

Två avdelningar kan starta samma initiativ under olika namn. Matcharen jämför de attribut som identifierar en plan: namn, mål, ägare, sponsor, överordnad plan, tidsram, nyckelord och identifierare. Den fungerar för alla plantyper och visar underlaget för varje träff, och det är en person som fattar beslutet. Två poster för samma plan kan slås samman.

## Rapporteringens kvalitet

- Ett saknat värde visas med en orsak, till exempel "ingen prognos, eftersom …".
- En plan utan bedömning visas som "inte bedömd".
- Leveranssäkerhet som personer har angett visas bredvid den beräknade statusen, och de två hålls isär.
- Privata planer räknas inte med i några summor eller antal för någon användare. Tester kontrollerar den regeln.
- Varje varaktighet anger om den är i kalenderdagar eller arbetsdagar.

## Beslutsunderlag

Varje ändring loggas, och granskningsloggen går inte att ändra via tjänsten. Den som väcker en ändringsbegäran kan inte godkänna den. Ett månatligt direktionspaket skrivs en enda gång, utfärdas av en namngiven person och fryses sedan av databasen. En rättelse utfärdas som en ny version med en anmärkning.

## Säkerhet

Inloggning är på som standard, och tjänsten startar inte i produktion med inloggningen avstängd. Säkerhetsrubriker, hastighetsbegränsning, en återställningsplan med angivna mål och ett paket för informationsstyrning ingår i förvaret.

## Kontroll och portabilitet

- **Licensiering.** Programvaran har öppen källkod, med val mellan MIT, Apache-2.0, BSD-3-Clause, GPL-2.0, GPL-3.0, OGL-UK-3.0 eller CC-BY-NC-SA-4.0.
- **Drift.** Programvaran driftas av dig själv: en Rust-tjänst ovanpå PostgreSQL, med ett SvelteKit-gränssnitt.
- **Dataimport.** CSV-filer från Jira och Asana kan importeras, med förhandsvisning och en angiven orsak till allt som utelämnas. Microsoft Project XML kan importeras och exporteras.
- **Språk.** Gränssnittet finns på tio språkversioner, däribland arabiska som skrivs från höger till vänster. Översättningarna är utkast som väntar på granskning av personer som talar språket.

## Relaterade sidor

Se även [funktioner](../funktioner/index.md), [jämförelser](../jämförelser/index.md) och [handledningen](../handledning/index.md).
