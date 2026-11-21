# Tutoriel

*Les étapes pour lancer la démonstration sur un ordinateur local et parcourir un exemple de portefeuille.*

La démonstration est une copie complète du logiciel, chargée avec les données d'une organisation fictive, la Northgate Transport Authority. Les personnes, plans, chiffres et dates sont tous fictifs, et les données peuvent être modifiées librement. Les libellés de l'interface de démonstration restent en anglais dans ce tutoriel : ils sont cités tels qu'ils s'affichent à l'écran.

**Note de sécurité.** La démonstration fait confiance à une clé de test publiée. Elle ne doit pas être lancée sur un réseau partagé ni utilisée avec des données réelles.

## Prérequis

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) avec Compose

## Étape 1 : télécharger le code

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Étape 2 : démarrer la démonstration

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Cette commande construit le logiciel, démarre la base de données, charge le portefeuille fictif et lance l'interface. La première construction est plus longue que les suivantes.

## Étape 3 : se connecter

Ouvrez <http://localhost:3000/fr-fr/signin>, laissez l'adresse `example@example.com` et appuyez sur **Sign in to the demonstration**. Aucun courriel n'est envoyé.

La première page est la **demonstration tour** (visite de la démonstration), qui énumère chaque fonctionnalité par groupe, avec une description et un lien. Un bandeau jaune, sur chaque page, indique que les données sont fictives.

## Étape 4 : parcourir le portefeuille

Depuis la visite, ouvrez **Plans** pour afficher le portefeuille, ses programmes et ses projets. Ouvrez ensuite le **Portfolio dashboard**.

## Étape 5 : parcourir un projet

Ouvrez le projet **North Line interlocking** par le groupe « réalisation » de la visite. Parcourez son diagramme de **Gantt**, son **Critical path** et sa **Earned value**.

## Étape 6 : parcourir le travail personnel

Ouvrez **My work** et **My kanban board**. La session est ouverte au nom d'Example Person, qui a des tâches en cours.

## Étape 7 : parcourir la capacité, le contrôle des démarrages et les rapports

Ouvrez **Capacity heatmap** pour voir les ressources surchargées. Ouvrez **Start queue**, où la limite de travaux en cours a retenu un programme. Ouvrez **Director packs**, le résumé mensuel qui est figé une fois publié.

## Étape 8 : arrêter la démonstration

Cette commande arrête tous les services et supprime les données de la démonstration :

```sh
podman compose --profile demo down -v
```

## Pour aller plus loin

- [Guide d'installation](../../../../INSTALL.md), qui couvre l'authentification et les fournisseurs d'identité
- [Fiches par fonction](../../for/) pour les fonctions de direction
- [Guide de la démonstration](../../../../operations/demo.md)
- [Liste des fonctionnalités](../fonctionnalités/index.md)

Les problèmes peuvent être signalés dans le [suivi des tickets](https://github.com/project-portfolio-management/project-portfolio-management/issues).
