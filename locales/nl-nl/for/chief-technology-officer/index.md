# Voor de technologiedirecteur

*Technologiedirecteur (CTO): architectuur, kwaliteit, hoe het uit te breiden en waar de techniek dun is.*

## Architectuur

Drie deelprojecten, één entiteit ([architectuur](../../../../spec/08-architecture.md)):

| Onderdeel | Technologie |
|---|---|
| Matchbibliotheek | Rust; deterministische en probabilistische, verklaarbare matching per paar; ingebed in de dienst |
| Dienst | Rust, axa op axum, sqlx, PostgreSQL 18; REST onder `/api`, OpenAPI afgeleid van de routes |
| Interface | SvelteKit 3, Svelte 5 SPA met een backend-for-frontend; i18n-catalogi |

De toolchain ligt vast in [`rust-toolchain.toml`](../../../../rust-toolchain.toml). Persistentie staat in [spec 10](../../../../spec/10-persistence.md); de API in [spec 09](../../../../spec/09-api-surface.md).

## Technische sterktes die het weten waard zijn

- **Invarianten leven in de database waar het ertoe doet:** het auditlog weigert wijziging en verwijdering; uitgegeven bestuurspakketten en beoordelingen van leveringszekerheid zijn via triggers onveranderlijk of alleen toe te voegen; CHECK-constraints ondersteunen de regels van de dienst.
- **Bewakingstests:** telltests (census tests) laten de build mislukken wanneer een nieuwe aggregatieroute het filter voor privéplannen overslaat, een lijstroute geen paginering heeft, een kolom met persoonsgegevens in de inventaris ontbreekt of een route in OpenAPI ontbreekt. Ze worden opgelost door het nieuwe te classificeren, niet door de test te verzwakken.
- **Tests met echte tokens:** autorisatie wordt van begin tot eind getest met ondertekende PASETO-tokens (elke test in een eigen proces, omdat de authenticatie-instellingen één keer per proces worden gelezen).
- **Toeleveringsketen:** `cargo deny`, een software bill of materials, bouwherkomst bij de release, een beveiligingsbeleid.
- **CI** bouwt en test alles bij elke push; zie `.github/workflows/`.

## Waar het dun is

- Eén beheerder, dus de busfactor is één ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- De prestaties zijn slechts ruwweg bekend; de antwoordcache omvat alleen de route voor het kritieke pad; zeer grote plannen zijn traag.
- Sessies staan in het geheugen tenzij Redis is geconfigureerd.
- Meerdere onderdelen van het bestuurspakket en alle pakketschermen zijn niet gebouwd ([`tasks.md`](../../../../tasks.md)).
- AI-hulpmiddelen worden gebruikt om het te bouwen; het product bevat geen model. Zie de [AI-verklaring](../../../../AI_STATEMENT.md).

## Besluiten die bij u liggen

Hosting en Redis-topologie, of u een fork maakt of de upstream volgt, wie de code onafhankelijk beoordeelt, en hoe de dienst past bij uw identiteitsprovider (PASETO-sleutelset, of OIDC via Keycloak of Entra).

## Lees verder

[Architectuur](../../../../spec/08-architecture.md) · [Teststrategie](../../../../spec/11-testing-strategy.md) · [Implementatiestatus](../../../../spec/14-implementation-status.md) · [Benchmarks](../../../../BENCHMARKS.md)
