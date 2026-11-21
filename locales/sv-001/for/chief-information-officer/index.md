# För IT-chefen

*IT-chef (CIO): integration, identitet, data, drift och den information du kommer att behöva lämna ut.*

## Identitet och åtkomst

- Inloggning **tillämpas som standard**; tjänsten vägrar starta utan nycklar för tokenverifiering, och i produktion med inloggning avstängd.
- Inloggning via en identitetsleverantör (däribland Microsoft Entra ID); flerfaktorsautentisering stannar hos leverantören ([inloggningsguide](../../../../operations/sign-in-with-entra.md)).
- Auktorisering använder tokenattribut (till exempel `access`, `directorpack`, `confidence`) med en attributpolicy. Privata planer döljs i alla aggregat; hanteringsmarkeringar döljer planer över en anropares behörighetsnivå, administratörer inräknade.
- Nödåtkomst till privata planer kräver behörighet, en angiven orsak, högst fyra timmar och att en andra person underrättas.
- Administratörer kan lista stående åtkomstbehörigheter (ett hjälpmedel för omcertifiering) och exportera eller pseudonymisera en persons data.

## Data och integration

| Ämne | Ställning |
|---|---|
| Lagrade data | Poster för planer, uppgifter, risker, budgetar, milstolpar, tid och kostnad; personer är ogenomskinliga referenser, inte namn. Inga patientuppgifter och inga särskilda kategorier av uppgifter, enligt konstruktionen |
| Förteckning | Varje kolumn med personuppgifter står i [förteckningen över personuppgifter](../../../../governance/personal-data-inventory.csv); ett test håller den i linje med schemat |
| Gränssnitt | REST med ett härlett OpenAPI-dokument; massimport och massexport; kalenderflöden; utgående händelser och webhooks endast till offentliga `https`-mål |
| Säkerhetskopior | Skriptad säkerhetskopiering med en testad återställning, valfri kryptering och angivna återställningsmål ([säkerhetskopiering och återställning](../../../../operations/backup-and-restore.md)) |
| Drift | Containeravbildningar och `compose.yaml`; programvaran gör inga egna överföringar, så dataplatsen är ditt val av driftsmiljö |

## Brister du måste åtgärda

- Penetrationstest, oberoende kodgranskning och certifiering: inget är gjort.
- Gallring tillämpas bara delvis; rensningen måste schemaläggas (se [gallringsschemat](../../../../governance/retention-schedule.md)).
- Verktyget för registrerades rättigheter hittar referenser, inte namn som skrivits i fritext eller i uppladdade filer.
- Granskningsloggen är inte manipuleringssäker mot en databasadministratör.

## Läs vidare

[Dataflödesdiagram](../../../../governance/data-flow.md) · [Checklista för kvalitetssäkring](../../../../governance/assurance-checklist.md) · [Driftsättning](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
