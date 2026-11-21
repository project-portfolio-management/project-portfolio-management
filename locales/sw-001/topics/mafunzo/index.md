# Mafunzo

*Hatua za kuendesha onyesho kwenye kompyuta ya ndani na kupitia jalada la mfano.*

Onyesho ni nakala kamili ya programu, iliyopakiwa data ya shirika la kubuni, Mamlaka ya Usafiri ya Northgate. Watu wote, mipango, takwimu na tarehe ni za kubuni, na data inaweza kubadilishwa kwa uhuru.

**Tahadhari ya usalama.** Onyesho linaamini ufunguo wa majaribio uliochapishwa. Haipaswi kuendeshwa kwenye mtandao wa pamoja wala kutumiwa na data halisi.

## Mahitaji

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) pamoja na Compose

## Hatua ya 1: pakua msimbo

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Hatua ya 2: anzisha onyesho

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Amri hii hujenga programu, huanzisha hifadhidata, hupakia jalada la kubuni na huanzisha kiolesura. Ujenzi wa kwanza huchukua muda mrefu kuliko ujao.

## Hatua ya 3: ingia

Fungua <http://localhost:3000/sw-001/signin>, acha anwani kama `example@example.com` kisha bonyeza **Sign in to the demonstration**. Hakuna barua pepe inayotumwa.

Ukurasa wa kwanza ni **ziara ya onyesho**, inayoorodhesha kila kipengele kwa kundi, pamoja na maelezo na kiungo. Bango la manjano kwenye kila ukurasa linaeleza kuwa data ni ya kubuni.

## Hatua ya 4: pitia jalada

Kutoka kwenye ziara, fungua **Plans** ili kuona jalada, programu zake na miradi yake. Kisha fungua **Portfolio dashboard**.

## Hatua ya 5: pitia mradi mmoja

Fungua mradi wa **North Line interlocking** kupitia kundi la utekelezaji la ziara. Pitia chati yake ya **Gantt**, **Critical path** na **Earned value**.

## Hatua ya 6: pitia kazi binafsi

Fungua **My work** na **My kanban board**. Kikao kimeingia kama Example Person, ambaye ana kazi zilizo wazi.

## Hatua ya 7: pitia uwezo, udhibiti wa kuanza na uripoti

Fungua **Capacity heatmap** ili kuona rasilimali zilizopangiwa kupita kiasi. Fungua **Start queue**, ambapo kikomo cha kazi zinazoendelea kimezuia programu moja. Fungua **Director packs**, muhtasari wa kila mwezi unaogandishwa mara unapotolewa.

## Hatua ya 8: simamisha onyesho

Amri hii husimamisha huduma zote na kufuta data ya onyesho:

```sh
podman compose --profile demo down -v
```

## Hatua zinazofuata

- [Mwongozo wa usakinishaji](../../../../INSTALL.md), unaohusu kuingia na watoa utambulisho
- [Muhtasari wa majukumu](../../for/) kwa majukumu ya juu
- [Mwongozo wa onyesho](../../../../operations/demo.md)
- [Orodha ya vipengele](../vipengele/index.md)

Matatizo yanaweza kuripotiwa kupitia [kifuatiliaji cha masuala](https://github.com/project-portfolio-management/project-portfolio-management/issues).
