# AnimeThemes Mobile

AnimeThemes Mobile est une application Android dédiée aux openings et endings d'anime. Elle permet de découvrir des séries, de consulter leur fiche AniList et de retrouver leurs thèmes musicaux au même endroit.

Le projet combine une interface Vue.js avec un backend Rust/Tauri afin de proposer une expérience mobile native sur Android.

## Fonctionnalités

- Découverte d'un anime mis en avant de manière aléatoire.
- Listes des anime populaires, de la saison en cours et les mieux notés.
- Consultation des informations AniList : titres, synopsis, couverture, genres, saisons, épisodes et personnages.
- Recherche et navigation pensées pour une utilisation mobile.
- Accès aux openings (OP) et endings (ED) associés aux anime via AnimeThemes.
- Stockage local SQLite prévu pour les favoris, l'historique d'écoute et les playlists.
- Interface disponible en français et en anglais.

## Technologies

- [Vue 3](https://vuejs.org/) et [TypeScript](https://www.typescriptlang.org/)
- [Vite](https://vite.dev/)
- [Tauri 2](https://tauri.app/) et [Rust](https://www.rust-lang.org/)
- [AniList GraphQL API](https://docs.anilist.co/)
- [AnimeThemes API](https://animethemes.moe/)
- SQLite via le plugin SQL de Tauri
- Tailwind CSS

## Prérequis

- [Node.js](https://nodejs.org/) 20 ou une version compatible avec le projet
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) et Cargo
- Les dépendances système nécessaires à [Tauri](https://tauri.app/start/prerequisites/)
- Android Studio, le SDK Android et les outils Tauri Android

## Installation

```bash
pnpm install
```

## Développement

Lancer uniquement l'interface web :

```bash
pnpm dev
```

Lancer l'application Android :

```bash
pnpm android:dev
```

## Scripts disponibles

| Commande           | Description                                              |
| ------------------ | -------------------------------------------------------- |
| `pnpm dev`         | Lance le serveur de développement Vite                   |
| `pnpm build`       | Vérifie les types et construit l'interface de production |
| `pnpm preview`     | Prévisualise le build web                                |
| `pnpm android:dev` | Lance l'application Tauri sur Android                    |
| `pnpm tauri`       | Accède directement à la CLI Tauri                        |

## Architecture

```text
src/              Interface Vue, composants, vues et traductions
src-tauri/        Backend Rust, commandes Tauri et configuration native
src-tauri/src/    Intégration AniList et logique applicative Rust
src-tauri/migrations/
				  Schéma SQLite et migrations locales
public/           Ressources statiques
```

Les données AniList servent de pivot pour relier la fiche d'un anime à ses thèmes AnimeThemes. Les métadonnées distantes sont mises en cache localement afin que les favoris, l'historique et les playlists puissent rester indépendants du rafraîchissement des données externes.

## État du projet

Le projet est actuellement en développement. L'écran d'accueil et la récupération des catalogues AniList sont en place ; le lecteur de thèmes, la recherche complète et les fonctions personnelles continuent d'être intégrés.

## Licence

Aucune licence open source n'a encore été définie pour ce projet.
