# Voor de beveiligingsdirecteur

*Beveiligingsdirecteur (CSO): wat de software afdwingt, wat het project heeft geverifieerd en wat u zelf moet testen en beheren.*

## Wat de software afdwingt

| Gebied | Gedrag |
|---|---|
| Authenticatie | Inloggen staat standaard aan. De dienst start niet zonder sleutels voor tokenverificatie, en start in productie niet met uitgeschakelde aanmelding. Aanmelden via een identiteitsprovider zoals Microsoft Entra ID; multifactorauthenticatie blijft bij de provider ([aanmeldgids](../../../../operations/sign-in-with-entra.md)) |
| Autorisatie | Op kenmerken gebaseerd beleid op tokenkenmerken. Privéplannen blijven voor iedereen buiten elke aggregatie; vertrouwelijkheidsmarkeringen verbergen plannen boven het niveau van de aanvrager, beheerders inbegrepen |
| Noodtoegang | Toegang tot privéplannen vereist geschiktheid, een vermelde reden, hooguit vier uur, en een tweede persoon die wordt geïnformeerd; elk verzoek wordt vastgelegd |
| Sessies | Time-out bij inactiviteit (standaard 30 minuten) en een harde sessielimiet |
| Transport en browser | Beveiligingsheaders, een strikte Content Security Policy, een CORS-toelatingslijst, snelheidsbeperking per aanvrager |
| Uitgaand | Door gebruikers opgegeven bestemmingen zijn beperkt tot openbare `https`-adressen |
| Uploads | De inhoud moet overeenkomen met het opgegeven type; programma's worden geweigerd; een optionele `clamd`-scan sluit bij een fout |
| Audit | Het auditspoor weigert wijziging en verwijdering in de database |
| Bevroren records | Uitgegeven bestuurspakketten en beoordelingen van leveringszekerheid zijn via een databasetrigger onveranderlijk of alleen toe te voegen |
| Toeleveringsketen | `cargo deny`, een software bill of materials, bouwherkomst bij de release, een [beveiligingsbeleid](../../../../SECURITY.md) |

## Wat niet is gedaan

- Geen penetratietest, audit door derden, onafhankelijke codebeoordeling of certificering.
- Geen meegeleverde malwarescanner; de `clamd`-koppeling is alleen getest met een vervanger.
- Het auditspoor kan door een databasebeheerder worden omzeild; er is geen hashketen.
- Vertrouwelijkheidsmarkeringen verbergen en labelen; ze versleutelen niet, en een markering wordt niet in gegenereerde bestanden gestempeld.
- Sessies staan in het geheugen tenzij Redis is geconfigureerd.
- Het hulpmiddel voor betrokkenen vindt verwijzingen, geen namen die in vrije tekst, geüploade bestanden of back-ups zijn getypt.

## Beheer

Back-ups hebben een geteste herstelprocedure, optionele versleuteling en vastgestelde hersteldoelen ([beheerpakket](../../../../operations/README.md)). Het [draaiboek](../../../../operations/runbook.md) geeft de technische stappen bij incidenten; het proces voor incidenten en datalekken is aan u. Meld een kwetsbaarheid privé zoals [SECURITY.md](../../../../SECURITY.md) beschrijft; de doelstelling is bevestiging binnen 5 werkdagen, geen garantie.

## Besluiten die bij u liggen

Wie de penetratietest uitvoert en wanneer; de voorwaardelijke toegang en rolkoppeling van de identiteitsprovider; voor welk markeringsniveau deze implementatie is getoetst; de scanner achter de koppeling voor bijlagen; hoe vaak toegang opnieuw wordt gecertificeerd.

## Lees verder

[Zekerheidschecklist](../../../../governance/assurance-checklist.md) · [Weg naar productie](../../path-to-production/index.md) · [Gegevensstroom](../../../../governance/data-flow.md)
