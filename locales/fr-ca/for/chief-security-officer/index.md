# Pour la direction de la sécurité (CSO)

*Direction de la sécurité (CSO) : ce que le logiciel applique, ce que le projet a vérifié, et ce que vous devez tester et exploiter vous-même.*

## Ce que le logiciel applique

| Domaine | Comportement |
|---|---|
| Authentification | L'authentification est activée par défaut. Le service ne démarre pas sans clés de vérification des jetons, ni en production avec l'authentification désactivée. Connexion par un fournisseur d'identité tel que Microsoft Entra ID ; l'authentification multifacteur reste du ressort du fournisseur ([guide de connexion](../../../../operations/sign-in-with-entra.md)) |
| Autorisation | Politique fondée sur les attributs des jetons. Les plans privés sont tenus hors de tous les agrégats pour tout le monde ; les marques de traitement masquent les plans situés au-dessus de l'habilitation de l'appelant, administrateurs compris |
| Accès d'urgence (break-glass) | L'accès aux plans privés exige d'y être éligible, un motif indiqué, quatre heures au plus, et qu'une seconde personne soit prévenue ; chaque demande est auditée |
| Sessions | Délai d'inactivité (30 minutes par défaut) et durée maximale de session |
| Transport et navigateur | En-têtes de sécurité, politique de sécurité du contenu stricte, liste d'autorisation CORS, limitation du débit par appelant |
| Sortant | Les destinations fournies par les utilisateurs sont limitées aux adresses `https` publiques |
| Téléversements | Le contenu doit correspondre au type déclaré ; les programmes sont refusés ; une analyse `clamd` facultative échoue en mode fermé |
| Audit | La piste d'audit refuse la modification et la suppression dans la base de données |
| Enregistrements figés | Les dossiers de direction publiés et les évaluations de confiance de réalisation sont immuables ou en ajout seul par déclencheur de base de données |
| Chaîne d'approvisionnement | `cargo deny`, une nomenclature logicielle, la provenance de la construction à la publication, une [politique de sécurité](../../../../SECURITY.md) |

## Ce qui n'est pas fait

- Aucun test d'intrusion, audit par un tiers, revue de code indépendante ni certification.
- Aucun détecteur de logiciels malveillants fourni ; le point d'accroche `clamd` n'a été testé que contre un substitut.
- La piste d'audit peut être contournée par un administrateur de base de données ; il n'y a pas de chaîne de hachage.
- Les marques de traitement masquent et étiquettent ; elles ne chiffrent pas, et une marque n'est pas apposée dans les fichiers générés.
- Les sessions sont conservées en mémoire, sauf si Redis est configuré.
- L'outil pour les personnes concernées retrouve des références, pas les noms saisis dans du texte libre, des fichiers téléversés ou des sauvegardes.

## L'exploiter

Les sauvegardes disposent d'une restauration testée, d'un chiffrement facultatif et d'objectifs de reprise chiffrés ([dossier d'exploitation](../../../../operations/README.md)). Le [guide d'exploitation](../../../../operations/runbook.md) donne les étapes techniques en cas d'incident ; le processus d'incident et de violation de données vous appartient. Signalez une vulnérabilité de manière confidentielle comme le décrit [SECURITY.md](../../../../SECURITY.md) ; les objectifs sont un accusé de réception sous 5 jours ouvrables, sans garantie.

## Les décisions qui vous reviennent

Qui réalise le test d'intrusion et quand ; l'accès conditionnel du fournisseur d'identité et la correspondance des rôles ; le niveau de marquage pour lequel ce déploiement est garanti ; l'analyseur placé derrière le point d'accroche des pièces jointes ; la fréquence de recertification des accès.

## À lire ensuite

[Liste de contrôle d'assurance](../../../../governance/assurance-checklist.md) · [Chemin vers la production](../../path-to-production/index.md) · [Flux de données](../../../../governance/data-flow.md)
