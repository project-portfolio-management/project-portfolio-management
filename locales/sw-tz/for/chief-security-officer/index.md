# Kwa Afisa Mkuu wa Usalama

*Afisa Mkuu wa Usalama: programu inatekeleza nini, mradi umethibitisha nini, na unachopaswa kujaribu na kuendesha mwenyewe.*

## Programu inatekeleza nini

| Eneo | Tabia |
|---|---|
| Uthibitishaji | Kuingia kumewashwa kwa chaguo-msingi. Huduma haitaanza bila funguo za kuthibitisha tokeni, wala haitaanza katika uzalishaji ikiwa kuingia kumezimwa. Kuingia kupitia mtoa utambulisho kama Microsoft Entra ID; uthibitishaji wa hatua nyingi hubaki kwa mtoa huduma ([mwongozo wa kuingia](../../../../operations/sign-in-with-entra.md)) |
| Uidhinishaji | Sera ya msingi wa sifa kwenye sifa za tokeni. Mipango ya faragha huachwa nje ya kila jumla kwa kila mtu; alama za ushughulikiaji huficha mipango iliyo juu ya kiwango cha idhini cha mpigaji, wasimamizi wakiwemo |
| Ufikiaji wa dharura | Ufikiaji wa mipango ya faragha unahitaji ustahiki, sababu iliyotajwa, saa nne zisizozidi, na mtu wa pili kuarifiwa; kila ombi hukaguliwa |
| Vikao | Muda wa kutotumika (dakika 30 kwa chaguo-msingi) na kikomo kigumu cha kikao |
| Usafirishaji na kivinjari | Vichwa vya usalama, Sera kali ya Usalama wa Maudhui, orodha ya CORS inayoruhusiwa, kikomo cha kasi ya maombi kwa kila mpigaji |
| Kutuma nje | Maeneo yaliyotolewa na mtumiaji yamezuiwa kwa anwani za umma za `https` |
| Upakiaji | Maudhui lazima yalingane na aina iliyotangazwa; programu hukataliwa; uchunguzi wa hiari wa `clamd` hushindwa kwa kufunga |
| Ukaguzi | Njia ya ukaguzi inakataa mabadiliko na ufutaji katika hifadhidata |
| Rekodi zilizogandishwa | Pakiti za wakurugenzi zilizotolewa na tathmini za imani ya utekelezaji haziwezi kubadilishwa au huongezwa tu kwa kichocheo cha hifadhidata |
| Mnyororo wa ugavi | `cargo deny`, hati ya vipengele vya programu, uthibitisho wa asili ya ujenzi wakati wa toleo, [sera ya usalama](../../../../SECURITY.md) |

## Kisichofanyika

- Hakuna jaribio la kuvunja usalama, ukaguzi wa mtu wa tatu, mapitio huru ya msimbo wala uthibitisho.
- Hakuna kichunguzi cha programu hasidi kilichojumuishwa; kiunganishi cha `clamd` kilijaribiwa dhidi ya mbadala tu.
- Njia ya ukaguzi inaweza kuzuiwa na msimamizi wa hifadhidata; hakuna mnyororo wa hashi.
- Alama za ushughulikiaji huficha na kuweka lebo; hazisimbi, na alama haichapishwi ndani ya faili zinazozalishwa.
- Vikao hushikiliwa kwenye kumbukumbu isipokuwa Redis imesanidiwa.
- Zana ya mwenye data hupata marejeleo, si majina yaliyoandikwa kwenye maandishi huru, faili zilizopakiwa au nakala rudufu.

## Kuiendesha

Nakala rudufu zina urejeshaji uliojaribiwa, usimbaji wa hiari na malengo ya urejeshaji yaliyotajwa ([pakiti ya uendeshaji](../../../../operations/README.md)). [Mwongozo wa uendeshaji](../../../../operations/runbook.md) unatoa hatua za kiufundi za tukio; mchakato wa tukio na uvunjaji ni wako. Ripoti udhaifu kwa faragha kama [SECURITY.md](../../../../SECURITY.md) inavyoeleza; malengo ni kukiri ndani ya siku 5 za kazi, si dhamana.

## Maamuzi ambayo ni yako

Nani anafanya jaribio la kuvunja usalama na lini; ufikiaji wa masharti wa mtoa utambulisho na uchoraji wa majukumu; kiwango cha alama ambacho usambazaji huu umehakikiwa kwacho; kichunguzi nyuma ya kiunganishi cha viambatisho; ufikiaji huthibitishwa upya mara ngapi.

## Soma ifuatayo

[Orodha ya uhakikisho](../../../../governance/assurance-checklist.md) · [Njia ya kuelekea uzalishaji](../../path-to-production/index.md) · [Mtiririko wa data](../../../../governance/data-flow.md)
