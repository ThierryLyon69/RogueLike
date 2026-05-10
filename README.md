# RogueLike Prototype (raylib + Rust)

Mini prototype de roguelike 2D top-down, developpe sans moteur de jeu.
Stack: Rust + raylib-rs + Cargo. Le code vise un mini moteur 2D specialise, pas un engine generaliste.

## Objectifs

- Apprendre le game dev bas niveau
- Comprendre l architecture d un jeu
- Obtenir un prototype jouable rapidement
- Garder un code propre, performant, modulaire

## Style et gameplay

Inspirations: The Binding of Isaac, Vampire Survivors, Enter the Gungeon.
Visuel: pixel art minimaliste, vue du dessus, ambiance sombre sci-fi.
Le prototype peut utiliser des rectangles colores et des placeholders.

Le gameplay doit etre rapide, satisfaisant, responsive, avec feedbacks simples:

- hit feedback
- petit ecran shake
- flash quand un ennemi prend un coup

## Architecture cible

```
/src
	main.rs
	game.rs
	renderer.rs
	input.rs
	player.rs
	enemy.rs
	bullet.rs
	room.rs
	collision.rs
	upgrade.rs
	ui.rs

/assets
	/sprites
	/audio
	/fonts

/Cargo.toml
```

## Boucle principale

Le jeu utilise une game loop classique avec delta time:

```
while running {
	handle_input();
	update(delta_time);
	render();
}
```

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