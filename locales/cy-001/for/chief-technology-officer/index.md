# I'r Prif Swyddog Technoleg

*Y Prif Swyddog Technoleg: pensaernïaeth, ansawdd, sut i'w ymestyn, a lle mae'r peirianneg yn denau.*

## Pensaernïaeth

Tri is-brosiect, un endid ([pensaernïaeth](../../../../spec/08-architecture.md)):

| Rhan | Technoleg |
|---|---|
| Llyfrgell paru | Rust; paru pâr penderfynol a thebygolrwydd y gellir ei egluro; wedi'i fewnosod gan y gwasanaeth |
| Gwasanaeth | Rust, Loco.rs ar axum, SeaORM, PostgreSQL 18; REST o dan `/api`, OpenAPI wedi'i ddeillio o'r llwybrau |
| Rhyngwyneb | SvelteKit 2, SPA Svelte 5 gyda ôl-wyneb pwrpasol i'r blaen; catalogau i18n |

Mae'r gadwyn offer wedi'i phinio yn [`rust-toolchain.toml`](../../../../rust-toolchain.toml). Mae parhad yn [manyleb 10](../../../../spec/10-persistence.md); yr API yn [manyleb 09](../../../../spec/09-api-surface.md).

## Cryfderau peirianneg sy'n werth eu gwybod

- **Mae'r anghyfnewidiadau lle mae ots yn y gronfa ddata:** mae'r log archwilio yn gwrthod newid a dileu; mae pecynnau cyfarwyddwyr a gyhoeddwyd ac asesiadau hyder cyflawni yn anghyfnewidiol neu'n ychwanegu-yn-unig drwy sbardun; mae cyfyngiadau CHECK yn cynnal rheolau'r gwasanaeth.
- **Profion gwarchod:** mae profion cyfrifiad yn methu'r adeilad pan fo llwybr crynhoi newydd yn hepgor yr hidlydd cynlluniau preifat, llwybr rhestr heb ei dudalennu, colofn data personol ar goll o'r rhestr eiddo, neu lwybr ar goll o OpenAPI. Eu trwsio drwy ddosbarthu'r peth newydd, nid drwy wanhau'r prawf.
- **Profion tocyn go iawn:** profir awdurdodi o'r dechrau i'r diwedd gyda thocynnau PASETO wedi'u llofnodi (un deuaidd i bob prawf, gan fod y dilyswr yn gyfun i'r broses).
- **Cadwyn gyflenwi:** `cargo deny`, bil deunyddiau meddalwedd, tarddiad adeiladu wrth ryddhau, polisi diogelwch.
- Mae **CI** yn adeiladu ac yn profi popeth ar bob gwthiad; gweler `.github/workflows/`.

## Lle mae'n denau

- Un cynhaliwr, felly un yw'r ffactor bws ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- Dim ond yn fras y gwyddys perfformiad; dim ond y llwybr critigol y mae'r storfa ymatebion yn ei gynnwys; mae cynlluniau mawr iawn yn araf.
- Cedwir sesiynau yn y cof oni ffurfweddir Redis.
- Nid yw sawl adran o'r pecyn cyfarwyddwyr na holl sgriniau'r pecyn wedi'u hadeiladu ([`tasks.md`](../../../../tasks.md)).
- Defnyddir offer AI i'w adeiladu; nid oes model yn y cynnyrch. Gweler y [datganiad AI](../../../../AI_STATEMENT.md).

## Penderfyniadau sy'n eiddo i chi

Gwesteio a thopoleg Redis, pa un ai i fforcio neu fabwysiadu'r cynsail, pwy sy'n gwneud adolygiad cod annibynnol, a sut mae'r gwasanaeth yn ffitio â'ch darparwr hunaniaeth (set allweddi PASETO, neu OIDC drwy Keycloak neu Entra).

## Darllenwch nesaf

[Pensaernïaeth](../../../../spec/08-architecture.md) · [Strategaeth brofi](../../../../spec/11-testing-strategy.md) · [Statws gweithredu](../../../../spec/14-implementation-status.md) · [Meincnodau](../../../../BENCHMARKS.md)
