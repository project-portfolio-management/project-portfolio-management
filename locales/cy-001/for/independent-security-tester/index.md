# Profi diogelwch annibynnol

*I brynwr, asesydd neu brofwr: beth ddylai profi diogelwch annibynnol o'r meddalwedd hwn ei gwmpasu, beth mae'r prosiect yn ei gyflenwi i helpu, beth sydd wedi'i wneud a beth sydd heb, a beth sy'n cael ei adael i chi. Canllawiau yw hyn, nid adroddiad prawf ac nid cyngor cyfreithiol.*

## Lle mae pethau'n sefyll

**Ni wnaed unrhyw brawf diogelwch annibynnol.** Nid oes prawf treiddio, archwiliad trydydd parti, adolygiad cod annibynnol nac ardystio ar gyfer y meddalwedd hwn, ac nid yw'r prosiect yn honni dim ([parodrwydd cynhyrchu](../../../../governance/production-readiness.md), [SECURITY](../../../../SECURITY.md)). Mae sgan goddefol yn rhedeg yn integreiddio parhaus y prosiect (rheolau yn `.zap/rules.tsv`), ac mae profion y prosiect ei hun yn gwirio'r rheolaethau a restrir isod; nid yw'r naill na'r llall yn cymryd lle profwr nad oedd yn ysgrifennu'r meddalwedd.

Eitem 1 ac eitem 9 o'r [rhestr wirio sicrwydd](../../../../governance/assurance-checklist.md) yw hyn. Mae'n perthyn i'r defnyddiwr, gyda phrofwr ardystiedig, yn erbyn **eich** defnydd chi: ni all y prosiect brofi eich gwesteio, eich darparwr hunaniaeth, eich rhwydwaith na'ch allweddi.

## Beth i'w brofi

Profwch y defnydd fel y bydd yn rhedeg, gyda mewngofnodi wedi'i orfodi a'ch darparwr hunaniaeth o'i flaen. Cwmpas a awgrymir, o'r rheolaethau y mae'r meddalwedd yn honni eu bod ganddo:

| Maes | Yr hyn y mae'r meddalwedd yn ei honni | Yr hyn y dylai profwr ei geisio |
|---|---|---|
| Dilysu | Mae mewngofnodi ymlaen yn ddiofyn; ni fydd y gwasanaeth yn cychwyn heb allweddi gwirio nac, mewn cynhyrchu, gyda mewngofnodi wedi'i ddiffodd; gwirir tocynnau all-lein yn erbyn allweddi cyhoeddedig | Tocynnau ffug, wedi dod i ben, wedi'u hailchwarae ac wedi'u llofnodi'n anghywir; cyhoeddwr neu gynulleidfa anghywir; cylchdroi allweddi; set allweddi ar goll |
| Awdurdodi | Polisi seiliedig ar briodoleddau; rheolau lefel cofnod | Dyrchafu breintiau llorweddol a fertigol ar bob llwybr; newid dynodwyr mewn llwybrau a chyrff; priodoleddau na ddylai galwr allu eu hychwanegu |
| Cynlluniau preifat a marciau trin | Nid yw cynllun preifat, na chynllun uwchlaw clirio galwr, mewn unrhyw restr, cyfrif, cyfanswm, canlyniad chwilio, porthiant nac allforio i'r galwr hwnnw, gweinyddwyr yn gynwysedig | Unrhyw lwybr sy'n gollwng bodolaeth, enw, cyfrif neu gyfanswm cynllun na all y galwr ei ddarllen; negeseuon gwall ac amseru sy'n wahanol rhwng "cudd" ac "absennol" |
| Mynediad brys | Pobl gymwys yn unig, rheswm a nodir, pedair awr ar y mwyaf, ail unigolyn yn cael gwybod | Defnyddio heb gymhwystra; ymestyn grant; gweithredu heb i'r ail unigolyn gael gwybod |
| Cofnodion wedi'u rhewi ac ychwanegu-yn-unig | Mae'r llwybr archwilio, pecynnau cyfarwyddwyr a gyhoeddwyd, hanes hyder cyflawni, hanes statws rhaglenni a chlustogau prosiect yn gwrthod newid a dileu yn y gronfa ddata | Diweddaru neu ddileu drwy'r API, drwy fewnforio swmp, a thrwy unrhyw lwybr sy'n cyffwrdd â'r un rhesi |
| Gwahanu dyletswyddau | Ni all gofynnwr gymeradwyo ei gais newid ei hun | Cymeradwyo fel y gofynnwr, fel gweinyddwr, a thrwy newid y cymeradwywr ar ôl cyflwyno |
| Uwchlwytho ac atodiadau | Rhaid i gynnwys gyfateb i'r math a ddatgenir; gwrthodir rhaglenni; mae bachyn sganio feirysau dewisol yn methu'n gaeedig | Ffeiliau amlffurf, rhaglenni wedi'u hailenwi, ffeiliau rhy fawr ac anghywir, triciau llwybr mewn enwau, ac a elwir eich sganiwr mewn gwirionedd |
| Ceisiadau allan | Cyfyngir cyrchfannau a ddarperir gan ddefnyddwyr i gyfeiriadau `https` cyhoeddus | Ffugio ceisiadau ochr gweinydd i gyfeiriadau preifat, cyswllt-lleol a metadata, ailgyfeirio, ailrwymo DNS |
| Sesiynau a'r porwr | Terfyn amser segur, terfyn caled, cwci `httpOnly`, Polisi Diogelwch Cynnwys llym, rhestr caniatáu CORS | Gosod ac ailddefnyddio sesiwn, ffugio ceisiadau traws-safle, chwistrellu i'r rhyngwyneb, clicjacio |
| Cyfyngu cyfraddau | Terfynau fesul galwr | Osgoi drwy newid hunaniaeth, pennawd neu lwybr; a yw'r terfynau'n goroesi eich dirprwy |
| Offer testun data | Allforio a ffugenwi, gweinyddwyr yn unig, rhagolwg yn gyntaf | Defnydd gan rywun nad yw'n weinyddwr; cyflawnrwydd yn erbyn y [rhestr eiddo data personol](../../../../governance/personal-data-inventory.csv) |
| Copïau wrth gefn | Amgryptio dewisol, adfer wedi'i brofi | Mynediad i ffeiliau ac allweddi copïau wrth gefn; adfer i amgylchedd glân |

## Beth mae'r prosiect yn ei gyflenwi

- Y ffynhonnell, y fanyleb a'r profion, fel y gall adolygiad cod ddechrau o'r honiadau.
- Disgrifiad darllenadwy gan beiriant o'r API (`/api-docs/openapi.json`), gyda sgemâu cais ac ymateb ar gyfer gwaith y pecyn cyfarwyddwyr; mae profion yn gwirio rhestr y llwybrau yn erbyn y cod.
- Ffurfwedd taflu-i-ffwrdd gyda mewngofnodi wedi'i ddiffodd (`dev-insecure`, gweler y [README](../../../../README.md)) **ar gyfer peiriant y profwr ei hun yn unig**, a sgript sy'n cynhyrchu set allweddi prawf (`scripts/smoke-keys.sh`) fel y gellir bathu tocynnau â phriodoleddau a ddewiswyd yn erbyn defnydd prawf.
- Y [diagram llif data](../../../../governance/data-flow.md), y [llawlyfr rhedeg](../../../../operations/runbook.md) a'r [amcanion adfer](../../../../operations/recovery-objectives.md).
- Ffordd breifat o adrodd am yr hyn a ganfyddir: [SECURITY.md](../../../../SECURITY.md).

## Rheolau ymgysylltu

Cytunwch ar y rhain yn ysgrifenedig cyn unrhyw brawf; ni all y prosiect eu gosod ar gyfer eich amgylchedd.

- **Amgylchedd.** Profwch gopi gyda data realistig ond nid go iawn. Peidiwch â phrofi cynhyrchu oni fyddwch wedi penderfynu hynny, yn ysgrifenedig.
- **Awdurdodi.** Mae gan y profwr eich caniatâd ysgrifenedig, cyswllt a enwir, a ffenestr amser a nodir.
- **Allan o gwmpas.** Mae atal gwasanaeth drwy gyfaint, peirianneg gymdeithasol staff, a gwasanaethau trydydd parti (eich darparwr hunaniaeth, eich gwesteiwr) allan o gwmpas oni chytuna perchennog pob un. Mae cwmpas y prosiect ei hun yn [SECURITY](../../../../SECURITY.md).
- **Data.** Ni ddefnyddir data personol go iawn, ni chopïir ef allan, ac ni roddir ef yn yr adroddiad.
- **Allweddi.** Mae allweddi prawf ar wahân i allweddi cynhyrchu ac fe'u dinistrir wedyn.

## Adrodd ac ail-brofi

- Mae canfyddiadau am y meddalwedd hwn yn mynd i'r cynhaliwr yn breifat, fel y dywed [SECURITY](../../../../SECURITY.md), gyda sut i'w hatgynhyrchu, y fersiwn neu'r ymrwymiad, a'r effaith. Targedau'r prosiect yw cydnabod o fewn 5 diwrnod gwaith a chynllun o fewn 15; targedau ydynt, nid gwarantau. Mae un cynhaliwr a dim cytundeb lefel gwasanaeth.
- Mae canfyddiadau am eich defnydd chi yn mynd i'ch perchennog eich hun.
- Gofynnwch i'r profwr raddio pob canfyddiad, dweud beth **na** phrofwyd, ac ail-brofi'r atgyweiriadau. Nid yw adroddiad sy'n rhestru'r hyn a ganfuwyd yn unig yn dweud wrthych beth a gollwyd.
- Cadwch yr adroddiad, y cwmpas, y dyddiadau a'r ail-brawf gyda'ch cofnodion eich hun. Ni all y prosiect eu cadw i chi.

## Cyfyngiadau'r ddogfen hon

Mae'n rhestru meysydd i'w cwmpasu, nid cynllun prawf cyflawn, ac honiadau'r prosiect ei hun yw'r rhai yn y tabl. Nid yw'n ardystio dim, ac mae prawf glân yn dystiolaeth am un defnydd ar un dyddiad.

## Penderfyniadau sy'n eiddo i chi

Pwy sy'n profi, pryd, yn erbyn beth, â pha ddata ac o dan ba reolau; sut y graddir canfyddiadau a pha bryd y mae'n rhaid eu trwsio; a pha un a rennir y canlyniad â'ch aseswyr.

## Darllenwch nesaf

[Rhestr wirio sicrwydd](../../../../governance/assurance-checklist.md) · [Parodrwydd cynhyrchu](../../../../governance/production-readiness.md) · [Prif Swyddog Diogelwch](../chief-security-officer/index.md) · [Llywodraethu, Risg, Cydymffurfiaeth](../governance-risk-compliance/index.md)
