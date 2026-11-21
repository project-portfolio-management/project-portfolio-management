# Pour la direction de la stratégie (CSO)

*Direction de la stratégie (CSO) : comment le logiciel relie la stratégie au travail, ce qu'il vous montre, et ce qu'il ne sait pas encore faire.*

## Ce qu'il apporte à la stratégie

- **Un seul registre de tout ce qui est financé.** Les portefeuilles, programmes, projets et produits forment un seul enregistrement récursif, de sorte que l'ensemble du patrimoine se lit comme un arbre ([finalité et vision](../../../../spec/01-purpose-and-vision.md)).
- **Détection des doublons.** Le rapprochement explicable montre lorsque deux parties de l'organisation ont lancé la même initiative, avant que les deux ne soient financées.
- **Finalités stratégiques et carte du portefeuille.** Vous marquez des objectifs comme finalités stratégiques et désignez une finalité principale pour chaque programme. `GET /api/portfolio-map` et le dossier mensuel de direction regroupent les programmes par finalité principale. Un programme apparaît une seule fois, avec ses autres finalités nommées ; un programme sans finalité principale est listé sous « aucune finalité stratégique », et les totaux ne sont jamais comptés deux fois.
- **La confiance de réalisation à côté de l'état calculé.** Les personnes expriment leur confiance dans la réalisation ; le logiciel l'affiche à côté du rouge/orange/vert calculé et signale un écart de deux niveaux. Un plan que personne n'a évalué affiche « non évalué », jamais vert.
- **Un dossier mensuel figé.** Ce qui a été dit aux dirigeants est conservé exactement tel que publié, avec la tendance mesurée par rapport au dossier publié précédemment.

## Ce qu'il ne fait pas encore

- Le dossier ne comprend pas encore les décisions de revue de jalon, le financement par exercice, le graphique de fièvre des tampons, la valeur acquise, les bénéfices validés ni la carte thermique de capacité (de `DP-5` à `DP-10` dans [`tasks.md`](../../../../tasks.md)). Le dossier indique quelles sections ne sont pas construites.
- « Programme » désigne un plan de type `Program`, faute d'autre définition dans les données ; les finalités « de premier rang » sont celles que vous marquez, car les objectifs n'ont pas de hiérarchie.
- Il n'y a pas de planification par scénarios, ni encore d'écran pour la carte ou le dossier : ils sont disponibles sous forme de données.
- Il n'est pas certifié, et il n'a qu'un seul mainteneur ([chemin vers la production](../../path-to-production/index.md)).

## Les décisions qui vous reviennent

Quels objectifs sont des finalités stratégiques ; la finalité principale de chaque programme ; qui exprime la confiance de réalisation et qui l'évalue de manière indépendante ; combien de mois de tendance les dirigeants doivent voir.

## À lire ensuite

[Feuille de route](../../../../spec/15-roadmap.md) · [Questions ouvertes](../../../../spec/16-open-questions.md) · [Périmètre](../../../../spec/02-scope.md)
