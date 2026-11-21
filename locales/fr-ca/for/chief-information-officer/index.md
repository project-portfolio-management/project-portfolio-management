# Pour la direction des systèmes d'information (CIO)

*Direction des systèmes d'information (CIO) : intégration, identité, données, hébergement et informations qu'on vous demandera de fournir.*

## Identité et accès

- L'authentification est **imposée par défaut** ; le service refuse de démarrer sans clés de vérification des jetons, et en production avec l'authentification désactivée.
- Connexion par un fournisseur d'identité (notamment Microsoft Entra ID) ; l'authentification multifacteur reste du ressort du fournisseur ([guide de connexion](../../../../operations/sign-in-with-entra.md)).
- Les autorisations reposent sur des attributs de jeton (par exemple `access`, `directorpack`, `confidence`) avec une politique d'attributs. Les plans privés sont masqués de tous les agrégats ; les marques de traitement masquent les plans situés au-dessus de l'habilitation de l'appelant, administrateurs compris.
- L'accès d'urgence (break-glass) aux plans privés exige d'y être éligible, un motif indiqué, quatre heures au plus, et qu'une seconde personne soit prévenue.
- Les administrateurs peuvent lister les droits d'accès permanents (une aide à la recertification) et exporter ou pseudonymiser les données d'une personne.

## Données et intégration

| Sujet | Position |
|---|---|
| Données conservées | Enregistrements de plans, tâches, risques, budgets, jalons, temps et coûts ; les personnes sont des références opaques, pas des noms. Aucune donnée de santé, aucune donnée de catégorie particulière, par conception |
| Inventaire | Chaque colonne de données personnelles figure dans l'[inventaire des données personnelles](../../../../governance/personal-data-inventory.csv) ; un test le maintient fidèle au schéma |
| Interfaces | REST avec un document OpenAPI dérivé ; import et export en masse ; flux de calendrier ; événements sortants et webhooks vers des destinations `https` publiques uniquement |
| Sauvegardes | Sauvegarde scriptée avec une restauration testée, chiffrement facultatif, objectifs de reprise chiffrés ([sauvegarde et restauration](../../../../operations/backup-and-restore.md)) |
| Hébergement | Images de conteneurs et `compose.yaml` ; le logiciel n'effectue aucun transfert de lui-même, de sorte que l'emplacement des données relève de votre choix d'hébergement |

## Lacunes que vous devez combler

- Test d'intrusion, revue de code indépendante et certification : aucun n'a été fait.
- La conservation n'est appliquée que partiellement ; la purge doit être planifiée (voir le [calendrier de conservation](../../../../governance/retention-schedule.md)).
- L'outil pour les personnes concernées retrouve des références, pas les noms saisis dans du texte libre ou dans des fichiers téléversés.
- La piste d'audit n'est pas à l'épreuve d'une altération par un administrateur de base de données.

## À lire ensuite

[Schéma des flux de données](../../../../governance/data-flow.md) · [Liste de contrôle d'assurance](../../../../governance/assurance-checklist.md) · [Déploiement](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
