# Onafhankelijk beveiligingstesten

*Voor een koper, een beoordelaar of een tester: wat onafhankelijk beveiligingstesten van deze software moet omvatten, wat het project ter ondersteuning aanlevert, wat wel en niet is gedaan en wat aan u wordt overgelaten. Dit is richtlijn, geen testrapport en geen juridisch advies.*

## Waar het nu staat

**Er is geen onafhankelijke beveiligingstest uitgevoerd.** Voor deze software bestaat geen penetratietest, audit door derden, onafhankelijke codebeoordeling of certificering, en het project beweert er geen ([weg naar productie](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). In de continue integratie van het project draait een passieve scan (regels in `.zap/rules.tsv`), en de eigen tests van het project controleren de hieronder genoemde maatregelen; geen van beide vervangt een tester die de software niet zelf heeft geschreven.

Dit is punt 1 en punt 9 van de [zekerheidschecklist](../../../../governance/assurance-checklist.md). Het ligt bij de implementeerder, met een gecertificeerde tester, tegen **uw** implementatie: het project kan uw hosting, identiteitsprovider, netwerk of sleutels niet testen.

## Wat te testen

Test de implementatie zoals ze zal draaien, met afgedwongen aanmelding en uw identiteitsprovider ervoor. Voorgesteld bereik, op basis van de maatregelen die de software claimt:

| Gebied | Wat de software claimt | Wat een tester moet proberen |
|---|---|---|
| Authenticatie | Inloggen staat standaard aan; de dienst start niet zonder verificatiesleutels of, in productie, met uitgeschakelde aanmelding; tokens worden offline geverifieerd tegen gepubliceerde sleutels | Vervalste, verlopen, opnieuw afgespeelde en verkeerd ondertekende tokens; verkeerde uitgever of doelgroep; sleutelrotatie; een ontbrekende sleutelset |
| Autorisatie | Op kenmerken gebaseerd beleid; regels op recordniveau | Horizontale en verticale rechtenescalatie op elke route; identificatiecodes in paden en bodies wijzigen; kenmerken die een aanvrager niet zou mogen toevoegen |
| Privéplannen en vertrouwelijkheidsmarkeringen | Een privéplan, en een plan boven het niveau van een aanvrager, staat voor die aanvrager in geen lijst, aantal, totaal, zoekresultaat, feed of export, beheerders inbegrepen | Elke route die het bestaan, een naam, een aantal of een totaal lekt van een plan dat de aanvrager niet kan lezen; foutmeldingen en reactietijden die verschillen tussen "verborgen" en "niet aanwezig" |
| Noodtoegang | Alleen geschikte personen, een vermelde reden, hooguit vier uur, een tweede persoon wordt geïnformeerd | Gebruik zonder geschiktheid; een toekenning verlengen; handelen zonder dat de tweede persoon wordt geïnformeerd |
| Bevroren en alleen-toevoegen-records | Het auditspoor, uitgegeven bestuurspakketten, de geschiedenis van leveringszekerheid, de statusgeschiedenis van programma's en projectbuffers weigeren wijziging en verwijdering in de database | Bijwerken of verwijderen via de API, via bulkimport en via elke route die dezelfde rijen raakt |
| Functiescheiding | Een aanvrager kan zijn eigen wijzigingsverzoek niet goedkeuren | Goedkeuren als aanvrager, als beheerder en door de goedkeurder na indiening te wijzigen |
| Uploads en bijlagen | De inhoud moet overeenkomen met het opgegeven type; programma's worden geweigerd; een optionele virusscankoppeling sluit bij een fout | Polyglotbestanden, hernoemde programma's, te grote en misvormde bestanden, padtrucs in namen, en of uw scanner echt wordt aangeroepen |
| Uitgaande verzoeken | Door gebruikers opgegeven bestemmingen zijn beperkt tot openbare `https`-adressen | Server-side request forgery (SSRF) naar privé-, linklokale en metadata-adressen, omleidingen, DNS-rebinding |
| Sessies en de browser | Time-out bij inactiviteit, een harde limiet, een `httpOnly`-cookie, een strikte Content Security Policy, een CORS-toelatingslijst | Sessiefixatie en hergebruik, cross-site request forgery (CSRF), injectie in de interface, clickjacking |
| Snelheidsbeperking | Limieten per aanvrager | Omzeilen door identiteit, header of pad te wijzigen; of de limieten standhouden achter uw proxy |
| Hulpmiddelen voor betrokkenen | Export en pseudonimisering, alleen beheerders, eerst een voorbeeldweergave | Gebruik door een niet-beheerder; volledigheid ten opzichte van de [inventaris van persoonsgegevens](../../../../governance/personal-data-inventory.csv) |
| Back-ups | Optionele versleuteling, een geteste herstelprocedure | Toegang tot back-upbestanden en sleutels; herstel naar een schone omgeving |

## Wat het project aanlevert

- De broncode, de specificatie en de tests, zodat een codebeoordeling bij de claims kan beginnen.
- Een machineleesbare beschrijving van de API (`/api-docs/openapi.json`), met aanvraag- en antwoordschema's voor het werk aan het bestuurspakket; de lijst met routes wordt door tests tegen de code gecontroleerd.
- Een wegwerpconfiguratie met uitgeschakelde aanmelding (`dev-insecure`, zie de [README](../../../../README.md)) **alleen voor de eigen machine van de tester**, en een script dat een testsleutelset genereert (`scripts/smoke-keys.sh`), zodat tokens met gekozen kenmerken tegen een testimplementatie kunnen worden aangemaakt.
- Het [gegevensstroomdiagram](../../../../governance/data-flow.md), het [draaiboek](../../../../operations/runbook.md) en de [hersteldoelen](../../../../operations/recovery-objectives.md).
- Een privé manier om te melden wat wordt gevonden: [SECURITY.md](../../../../SECURITY.md).

## Spelregels voor de opdracht

Leg deze vóór elke test schriftelijk vast; het project kan ze niet voor uw omgeving bepalen.

- **Omgeving.** Test een kopie met realistische maar niet echte gegevens. Test niet in productie tenzij u dat schriftelijk hebt besloten.
- **Machtiging.** De tester heeft uw schriftelijke toestemming, een benoemd contactpersoon en een vermeld tijdvenster.
- **Buiten het bereik.** Denial of service door volume, social engineering van personeel en diensten van derden (uw identiteitsprovider, uw hostingpartij) vallen buiten het bereik, tenzij de eigenaar van elk daarmee instemt. Het eigen bereik van het project staat in [SECURITY](../../../../SECURITY.md).
- **Gegevens.** Echte persoonsgegevens worden niet gebruikt, niet naar buiten gekopieerd en niet in het rapport opgenomen.
- **Sleutels.** Testsleutels zijn gescheiden van productiesleutels en worden daarna vernietigd.

## Rapportage en hertest

- Bevindingen over deze software gaan privé naar de beheerder, zoals [SECURITY](../../../../SECURITY.md) aangeeft, met hoe ze te reproduceren zijn, de versie of commit en de impact. De doelstellingen van het project zijn bevestiging binnen 5 werkdagen en een plan binnen 15; het zijn doelstellingen, geen garanties. Er is één beheerder en geen serviceniveauovereenkomst.
- Bevindingen over uw implementatie gaan naar uw eigen eigenaar.
- Vraag de tester elke bevinding te beoordelen, te vermelden wat **niet** is getest en de oplossingen opnieuw te testen. Een rapport dat alleen opsomt wat is gevonden, zegt niet wat is gemist.
- Bewaar het rapport, het bereik, de data en de hertest bij uw eigen administratie. Het project kan ze niet voor u bewaren.

## Grenzen van dit document

Het somt gebieden op die gedekt moeten worden, geen volledig testplan, en de claims in de tabel zijn die van het project zelf. Het certificeert niets, en een schone test is bewijs over één implementatie op één datum.

## Besluiten die bij u liggen

Wie test, wanneer, waartegen, met welke gegevens en onder welke regels; hoe bevindingen worden beoordeeld en voor wanneer ze moeten zijn opgelost; en of het resultaat met uw beoordelaars wordt gedeeld.

## Lees verder

[Zekerheidschecklist](../../../../governance/assurance-checklist.md) · [Weg naar productie](../../path-to-production/index.md) · [Beveiligingsdirecteur](../chief-security-officer/index.md) · [Governance, risico en naleving](../governance-risk-compliance/index.md)
