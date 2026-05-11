# RogueLike

Un petit jeu roguelike en Rust avec Raylib.

## Fonctionnalités
- Déplacement du héros
- Animation du héros (sprites configurables via .env)
- Tir et gestion des projectiles
- Apparition d’ennemis (ex : squelette)
- Barre de vie pour les ennemis
- Collisions tirs/ennemis
- Gestion de plusieurs types d’ennemis

## Installation

1. Installez Rust : https://rustup.rs/
2. Clonez le repo et placez-vous dans le dossier :
   ```sh
   git clone <repo-url>
   cd RogueLike
   ```
3. Installez les dépendances Raylib (voir https://github.com/deltaphc/raylib-rs#dependencies)
4. Lancez le jeu :
   ```sh
   cargo run
   ```

## Configuration

Tous les chemins vers les sprites et animations sont configurables dans le fichier `.env`.

Exemple :
```
HERO_IDLE_FRAMES=assets/sprites/frames/hero/frame_0_0.png;assets/sprites/frames/hero/frame_0_1.png
BULLET_SPRITE=assets/sprites/frames/fireball/fireball_0.png
```

## TODO
Voir le fichier `todo.md` pour les tâches à venir.

## Features du prototype

### Joueur

- Deplacement ZQSD
- Viser a la souris
- Tir au clic gauche
- Points de vie, mort

### Ennemis

- Type 1: suit le joueur
- Type 2: tire des projectiles
- PV, degats, mort

### Projectiles

- Direction, vitesse, degats
- Collisions
- Destruction hors ecran
- Systeme reutilisable

### Collisions

- Joueur / mur
- Bullet / ennemi
- Ennemi / joueur
- AABB simples

### Salles

- Plusieurs salles
- Transitions simples
- Selection aleatoire
- 3 salles faites a la main au debut

### Upgrades

Apres certaines salles, proposer 3 ameliorations aleatoires:

- Degats
- Vitesse
- Cadence de tir
- Points de vie

### UI

- Vie du joueur
- FPS
- Nombre d ennemis
- Upgrade actuelle

### Audio

- Son de tir
- Son d impact
- Musique de fond

## Roadmap

1. Creer la fenetre
2. Creer le renderer
3. Creer le joueur
4. Ajouter le deplacement
5. Ajouter le tir
6. Creer les ennemis
7. Creer les collisions
8. Creer le systeme de salles
9. Creer les upgrades
10. Ajouter le polish

## Contraintes

- Pas de moteur de jeu
- Pas de 3D
- Pas de multijoueur
- Pas d open world
- Pas de systemes inutiles
- Priorite au gameplay

## Lancer

1. Installer les dependances raylib (Linux):
	- Ubuntu: `sudo apt install libraylib-dev`
2. Build & run:
	- `cargo run`

## Controles

- Deplacement: ZQSD (WASD accepte)
- Viser: souris
- Tirer: clic gauche
- Choisir une amelioration: 1 / 2 / 3

## Audio (optionnel)

Deposez vos fichiers dans:

- `assets/audio/music.ogg`
- `assets/audio/shoot.wav`
- `assets/audio/hit.wav`

Sinon, le jeu tourne sans audio.