# Pour la direction juridique (CLO)

*Direction juridique (CLO) : licence, protection des données, accessibilité, obligations linguistiques et ce que le projet ne garantit pas. Ceci n'est pas un avis juridique.*

## Licence

Proposé au choix sous l'une des licences MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, GPL-3.0-only, OGL-UK-3.0 ou CC-BY-NC-SA-4.0, ou sous des conditions sur mesure par accord. Le choix et les textes figurent dans [`LICENSE.md`](../../../../LICENSE.md) et dans `LICENSE/` ; un test maintient tous les manifestes cohérents avec eux.

## Protection des données (le droit qui s'applique à vous)

Rédigé pour n'importe quelle loi de protection des données : la [spécification de conformité §12.2a](../../../../spec/12-compliance.md) recense les plus courantes par région (le RGPD et le UK GDPR, la LGPD, la PIPL, la loi DPDP, la PIPEDA, le CCPA et d'autres) et ce que chacune implique ici, de façon générale et sans vérification par rapport au texte de chaque loi. Les documents sont préremplis et à adapter ; chaque marqueur `‹buyer›` est une décision qui vous revient (responsable de traitement, base légale, durées de conservation, sous-traitants, validation) :

- [Modèle d'analyse d'impact (AIPD)](../../../../governance/dpia.md) et [registre des traitements](../../../../governance/record-of-processing.md)
- [Inventaire des données personnelles](../../../../governance/personal-data-inventory.csv) : chaque colonne susceptible de contenir des données personnelles
- [Calendrier de conservation](../../../../governance/retention-schedule.md) : des propositions, avec ce que le code applique aujourd'hui
- [Schéma des flux de données](../../../../governance/data-flow.md)

Le logiciel conserve les personnes sous forme de références opaques, avec le temps enregistré, les disponibilités et les taux de coût, ce qui s'apparente à de la surveillance et justifie donc une analyse d'impact. Il propose un export pour la personne concernée et un effacement par pseudonymisation ou, si le déploiement le choisit, par suppression (administrateurs uniquement, avec aperçu préalable ; WW-1.8). Dans les dossiers de direction figés et dans l'historique de confiance en ajout seul, la personne qui a rédigé ou évalué est effacée, jamais supprimée, de sorte qu'une demande d'effacement peut être satisfaite sans rompre l'enregistrement. Il ne retrouve **pas** les noms saisis dans du texte libre, et les sauvegardes contiennent des enregistrements que l'application a supprimés.

## Cadres de référence

HIPAA et les régimes américains de certification des données de santé **ne sont pas concernés** (aucune donnée de patient) ; le régime des dispositifs médicaux n'est pas concerné non plus. La [liste de contrôle DTAC](../../../../governance/dtac-checklist.md) répond aux critères d'évaluation des technologies numériques (DTAC) du NHS britannique, un dispositif national conservé comme exemple détaillé ; pour les dispositifs d'autres pays et d'autres secteurs (ISO/IEC 27001, SOC 2, dispositifs nationaux relatifs au cloud et au secteur public), partez de la [liste de contrôle d'assurance](../../../../governance/assurance-checklist.md). Rien n'est certifié.

## Accessibilité et langue

WCAG 2.2 AA est testé automatiquement sur chaque route ; aucun test manuel ni avec des technologies d'assistance n'a été fait ([déclaration](../../../../governance/accessibility-statement.md)). Les traductions en gallois et dans d'autres langues sont des brouillons en attente d'un traducteur professionnel ([déclaration linguistique](../../../../governance/language-statement.md)) ; cela compte si vous avez des obligations linguistiques légales.

## Diligence raisonnable sur le fournisseur

Un seul mainteneur, aucun support contractuel, aucune garantie revendiquée, des outils d'IA utilisés dans le développement et déclarés dans la [déclaration sur l'IA](../../../../AI_STATEMENT.md). Le produit ne contient aucun modèle d'IA. Signalement des vulnérabilités : [SECURITY.md](../../../../SECURITY.md).

## Les décisions qui vous reviennent

La base légale et les durées de conservation ; la nécessité d'une procédure de conservation pour raison juridique (aucune n'est construite) ; les conditions de contrat et de support avec le mainteneur.

## À lire ensuite

[Liste de contrôle d'assurance](../../../../governance/assurance-checklist.md) · [Index du dossier de gouvernance](../../../../governance/README.md)
