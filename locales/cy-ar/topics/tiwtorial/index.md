# Tiwtorial

*Camau i redeg yr arddangosiad ar gyfrifiadur lleol ac adolygu portffolio enghreifftiol.*

Copi cyflawn o'r feddalwedd yw'r arddangosiad, wedi'i lwytho â data ar gyfer sefydliad ffug, y Northgate Transport Authority. Mae'r holl bobl, cynlluniau, ffigurau a dyddiadau yn ffug, a gellir newid y data yn rhydd.

**Nodyn diogelwch.** Mae'r arddangosiad yn ymddiried mewn allwedd brawf gyhoeddedig. Rhaid peidio â'i redeg ar rwydwaith a rennir na'i ddefnyddio gyda data go iawn.

## Gofynion

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) gyda Compose

## Cam 1: lawrlwytho'r cod

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Cam 2: cychwyn yr arddangosiad

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Mae'r gorchymyn hwn yn adeiladu'r feddalwedd, yn cychwyn y gronfa ddata, yn llwytho'r portffolio ffug ac yn cychwyn y rhyngwyneb. Mae'r adeiladu cyntaf yn cymryd yn hirach na'r rhai dilynol.

## Cam 3: mewngofnodi

Agorwch <http://localhost:3000/cy-001/signin>, gadewch y cyfeiriad fel `example@example.com` a gwasgwch y botwm mewngofnodi i'r arddangosiad. Ni anfonir e-bost.

Y dudalen gyntaf yw **taith yr arddangosiad**, sy'n rhestru pob nodwedd fesul grŵp, gyda disgrifiad a dolen. Mae baner felen ar bob tudalen yn nodi bod y data'n ffug.

## Cam 4: adolygu'r portffolio

O'r daith, agorwch y cynlluniau i weld y portffolio, ei raglenni a'i brosiectau. Yna agorwch ddangosfwrdd y portffolio.

## Cam 5: adolygu un prosiect

Agorwch y prosiect **North Line interlocking** drwy grŵp cyflawni'r daith. Adolygwch ei siart Gantt, ei lwybr critigol a'i werth a enillwyd.

## Cam 6: adolygu gwaith personol

Agorwch y dudalen waith bersonol a'r bwrdd kanban. Mae'r sesiwn wedi mewngofnodi fel Example Person, sydd â thasgau agored.

## Cam 7: adolygu capasiti, rheoli cychwyn ac adrodd

Agorwch y map gwres capasiti i weld adnoddau sydd wedi'u gor-archebu. Agorwch y ciw cychwyn, lle mae'r cyfyngiad ar waith ar y gweill wedi dal un rhaglen yn ôl. Agorwch y pecynnau i gyfarwyddwyr, y crynodeb misol sy'n cael ei rewi ar ôl ei gyhoeddi.

## Cam 8: stopio'r arddangosiad

Mae'r gorchymyn hwn yn stopio pob gwasanaeth ac yn dileu data'r arddangosiad:

```sh
podman compose --profile demo down -v
```

## Camau nesaf

- [Canllaw gosod](../../../../INSTALL.md), sy'n ymdrin â mewngofnodi a darparwyr hunaniaeth
- [Briffiau rôl](../../for/) ar gyfer rolau uwch
- [Canllaw arddangos](../../../../operations/demo.md)
- [Rhestr nodweddion](../nodweddion/index.md)

Gellir adrodd am broblemau drwy'r [traciwr materion](https://github.com/project-portfolio-management/project-portfolio-management/issues).
