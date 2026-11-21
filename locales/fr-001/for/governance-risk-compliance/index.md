# Gouvernance, risques, conformité

*Pour les personnes qui dirigent la gouvernance, les risques et la conformité : ce que le logiciel vous aide à prouver, ce qu'il applique, et ce que vous devez fournir. Pas un avis juridique.*

## Gouvernance

- **Les décisions sont consignées.** Les demandes de changement exigent un approbateur désigné qui n'est pas le demandeur, un motif indiqué d'au moins 10 caractères, et sont auditées par acteur ; la base de données refuse une approbation sans décideur ni motif.
- **Un dossier mensuel de direction figé.** Rédigé une fois, publié par une personne, immuable une fois publié ; les corrections passent par une republication avec une note. L'accès en lecture dépend du public visé.
- **Confiance de réalisation et exceptions.** La confiance exprimée est en ajout seul et affichée à côté de l'état calculé, sans jamais être mêlée à lui. Une évaluation rouge ou orange-rouge ouvre une exception avec un délai de 48 heures (configurable), que seule une décision enregistrée ferme.
- **Gouvernance de projet du logiciel lui-même :** [GOVERNANCE.md](../../../../GOVERNANCE.md), [maîtrise des changements](../../../../spec/18-change-control.md), [déclaration sur l'IA](../../../../AI_STATEMENT.md).

## Risques

- Risques, problèmes et demandes de changement par plan ; un registre des problèmes sur l'ensemble des plans lisibles.
- Une donnée manquante est présentée comme « non évalué », jamais comme verte.
- Les risques ouverts du projet lui-même (aucun test d'intrusion, conservation partielle, un seul mainteneur, traductions en brouillon) figurent sur la page [chemin vers la production](../../path-to-production/index.md), point par point et avec une estimation pour chacun.

## Conformité

| Domaine | Où | État |
|---|---|---|
| Protection des données (le droit applicable ; le [§12.2a](../../../../spec/12-compliance.md) recense le RGPD, le UK GDPR, la LGPD, la PIPL, la DPDP, la PIPEDA, le CCPA et d'autres) | [AIPD](../../../../governance/dpia.md), [registre des traitements](../../../../governance/record-of-processing.md), [inventaire des données personnelles](../../../../governance/personal-data-inventory.csv), [calendrier de conservation](../../../../governance/retention-schedule.md), [flux de données](../../../../governance/data-flow.md) | Prérempli ; les marqueurs `‹buyer›` vous reviennent ; conservation appliquée en partie seulement |
| DTAC du NHS britannique (un dispositif national, un exemple détaillé ; les autres dispositifs partent de la [liste de contrôle d'assurance](../../../../governance/assurance-checklist.md)) | [Liste de contrôle DTAC](../../../../governance/dtac-checklist.md) | Réponse section par section ; lignes de l'acheteur et du fournisseur ouvertes |
| Accessibilité | [déclaration](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA automatisé ; aucun test manuel |
| Obligations linguistiques | [déclaration linguistique](../../../../governance/language-statement.md) | Les traductions sont des brouillons |
| ISO/IEC 27001 | [spécification de conformité](../../../../spec/12-compliance.md) | Contrôles côté déploiement ; non certifié |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [spécification de conformité](../../../../spec/12-compliance.md) §12.4 | Non concernés (aucune donnée de patient) ; énoncés, pas fabriqués |

## Preuves et assurance

La [liste de contrôle d'assurance](../../../../governance/assurance-checklist.md) énumère 14 points qu'un évaluateur demandera, avec leur responsable et leur état. La plupart sont ouverts et reviennent au déployeur : test d'intrusion, audit d'accessibilité, exercice de reprise après sinistre, assurance sur l'hébergement, configuration du fournisseur d'identité, recertification des accès, processus d'incident, gestion des archives. La piste d'audit refuse la modification et la suppression dans la base de données, mais un administrateur de base de données peut la contourner ; il n'y a pas de chaîne de hachage.

## Les décisions qui vous reviennent

La base légale et les durées de conservation ; la fenêtre d'exception ; le niveau de marquage du déploiement ; qui revoit les accès et à quelle fréquence ; la nécessité d'une procédure de conservation pour raison juridique (aucune n'est construite).

## À lire ensuite

[Index du dossier de gouvernance](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Spécification de conformité](../../../../spec/12-compliance.md)
