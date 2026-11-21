# Pour la direction des risques (CRO)

*Direction des risques (CRO) : les risques que le logiciel vous aide à voir, et les risques liés à son utilisation.*

## Les risques qu'il aide à gérer

- **Risques, problèmes et demandes de changement** enregistrés par plan ; un registre des problèmes sur l'ensemble des plans lisibles ; des demandes de changement avec séparation des tâches.
- **Confiance de réalisation et exceptions :** la confiance exprimée est tenue à part du RAG calculé, signalée lorsqu'ils divergent de deux niveaux, avec un historique en ajout seul, et un délai d'exception de 48 heures qui ne redémarre pas lorsqu'une évaluation se dégrade et ne se ferme que par une décision enregistrée.
- **Une absence assumée :** une donnée manquante apparaît comme « non évalué » ou nulle, avec une raison, jamais comme verte ou égale à zéro.
- **Un dossier mensuel figé et versionné,** de sorte que ce qui a été dit aux dirigeants puisse être montré plus tard exactement tel qu'il a été publié.

## Les risques liés à son utilisation (le registre du projet lui-même)

| Risque | État |
|---|---|
| Aucun test d'intrusion, audit par un tiers ni certification | Ouvert : à commander par l'acheteur ([liste de contrôle d'assurance](../../../../governance/assurance-checklist.md)) |
| Conservation appliquée à une partie seulement des données | Ouvert ; la purge doit être planifiée |
| L'outil pour les personnes concernées ne cherche pas dans le texte libre, les pièces jointes ni les sauvegardes | Ouvert |
| La piste d'audit peut être contournée par un administrateur de base de données ; pas de chaîne de hachage | Indiqué dans l'[ÉFVP](../../../../governance/dpia.md) |
| Les marques de traitement masquent et étiquettent, elles ne chiffrent pas | Indiqué |
| L'analyse des pièces jointes exige un analyseur exploité par le déployeur | Ouvert |
| Un seul mainteneur, aucun accord de niveau de service | Ouvert |
| Les traductions sont des brouillons | Ouvert |
| Usage non clinique uniquement ; pas de dossier de sécurité clinique | Ne saisissez pas de contenu clinique |

## Les contrôles que le logiciel applique

Authentification par défaut, limitation du débit, en-têtes de sécurité stricts, délai d'inactivité de session (30 minutes) et durée maximale de session, destinations sortantes limitées à `https` public, pièces jointes dont la signature est vérifiée, lignes d'audit immuables, accès d'urgence (break-glass) avec audit. Chacun est couvert par un test cité dans le [chemin vers la production](../../path-to-production/index.md).

## Les décisions qui vous reviennent

L'appétence au risque pour un pilote ; la fenêtre d'exception (48 heures par défaut) ; le niveau de marquage pour lequel le déploiement est garanti ; qui revoit la liste des accès et à quelle fréquence.

## À lire ensuite

[Chemin vers la production](../../path-to-production/index.md) · [ÉFVP](../../../../governance/dpia.md) · [Conformité](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
