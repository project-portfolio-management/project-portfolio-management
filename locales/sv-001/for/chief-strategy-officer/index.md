# För strategichefen

*Strategichef (CSO): hur programvaran kopplar strategi till arbetet, vad den visar dig och vad den ännu inte kan.*

## Vad den gör för strategin

- **Ett enda register över allt som finansieras.** Portföljer, program, projekt och produkter är en rekursiv post, så att hela beståndet kan läsas som ett träd ([syfte och vision](../../../../spec/01-purpose-and-vision.md)).
- **Dubblettsökning.** Förklarbar matchning visar när två delar av organisationen har startat samma initiativ, innan båda finansieras.
- **Strategiska mål och portföljkartan.** Du markerar mål som strategiska mål och anger ett primärt mål för varje program. `GET /api/portfolio-map` och det månatliga direktionspaketet grupperar program efter primärt mål. Ett program förekommer en gång, med sina övriga mål namngivna; ett program utan primärt mål förs upp under "inget strategiskt mål", och summor dubbelräknas aldrig.
- **Leveranssäkerhet bredvid beräknad status.** Personer anger sin säkerhet om leveransen; programvaran visar den bredvid den beräknade röda/bärnstensfärgade/gröna statusen och flaggar ett glapp på två steg. En plan som ingen har bedömt visar "inte bedömd", aldrig grönt.
- **Ett fruset månatligt paket.** Det som direktionen fick veta bevaras exakt som det utfärdades, med trenden mätt mot det föregående utfärdade paketet.

## Vad den ännu inte gör

- Paketet innehåller ännu inte grindbeslut, finansiering per räkenskapsår, bufferttemperaturkurvan, intjänat värde, validerade nyttor eller kapacitetsvärmekartan (`DP-5` till `DP-10` i [`tasks.md`](../../../../tasks.md)). Paketet anger vilka avsnitt som inte är byggda.
- "Program" betyder en plan vars slag är `Program`, eftersom data inte har någon annan definition; mål på "högsta nivå" är de du markerar, eftersom mål saknar hierarki.
- Det finns ingen scenarioplanering, och ännu ingen skärm för kartan eller paketet: de finns som data.
- Det är inte certifierat, och det har en enda underhållare ([Vägen till produktion](../../path-to-production/index.md)).

## Beslut som är dina

Vilka mål som är strategiska mål; det primära målet för varje program; vem som anger och vem som oberoende bedömer leveranssäkerhet; hur många månaders trend direktionen ska se.

## Läs vidare

[Färdplan](../../../../spec/15-roadmap.md) · [Öppna frågor](../../../../spec/16-open-questions.md) · [Omfattning](../../../../spec/02-scope.md)
