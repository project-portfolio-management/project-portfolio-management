# Voor de operationeel directeur

*Operationeel directeur (COO): wat u dagelijks aanstuurt, wat de software voor de uitvoering doet en wat het kost om haar te beheren.*

## Wat het voor de operatie doet

- **Leveringsplanning en -beheersing:** plannen, taken, afhankelijkheden, een berekening van het kritieke pad, mijlpalen over plannen heen, sprints en fasen, burndown en doorstroomstatistieken.
- **Capaciteit:** toewijzingen, beschikbaarheid, verlof en niet-werkperioden, resourcerollen, afgeleide werklast en bezettingsgraad. (Een capaciteitsheatmap per vaardighedenpool en maand voor het bestuurspakket is niet gebouwd: `DP-10`.)
- **Tijd en kosten:** tijdregistraties, vastgelegde tijd, kostentarieven (beperkt tot financiën), budgetten en afgeleide kosten.
- **Governancestroom:** issueregister, wijzigingsverzoeken met functiescheiding (de aanvrager kan niet goedkeuren), goedkeuringen, beoordelingen, dossiers voor nieuwe medewerkers, overplaatsingen en vertrekkers.
- **Uitzonderingen:** een rode of oranjerode leveringszekerheid opent een uitzondering waarvoor een oorzaak, het benodigde besluit, een benoemde eigenaar en een uiterste beslisdatum binnen 48 uur nodig zijn (termijn configureerbaar, 1 tot 720 uur). Overschrijdingen verschijnen in het bestuurspakket en in `GET /api/exceptions`.

## Wat het kost om te beheren

| Behoefte | Details |
|---|---|
| Dienst | Eén Rust-dienst, PostgreSQL 18, optioneel Redis (productiecache, optioneel gedeelde sessies), een SvelteKit-interface |
| Draaiboek | [beheerpakket](../../../../operations/README.md): implementatie, draaiboek, back-up en herstel, hersteldoelen, upgrade en migratie |
| Geplande taken | Niets plant zichzelf in. Opschoning van bewaartermijnen, meldingen van uitzonderingsoverschrijdingen en dergelijke taken moeten door uw beheerders op een planner worden gezet |
| Ondersteuning | Eén beheerder, geen SLA; begroot uw eigen eerstelijnsondersteuning |

## Ken de grenzen

- Een plan van enkele duizenden taken is traag te plannen; het volledige antwoord wordt boven 5.000 taken geweigerd tenzij erom wordt gevraagd (`?full=true` of `?only=critical`). Cijfers en kanttekeningen staan in [`BENCHMARKS.md`](../../../../BENCHMARKS.md); ze geven een ordegrootte van één machine, geen capaciteit.
- Voor een groot deel van de werkruimte bestaan schermen, maar **nog niet** voor wijzigingsverzoeken, issues, pakketten of leveringszekerheid; die zijn vandaag alleen via de API beschikbaar.

## Besluiten die bij u liggen

Wie elke uitzondering beheert, wie het maandelijkse pakket uitgeeft (ritme en herinneringen zijn `DP-12`, niet gebouwd), en welke beoordelingscadans de uitzonderingsregel voedt.

## Lees verder

[Beheerpakket](../../../../operations/README.md) · [Draaiboek](../../../../operations/runbook.md) · [Hersteldoelen](../../../../operations/recovery-objectives.md)
