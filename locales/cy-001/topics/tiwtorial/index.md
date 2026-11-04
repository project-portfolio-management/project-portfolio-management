# Tiwtorial

*Rhedwch yr arddangosiad ar eich cyfrifiadur eich hun ac ewch ar daith o amgylch portffolio cyfan.*

Byddwch yn cychwyn copi cyflawn o'r feddalwedd, wedi'i lenwi â sefydliad dychmygol o'r enw Northgate Transport Authority. Mae'r holl bobl, cynlluniau, arian a dyddiadau yn ffug, felly mae croeso i chi newid unrhyw beth.

**Cadwch ef ar eich cyfrifiadur eich hun.** Mae'r arddangosiad yn ymddiried mewn allwedd brawf gyhoeddedig, felly peidiwch byth â'i roi ar rwydwaith a rennir na'i ddefnyddio gyda data go iawn.

## Beth sydd ei angen arnoch

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) gyda Compose

## Cam 1: cael y cod

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Cam 2: cychwyn yr arddangosiad

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Mae hyn yn adeiladu'r feddalwedd, yn cychwyn y gronfa ddata, yn ei llenwi â'r portffolio ffug ac yn cychwyn y rhyngwyneb. Mae'r adeiladu cyntaf yn cymryd cryn amser.

## Cam 3: mewngofnodi

Agorwch <http://localhost:3000/cy-001/signin>, gadewch y cyfeiriad fel `example@example.com` a gwasgwch y botwm mewngofnodi i'r arddangosiad. Ni anfonir e-bost.

Byddwch yn glanio ar **daith yr arddangosiad**, sy'n rhestru pob nodwedd mewn grwpiau, gyda'r hyn i chwilio amdano a dolen. Mae baner felen ar bob tudalen yn eich atgoffa bod y data'n ffug.

## Cam 4: gweld y portffolio ar yr olwg gyntaf

O'r daith, agorwch y cynlluniau i weld y portffolio, ei raglenni a'i brosiectau, yna agorwch ddangosfwrdd y portffolio. Sylwch y gall unrhyw gynllun gynnwys unrhyw un arall.

## Cam 5: dilyn un prosiect

Agorwch y prosiect **North Line interlocking** drwy grŵp cyflawni'r daith. Edrychwch ar ei siart Gantt, yna ei lwybr critigol, yna ei werth a enillwyd.

## Cam 6: gweld eich gwaith eich hun

Agorwch eich gwaith a'ch bwrdd kanban. Rydych wedi mewngofnodi fel Example Person, sydd â rhai tasgau agored. Symudwch gerdyn i weld sut mae'n gweithio.

## Cam 7: gweld sut mae'n siarad yn blaen

Agorwch y map gwres capasiti i weld pwy sydd wedi'i or-archebu. Yna agorwch y ciw cychwyn, lle mae un rhaglen wedi'i gwrthod gan y cyfyngiad ar waith ar y gweill ac yn aros. Gorffennwch gyda'r pecynnau i gyfarwyddwyr, y crynodeb misol sy'n cael ei rewi ar ôl ei gyhoeddi.

## Cam 8: stopio a thacluso

Mae hyn yn stopio popeth ac yn dileu data'r arddangosiad:

```sh
podman compose --profile demo down -v
```

## Ble i fynd nesaf

- [Gosodwch ef yn iawn](../../../../INSTALL.md), gyda mewngofnodi a'ch darparwr hunaniaeth eich hun
- Darllenwch y [briffiau rôl](../../for/) ar gyfer rolau uwch
- Darllenwch y [canllaw arddangos llawn](../../../../operations/demo.md)
- [Gweld pob nodwedd](../nodweddion/index.md)

Wedi mynd yn sownd? [Agorwch fater](https://github.com/project-portfolio-management/project-portfolio-management/issues).
