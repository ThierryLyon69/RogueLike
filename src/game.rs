use raylib::prelude::*;

use crate::{
    bullet::Bullet, collision::aabb_intersects, enemy::Enemy, enemy::EnemyKind,
    input::InputState, player::Player, room::Room, ui::Ui,
};

use crate::renderer::Renderer;

pub struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    room: Room,
    ui: Ui,
    input: InputState,
    game_over: bool,
    renderer: Renderer,
}

impl Game {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            player: Player::new(Vector2::new(480.0, 270.0)),
            enemies: vec![
                Enemy::new(EnemyKind::Skeleton, Vector2::new(640.0, 270.0)),
                Enemy::new(EnemyKind::Skeleton, Vector2::new(540.0, 270.0)),
                Enemy::new(EnemyKind::Skeleton, Vector2::new(440.0, 270.0)),
            ],
            bullets: Vec::new(),
            room: Room::new(0.0, 0.0, 960.0, 540.0),
            ui: Ui::new(),
            input: InputState::default(),
            game_over: false,
            renderer,
        }
    }

    pub fn handle_input(&mut self, rl: &mut RaylibHandle) {
        self.input = InputState::read(rl);
    }

    pub fn update(&mut self, dt: f32) {
        if self.game_over {
            return;
        }

        self.player.update(&self.input, dt, self.room.bounds);

        let player_pos = self.player.pos();

        for enemy in &mut self.enemies {
            enemy.update(player_pos, dt);
            if let Some(dmg) = enemy.try_melee_attack(player_pos) {
                self.player.damage(dmg);
            }
        }

        if let Some(bullet) = self.player.try_shoot(&self.input) {
            self.bullets.push(bullet);
        }

        for bullet in &mut self.bullets {
            bullet.update(dt, self.room.bounds);
        }

        for enemy in &mut self.enemies {
            for bullet in &mut self.bullets {
                if !bullet.is_alive() {
                    continue;
                }
                if aabb_intersects(bullet.hitbox(), enemy.hitbox()) {
                    enemy.damage(bullet.damage());
                    bullet.kill();
                }
            }
        }

        self.bullets.retain(|bullet| bullet.is_alive());
        self.enemies.retain(|enemy| !enemy.is_dead());

        if self.player.hp <= 0 {
            self.game_over = true;
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, fps: i32) {
        self.room.draw(d);
        self.player.draw(&self.renderer, d);

        for enemy in &self.enemies {
            enemy.draw(d);
        }

        for bullet in &self.bullets {
            bullet.draw(&self.renderer, d);
        }

        self.ui.draw(
            d,
            fps,
            self.player.hp,
            self.enemies.len() as i32,
            self.player.upgrade_label(),
        );

        if self.game_over {
            d.draw_text("GAME OVER", 360, 250, 24, Color::RED);
        }
    }
}
