# Chemin vers la production

*Ce que « production » signifie pour ce logiciel, et la distance qui l’en sépare, point par point.*

Ici, « production » signifie qu’une organisation utilise le logiciel pour un travail réel. Des plans réels, les noms de
personnes réelles et des décisions réelles en dépendent ; une panne ou une fuite a des conséquences, et quelqu’un en
répond. Dans un grand organisme de santé ou une administration, la production suppose aussi que l’organisation a mené à
bien sa propre évaluation : un test de sécurité, une analyse d’impact relative à la protection des données, un audit
d’accessibilité et, au sein du National Health Service britannique, les Digital Technology Assessment Criteria (DTAC).

## Où il en est

- Aucune certification, homologation ni audit n’est revendiqué. Cette page est l’estimation du projet lui-même.
- « Médical » désigne un outil de portefeuille et de projets **non clinique** : pas de dossiers de patients ni d’aide à
  la décision clinique.
- Le projet suggère de commencer par un pilote sur des données non cliniques et non secrètes, mené par une organisation
  qui réalise sa propre évaluation.

## Maturité, point par point

Chaque chiffre est l’estimation du projet au 13 novembre 2026, et non une mesure. Chaque curseur part de cette
estimation. Déplacer un curseur note votre propre avis pendant la lecture ; rien n’est enregistré ni envoyé.

- **Assurance de sécurité indépendante** — 0%. Aucun test d’intrusion, aucun audit par un tiers et aucune certification. Un acheteur ou un fournisseur doit les commander (RG-13).
- **Accessibilité** — 40%. Un audit automatique WCAG 2.2 AA de chaque route s’exécute à chaque modification. Aucun test manuel, ni avec des technologies d’assistance ou des personnes handicapées, n’a été mené (RG-13).
- **Demandes des personnes concernées** — 50%. Les administrateurs peuvent exporter et pseudonymiser une personne dans chaque référence du schéma, avec un aperçu préalable (RG-6). Les noms saisis dans des titres, des commentaires ou des documents, les fichiers téléversés, les instantanés d’audit et les sauvegardes ne sont pas retrouvés automatiquement.
- **Conservation** — 35%. Une purge, à blanc par défaut, couvre les notifications, les journaux de livraison, les clés d’idempotence et les instantanés de tendance et d’adoption, avec des durées fixées par déploiement (RG-4). Les plans, tâches, relevés de temps, documents et la piste d’audit sont conservés jusqu’à ce qu’un opérateur les supprime.
- **Mentions de classification** — 60%. Une classification configurable masque les plans au-dessus de l’habilitation de l’appelant, administrateurs compris, et étiquette chaque réponse (RG-7). La mention n’est pas apposée dans les fichiers générés, et l’habilitation ne vaut que ce que vaut l’attribut du fournisseur d’identité.
- **Sécurité des pièces jointes** — 50%. Le contenu doit correspondre au type déclaré, les programmes sont refusés et une analyse `clamd` facultative échoue en mode fermé (RG-8). Aucun antivirus n’est fourni, et le branchement n’a été testé que contre un substitut.
- **Revue des accès et accès d’urgence** — 45%. Un rapport des droits en vigueur sert à la recertification (RG-9), et l’accès d’urgence est limité dans le temps et exige un motif déclaré (RG-10). Le rapport montre ce qui a été enregistré, et non ce que le fournisseur d’identité applique ; aucune route n’enregistre la décision d’un réviseur ; et un avis d’accès d’urgence peut désigner quelqu’un qui ne reçoit rien alors que le droit reste en vigueur.
- **Performances et capacité** — 40%. Un test de charge et d’endurance sur une seule machine figure dans [`BENCHMARKS.md`](../../../BENCHMARKS.md) : les requêtes ordinaires répondent en quelques dizaines de millisecondes, un plan d’environ 12 700 tâches est lent à planifier et le processus grimpe à environ 1 Go. Il n’existe ni plan de capacité, ni test avec de nombreux utilisateurs, ni exécution sur plusieurs machines.
- **Sessions sur plusieurs serveurs frontaux** — 70%. Les sessions sont gardées en mémoire tant que Redis n’est pas configuré (RG-12) ; avec Redis, la connexion en dépend.
- **Intégrité de la piste d’audit** — 50%. Les écritures sont auditées et un déclencheur de la base de données refuse les modifications et les suppressions, avec des exceptions déclarées (RG-3). Un administrateur de la base de données peut supprimer le déclencheur, et il n’y a pas de chaîne de hachage.
- **Sauvegarde et restauration** — 80%. Une restauration est testée en intégration continue, en clair et chiffrée, au regard d’objectifs de reprise déclarés (RG-5). Le chiffrement est facultatif, et l’exercice tourne sur l’intégration continue du projet, et non chez l’hébergeur de qui déploie.
- **Support et continuité** — 10%. Un seul mainteneur et aucun accord de niveau de service ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Traductions** — 20%. Les textes dans d’autres langues que l’anglais sont rédigés par une machine jusqu’à leur relecture par un locuteur, et les obligations linguistiques légales exigent des textes relus ([déclaration linguistique](../../../governance/language-statement.md)).
- **Usage clinique** — 0%. Aucun dossier de sécurité clinique n’a été rédigé, et l’usage clinique est hors du périmètre.

## Ce qu’un premier déploiement devrait faire

1. Compléter la [liste de contrôle d’assurance](../../../governance/assurance-checklist.md), en commençant par un test d’intrusion et un audit d’accessibilité.
2. Remplir chaque ligne « Buyer to complete » de la [liste de contrôle DTAC](../../../governance/dtac-checklist.md) et les repères de la [DPIA](../../../governance/dpia.md).
3. Convenir des durées de conservation avec le responsable des archives et planifier la purge.
4. Activer les sauvegardes chiffrées et réaliser l’exercice de restauration chez l’hébergeur du déploiement.
5. Garder les données non cliniques et en dessous du niveau de classification pour lequel le déploiement a été évalué.
6. Prévoir les lacunes ci-dessus, ou financer leur suppression.

Les éléments de preuve de chaque point figurent dans l’[évaluation](../../../governance/production-readiness.md).
