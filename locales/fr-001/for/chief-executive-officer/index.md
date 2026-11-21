# Pour la direction générale (CEO)

*Direction générale (CEO) : ce qu'est ce logiciel, ce qu'il vous apporte, ce qu'il n'apporte pas, et les décisions qui vous reviennent.*

## De quoi il s'agit, en un paragraphe

Un registre de chaque **plan** que l'organisation finance (portefeuilles, programmes, projets, produits, etc., un seul enregistrement récursif), avec en dessous un espace de gestion de projet (tâches, calendrier, coûts, ressources, risques, problèmes, demandes de changement, gouvernance). Il repère les initiatives en double par un rapprochement explicable : lorsque deux services lancent le même travail, cela apparaît avant que les deux ne soient financés. Voir [finalité et vision](../../../../spec/01-purpose-and-vision.md).

## Ce sur quoi vous pouvez compter

- **Un seul endroit où demander « que faisons-nous ? ».** Les plans s'emboîtent à n'importe quelle profondeur ; les plans privés restent hors de tous les totaux et décomptes, pour tout le monde, selon une règle que des tests vérifient.
- **Un dossier mensuel de direction** rédigé une fois, publié par une personne désignée, puis **figé par la base de données** : un dossier publié ne peut pas être modifié ; une correction est une nouvelle version republiée, avec une note. Aujourd'hui, il présente la carte du portefeuille par finalité stratégique, la confiance de réalisation, les exceptions, les problèmes et les demandes de changement.
- **Une confiance de réalisation exprimée par des personnes, affichée à côté de l'état calculé, sans jamais être mêlée à lui.** Un plan que personne n'a évalué affiche « non évalué », jamais vert. Une évaluation orange-rouge ou rouge ouvre une exception avec un délai de 48 heures (configurable) ; un délai dépassé apparaît dans le dossier.
- **Les décisions laissent une trace.** Chaque modification est auditée, la piste d'audit ne peut pas être altérée par le service, et une demande de changement ne peut pas être approuvée par la personne qui l'a déposée.

## Ce qu'il ne vous apporte pas encore

- Le dossier ne comprend **pas** encore les décisions de revue de jalon, le financement et les montants par exercice, le graphique de fièvre des tampons, la valeur acquise, les bénéfices validés ni la carte thermique de capacité. Ils figurent de `DP-5` à `DP-10` dans [`tasks.md`](../../../../tasks.md), et le dossier indique quelles sections ne sont pas encore construites au lieu de laisser un blanc.
- Pas de PDF ni d'écran pour le dossier : il est fourni sous forme de données (`DP-11`).
- Il n'est **ni certifié ni audité** par personne. Ce qu'exigerait une utilisation en production, point par point et avec une estimation pour chacun, figure sur la page [chemin vers la production](../../path-to-production/index.md).
- Un seul mainteneur, aucun accord de niveau de service ([gouvernance](../../../../GOVERNANCE.md)).

## Les décisions qui vous reviennent

1. Faut-il lancer un pilote, et sur quelle partie du portefeuille.
2. Qui est le **sponsor** pour la règle d'exception de 48 heures, et qui peut publier le dossier mensuel.
3. Faut-il financer les lacunes ci-dessus (évaluation, test d'intrusion, dispositif de support) ou les accepter.

## À lire ensuite

[Chemin vers la production](../../path-to-production/index.md) · [Feuille de route](../../../../spec/15-roadmap.md) · [Questions ouvertes](../../../../spec/16-open-questions.md)
