# För riskchefen

*Riskchef (CRO): de risker programvaran hjälper dig se, och riskerna med att använda den.*

## Risker den hjälper dig hantera

- **Risker, problem och ändringsbegäranden** registrerade per plan; ett problemregister över läsbara planer; ändringsbegäranden med funktionsseparation.
- **Leveranssäkerhet och undantag:** angiven säkerhet hålls isär från beräknad RAG-status, flaggas när de skiljer sig två steg, med en historik där man bara lägger till, och en undantagsklocka på 48 timmar som inte startar om när ett omdöme försämras och som bara stängs av ett registrerat beslut.
- **Ärlig frånvaro:** saknade data visas som "inte bedömd" eller null med en orsak, aldrig som grönt eller noll.
- **Ett fruset, versionshanterat månadspaket,** så att det som direktionen fick veta senare kan visas exakt som det utfärdades.

## Risker med att använda den (projektets eget register)

| Risk | Läge |
|---|---|
| Inget penetrationstest, ingen tredjepartsgranskning eller certifiering | Öppen: köparen ska beställa ([checklista för kvalitetssäkring](../../../../governance/assurance-checklist.md)) |
| Gallring tillämpas bara för en del av data | Öppen; rensningen måste schemaläggas |
| Verktyget för registrerades rättigheter söker inte i fritext, bilagor eller säkerhetskopior | Öppen |
| Granskningsloggen kan sättas ur spel av en databasadministratör; ingen hashkedja | Anges i [DPIA](../../../../governance/dpia.md) |
| Hanteringsmarkeringar döljer och märker, de krypterar inte | Anges |
| Skanning av bilagor kräver en skanner som driftsättaren kör | Öppen |
| En enda underhållare, inget servicenivåavtal | Öppen |
| Översättningar är utkast | Öppen |
| Endast icke-klinisk; inget kliniskt säkerhetsunderlag | Skriv inte in kliniskt innehåll |

## Kontroller som programvaran tillämpar

Inloggning som standard, hastighetsbegränsning, strikta säkerhetsrubriker, en tidsgräns för overksamma sessioner (30 minuter) och en hård sessionsgräns, utgående destinationer begränsade till offentliga `https`, signaturkontrollerade bilagor, oföränderliga granskningsrader, nödåtkomst med granskning. Var och en täcks av ett test som anges i [Vägen till produktion](../../path-to-production/index.md).

## Beslut som är dina

Riskaptit för en pilot; undantagsfönstret (standard 48 timmar); vilken markeringsnivå driftsättningen är kvalitetssäkrad för; vem som granskar åtkomstlistan och hur ofta.

## Läs vidare

[Vägen till produktion](../../path-to-production/index.md) · [DPIA](../../../../governance/dpia.md) · [Efterlevnad](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
