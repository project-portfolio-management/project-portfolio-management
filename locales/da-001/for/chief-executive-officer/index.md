# Til administrerende direktør

*Administrerende direktør: hvad denne software er, hvad den giver dig, hvad den ikke giver, og de beslutninger, der er dine.*

## Hvad den er, i ét afsnit

Et register over alle de **planer**, organisationen finansierer (porteføljer, programmer, projekter, produkter og så videre, én rekursiv post), med et arbejdsområde til projektledelse nedenunder (opgaver, tidsplan, omkostninger, ressourcer, risici, problemer, ændringsanmodninger, governance). Den finder dobbeltarbejde gennem forklarlig matchning, så to afdelinger, der opretter det samme stykke arbejde, bliver synlige, før begge er finansieret. Se [formål og vision](../../../../spec/01-purpose-and-vision.md).

## Hvad du kan stole på

- **Ét sted at spørge "hvad laver vi?"** Planer kan indlejres i vilkårlig dybde; private planer holdes uden for alle totaler og optællinger, for alle, efter en regel, som tests kontrollerer.
- **En månedlig direktionspakke**, der udarbejdes én gang, udstedes af en navngiven person og derefter **fryses af databasen**: en udstedt pakke kan ikke redigeres; en rettelse er en ny, genudstedt version med en note. I dag indeholder den porteføljekortet efter strategisk mål, leveringssikkerhed, undtagelser, problemer og ændringsanmodninger.
- **Leveringssikkerhed angivet af mennesker, vist ved siden af den beregnede status og aldrig blandet med den.** En plan, ingen har vurderet, viser "ikke vurderet", aldrig grøn. En ravgul-rød eller rød vurdering åbner en undtagelse med et ur på 48 timer (kan konfigureres); et overskredet ur vises i pakken.
- **Beslutninger efterlader et spor.** Hver ændring bliver revideret, revisionssporet kan ikke ændres gennem tjenesten, og en ændringsanmodning kan ikke godkendes af den, der har stillet den.

## Hvad den endnu ikke giver dig

- Pakken indeholder **endnu ikke** gate-beslutninger, finansiering og penge fordelt på regnskabsår, bufferfeberdiagrammet, earned value, validerede gevinster eller kapacitetsheatmappet. De er opført som `DP-5` til `DP-10` i [`tasks.md`](../../../../tasks.md), og pakken oplyser, hvilke afsnit der endnu ikke er bygget, i stedet for at lade dem stå tomme.
- Ingen PDF eller skærm til pakken: den leveres som data (`DP-11`).
- Den er **ikke certificeret eller revideret** af nogen. Hvad det vil kræve at tage den i produktion, punkt for punkt med et estimat for hvert, står på siden [vejen til produktion](../../path-to-production/index.md).
- Én vedligeholder, ingen serviceaftale ([governance](../../../../GOVERNANCE.md)).

## Beslutninger, der er dine

1. Om der skal køres en pilot, og på hvilken del af porteføljen.
2. Hvem der er **sponsor** for 48-timersreglen for undtagelser, og hvem der må udstede den månedlige pakke.
3. Om hullerne ovenfor skal finansieres (vurdering, penetrationstest, supportaftale) eller accepteres.

## Læs videre

[Vejen til produktion](../../path-to-production/index.md) · [Køreplan](../../../../spec/15-roadmap.md) · [Åbne spørgsmål](../../../../spec/16-open-questions.md)
