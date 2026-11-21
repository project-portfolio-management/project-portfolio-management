# Governance, risico en naleving

*Voor de mensen die governance, risico en naleving (GRC) uitvoeren: waarvoor de software u helpt bewijs te leveren, wat ze afdwingt en wat u zelf moet aanleveren. Geen juridisch advies.*

## Governance

- **Besluiten worden vastgelegd.** Wijzigingsverzoeken vereisen een benoemde goedkeurder die niet de aanvrager is, een vermelde reden van minstens 10 tekens, en worden per actor vastgelegd; de database weigert een goedkeuring zonder beslisser en reden.
- **Een bevroren maandelijks bestuurspakket.** Eenmaal opgesteld, door een persoon uitgegeven, onveranderlijk zodra het is uitgegeven; correcties zijn een heruitgave met een toelichting. Leestoegang is per doelgroep.
- **Leveringszekerheid en uitzonderingen.** Opgegeven zekerheid is alleen toe te voegen en wordt naast de berekende status getoond, nooit ermee vermengd. Een rode of oranjerode beoordeling opent een uitzondering met een klok van 48 uur (configureerbaar) die alleen een vastgelegd besluit sluit.
- **Projectgovernance van de software zelf:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [wijzigingsbeheer](../../../../spec/18-change-control.md), [AI-verklaring](../../../../AI_STATEMENT.md).

## Risico

- Risico's, issues en wijzigingsverzoeken per plan; een issueregister over leesbare plannen heen.
- Ontbrekende gegevens worden getoond als "niet beoordeeld", nooit als groen.
- De eigen openstaande risico's van het project (geen penetratietest, gedeeltelijke bewaartermijnen, één beheerder, conceptvertalingen) staan op de pagina [weg naar productie](../../path-to-production/index.md), punt voor punt met een schatting per punt.

## Naleving

| Gebied | Waar | Stand |
|---|---|---|
| Gegevensbescherming (de toepasselijke wet; [§12.2a](../../../../spec/12-compliance.md) koppelt AVG, Britse GDPR, LGPD, PIPL, DPDP, PIPEDA, CCPA en andere) | [DPIA](../../../../governance/dpia.md), [verwerkingsregister](../../../../governance/record-of-processing.md), [inventaris van persoonsgegevens](../../../../governance/personal-data-inventory.csv), [bewaarschema](../../../../governance/retention-schedule.md), [gegevensstroom](../../../../governance/data-flow.md) | Vooraf ingevuld; `‹buyer›`-markeringen zijn van u; bewaartermijnen slechts gedeeltelijk afgedwongen |
| Britse NHS DTAC (één nationaal stelsel, een uitgewerkt voorbeeld; andere stelsels beginnen bij de [zekerheidschecklist](../../../../governance/assurance-checklist.md)) | [DTAC-checklist](../../../../governance/dtac-checklist.md) | Sectie voor sectie beantwoord; rijen voor koper en leverancier open |
| Toegankelijkheid | [verklaring](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA geautomatiseerd; geen handmatige test |
| Taalverplichtingen | [taalverklaring](../../../../governance/language-statement.md) | Vertalingen zijn concepten |
| ISO/IEC 27001 | [nalevingsspecificatie](../../../../spec/12-compliance.md) | Beheersmaatregelen aan de kant van de implementatie; niet gecertificeerd |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [nalevingsspecificatie](../../../../spec/12-compliance.md) §12.4 | Niet van toepassing (geen patiëntgegevens); vermeld, niet gefabriceerd |

## Bewijs en zekerheid

De [zekerheidschecklist](../../../../governance/assurance-checklist.md) somt 14 punten op waar een beoordelaar naar zal vragen, met eigenaar en stand. De meeste staan open en zijn van de implementeerder: penetratietest, toegankelijkheidsaudit, calamiteitenherstelproef, zekerheid over de hosting, configuratie van de identiteitsprovider, hercertificering van toegang, incidentproces, archiefbeheer. Het auditspoor weigert wijziging en verwijdering in de database, maar een databasebeheerder kan het omzeilen; er is geen hashketen.

## Besluiten die bij u liggen

Rechtsgrond en bewaartermijnen; uitzonderingsvenster; markeringsniveau voor de implementatie; wie toegang beoordeelt en hoe vaak; of een procedure voor juridische bewaring (legal hold) nodig is (er is er geen gebouwd).

## Lees verder

[Index van het governancepakket](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Nalevingsspecificatie](../../../../spec/12-compliance.md)
