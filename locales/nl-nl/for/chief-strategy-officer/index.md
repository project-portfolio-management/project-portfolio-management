# Voor de strategiedirecteur

*Strategiedirecteur (CSO): hoe de software strategie met het werk verbindt, wat ze u toont en wat ze nog niet kan.*

## Wat het voor strategie doet

- **Eén register van alles wat wordt gefinancierd.** Portfolio's, programma's, projecten en producten zijn één recursief record, zodat het hele bezit als een boom kan worden gelezen ([doel en visie](../../../../spec/01-purpose-and-vision.md)).
- **Detectie van dubbelingen.** Verklaarbare matching toont wanneer twee delen van de organisatie hetzelfde initiatief zijn gestart, voordat beide worden gefinancierd.
- **Strategische doelen en de portfoliokaart.** U markeert doelstellingen als strategische doelen en benoemt voor elk programma één primair doel. `GET /api/portfolio-map` en het maandelijkse bestuurspakket groeperen programma's per primair doel. Een programma verschijnt één keer, met zijn overige doelen genoemd; een programma zonder primair doel staat onder "geen strategisch doel", en totalen worden nooit dubbel geteld.
- **Leveringszekerheid naast de berekende status.** Mensen geven hun zekerheid over de oplevering op; de software toont die naast de berekende rood/oranje/groen-status en markeert een verschil van twee stappen. Een plan dat niemand heeft beoordeeld, toont "niet beoordeeld", nooit groen.
- **Een bevroren maandelijks pakket.** Wat de bestuurders is verteld, blijft precies zoals het is uitgegeven, met de trend gemeten ten opzichte van het vorige uitgegeven pakket.

## Wat het nog niet doet

- Het pakket bevat nog geen poortbesluiten, financiering per boekjaar, de bufferkoortscurve, verdiende waarde, gevalideerde baten of de capaciteitsheatmap (`DP-5` tot en met `DP-10` in [`tasks.md`](../../../../tasks.md)). Het pakket vermeldt welke onderdelen niet zijn gebouwd.
- "Programma" betekent een plan van het soort `Program`, omdat de gegevens geen andere definitie hebben; doelen van het "hoogste niveau" zijn wat u markeert, omdat doelstellingen geen hiërarchie hebben.
- Er is geen scenarioplanning en nog geen scherm voor de kaart of het pakket: ze zijn als gegevens beschikbaar.
- Het is niet gecertificeerd en het heeft één beheerder ([weg naar productie](../../path-to-production/index.md)).

## Besluiten die bij u liggen

Welke doelstellingen strategische doelen zijn; het primaire doel van elk programma; wie leveringszekerheid opgeeft en wie haar onafhankelijk beoordeelt; hoeveel maanden trend de bestuurders moeten zien.

## Lees verder

[Roadmap](../../../../spec/15-roadmap.md) · [Open vragen](../../../../spec/16-open-questions.md) · [Reikwijdte](../../../../spec/02-scope.md)
