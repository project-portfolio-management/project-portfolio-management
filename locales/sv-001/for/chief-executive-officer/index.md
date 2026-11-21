# För verkställande direktören

*Verkställande direktör: vad programvaran är, vad den ger dig, vad den inte ger, och vilka beslut som är dina.*

## Vad det är, i ett stycke

Ett register över varje **plan** som organisationen finansierar (portföljer, program, projekt, produkter och så vidare, en rekursiv post), med en arbetsyta för projektledning under (uppgifter, schema, kostnad, resurser, risker, problem, ändringsbegäranden, styrning). Den hittar dubblerade initiativ genom förklarbar matchning, så att två avdelningar som startar samma arbete syns innan båda finansieras. Se [syfte och vision](../../../../spec/01-purpose-and-vision.md).

## Vad du kan lita på

- **Ett enda ställe att fråga "vad håller vi på med?"** Planer kan nästlas på valfritt djup; privata planer räknas inte med i några summor eller antal, för någon, enligt en regel som tester kontrollerar.
- **Ett månatligt direktionspaket** som skrivs en gång, utfärdas av en namngiven person och sedan **fryses av databasen**: ett utfärdat paket kan inte redigeras; en rättelse är en ny, återutfärdad version med en anmärkning. I dag innehåller det portföljkartan efter strategiskt mål, leveranssäkerhet, undantag, problem och ändringsbegäranden.
- **Leveranssäkerhet som personer anger, visad bredvid den beräknade statusen och aldrig blandad med den.** En plan som ingen har bedömt visar "inte bedömd", aldrig grönt. Ett omdöme i bärnstensrött eller rött öppnar ett undantag med en klocka på 48 timmar (kan konfigureras); en klocka som löpt ut visas i paketet.
- **Beslut lämnar spår.** Varje ändring loggas, granskningsloggen går inte att ändra via tjänsten, och en ändringsbegäran kan inte godkännas av den som väckte den.

## Vad det inte ger dig ännu

- Paketet innehåller **ännu inte** grindbeslut, finansiering och pengar per räkenskapsår, bufferttemperaturkurvan, intjänat värde, validerade nyttor eller kapacitetsvärmekartan. De förs upp som `DP-5` till `DP-10` i [`tasks.md`](../../../../tasks.md), och paketet anger vilka avsnitt som ännu inte byggts i stället för att lämna en lucka.
- Ingen PDF eller skärm för paketet: det levereras som data (`DP-11`).
- Det är **inte certifierat eller granskat** av någon. Vad produktionsanvändning skulle kräva, punkt för punkt med en uppskattning för var och en, finns på sidan [Vägen till produktion](../../path-to-production/index.md).
- En underhållare, inget servicenivåavtal ([styrning](../../../../GOVERNANCE.md)).

## Beslut som är dina

1. Om en pilot ska genomföras, och på vilken del av portföljen.
2. Vem som är **sponsor** för undantagsregeln på 48 timmar och vem som får utfärda det månatliga paketet.
3. Om du ska finansiera bristerna ovan (bedömning, penetrationstest, supportavtal) eller acceptera dem.

## Läs vidare

[Vägen till produktion](../../path-to-production/index.md) · [Färdplan](../../../../spec/15-roadmap.md) · [Öppna frågor](../../../../spec/16-open-questions.md)
