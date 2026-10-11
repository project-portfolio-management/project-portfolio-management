# I'r Prif Swyddog Gwybodaeth

*Y Prif Swyddog Gwybodaeth: integreiddio, hunaniaeth, data, gwesteio a'r wybodaeth y gofynnir i chi ei darparu.*

## Hunaniaeth a mynediad

- Mae mewngofnodi'n **orfodol yn ddiofyn**; mae'r gwasanaeth yn gwrthod cychwyn heb allweddi gwirio tocynnau, ac yn gwrthod cychwyn mewn cynhyrchu gyda mewngofnodi wedi'i ddiffodd.
- Mewngofnodi drwy ddarparwr hunaniaeth (gan gynnwys Microsoft Entra ID); mae dilysu aml-ffactor yn aros gyda'r darparwr ([canllaw mewngofnodi](../../../../operations/sign-in-with-entra.md)).
- Mae awdurdodi'n defnyddio priodoleddau tocyn (er enghraifft `access`, `directorpack`, `confidence`) a pholisi priodoleddau. Cuddir cynlluniau preifat o bob crynhoad; mae marciau trin yn cuddio cynlluniau uwchlaw clirio'r galwr, gweinyddwyr yn gynwysedig.
- Mae mynediad brys i gynlluniau preifat angen cymhwystra, rheswm a nodir, pedair awr ar y mwyaf, a hysbysu ail unigolyn.
- Gall gweinyddwyr restru'r grantiau mynediad sy'n sefyll (cymorth i ailardystio) ac allforio neu ffugenwi data un unigolyn.

## Data ac integreiddio

| Pwnc | Sefyllfa |
|---|---|
| Data a gedwir | Cofnodion cynllun, tasg, risg, cyllideb, carreg filltir, amser a chost; cyfeiriadau anhryloyw yw pobl, nid enwau. Dim data cleifion na data categori arbennig yn ôl dyluniad |
| Rhestr eiddo | Rhestrir pob colofn data personol yn y [rhestr eiddo data personol](../../../../governance/personal-data-inventory.csv); mae prawf yn ei chadw'n gywir i'r sgema |
| Rhyngwynebau | REST gyda dogfen OpenAPI wedi'i deillio; mewnforio ac allforio swmp; porthiannau calendr; digwyddiadau a bachau gwe allan i gyrchfannau `https` cyhoeddus yn unig |
| Copïau wrth gefn | Copi wrth gefn wedi'i sgriptio gydag adfer wedi'i brofi, amgryptio dewisol, amcanion adfer a nodir ([copïo wrth gefn ac adfer](../../../../operations/backup-and-restore.md)) |
| Gwesteio | Delweddau cynhwysydd a `compose.yaml`; nid yw'r meddalwedd yn trosglwyddo dim ei hun, felly eich dewis gwesteio chi yw lleoliad y data |

## Bylchau y mae'n rhaid i chi eu cau

- Prawf treiddio, adolygiad cod annibynnol ac ardystio: dim wedi'i wneud.
- Dim ond yn rhannol y gorfodir cadw; rhaid amserlennu glanhau (gweler yr [amserlen gadw](../../../../governance/retention-schedule.md)).
- Mae'r offeryn testun data yn canfod cyfeiriadau, nid enwau a deipiwyd mewn testun rhydd na ffeiliau a uwchlwythwyd.
- Nid yw'r llwybr archwilio yn ddiogel rhag ymyrraeth gan weinyddwr cronfa ddata.

## Darllenwch nesaf

[Diagram llif data](../../../../governance/data-flow.md) · [Rhestr wirio sicrwydd](../../../../governance/assurance-checklist.md) · [Defnyddio](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
