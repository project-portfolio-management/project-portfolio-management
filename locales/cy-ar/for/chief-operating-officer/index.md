# I'r Prif Swyddog Gweithrediadau

*Y Prif Swyddog Gweithrediadau: yr hyn rydych yn ei redeg o ddydd i ddydd, yr hyn y mae'r meddalwedd yn ei wneud ar gyfer cyflawni, a'r hyn sydd ei angen i'w weithredu.*

## Yr hyn y mae'n ei wneud ar gyfer gweithrediadau

- **Cynllunio a rheoli cyflawni:** cynlluniau, tasgau, dibyniaethau, cyfrifiad y llwybr critigol, carreg filltiroedd ar draws cynlluniau, sbrintiau a chyfnodau, llosgi i lawr a metrigau llif.
- **Capasiti:** dyraniadau, argaeledd, gwyliau a chyfnodau heb waith, rolau adnoddau, a llwyth gwaith a defnydd a ddeilliwyd. (Nid yw map gwres capasiti yn ôl cronfa sgiliau a mis ar gyfer y pecyn cyfarwyddwyr wedi'i adeiladu: `DP-10`.)
- **Amser a chost:** taflenni amser, amser a gofnodwyd, cyfraddau cost (cyfyngedig i gyllid), cyllidebau a chost a ddeilliwyd.
- **Llif llywodraethu:** cofrestr materion, ceisiadau newid gyda gwahanu dyletswyddau (ni all y sawl sy'n gofyn gymeradwyo), cymeradwyaethau, adolygiadau, achosion ymuno/symud/gadael.
- **Eithriadau:** mae hyder cyflawni coch neu ambr-coch yn agor eithriad sydd angen achos, y penderfyniad sydd ei angen, perchennog a enwir a dyddiad penderfynu o fewn 48 awr (ffenestr y gellir ei ffurfweddu, 1 i 720 awr). Dangosir toriadau yn y pecyn cyfarwyddwyr ac yn `GET /api/exceptions`.

## Yr hyn sydd ei angen i'w redeg

| Angen | Manylion |
|---|---|
| Gwasanaeth | Un gwasanaeth Rust, PostgreSQL 18, Redis dewisol (storfa gynhyrchu, sesiynau a rennir dewisol), rhyngwyneb SvelteKit |
| Llawlyfr rhedeg | [pecyn gweithrediadau](../../../../operations/README.md): defnyddio, llawlyfr rhedeg, copïo wrth gefn ac adfer, amcanion adfer, uwchraddio a mudo |
| Swyddi wedi'u hamserlennu | Nid oes dim yn amserlennu ei hun. Rhaid i'ch gweithredwyr roi tasgau fel glanhau cadw a hysbysiadau toriad eithriad ar amserlennydd |
| Cymorth | Un cynhaliwr, dim cytundeb lefel gwasanaeth; cyllidebwch ar gyfer eich cymorth rheng flaen eich hun |

## Gwybod y cyfyngiadau

- Mae cynllun o sawl mil o dasgau yn araf i'w amserlennu; uwchlaw 5,000 o dasgau gwrthodir yr ateb llawn oni ofynnir amdano (`?full=true` neu `?only=critical`). Mae'r ffigurau a'r rhybuddion yn [`BENCHMARKS.md`](../../../../BENCHMARKS.md); trefn maint o un peiriant ydynt, nid capasiti.
- Mae sgriniau ar gyfer llawer o'r man gwaith ond **nid** ar gyfer ceisiadau newid, materion, pecynnau na hyder cyflawni eto; dim ond drwy'r API y maent ar gael heddiw.

## Penderfyniadau sy'n eiddo i chi

Pwy sy'n berchen ar bob eithriad, pwy sy'n cyhoeddi'r pecyn misol (rhythm a nodiadau atgoffa yw `DP-12`, heb eu hadeiladu), a pha gylch adolygu y mae'r rheol eithriad yn ei fwydo.

## Darllenwch nesaf

[Pecyn gweithrediadau](../../../../operations/README.md) · [Llawlyfr rhedeg](../../../../operations/runbook.md) · [Amcanion adfer](../../../../operations/recovery-objectives.md)
