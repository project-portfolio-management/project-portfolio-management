# Kwa Afisa Mkuu wa Taarifa

*Afisa Mkuu wa Taarifa: ujumuishaji, utambulisho, data, upangishaji na taarifa utakazoombwa kukabidhi.*

## Utambulisho na ufikiaji

- Kuingia **kunalazimishwa kwa chaguo-msingi**; huduma inakataa kuanza bila funguo za kuthibitisha tokeni, na katika uzalishaji ikiwa kuingia kumezimwa.
- Kuingia kupitia mtoa utambulisho (ikiwemo Microsoft Entra ID); uthibitishaji wa hatua nyingi hubaki kwa mtoa huduma ([mwongozo wa kuingia](../../../../operations/sign-in-with-entra.md)).
- Uidhinishaji hutumia sifa za tokeni (kwa mfano `access`, `directorpack`, `confidence`) pamoja na sera ya sifa. Mipango ya faragha huachwa nje ya kila jumla; alama za ushughulikiaji huficha mipango iliyo juu ya kiwango cha idhini cha mpigaji, wasimamizi wakiwemo.
- Ufikiaji wa dharura wa mipango ya faragha unahitaji ustahiki, sababu iliyotajwa, saa nne zisizozidi, na mtu wa pili kuarifiwa.
- Wasimamizi wanaweza kuorodhesha ruhusa za ufikiaji zilizopo (msaada wa kuthibitisha upya) na kutoa au kuficha utambulisho wa data ya mtu mmoja.

## Data na ujumuishaji

| Mada | Msimamo |
|---|---|
| Data inayoshikiliwa | Rekodi za mpango, kazi, hatari, bajeti, hatua muhimu, muda na gharama; watu ni marejeleo yasiyoeleweka, si majina. Hakuna data ya wagonjwa wala data ya aina maalum kwa usanifu |
| Orodha | Kila safu ya data binafsi imeorodheshwa katika [orodha ya data binafsi](../../../../governance/personal-data-inventory.csv); jaribio hulinda usahihi wake dhidi ya skima |
| Violesura | REST pamoja na hati ya OpenAPI inayotokana na njia; uingizaji na utoaji kwa wingi; milisho ya kalenda; matukio na webhook zinazotoka kwenda kwenye maeneo ya umma ya `https` pekee |
| Nakala rudufu | Nakala rudufu kwa hati pamoja na urejeshaji uliojaribiwa, usimbaji wa hiari, malengo ya urejeshaji yaliyotajwa ([nakala rudufu na urejeshaji](../../../../operations/backup-and-restore.md)) |
| Upangishaji | Picha za kontena na `compose.yaml`; programu haifanyi uhamishaji wowote yenyewe, kwa hivyo mahali pa data ni uchaguzi wako wa upangishaji |

## Mapungufu unayopaswa kuziba

- Jaribio la kuvunja usalama, mapitio huru ya msimbo na uthibitisho: hakuna kilichofanyika.
- Uhifadhi unatekelezwa kwa sehemu tu; usafishaji lazima upangwe (tazama [ratiba ya uhifadhi](../../../../governance/retention-schedule.md)).
- Zana ya mwenye data hupata marejeleo, si majina yaliyoandikwa kwenye maandishi huru au faili zilizopakiwa.
- Njia ya ukaguzi haiwezi kuzuia upotoshaji na msimamizi wa hifadhidata.

## Soma ifuatayo

[Mchoro wa mtiririko wa data](../../../../governance/data-flow.md) · [Orodha ya uhakikisho](../../../../governance/assurance-checklist.md) · [Usambazaji](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
