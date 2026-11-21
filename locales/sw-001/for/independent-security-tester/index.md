# Majaribio huru ya usalama

*Kwa mnunuzi, mkaguzi au mjaribu: majaribio huru ya usalama ya programu hii yanapaswa kuhusu nini, mradi unatoa nini kusaidia, nini kimefanyika na nini hakijafanyika, na nini kimeachwa kwako. Huu ni mwongozo, si ripoti ya majaribio wala ushauri wa kisheria.*

## Hali ilivyo

**Hakuna jaribio huru la usalama lililofanyika.** Hakuna jaribio la kuvunja usalama, ukaguzi wa mtu wa tatu, mapitio huru ya msimbo wala uthibitisho kwa programu hii, na mradi haudai chochote ([njia ya kuelekea uzalishaji](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). Uchunguzi tulivu hufanyika katika ujumuishaji endelevu wa mradi (kanuni ziko katika `.zap/rules.tsv`), na majaribio ya mradi wenyewe hukagua udhibiti ulioorodheshwa hapa chini; hakuna kati ya hivyo viwili kinachochukua nafasi ya mjaribu ambaye hakuandika programu.

Hiki ni kipengele cha 1 na cha 9 cha [orodha ya uhakikisho](../../../../governance/assurance-checklist.md). Ni cha msambazaji wa huduma, pamoja na mjaribu aliyethibitishwa, dhidi ya usambazaji **wako**: mradi hauwezi kujaribu upangishaji wako, mtoa utambulisho, mtandao wala funguo.

## Cha kujaribu

Jaribu usambazaji kama utakavyoendeshwa, kuingia kukilazimishwa na mtoa utambulisho wako mbele yake. Wigo unaopendekezwa, kutokana na udhibiti ambao programu inadai:

| Eneo | Programu inadai nini | Mjaribu anapaswa kujaribu nini |
|---|---|---|
| Uthibitishaji | Kuingia kumewashwa kwa chaguo-msingi; huduma haitaanza bila funguo za kuthibitisha au, katika uzalishaji, kuingia kukizimwa; tokeni huthibitishwa nje ya mtandao dhidi ya funguo zilizochapishwa | Tokeni zilizoghushiwa, zilizoisha muda, zilizorudiwa na zilizosainiwa vibaya; mtoaji au hadhira isiyo sahihi; mzunguko wa funguo; seti ya funguo inayokosekana |
| Uidhinishaji | Sera ya msingi wa sifa; kanuni za kiwango cha rekodi | Kupandisha ruhusa kwa mlalo na wima kwenye kila njia; kubadilisha vitambulisho katika njia na miili; sifa ambazo mpigaji hapaswi kuweza kuongeza |
| Mipango ya faragha na alama za ushughulikiaji | Mpango wa faragha, na mpango ulio juu ya kiwango cha idhini cha mpigaji, hauko katika orodha, hesabu, jumla, matokeo ya utafutaji, mlisho wala usafirishaji wowote kwa mpigaji huyo, wasimamizi wakiwemo | Njia yoyote inayovujisha uwepo, jina, hesabu au jumla ya mpango ambao mpigaji hawezi kuusoma; ujumbe wa makosa na muda unaotofautiana kati ya "umefichwa" na "haupo" |
| Ufikiaji wa dharura | Watu wanaostahili tu, sababu iliyotajwa, saa nne zisizozidi, mtu wa pili kuarifiwa | Matumizi bila ustahiki; kuongeza ruhusa; kutenda bila mtu wa pili kuarifiwa |
| Rekodi zilizogandishwa na za kuongezwa tu | Njia ya ukaguzi, pakiti za wakurugenzi zilizotolewa, historia ya imani ya utekelezaji, historia ya hali ya programu na akiba za mradi hukataa mabadiliko na ufutaji katika hifadhidata | Kusasisha au kufuta kupitia API, kupitia uingizaji kwa wingi, na kupitia njia yoyote inayogusa safu zilezile |
| Mgawanyo wa majukumu | Mwombaji hawezi kuidhinisha ombi lake mwenyewe la mabadiliko | Kuidhinisha kama mwombaji, kama msimamizi, na kwa kubadilisha mwidhinishaji baada ya kuwasilisha |
| Upakiaji na viambatisho | Maudhui lazima yalingane na aina iliyotangazwa; programu hukataliwa; kiunganishi cha hiari cha uchunguzi wa virusi hushindwa kwa kufunga | Faili za polyglot, programu zilizobadilishwa jina, faili kubwa kupita kiasi na zilizoharibika, hila za njia katika majina, na kama kichunguzi chako kinaitwa kweli |
| Maombi ya kutoka nje | Maeneo yaliyotolewa na mtumiaji yamezuiwa kwa anwani za umma za `https` | Ughushi wa maombi upande wa seva kwenda anwani za faragha, link-local na za metadata, uelekezaji upya, DNS rebinding |
| Vikao na kivinjari | Muda wa kutotumika, kikomo kigumu, kuki ya `httpOnly`, Sera kali ya Usalama wa Maudhui, orodha ya CORS inayoruhusiwa | Kurekebisha na kutumia tena kikao, ughushi wa maombi kati ya tovuti, uingizaji ndani ya kiolesura, clickjacking |
| Kikomo cha kasi ya maombi | Vikomo kwa kila mpigaji | Kukwepa kwa kubadilisha utambulisho, kichwa au njia; kama vikomo vinadumu kupitia proksi yako |
| Zana za mwenye data | Usafirishaji na kuficha utambulisho, wasimamizi pekee, mwonekano wa awali kwanza | Matumizi na asiye msimamizi; ukamilifu dhidi ya [orodha ya data binafsi](../../../../governance/personal-data-inventory.csv) |
| Nakala rudufu | Usimbaji wa hiari, urejeshaji uliojaribiwa | Ufikiaji wa faili za nakala rudufu na funguo; kurejesha kwenye mazingira safi |

## Mradi unatoa nini

- Chanzo, maelezo na majaribio, ili mapitio ya msimbo yaanze kutoka kwenye madai.
- Maelezo yanayosomeka na mashine ya API (`/api-docs/openapi.json`), pamoja na skima za ombi na jibu kwa kazi ya pakiti ya wakurugenzi; orodha ya njia hukaguliwa dhidi ya msimbo na majaribio.
- Usanidi wa kutupwa wenye kuingia kumezimwa (`dev-insecure`, tazama [README](../../../../README.md)) **kwa mashine ya mjaribu mwenyewe pekee**, na hati inayozalisha seti ya funguo za majaribio (`scripts/smoke-keys.sh`) ili tokeni zenye sifa zilizochaguliwa ziweze kutengenezwa dhidi ya usambazaji wa majaribio.
- [Mchoro wa mtiririko wa data](../../../../governance/data-flow.md), [mwongozo wa uendeshaji](../../../../operations/runbook.md) na [malengo ya urejeshaji](../../../../operations/recovery-objectives.md).
- Njia ya faragha ya kuripoti yaliyopatikana: [SECURITY.md](../../../../SECURITY.md).

## Kanuni za ushiriki

Kubaliana haya kwa maandishi kabla ya jaribio lolote; mradi hauwezi kuyaweka kwa mazingira yako.

- **Mazingira.** Jaribu nakala yenye data halisi-kidogo lakini si halisi. Usijaribu uzalishaji isipokuwa umeamua hivyo, kwa maandishi.
- **Idhini.** Mjaribu ana ruhusa yako iliyoandikwa, mtu wa mawasiliano aliyetajwa, na dirisha la muda lililotajwa.
- **Nje ya mipaka.** Kunyima huduma kwa wingi, hila za kijamii dhidi ya wafanyakazi, na huduma za mtu wa tatu (mtoa utambulisho wako, mpangishaji wako) ziko nje ya wigo isipokuwa mmiliki wa kila moja akubali. Wigo wa mradi wenyewe uko katika [SECURITY](../../../../SECURITY.md).
- **Data.** Data binafsi halisi haitumiki, hainakiliwi nje, na haiwekwi kwenye ripoti.
- **Funguo.** Funguo za majaribio ni tofauti na funguo za uzalishaji na huharibiwa baadaye.

## Kuripoti na kujaribu upya

- Matokeo kuhusu programu hii hupelekwa kwa mtunzaji kwa faragha, kama [SECURITY](../../../../SECURITY.md) inavyosema, pamoja na jinsi ya kuyarudia, toleo au commit, na athari. Malengo ya mradi ni kukiri ndani ya siku 5 za kazi na mpango ndani ya 15; ni malengo, si dhamana. Kuna mtunzaji mmoja na hakuna makubaliano ya kiwango cha huduma.
- Matokeo kuhusu usambazaji wako hupelekwa kwa mmiliki wako mwenyewe.
- Mwombe mjaribu apime kila matokeo, aseme kilichokuwa **hakijajaribiwa**, na ajaribu upya marekebisho. Ripoti inayoorodhesha yaliyopatikana tu haikuambii kilichokosekana.
- Hifadhi ripoti, wigo, tarehe na jaribio la upya pamoja na kumbukumbu zako mwenyewe. Mradi hauwezi kuzishikilia kwa niaba yako.

## Mipaka ya waraka huu

Unaorodhesha maeneo ya kufunika, si mpango kamili wa majaribio, na madai katika jedwali ni ya mradi wenyewe. Hauthibitishi chochote, na jaribio safi ni ushahidi kuhusu usambazaji mmoja katika tarehe moja.

## Maamuzi ambayo ni yako

Nani anajaribu, lini, dhidi ya nini, kwa data gani na chini ya kanuni zipi; matokeo yanapimwaje na yanapaswa kurekebishwa kufikia lini; na kama matokeo yatashirikiwa na wakaguzi wako.

## Soma ifuatayo

[Orodha ya uhakikisho](../../../../governance/assurance-checklist.md) · [Njia ya kuelekea uzalishaji](../../path-to-production/index.md) · [Afisa Mkuu wa Usalama](../chief-security-officer/index.md) · [Utawala, Hatari, Kufuata Sheria](../governance-risk-compliance/index.md)
