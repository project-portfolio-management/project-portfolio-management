# För personalchefen

*Personalchef (CPO): vad programvaran registrerar om dina medarbetare, hur den skyddar dem och vad den gör för nyanställda, flyttade och avgående.*

## Vad den registrerar om personer

Personer förekommer som **ogenomskinliga referenser**, inte som namn eller kontaktuppgifter, i tilldelningar, ägare, allokeringar, notiser om tillgänglighet och ledighet, tidrapporter och registrerad tid, kostnadssatser (begränsade till ekonomi), kommentarer och godkännanden, och i granskningsloggen. Varje användare kan ange en personlig destination för notiser (en e-postadress eller en chatt-webhook). Varje sådan kolumn finns i [förteckningen över personuppgifter](../../../../governance/personal-data-inventory.csv).

Eftersom den kombinerar tilldelningar, tid, tillgänglighet och kostnad ligger den **nära övervakning**, och därför förväntas en konsekvensbedömning avseende dataskydd ([DPIA](../../../../governance/dpia.md)); rådgör med dina personalrepresentanter enligt vad DPIA:n anger.

## Skydd för personer

- **Kostnadssatser** syns bara för roller som är begränsade till ekonomi.
- **Direktionspaketet visar ägare efter roll, aldrig efter person,** och innehåller inga personreferenser.
- **Kapacitet och arbetsbelastning** härleds från poster som personer eller chefer anger; de är inte ett prestationsbetyg.
- **Radering och åtkomst:** en administratör kan exportera eller pseudonymisera en persons data efter en förhandsvisning. Pseudonymisering tömmer uppgiften om vem som skrev eller bedömde en fryst post; den raderar inte posten.
- **Funktionsseparation:** ingen godkänner sin egen ändringsbegäran.
- **Gallring:** föreslagna perioder för personkopplade poster finns i [gallringsschemat](../../../../governance/retention-schedule.md); bara en del tillämpas automatiskt.

## Nyanställda, flyttade och avgående

Ärenden för introduktion och avslut har checklistor med en ägare, och åtkomstlistan kan granskas för att kontrollera att avgåendes behörigheter har tagits bort ([checklista för kvalitetssäkring](../../../../governance/assurance-checklist.md), punkt 11). Den visar vad som registrerats i tjänsten, inte vad din identitetsleverantör tillämpar; jämför de två.

## Omfattas inte

Inget personalregister, ingen löneadministration, ingen prestationshantering och inga särskilda kategorier av uppgifter ingår. Namn som skrivits i fritext hittas inte av verktyget för registrerades rättigheter. En kapacitetsvärmekarta per kompetenspool för direktioner är inte byggd (`DP-10`). Översättningar (walesiska inräknad) är utkast.

## Beslut som är dina

Vad personalen får veta, den rättsliga grunden för att registrera tid och tillgänglighet, hur länge personkopplade poster sparas och vem som får se kostnadssatser.

## Läs vidare

[Register över behandling](../../../../governance/record-of-processing.md) · [Gallringsschema](../../../../governance/retention-schedule.md) · [Intressenter och användare](../../../../spec/03-stakeholders-and-users.md)
