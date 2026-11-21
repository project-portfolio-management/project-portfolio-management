# Weg naar productie

*Wat "productie" voor deze software betekent en hoe ver de software daarvan af staat, punt voor punt.*

"Productie" betekent hier dat een organisatie de software voor echt werk gebruikt. Echte plannen, namen van echte mensen en echte besluiten hangen ervan af, een storing of een lek heeft gevolgen, en iemand is er verantwoordelijk voor. Bij een grote zorginstelling of een overheidsdepartement betekent productie ook dat de eigen zekerheidstoetsing van de organisatie is doorlopen: een beveiligingstest, een gegevensbeschermingseffectbeoordeling (DPIA), een toegankelijkheidsaudit en, in de Britse National Health Service, de Digital Technology Assessment Criteria (DTAC).

## Waar het nu staat

- Er wordt geen certificering, goedkeuring of audit beweerd. Deze pagina is de eigen schatting van het project.
- "Medisch" betekent een **niet-klinisch** hulpmiddel voor portfolio en projecten: geen patiëntdossiers en geen klinische beslisondersteuning.
- Het project raadt aan te beginnen met een pilot op niet-klinische, niet-geheime gegevens, uitgevoerd door een organisatie die haar eigen zekerheidstoetsing afrondt.

## Gereedheid, punt voor punt

Elk cijfer is de schatting van het project op 13 november 2026, geen meting. Elke schuifregelaar begint bij die schatting. Wie een schuifregelaar verplaatst, legt tijdens het lezen een eigen mening vast; er wordt niets opgeslagen of verzonden.

- **Onafhankelijke beveiligingszekerheid** — 0%. Geen penetratietest, geen audit door derden en geen certificering. Een koper of een leverancier moet ze laten uitvoeren (RG-13).
- **Toegankelijkheid** — 40%. Op elke route wordt bij elke wijziging een geautomatiseerde WCAG 2.2 AA-audit uitgevoerd. Er is geen handmatige test gedaan, en geen test met hulptechnologie of met mensen met een beperking (RG-13).
- **Verzoeken van betrokkenen** — 50%. Beheerders kunnen een persoon exporteren en pseudonimiseren over elke verwijzing in het schema, met eerst een voorbeeldweergave (RG-6). Namen die in titels, opmerkingen of documenten zijn getypt, geüploade bestanden, auditmomentopnamen en back-ups worden niet automatisch gevonden.
- **Bewaartermijnen** — 35%. Een opschoning, standaard als proefrun, omvat meldingen, bezorglogboeken, idempotentiesleutels en trend- en adoptiemomentopnamen, met termijnen die per implementatie worden ingesteld (RG-4). Plannen, taken, tijdregistraties, documenten en het auditspoor blijven bewaard totdat een beheerder ze verwijdert.
- **Vertrouwelijkheidsmarkeringen** — 60%. Een configureerbare classificatie verbergt plannen boven het niveau van de aanvrager, beheerders inbegrepen, en labelt elk antwoord (RG-7). De markering wordt niet in gegenereerde bestanden gestempeld, en het niveau is slechts zo goed als het kenmerk van de identiteitsprovider.
- **Veiligheid van bijlagen** — 50%. De inhoud moet overeenkomen met het opgegeven type, programma's worden geweigerd en een optionele `clamd`-scan sluit bij een fout (RG-8). Er wordt geen scanner meegeleverd, en de koppeling is alleen getest met een vervanger.
- **Toegangsbeoordeling en noodtoegang** — 45%. Een rapport van permanente rechten ondersteunt hercertificering (RG-9), en noodtoegang is tijdelijk en vereist een vermelde reden (RG-10). Het rapport toont wat is vastgelegd, niet wat de identiteitsprovider afdwingt; geen enkele route legt het oordeel van een beoordelaar vast; en een melding van noodtoegang kan iemand noemen die niets ontvangt terwijl het recht nog geldt.
- **Prestaties en capaciteit** — 40%. Een belasting- en duurtest op één machine staat in [`BENCHMARKS.md`](../../../BENCHMARKS.md): gewone verzoeken worden in tientallen milliseconden beantwoord, een plan van ongeveer 12.700 taken is traag te plannen en het proces groeit tot ongeveer 1 GB. Er is geen capaciteitsplan, geen test met veel gebruikers en geen run over meerdere machines.
- **Sessies over meerdere frontendservers** — 70%. Sessies staan in het geheugen tenzij Redis is geconfigureerd (RG-12); met Redis hangt het inloggen ervan af.
- **Integriteit van het auditspoor** — 50%. Schrijfacties worden vastgelegd en een databasetrigger weigert wijzigingen en verwijderingen, met vermelde uitzonderingen (RG-3). Een databasebeheerder kan de trigger verwijderen, en er is geen hashketen.
- **Back-up en herstel** — 80%. Een herstel wordt in de continue integratie getest, onversleuteld en versleuteld, tegen vastgestelde hersteldoelen (RG-5). Versleuteling is optioneel, en de proef draait op de eigen CI van het project, niet op de hosting van de gebruiker.
- **Ondersteuning en continuïteit** — 10%. Eén beheerder en geen serviceniveauovereenkomst ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Vertalingen** — 20%. Tekst in andere talen dan het Engels is machinaal geschreven totdat een moedertaalspreker haar beoordeelt, en wettelijke taalverplichtingen vereisen beoordeelde tekst ([taalverklaring](../../../governance/language-statement.md)).
- **Klinisch gebruik** — 0%. Er is geen klinisch veiligheidsdossier opgesteld, en klinisch gebruik valt buiten het bereik.

## Wat een eerste implementatie moet doen

1. Werk de [zekerheidschecklist](../../../governance/assurance-checklist.md) af, te beginnen met een penetratietest en een toegankelijkheidsaudit.
2. Vul elke rij "Buyer to complete" in de [DTAC-checklist](../../../governance/dtac-checklist.md) en de markeringen in de [DPIA](../../../governance/dpia.md) in.
3. Spreek bewaartermijnen af met de archiefbeheerder en plan de opschoning in.
4. Schakel versleutelde back-ups in en voer de herstelproef uit op de eigen hosting van de implementatie.
5. Houd de gegevens niet-klinisch en onder het markeringsniveau waarvoor de implementatie is getoetst.
6. Plan voor de bovenstaande hiaten, of financier het wegnemen ervan.

Het bewijs voor elk punt staat in de [beoordeling](../../../governance/production-readiness.md).
