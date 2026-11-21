# För operativa chefen

*Operativ chef (COO): vad du driver i det dagliga, vad programvaran gör för leveransen och vad det krävs för att driva den.*

## Vad den gör för verksamheten

- **Leveransplanering och styrning:** planer, uppgifter, beroenden, en beräkning av kritisk linje, milstolpar över planer, sprintar och faser, burndown och flödesmått.
- **Kapacitet:** tilldelningar, tillgänglighet, ledighet och icke arbetade perioder, resursroller, härledd arbetsbelastning och beläggning. (En kapacitetsvärmekarta per kompetenspool och månad för direktionspaketet är inte byggd: `DP-10`.)
- **Tid och kostnad:** tidrapporter, registrerad tid, kostnadssatser (begränsade till ekonomi), budgetar och härledd kostnad.
- **Styrningsflöde:** problemregister, ändringsbegäranden med funktionsseparation (den som begär kan inte godkänna), godkännanden, granskningar, ärenden för nyanställda, flyttade och avgående.
- **Undantag:** en leveranssäkerhet i rött eller bärnstensrött öppnar ett undantag som kräver en orsak, det beslut som behövs, en namngiven ägare och ett beslutsdatum inom 48 timmar (fönstret kan konfigureras, 1 till 720 timmar). Överträdelser syns i direktionspaketet och i `GET /api/exceptions`.

## Vad det krävs för att driva den

| Behov | Detaljer |
|---|---|
| Tjänst | En Rust-tjänst, PostgreSQL 18, valfri Redis (produktionscache, valfria delade sessioner), ett SvelteKit-gränssnitt |
| Driftshandbok | [driftpaketet](../../../../operations/README.md): driftsättning, driftshandbok, säkerhetskopiering och återställning, återställningsmål, uppgradering och migrering |
| Schemalagda jobb | Ingenting schemalägger sig självt. Gallringsrensning, meddelanden om överträdna undantag och liknande uppgifter måste läggas på en schemaläggare av dina operatörer |
| Support | En underhållare, inget servicenivåavtal; budgetera för din egen första linjens support |

## Känn till gränserna

- En plan med flera tusen uppgifter är långsam att schemalägga; det fullständiga svaret nekas över 5 000 uppgifter om det inte begärs (`?full=true` eller `?only=critical`). Siffror och förbehåll finns i [`BENCHMARKS.md`](../../../../BENCHMARKS.md); de är en storleksordning från en maskin, inte kapacitet.
- Det finns skärmar för stora delar av arbetsytan men **inte** ännu för ändringsbegäranden, problem, paket eller leveranssäkerhet; de är i dag endast tillgängliga via API.

## Beslut som är dina

Vem som äger varje undantag, vem som utfärdar det månatliga paketet (rytm och påminnelser är `DP-12`, inte byggda) och vilken granskningskadens undantagsregeln matar.

## Läs vidare

[Driftpaket](../../../../operations/README.md) · [Driftshandbok](../../../../operations/runbook.md) · [Återställningsmål](../../../../operations/recovery-objectives.md)
