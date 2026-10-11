# I'r Prif Swyddog Diogelwch

*Y Prif Swyddog Diogelwch: yr hyn y mae'r meddalwedd yn ei orfodi, yr hyn y mae'r prosiect wedi'i wirio, a'r hyn y mae'n rhaid i chi ei brofi a'i weithredu eich hun.*

## Yr hyn y mae'r meddalwedd yn ei orfodi

| Maes | Ymddygiad |
|---|---|
| Dilysu | Mae mewngofnodi ymlaen yn ddiofyn. Ni fydd y gwasanaeth yn cychwyn heb allweddi gwirio tocynnau, ac ni fydd yn cychwyn mewn cynhyrchu gyda mewngofnodi wedi'i ddiffodd. Mewngofnodi drwy ddarparwr hunaniaeth fel Microsoft Entra ID; mae dilysu aml-ffactor yn aros gyda'r darparwr ([canllaw mewngofnodi](../../../../operations/sign-in-with-entra.md)) |
| Awdurdodi | Polisi seiliedig ar briodoleddau tocyn. Cedwir cynlluniau preifat allan o bob crynhoad i bawb; mae marciau trin yn cuddio cynlluniau uwchlaw clirio'r galwr, gweinyddwyr yn gynwysedig |
| Mynediad brys | Mae mynediad i gynlluniau preifat angen cymhwystra, rheswm a nodir, pedair awr ar y mwyaf, a hysbysu ail unigolyn; archwilir pob cais |
| Sesiynau | Terfyn amser segur (30 munud yn ddiofyn) a therfyn sesiwn caled |
| Trafnidiaeth a phorwr | Penawdau diogelwch, Polisi Diogelwch Cynnwys llym, rhestr caniatáu CORS, cyfyngu cyfraddau fesul galwr |
| Allan | Cyfyngir cyrchfannau a ddarperir gan ddefnyddwyr i gyfeiriadau `https` cyhoeddus |
| Uwchlwytho | Rhaid i gynnwys gyfateb i'r math a ddatgenir; gwrthodir rhaglenni; mae sgan `clamd` dewisol yn methu'n gaeedig |
| Archwilio | Mae'r gronfa ddata yn gwrthod newid a dileu'r llwybr archwilio |
| Cofnodion wedi'u rhewi | Mae pecynnau cyfarwyddwyr a gyhoeddwyd ac asesiadau hyder cyflawni yn anghyfnewidiol neu'n ychwanegu-yn-unig drwy sbardun cronfa ddata |
| Cadwyn gyflenwi | `cargo deny`, bil deunyddiau meddalwedd, tarddiad adeiladu wrth ryddhau, [polisi diogelwch](../../../../SECURITY.md) |

## Yr hyn nad yw wedi'i wneud

- Dim prawf treiddio, archwiliad trydydd parti, adolygiad cod annibynnol nac ardystio.
- Dim sganiwr meddalwedd maleisus wedi'i gynnwys; profwyd y bachyn `clamd` yn erbyn dirprwy yn unig.
- Gall gweinyddwr cronfa ddata drechu'r llwybr archwilio; dim cadwyn hash.
- Mae marciau trin yn cuddio ac yn labelu, nid ydynt yn amgryptio, ac ni stampir marc i ffeiliau a gynhyrchir.
- Cedwir sesiynau yn y cof oni ffurfweddir Redis.
- Mae'r offeryn testun data yn canfod cyfeiriadau, nid enwau a deipiwyd mewn testun rhydd, ffeiliau a uwchlwythwyd na chopïau wrth gefn.

## Ei weithredu

Mae gan gopïau wrth gefn adfer wedi'i brofi, amgryptio dewisol ac amcanion adfer a nodir ([pecyn gweithrediadau](../../../../operations/README.md)). Mae'r [llawlyfr rhedeg](../../../../operations/runbook.md) yn rhoi'r camau technegol ar gyfer digwyddiadau; eich proses chi yw'r broses digwyddiadau a thorri data. Adroddwch am wendid yn breifat fel y disgrifia [SECURITY.md](../../../../SECURITY.md); y targedau yw cydnabod o fewn 5 diwrnod gwaith, nid gwarantau.

## Penderfyniadau sy'n eiddo i chi

Pwy sy'n cynnal y prawf treiddio a phryd; mynediad amodol a mapio rolau'r darparwr hunaniaeth; y lefel marcio y sicrheir y defnydd ar ei chyfer; y sganiwr y tu ôl i'r bachyn atodiadau; pa mor aml yr ailardystir mynediad.

## Darllenwch nesaf

[Rhestr wirio sicrwydd](../../../../governance/assurance-checklist.md) · [Parodrwydd cynhyrchu](../../../../governance/production-readiness.md) · [Diagram llif data](../../../../governance/data-flow.md)
