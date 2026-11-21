# Avantages

*Les principaux avantages pour une organisation, par domaine.*

## Visibilité sur le portefeuille

Les informations sur le portefeuille sont souvent réparties à plusieurs endroits. Dans ce logiciel, chaque portefeuille, programme, projet et produit est un seul type d'enregistrement, appelé **plan**. Tout plan peut en contenir un autre, à n'importe quelle profondeur, de sorte que la structure peut refléter l'organisation. Une étiquette telle que « Projet » est descriptive et n'impose aucune restriction à un plan.

## Réduction des doublons

Deux services peuvent lancer la même initiative sous des noms différents. Le moteur de rapprochement compare les attributs qui identifient un plan : nom, buts, responsable, sponsor, plan parent, période, mots-clés et identifiants. Il fonctionne sur tous les types de plans et montre les éléments de preuve de chaque rapprochement, et c'est une personne qui décide. Deux enregistrements d'un même plan peuvent être fusionnés.

## Qualité des rapports

- Un chiffre manquant est présenté avec une raison, par exemple « pas de prévision, car… ».
- Un plan sans évaluation est présenté comme « non évalué ».
- La confiance de réalisation saisie par des personnes est affichée à côté de l'état calculé, et les deux restent distincts.
- Les plans privés sont exclus de tous les totaux et de tous les décomptes pour tous les utilisateurs. Des tests vérifient cette règle.
- Chaque durée indique si elle est exprimée en jours calendaires ou en jours ouvrables.

## Traçabilité des décisions

Chaque modification est auditée, et la piste d'audit ne peut pas être altérée par le service. La personne qui dépose une demande de changement ne peut pas l'approuver. Un dossier mensuel de direction est rédigé une fois, publié par une personne désignée, puis figé par la base de données. Une correction est publiée sous la forme d'une nouvelle version accompagnée d'une note.

## Sécurité

L'authentification est activée par défaut, et le service ne démarre pas en production lorsqu'elle est désactivée. Des en-têtes de sécurité, une limitation du débit, un plan de reprise avec des objectifs chiffrés et un dossier de gouvernance de l'information sont inclus dans le dépôt.

## Maîtrise et portabilité

- **Licences.** Le logiciel est libre, au choix sous MIT, Apache-2.0, BSD-3-Clause, GPL-2.0, GPL-3.0, OGL-UK-3.0 ou CC-BY-NC-SA-4.0.
- **Hébergement.** Le logiciel est auto-hébergé : un seul service Rust au-dessus de PostgreSQL, avec une interface SvelteKit.
- **Import de données.** Les fichiers CSV de Jira et d'Asana peuvent être importés, avec un aperçu et une raison indiquée pour tout ce qui est exclu. Le XML de Microsoft Project peut être importé et exporté.
- **Langues.** L'interface est disponible dans dix paramètres régionaux, dont l'arabe, qui s'écrit de droite à gauche. Les traductions sont des brouillons en attente de relecture par des locuteurs.

## Pages connexes

Voir aussi les [fonctionnalités](../fonctionnalités/index.md), les [comparaisons](../comparaisons/index.md) et le [tutoriel](../tutoriel/index.md).
