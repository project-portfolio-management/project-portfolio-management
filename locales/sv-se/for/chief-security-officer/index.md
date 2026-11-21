# För säkerhetschefen

*Säkerhetschef (CSO): vad programvaran tillämpar, vad projektet har verifierat och vad du själv måste testa och driva.*

## Vad programvaran tillämpar

| Område | Beteende |
|---|---|
| Autentisering | Inloggning är på som standard. Tjänsten startar inte utan nycklar för tokenverifiering, och startar inte i produktion med inloggningen avstängd. Inloggning via en identitetsleverantör som Microsoft Entra ID; flerfaktorsautentisering stannar hos leverantören ([inloggningsguide](../../../../operations/sign-in-with-entra.md)) |
| Auktorisering | Attributbaserad policy på tokenattribut. Privata planer hålls utanför alla aggregat för alla; hanteringsmarkeringar döljer planer över en anropares behörighetsnivå, administratörer inräknade |
| Nödåtkomst | Åtkomst till privata planer kräver behörighet, en angiven orsak, högst fyra timmar och att en andra person underrättas; varje begäran granskas |
| Sessioner | Tidsgräns för overksamhet (30 minuter som standard) och en hård sessionsgräns |
| Transport och webbläsare | Säkerhetsrubriker, en strikt Content Security Policy, en CORS-tillåtelselista, hastighetsbegränsning per anropare |
| Utgående | Destinationer som användaren anger begränsas till offentliga `https`-adresser |
| Uppladdningar | Innehållet måste stämma med den angivna typen; program nekas; en valfri `clamd`-skanning stänger vid fel |
| Granskning | Granskningsloggen nekar ändring och radering i databasen |
| Frysta poster | Utfärdade direktionspaket och bedömningar av leveranssäkerhet är oföränderliga eller bara tillägg via databastrigger |
| Leveranskedja | `cargo deny`, en programvaruförteckning (SBOM), byggproveniens vid utgivning, en [säkerhetspolicy](../../../../SECURITY.md) |

## Vad som inte är gjort

- Inget penetrationstest, ingen tredjepartsgranskning, ingen oberoende kodgranskning eller certifiering.
- Ingen medföljande skanner för skadlig kod; `clamd`-kopplingen testades bara mot en ersättare.
- Granskningsloggen kan sättas ur spel av en databasadministratör; det finns ingen hashkedja.
- Hanteringsmarkeringar döljer och märker; de krypterar inte, och en markering stämplas inte in i genererade filer.
- Sessioner hålls i minnet om inte Redis har konfigurerats.
- Verktyget för registrerades rättigheter hittar referenser, inte namn som skrivits i fritext, uppladdade filer eller säkerhetskopior.

## Att driva den

Säkerhetskopior har en testad återställning, valfri kryptering och angivna återställningsmål ([driftpaketet](../../../../operations/README.md)). [Driftshandboken](../../../../operations/runbook.md) ger de tekniska stegen vid en incident; processen för incidenter och dataintrång är din. Rapportera en sårbarhet privat enligt [SECURITY.md](../../../../SECURITY.md); målen är bekräftelse inom 5 arbetsdagar, inte garantier.

## Beslut som är dina

Vem som utför penetrationstestet och när; identitetsleverantörens villkorade åtkomst och rollmappning; den markeringsnivå som driftsättningen är kvalitetssäkrad för; skannern bakom kopplingen för bilagor; hur ofta åtkomst omcertifieras.

## Läs vidare

[Checklista för kvalitetssäkring](../../../../governance/assurance-checklist.md) · [Vägen till produktion](../../path-to-production/index.md) · [Dataflöde](../../../../governance/data-flow.md)
