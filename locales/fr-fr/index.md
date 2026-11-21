# Gestion de portefeuille de projets

*Un registre des travaux financés, avec détection des doublons et un espace de gestion de projet.*

Project Portfolio Management est un logiciel libre qui sert à enregistrer les plans qu'une organisation finance et à gérer les travaux qu'ils contiennent. Il couvre les tâches, les calendriers, les coûts, les ressources, les risques et la gouvernance. Il compare les plans pour repérer les doublons possibles et montre les éléments de preuve de chaque rapprochement. Lorsqu'un chiffre ne peut pas être calculé, le logiciel en indique la raison.

## Principaux avantages

- **Un registre unique du portefeuille.** Les portefeuilles, programmes, projets et produits utilisent un seul type d'enregistrement, et tout plan peut en contenir un autre. Les rapports s'appuient sur une seule liste commune à tous les services.
- **Détection des doublons.** Le rapprochement compare des plans enregistrés sous des noms différents et montre les éléments de preuve de chaque résultat, afin que les relecteurs puissent agir avant que les deux ne soient financés.
- **Chiffres traçables.** Un chiffre manquant est présenté avec la raison de son absence. Un plan sans évaluation est présenté comme « non évalué ».
- **Piste d'audit.** Chaque modification est auditée. La personne qui dépose une demande de changement ne peut pas l'approuver.
- **Contrôle d'accès.** L'authentification est activée par défaut, et le service ne s'exécute pas en production lorsqu'elle est désactivée.
- **Déploiement et licences.** Le logiciel est auto-hébergé et proposé sous cinq licences libres. Les travaux peuvent être importés depuis Jira, Asana et Microsoft Project.

[Voir tous les avantages](topics/avantages/index.md)

## Domaines fonctionnels

- **Plans et rapprochement.** Registre récursif, détection des doublons explicable et fusion.
- **Réalisation.** Tableaux kanban, diagrammes de Gantt, calendriers, jalons, chemin critique et chaîne critique.
- **Finances.** Budgets, registre de financement, valeur acquise, et risque sur les coûts et sur le calendrier.
- **Ressources.** Viviers de compétences, carte thermique de capacité et personnes clés.
- **Stratégie.** Réception des demandes de travaux, idées, objectifs, buts et résultats clés, et scénarios.
- **Gouvernance.** Risques, problèmes, revues de jalon, escalade et dossiers mensuels de direction.

[Voir toutes les fonctionnalités](topics/fonctionnalités/index.md)

## Utilisateurs visés

- **Bureaux de portefeuille.** Rapports sur les programmes et les projets.
- **Responsables de projet et de réalisation.** Gestion du calendrier, des coûts et des risques.
- **Membres d'équipe.** Une vue personnelle des travaux qui leur sont confiés.
- **Dirigeants.** Les [fiches par fonction](for/) exposent ce que le logiciel apporte à chaque fonction, ses limites et les décisions à prendre.

## État actuel

Le logiciel en est à un stade précoce. Il n'a qu'un seul mainteneur, aucun audit par un tiers et aucun accord de support. Ce qu'exigerait une utilisation en production, point par point et avec une estimation pour chacun, figure sur la page [chemin vers la production](path-to-production/index.md). Les limites connues sont énumérées sur la [page de comparaisons](topics/comparaisons/index.md).

## Démonstration

Une démonstration locale utilise une autorité de transport fictive et nécessite Git et Podman. Les étapes figurent dans le [tutoriel](topics/tutoriel/index.md).
