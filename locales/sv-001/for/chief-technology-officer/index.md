# För teknikchefen

*Teknikchef (CTO): arkitektur, kvalitet, hur den byggs ut och var tekniken är tunn.*

## Arkitektur

Tre delprojekt, en entitet ([arkitektur](../../../../spec/08-architecture.md)):

| Del | Teknik |
|---|---|
| Matchningsbibliotek | Rust; deterministisk och probabilistisk, förklarbar parvis matchning; inbyggt i tjänsten |
| Tjänst | Rust, axa ovanpå axum, sqlx, PostgreSQL 18; REST under `/api`, OpenAPI härlett från rutterna |
| Gränssnitt | SvelteKit 3, Svelte 5 SPA med en backend-for-frontend; i18n-kataloger |

Verktygskedjan är fastlåst i [`rust-toolchain.toml`](../../../../rust-toolchain.toml). Persistens beskrivs i [spec 10](../../../../spec/10-persistence.md); API:et i [spec 09](../../../../spec/09-api-surface.md).

## Tekniska styrkor värda att känna till

- **Invarianter finns i databasen där det spelar roll:** granskningsloggen nekar ändring och radering; utfärdade direktionspaket och bedömningar av leveranssäkerhet är oföränderliga eller bara tillägg via trigger; CHECK-villkor stöder tjänstens regler.
- **Vakttester:** folkräkningstester (census tests) får bygget att misslyckas när en ny aggregatrutt hoppar över filtret för privata planer, en listrutt saknar sidindelning, en kolumn med personuppgifter saknas i förteckningen, eller en rutt saknas i OpenAPI. De åtgärdas genom att klassificera det nya, inte genom att försvaga testet.
- **Tester med riktiga tokens:** auktorisering testas från början till slut med signerade PASETO-tokens (varje test i en egen process, eftersom autentiseringsinställningarna läses en gång per process).
- **Leveranskedja:** `cargo deny`, en programvaruförteckning (SBOM), byggproveniens vid utgivning, en säkerhetspolicy.
- **CI** bygger och testar allt vid varje push; se `.github/workflows/`.

## Var den är tunn

- En enda underhållare, så bussfaktorn är ett ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- Prestandan är känd i grova drag; svarscachen omfattar bara rutten för kritisk linje; mycket stora planer är långsamma.
- Sessioner finns i minnet om inte Redis har konfigurerats.
- Flera avsnitt av direktionspaketet och alla paketskärmar är inte byggda ([`tasks.md`](../../../../tasks.md)).
- AI-verktyg används för att bygga den; produkten innehåller ingen modell. Se [AI-uttalandet](../../../../AI_STATEMENT.md).

## Beslut som är dina

Drift och Redis-topologi, om du ska göra en avknoppning eller följa uppströmsprojektet, vem som oberoende granskar koden och hur tjänsten passar din identitetsleverantör (PASETO-nyckeluppsättning, eller OIDC via Keycloak eller Entra).

## Läs vidare

[Arkitektur](../../../../spec/08-architecture.md) · [Teststrategi](../../../../spec/11-testing-strategy.md) · [Genomförandestatus](../../../../spec/14-implementation-status.md) · [Prestandamätningar](../../../../BENCHMARKS.md)
