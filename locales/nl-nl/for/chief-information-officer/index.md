# Voor de CIO

*Chief Information Officer (CIO): integratie, identiteit, gegevens, hosting en de informatie die u zult moeten overleggen.*

## Identiteit en toegang

- Inloggen wordt **standaard afgedwongen**; de dienst weigert te starten zonder sleutels voor tokenverificatie, en in productie met uitgeschakelde aanmelding.
- Aanmelden via een identiteitsprovider (waaronder Microsoft Entra ID); multifactorauthenticatie blijft bij de provider ([aanmeldgids](../../../../operations/sign-in-with-entra.md)).
- Autorisatie gebruikt tokenkenmerken (bijvoorbeeld `access`, `directorpack`, `confidence`) met een kenmerkenbeleid. Privéplannen zijn voor elke aggregatie verborgen; vertrouwelijkheidsmarkeringen verbergen plannen boven het niveau van de aanvrager, beheerders inbegrepen.
- Noodtoegang tot privéplannen vereist geschiktheid, een vermelde reden, hooguit vier uur, en een tweede persoon die wordt geïnformeerd.
- Beheerders kunnen permanente toegangsrechten opvragen (een hulpmiddel voor hercertificering) en de gegevens van één persoon exporteren of pseudonimiseren.

## Gegevens en integratie

| Onderwerp | Stand van zaken |
|---|---|
| Opgeslagen gegevens | Records van plannen, taken, risico's, budgetten, mijlpalen, tijd en kosten; personen zijn ondoorzichtige verwijzingen, geen namen. Per ontwerp geen patiëntgegevens en geen bijzondere categorieën van gegevens |
| Inventaris | Elke kolom met persoonsgegevens staat in de [inventaris van persoonsgegevens](../../../../governance/personal-data-inventory.csv); een test houdt die in overeenstemming met het schema |
| Interfaces | REST met een afgeleid OpenAPI-document; bulkimport en -export; agendafeeds; uitgaande gebeurtenissen en webhooks alleen naar openbare `https`-bestemmingen |
| Back-ups | Gescripte back-up met een geteste herstelprocedure, optionele versleuteling, vastgestelde hersteldoelen ([back-up en herstel](../../../../operations/backup-and-restore.md)) |
| Hosting | Containerimages en `compose.yaml`; de software verricht zelf geen doorgifte, dus de locatie van de gegevens is uw keuze van hosting |

## Hiaten die u moet dichten

- Penetratietest, onafhankelijke codebeoordeling en certificering: geen uitgevoerd.
- Bewaartermijnen worden maar gedeeltelijk afgedwongen; opschoning moet worden ingepland (zie het [bewaarschema](../../../../governance/retention-schedule.md)).
- Het hulpmiddel voor betrokkenen vindt verwijzingen, geen namen die in vrije tekst of geüploade bestanden zijn getypt.
- Het auditspoor is niet manipulatiebestendig tegenover een databasebeheerder.

## Lees verder

[Gegevensstroomdiagram](../../../../governance/data-flow.md) · [Zekerheidschecklist](../../../../governance/assurance-checklist.md) · [Implementatie](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
