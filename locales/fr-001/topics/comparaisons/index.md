# Comparaisons

*Un résumé de la place du logiciel par rapport à d'autres produits.*

Les produits ci-dessous répondent à des besoins différents et ne sont pas notés. Ces notes reposent sur la documentation publique de chaque projet. **Aucun autre produit n'a été installé ni exécuté**, et chacun a pu évoluer depuis. Il convient de consulter la documentation actuelle des produits avant de prendre une décision.

| Projet | De quoi il s'agit | En quoi ce projet diffère |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Gestion de projet pour les personnes qui ne sont pas chefs de projet | Ce projet inclut aussi les fonctions de planification, de coûts et de gouvernance qu'utilise un bureau de portefeuille |
| [Redmine](https://www.redmine.org/) | Un outil de suivi généraliste et configurable, avec des types de problèmes, des statuts et des rôles définis par l'organisation | Les types de problèmes, les statuts et les circuits sont définis par l'organisation. Les rôles portent des droits sur les problèmes, attribués plan par plan, dans les limites de ce que permet la connexion de la personne. Voir la [comparaison avec Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (par son fork, Platform Collective) | Un cadre d'applications métier reposant sur de nombreux services | Huly est une plateforme à plusieurs services. Ce projet est un seul service au-dessus de PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Une alternative de bureau à Microsoft Project, pour un responsable et un projet | Ce projet est un service hébergé, multi-utilisateurs, avec contrôle d'accès |

## Comparaisons par produit

Pages de comparaison individuelles : [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interopérabilité

- **Jira et Asana :** import CSV, avec un aperçu et une raison indiquée pour tout ce qui est laissé de côté.
- **Microsoft Project :** import et export du XML MSPDI, couvrant les tâches, le plan hiérarchique, les dates, les durées, les contraintes, les dépendances et le calendrier. Les affectataires, les ressources et les références de base sont exclus, et le fichier le consigne.

## Différences de conception

- **Raisons indiquées.** Un chiffre manquant est présenté avec une raison, par exemple « pas de prévision, car… ».
- **Détection des doublons explicable.** Le rapprochement fonctionne sur tous les types de plans et montre ses éléments de preuve.
- **Sécurité par défaut.** L'authentification est activée par défaut, et les documents de sécurité sont inclus dans le dépôt.
- **Durées explicites.** Chaque durée est exprimée en jours calendaires ou en jours ouvrés.
- **Limites consignées.** Le backlog consigne les limites de chaque élément.

## Limites connues

- Pas d'application mobile ni de bureau.
- Pas de co-édition en temps réel.
- Un seul mainteneur, un historique de production limité et aucun audit par un tiers.
- Le test de charge publié a été exécuté sur une seule machine. Il donne un ordre de grandeur et n'est pas un plan de capacité.
- Les traductions sont des brouillons en attente de relecture.
- Pas d'écran pour rédiger ou trancher les demandes de changement.
- Pas d'écosystème de modules d'extension ni de place de marché.

Des précisions figurent dans les [notes de comparaison](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Des corrections peuvent être proposées sous forme de signalement accompagné d'une source.
