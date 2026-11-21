# Test de sécurité indépendant

*Pour un acheteur, un évaluateur ou un testeur : ce qu'un test de sécurité indépendant de ce logiciel devrait couvrir, ce que le projet fournit pour y aider, ce qui a été fait et ce qui ne l'a pas été, et ce qui vous est laissé. Ceci est un guide, pas un rapport de test ni un avis juridique.*

## Où en sont les choses

**Aucun test de sécurité indépendant n'a été réalisé.** Il n'existe, pour ce logiciel, ni test d'intrusion, ni audit par un tiers, ni revue de code indépendante, ni certification, et le projet n'en revendique aucun ([chemin vers la production](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). Une analyse passive s'exécute dans l'intégration continue du projet (règles dans `.zap/rules.tsv`), et les tests du projet vérifient les contrôles énumérés ci-dessous ; ni l'une ni les autres ne remplacent un testeur qui n'a pas écrit le logiciel.

Ce sont les points 1 et 9 de la [liste de contrôle d'assurance](../../../../governance/assurance-checklist.md). Ils reviennent au déployeur, avec un testeur certifié, sur **votre** déploiement : le projet ne peut tester ni votre hébergement, ni votre fournisseur d'identité, ni votre réseau, ni vos clés.

## Ce qu'il faut tester

Testez le déploiement tel qu'il fonctionnera, avec l'authentification imposée et votre fournisseur d'identité en amont. Périmètre suggéré, tiré des contrôles que le logiciel revendique :

| Domaine | Ce que le logiciel revendique | Ce qu'un testeur devrait tenter |
|---|---|---|
| Authentification | L'authentification est activée par défaut ; le service ne démarre pas sans clés de vérification ni, en production, avec l'authentification désactivée ; les jetons sont vérifiés hors ligne par rapport à des clés publiées | Jetons falsifiés, expirés, rejoués et mal signés ; émetteur ou audience erroné ; rotation des clés ; jeu de clés manquant |
| Autorisation | Politique fondée sur les attributs ; règles au niveau de l'enregistrement | Élévation de privilèges horizontale et verticale sur chaque route ; modification des identifiants dans les chemins et les corps de requête ; attributs qu'un appelant ne devrait pas pouvoir ajouter |
| Plans privés et marques de traitement | Un plan privé, ou un plan situé au-dessus de l'habilitation d'un appelant, ne figure dans aucune liste, aucun décompte, aucun total, aucun résultat de recherche, aucun flux ni aucun export de cet appelant, administrateurs compris | Toute route qui révèle l'existence, le nom, le nombre ou le total d'un plan que l'appelant ne peut pas lire ; messages d'erreur et temps de réponse qui diffèrent entre « masqué » et « absent » |
| Accès d'urgence (break-glass) | Personnes éligibles uniquement, un motif indiqué, quatre heures au plus, une seconde personne prévenue | Utilisation sans éligibilité ; prolongation d'un droit ; action sans que la seconde personne soit prévenue |
| Enregistrements figés et en ajout seul | La piste d'audit, les dossiers de direction publiés, l'historique de confiance de réalisation, l'historique d'état des programmes et les tampons de projet refusent la modification et la suppression dans la base de données | Modification ou suppression par l'API, par l'import en masse et par toute route qui touche les mêmes lignes |
| Séparation des tâches | Un demandeur ne peut pas approuver sa propre demande de changement | Approbation en tant que demandeur, en tant qu'administrateur, et par changement d'approbateur après le dépôt |
| Téléversements et pièces jointes | Le contenu doit correspondre au type déclaré ; les programmes sont refusés ; un point d'accroche facultatif d'analyse antivirus échoue en mode fermé | Fichiers polyglottes, programmes renommés, fichiers surdimensionnés et mal formés, astuces de chemin dans les noms, et vérification que votre analyseur est réellement appelé |
| Requêtes sortantes | Les destinations fournies par les utilisateurs sont limitées aux adresses `https` publiques | Falsification de requête côté serveur vers des adresses privées, de lien local et de métadonnées, redirections, rebond DNS |
| Sessions et navigateur | Délai d'inactivité, limite maximale, cookie `httpOnly`, politique de sécurité du contenu stricte, liste d'autorisation CORS | Fixation et réutilisation de session, falsification de requête intersite, injection dans l'interface, détournement de clic |
| Limitation du débit | Limites par appelant | Contournement en changeant d'identité, d'en-tête ou de chemin ; les limites survivent-elles à votre proxy |
| Outils pour les personnes concernées | Export et pseudonymisation, administrateurs uniquement, aperçu préalable | Utilisation par un non-administrateur ; exhaustivité par rapport à l'[inventaire des données personnelles](../../../../governance/personal-data-inventory.csv) |
| Sauvegardes | Chiffrement facultatif, restauration testée | Accès aux fichiers de sauvegarde et aux clés ; restauration dans un environnement propre |

## Ce que le projet fournit

- Le code source, la spécification et les tests, pour qu'une revue de code puisse partir des affirmations.
- Une description lisible par machine de l'API (`/api-docs/openapi.json`), avec des schémas de requête et de réponse pour le travail sur le dossier de direction ; la liste des routes est vérifiée par rapport au code par des tests.
- Une configuration jetable avec l'authentification désactivée (`dev-insecure`, voir le [README](../../../../README.md)) **pour la machine du testeur uniquement**, et un script qui génère un jeu de clés de test (`scripts/smoke-keys.sh`) afin de pouvoir émettre des jetons aux attributs choisis contre un déploiement de test.
- Le [schéma des flux de données](../../../../governance/data-flow.md), le [guide d'exploitation](../../../../operations/runbook.md) et les [objectifs de reprise](../../../../operations/recovery-objectives.md).
- Un moyen confidentiel de signaler ce qui est trouvé : [SECURITY.md](../../../../SECURITY.md).

## Règles d'engagement

Convenez-en par écrit avant tout test ; le projet ne peut pas les fixer pour votre environnement.

- **Environnement.** Testez une copie avec des données réalistes mais non réelles. Ne testez pas la production, sauf si vous l'avez décidé par écrit.
- **Autorisation.** Le testeur dispose de votre permission écrite, d'un contact désigné et d'une plage horaire précise.
- **Hors limites.** Le déni de service par le volume, l'ingénierie sociale visant le personnel et les services tiers (votre fournisseur d'identité, votre hébergeur) sont hors périmètre, sauf accord de leur propriétaire respectif. Le périmètre propre au projet figure dans [SECURITY](../../../../SECURITY.md).
- **Données.** Les données personnelles réelles ne sont ni utilisées, ni copiées à l'extérieur, ni placées dans le rapport.
- **Clés.** Les clés de test sont distinctes des clés de production et sont détruites ensuite.

## Signalement et nouveau test

- Les constats concernant ce logiciel sont adressés au mainteneur de façon confidentielle, comme l'indique [SECURITY](../../../../SECURITY.md), avec la manière de les reproduire, la version ou le commit, et l'impact. Les objectifs du projet sont un accusé de réception sous 5 jours ouvrables et un plan sous 15 jours ; ce sont des objectifs, pas des garanties. Il y a un seul mainteneur et aucun accord de niveau de service.
- Les constats concernant votre déploiement vont à votre propre responsable.
- Demandez au testeur de noter chaque constat, d'indiquer ce qui n'a **pas** été testé, et de retester les correctifs. Un rapport qui n'énumère que ce qui a été trouvé ne vous dit pas ce qui a été manqué.
- Conservez le rapport, le périmètre, les dates et le nouveau test dans vos propres archives. Le projet ne peut pas les conserver pour vous.

## Limites de ce document

Il énumère des domaines à couvrir, pas un plan de test complet, et les affirmations du tableau sont celles du projet lui-même. Il ne certifie rien, et un test sans anomalie est une preuve valable pour un seul déploiement à une date donnée.

## Les décisions qui vous reviennent

Qui teste, quand, contre quoi, avec quelles données et selon quelles règles ; comment les constats sont notés et sous quel délai ils doivent être corrigés ; et si le résultat est partagé avec vos évaluateurs.

## À lire ensuite

[Liste de contrôle d'assurance](../../../../governance/assurance-checklist.md) · [Chemin vers la production](../../path-to-production/index.md) · [Direction de la sécurité](../chief-security-officer/index.md) · [Gouvernance, risques, conformité](../governance-risk-compliance/index.md)
