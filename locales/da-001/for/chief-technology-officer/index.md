# Til teknologidirektør

*Teknologidirektør: arkitektur, kvalitet, hvordan den udvides, og hvor ingeniørarbejdet er tyndt.*

## Arkitektur

Tre delprojekter, én enhed ([arkitektur](../../../../spec/08-architecture.md)):

| Del | Teknologi |
|---|---|
| Matcher-bibliotek | Rust; deterministisk og probabilistisk, forklarlig parvis matchning; indlejret i tjenesten |
| Tjeneste | Rust, axa på axum, sqlx, PostgreSQL 18; REST under `/api`, OpenAPI afledt af ruterne |
| Brugerflade | SvelteKit 3, Svelte 5 SPA med en backend-for-frontend; i18n-kataloger |

Værktøjskæden er fastlagt i [`rust-toolchain.toml`](../../../../rust-toolchain.toml). Persistens er beskrevet i [specifikation 10](../../../../spec/10-persistence.md); API'et i [specifikation 09](../../../../spec/09-api-surface.md).

## Tekniske styrker, det er værd at kende

- **Invarianter bor i databasen, hvor de betyder noget:** revisionsloggen afviser ændring og sletning; udstedte direktionspakker og vurderinger af leveringssikkerhed er uforanderlige eller kan kun tilføjes til via trigger; CHECK-begrænsninger understøtter tjenestens regler.
- **Vagttests:** optællingstests (census) lader buildet fejle, når en ny aggregeret rute springer filteret for private planer over, en listerute ikke er pagineret, en kolonne med personoplysninger mangler i fortegnelsen, eller en rute mangler i OpenAPI. De rettes ved at klassificere det nye, ikke ved at svække testen.
- **Tests med rigtige tokens:** autorisation testes fra ende til anden med signerede PASETO-tokens (hver test i sin egen proces, fordi godkendelsesindstillingerne læses én gang pr. proces).
- **Forsyningskæde:** `cargo deny`, en softwarestykliste, byggeoprindelse ved udgivelse, en sikkerhedspolitik.
- **CI** bygger og tester alt ved hvert push; se `.github/workflows/`.

## Hvor den er tynd

- Én vedligeholder, så busfaktoren er én ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- Ydeevnen kendes kun nogenlunde; svarcachen dækker kun ruten for den kritiske vej; meget store planer er langsomme.
- Sessioner ligger i hukommelsen, medmindre Redis er konfigureret.
- Flere afsnit i direktionspakken og alle skærme til pakken er ikke bygget ([`tasks.md`](../../../../tasks.md)).
- AI-værktøjer bruges til at bygge den; produktet indeholder ingen model. Se [AI-erklæringen](../../../../AI_STATEMENT.md).

## Beslutninger, der er dine

Drift og Redis-topologi, om der skal forkes eller følges med upstream, hvem der gennemgår koden uafhængigt, og hvordan tjenesten passer ind i din identitetsudbyder (PASETO-nøglesæt eller OIDC gennem Keycloak eller Entra).

## Læs videre

[Arkitektur](../../../../spec/08-architecture.md) · [Teststrategi](../../../../spec/11-testing-strategy.md) · [Status for implementeringen](../../../../spec/14-implementation-status.md) · [Benchmarks](../../../../BENCHMARKS.md)
