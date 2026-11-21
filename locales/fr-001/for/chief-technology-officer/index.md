# Pour la direction technique (CTO)

*Direction technique (CTO) : architecture, qualité, comment l'étendre, et où l'ingénierie est mince.*

## Architecture

Trois sous-projets, une seule entité ([architecture](../../../../spec/08-architecture.md)) :

| Partie | Technologie |
|---|---|
| Bibliothèque de rapprochement | Rust ; rapprochement par paires déterministe et probabiliste, explicable ; intégrée par le service |
| Service | Rust, axa sur axum, sqlx, PostgreSQL 18 ; REST sous `/api`, OpenAPI dérivé des routes |
| Interface | SvelteKit 3, application monopage Svelte 5 avec un backend-for-frontend ; catalogues d'internationalisation |

La chaîne d'outils est figée dans [`rust-toolchain.toml`](../../../../rust-toolchain.toml). La persistance est décrite dans la [spécification 10](../../../../spec/10-persistence.md) ; l'API, dans la [spécification 09](../../../../spec/09-api-surface.md).

## Points forts de l'ingénierie à connaître

- **Les invariants résident dans la base de données, là où ils comptent :** le journal d'audit refuse la modification et la suppression ; les dossiers de direction publiés et les évaluations de confiance de réalisation sont immuables ou en ajout seul par déclencheur ; des contraintes CHECK appuient les règles du service.
- **Tests de garde :** des tests de recensement font échouer la construction lorsqu'une nouvelle route d'agrégation oublie le filtre des plans privés, qu'une route de liste n'est pas paginée, qu'une colonne de données personnelles manque à l'inventaire, ou qu'une route manque dans OpenAPI. On les corrige en classant l'élément nouveau, pas en affaiblissant le test.
- **Tests avec de vrais jetons :** l'autorisation est testée de bout en bout avec des jetons PASETO signés (chaque test dans son propre processus, car les paramètres d'authentification sont lus une fois par processus).
- **Chaîne d'approvisionnement :** `cargo deny`, une nomenclature logicielle, la provenance de la construction à la publication, une politique de sécurité.
- **L'intégration continue** construit et teste tout à chaque push ; voir `.github/workflows/`.

## Où elle est mince

- Un seul mainteneur, donc un facteur de bus de un ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- Les performances ne sont connues que grossièrement ; le cache de réponses ne couvre que la route du chemin critique ; les très grands plans sont lents.
- Les sessions sont en mémoire, sauf si Redis est configuré.
- Plusieurs sections du dossier de direction et tous les écrans du dossier ne sont pas construits ([`tasks.md`](../../../../tasks.md)).
- Des outils d'IA servent à le construire ; le produit ne contient aucun modèle. Voir la [déclaration sur l'IA](../../../../AI_STATEMENT.md).

## Les décisions qui vous reviennent

L'hébergement et la topologie de Redis, s'il faut créer un fork ou suivre l'amont, qui relit le code de façon indépendante, et comment le service s'accorde avec votre fournisseur d'identité (jeu de clés PASETO, ou OIDC par Keycloak ou Entra).

## À lire ensuite

[Architecture](../../../../spec/08-architecture.md) · [Stratégie de test](../../../../spec/11-testing-strategy.md) · [État de la mise en œuvre](../../../../spec/14-implementation-status.md) · [Benchmarks](../../../../BENCHMARKS.md)
