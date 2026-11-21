# Kwa Afisa Mkuu wa Teknolojia

*Afisa Mkuu wa Teknolojia: usanifu, ubora, jinsi ya kuipanua, na pale uhandisi ulipo dhaifu.*

## Usanifu

Miradi midogo mitatu, kiumbe kimoja ([usanifu](../../../../spec/08-architecture.md)):

| Sehemu | Teknolojia |
|---|---|
| Maktaba ya kilinganishi | Rust; ulinganishaji wa jozi unaoelezeka, wa kubainisha na wa uwezekano; hujengwa ndani ya huduma |
| Huduma | Rust, axa juu ya axum, sqlx, PostgreSQL 18; REST chini ya `/api`, OpenAPI inayotokana na njia |
| Kiolesura | SvelteKit 3, SPA ya Svelte 5 yenye backend-for-frontend; katalogi za i18n |

Zana za ujenzi zimefungwa katika [`rust-toolchain.toml`](../../../../rust-toolchain.toml). Uhifadhi uko katika [maelezo 10](../../../../spec/10-persistence.md); API katika [maelezo 09](../../../../spec/09-api-surface.md).

## Nguvu za uhandisi zinazostahili kujulikana

- **Vigezo visivyobadilika viko kwenye hifadhidata pale vinapohusika:** kumbukumbu ya ukaguzi inakataa mabadiliko na ufutaji; pakiti za wakurugenzi zilizotolewa na tathmini za imani ya utekelezaji haziwezi kubadilishwa au huongezwa tu kwa kichocheo; vikwazo vya CHECK vinaunga mkono kanuni za huduma.
- **Majaribio ya ulinzi:** majaribio ya sensa hushindwa kujenga pale njia mpya ya jumla inaporuka kichujio cha mipango ya faragha, njia ya orodha haina kurasa, safu ya data binafsi inakosekana kwenye orodha, au njia inakosekana kwenye OpenAPI. Hurekebishwa kwa kuainisha kitu kipya, si kwa kudhoofisha jaribio.
- **Majaribio ya tokeni halisi:** uidhinishaji hujaribiwa mwanzo hadi mwisho kwa tokeni za PASETO zilizosainiwa (kila jaribio katika mchakato wake, kwa sababu mipangilio ya uthibitishaji husomwa mara moja kwa kila mchakato).
- **Mnyororo wa ugavi:** `cargo deny`, hati ya vipengele vya programu, uthibitisho wa asili ya ujenzi wakati wa toleo, sera ya usalama.
- **CI** hujenga na kujaribu kila kitu kwa kila push; tazama `.github/workflows/`.

## Pale ilipo dhaifu

- Mtunzaji mmoja, hivyo kigezo cha basi ni kimoja ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- Utendaji unajulikana kwa takriban; akiba ya majibu inahusu njia muhimu pekee; mipango mikubwa sana ni polepole.
- Vikao viko kwenye kumbukumbu isipokuwa Redis imesanidiwa.
- Sehemu kadhaa za pakiti ya wakurugenzi na skrini zote za pakiti hazijajengwa ([`tasks.md`](../../../../tasks.md)).
- Zana za AI hutumika kuijenga; bidhaa haina modeli yoyote. Tazama [taarifa ya AI](../../../../AI_STATEMENT.md).

## Maamuzi ambayo ni yako

Upangishaji na mpangilio wa Redis, kama utafanya nakala ya mradi au kufuata chanzo cha awali, nani anapitia msimbo kwa kujitegemea, na jinsi huduma inavyolingana na mtoa utambulisho wako (seti ya funguo za PASETO, au OIDC kupitia Keycloak au Entra).

## Soma ifuatayo

[Usanifu](../../../../spec/08-architecture.md) · [Mkakati wa majaribio](../../../../spec/11-testing-strategy.md) · [Hali ya utekelezaji](../../../../spec/14-implementation-status.md) · [Vipimo vya utendaji](../../../../BENCHMARKS.md)
