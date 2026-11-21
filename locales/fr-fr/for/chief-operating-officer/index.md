# Pour la direction des opérations (COO)

*Direction des opérations (COO) : ce que vous pilotez au quotidien, ce que le logiciel apporte à la réalisation, et ce qu'exige son exploitation.*

## Ce qu'il apporte aux opérations

- **Planification et pilotage de la réalisation :** plans, tâches, dépendances, calcul du chemin critique, jalons entre plans, sprints et phases, courbes d'avancement et indicateurs de flux.
- **Capacité :** affectations, disponibilités, congés et périodes non travaillées, rôles de ressources, charge et taux d'utilisation dérivés. (Une carte thermique de capacité par vivier de compétences et par mois pour le dossier de direction n'est pas construite : `DP-10`.)
- **Temps et coûts :** feuilles de temps, temps enregistré, taux de coût (réservés à la finance), budgets et coûts dérivés.
- **Circuit de gouvernance :** registre des problèmes, demandes de changement avec séparation des tâches (le demandeur ne peut pas approuver), approbations, revues, dossiers d'arrivée, de mobilité et de départ.
- **Exceptions :** une confiance de réalisation rouge ou orange-rouge ouvre une exception qui exige une cause, la décision nécessaire, un responsable désigné et une date limite de décision dans les 48 heures (fenêtre configurable, de 1 à 720 heures). Les dépassements apparaissent dans le dossier de direction et dans `GET /api/exceptions`.

## Ce qu'il faut pour l'exploiter

| Besoin | Détail |
|---|---|
| Service | Un service Rust, PostgreSQL 18, Redis facultatif (cache de production, sessions partagées facultatives), une interface SvelteKit |
| Guide d'exploitation | [dossier d'exploitation](../../../../operations/README.md) : déploiement, guide d'exploitation, sauvegarde et restauration, objectifs de reprise, mise à niveau et migration |
| Tâches planifiées | Rien ne se planifie tout seul. La purge de conservation, les avis de dépassement d'exception et les tâches similaires doivent être placés sur un ordonnanceur par vos exploitants |
| Support | Un seul mainteneur, aucun accord de niveau de service ; prévoyez votre propre support de premier niveau |

## Connaître les limites

- Un plan de plusieurs milliers de tâches est lent à planifier ; la réponse complète est refusée au-delà de 5 000 tâches, sauf demande expresse (`?full=true` ou `?only=critical`). Les chiffres et les réserves figurent dans [`BENCHMARKS.md`](../../../../BENCHMARKS.md) ; ils donnent un ordre de grandeur sur une seule machine, pas une capacité.
- Des écrans existent pour une grande partie de l'espace de travail, mais **pas** encore pour les demandes de changement, les problèmes, les dossiers ni la confiance de réalisation ; ceux-ci ne sont aujourd'hui accessibles que par l'API.

## Les décisions qui vous reviennent

Qui est responsable de chaque exception, qui publie le dossier mensuel (le rythme et les rappels relèvent de `DP-12`, non construit), et à quel rythme de revue la règle d'exception alimente.

## À lire ensuite

[Dossier d'exploitation](../../../../operations/README.md) · [Guide d'exploitation](../../../../operations/runbook.md) · [Objectifs de reprise](../../../../operations/recovery-objectives.md)
