# Voor de juridisch directeur

*Juridisch directeur (CLO): licentie, gegevensbescherming, toegankelijkheid, taalverplichtingen en wat het project niet garandeert. Dit is geen juridisch advies.*

## Licentie

Aangeboden onder één van MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only of GPL-3.0-only, of onder aangepaste voorwaarden in overleg. De keuze en de teksten staan in [`LICENSE.md`](../../../../LICENSE.md) en `LICENSE/`; een test houdt elk manifest daarmee consistent.

## Gegevensbescherming (de wet die voor u geldt)

Geschreven voor elke wet op gegevensbescherming: de [nalevingsspecificatie §12.2a](../../../../spec/12-compliance.md) koppelt de gangbare wetten per regio (de AVG en de Britse GDPR, de LGPD, de PIPL, de DPDP Act, PIPEDA, de CCPA en andere) aan wat elk hier aanstuurt, uit algemene kennis en niet getoetst aan de tekst van elke wet. Vooraf ingevulde documenten die u aanpast; elke `‹buyer›`-markering is een besluit dat bij u ligt (verwerkingsverantwoordelijke, rechtsgrond, bewaartermijnen, verwerkers, goedkeuring):

- [DPIA-sjabloon](../../../../governance/dpia.md) en [verwerkingsregister](../../../../governance/record-of-processing.md)
- [Inventaris van persoonsgegevens](../../../../governance/personal-data-inventory.csv): elke kolom die persoonsgegevens kan bevatten
- [Bewaarschema](../../../../governance/retention-schedule.md): voorstellen, met wat de code vandaag afdwingt
- [Gegevensstroomdiagram](../../../../governance/data-flow.md)

De software bewaart personen als ondoorzichtige verwijzingen plus vastgelegde tijd, beschikbaarheid en kostentarieven, wat aan monitoring grenst en dus een DPIA rechtvaardigt. Ze biedt een export voor betrokkenen en wissing door pseudonimisering of, als de implementatie dat kiest, verwijdering (alleen beheerders, eerst een voorbeeldweergave; WW-1.8). Bevroren bestuurspakketten en de alleen-toevoegen-geschiedenis van zekerheid worden ontdaan van de persoon die ze heeft opgesteld of beoordeeld, nooit verwijderd, zodat een verzoek tot wissing kan worden ingewilligd zonder het record te breken. Ze vindt **geen** namen die in vrije tekst zijn getypt, en back-ups bevatten records die de applicatie heeft verwijderd.

## Kaders

HIPAA en de Amerikaanse certificeringsstelsels voor gezondheidsgegevens zijn **niet van toepassing** (geen patiëntgegevens); het stelsel voor klinische hulpmiddelen is niet van toepassing. De [DTAC-checklist](../../../../governance/dtac-checklist.md) beantwoordt de Digital Technology Assessment Criteria van de Britse NHS, één nationaal stelsel dat als uitgewerkt voorbeeld wordt bewaard; voor stelsels van andere landen en sectoren (ISO/IEC 27001, SOC 2, nationale cloud- en overheidsstelsels) begint u bij de [zekerheidschecklist](../../../../governance/assurance-checklist.md). Er is niets gecertificeerd.

## Toegankelijkheid en taal

WCAG 2.2 AA wordt op elke route automatisch getest; er is geen handmatige test of test met hulptechnologie gedaan ([verklaring](../../../../governance/accessibility-statement.md)). Welshe en andere vertalingen zijn concepten die op een professionele vertaler wachten ([taalverklaring](../../../../governance/language-statement.md)); relevant als u wettelijke taalverplichtingen hebt.

## Due diligence van de leverancier

Eén beheerder, geen contractuele ondersteuning, geen garantie beweerd, AI-hulpmiddelen gebruikt bij de ontwikkeling en bekendgemaakt in de [AI-verklaring](../../../../AI_STATEMENT.md). Het product bevat geen AI-model. Melding van kwetsbaarheden: [SECURITY.md](../../../../SECURITY.md).

## Besluiten die bij u liggen

Rechtsgrond en bewaartermijnen; of u een procedure voor juridische bewaring (legal hold) nodig hebt (er is er geen gebouwd); contract- en ondersteuningsvoorwaarden met de beheerder.

## Lees verder

[Zekerheidschecklist](../../../../governance/assurance-checklist.md) · [Index van het governancepakket](../../../../governance/README.md)
