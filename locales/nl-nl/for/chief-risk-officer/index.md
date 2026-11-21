# Voor de risicodirecteur

*Risicodirecteur (CRO): de risico's die de software u helpt te zien, en de risico's van het gebruik ervan.*

## Risico's die ze u helpt beheersen

- **Risico's, issues en wijzigingsverzoeken** per plan vastgelegd; een issueregister over leesbare plannen heen; wijzigingsverzoeken met functiescheiding.
- **Leveringszekerheid en uitzonderingen:** opgegeven zekerheid gescheiden gehouden van de berekende RAG-status, gemarkeerd wanneer ze twee stappen uiteenlopen, met een alleen-toevoegen-geschiedenis, en een uitzonderingsklok van 48 uur die niet opnieuw begint wanneer een beoordeling verslechtert en alleen wordt gesloten door een vastgelegd besluit.
- **Eerlijke afwezigheid:** ontbrekende gegevens verschijnen als "niet beoordeeld" of als null met een reden, nooit als groen of nul.
- **Een bevroren, geversioneerd maandelijks pakket,** zodat wat aan bestuurders is verteld later precies zo kan worden getoond als het is uitgegeven.

## Risico's van het gebruik (het eigen register van het project)

| Risico | Stand |
|---|---|
| Geen penetratietest, audit door derden of certificering | Open: door de koper te laten uitvoeren ([zekerheidschecklist](../../../../governance/assurance-checklist.md)) |
| Bewaartermijnen voor slechts een deel van de gegevens afgedwongen | Open; opschoning moet worden ingepland |
| Het hulpmiddel voor betrokkenen doorzoekt geen vrije tekst, bijlagen of back-ups | Open |
| Het auditspoor kan door een databasebeheerder worden omzeild; geen hashketen | Vermeld in de [DPIA](../../../../governance/dpia.md) |
| Vertrouwelijkheidsmarkeringen verbergen en labelen, ze versleutelen niet | Vermeld |
| Het scannen van bijlagen vereist een scanner die de implementeerder draait | Open |
| Eén beheerder, geen SLA | Open |
| Vertalingen zijn concepten | Open |
| Alleen niet-klinisch; geen klinisch veiligheidsdossier | Voer geen klinische inhoud in |

## Beheersmaatregelen die de software afdwingt

Standaard inloggen, snelheidsbeperking, strikte beveiligingsheaders, een time-out bij inactiviteit van de sessie (30 minuten) en een harde sessielimiet, uitgaande bestemmingen beperkt tot openbare `https`, bijlagen met gecontroleerde handtekening, onveranderlijke auditrijen, noodtoegang met audit. Elk wordt gedekt door een test die wordt aangehaald in [weg naar productie](../../path-to-production/index.md).

## Besluiten die bij u liggen

De risicobereidheid voor een pilot; het uitzonderingsvenster (standaard 48 uur); voor welk markeringsniveau de implementatie is getoetst; wie de toegangslijst beoordeelt en hoe vaak.

## Lees verder

[Weg naar productie](../../path-to-production/index.md) · [DPIA](../../../../governance/dpia.md) · [Naleving](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
