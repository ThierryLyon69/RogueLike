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
    wave: i32,
    enemies_per_wave: i32,
    shop_open: bool,
}

impl Game {
    pub fn new(renderer: Renderer) -> Self {
        let wave = 1;
        let enemies_per_wave = 3;
        let room = Room::new(0.0, 0.0, 960.0, 540.0);
        Self {
            player: Player::new(Vector2::new(480.0, 270.0)),
            enemies: Self::spawn_wave_enemies(enemies_per_wave, room.bounds),
            bullets: Vec::new(),
            room,
            ui: Ui::new(),
            input: InputState::default(),
            game_over: false,
            renderer,
            wave,
            enemies_per_wave,
            shop_open: false,
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

        if self.shop_open {
            if self.input.shop_confirm {
                self.shop_open = false;
                self.wave += 1;
                self.enemies_per_wave += 1;
                self.enemies = Self::spawn_wave_enemies(self.enemies_per_wave, self.room.bounds);
            }
            return;
        }

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

        if self.enemies.is_empty() {
            self.shop_open = true;
            self.bullets.clear();
        }

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
            self.wave,
            self.shop_open,
        );

        if self.game_over {
            d.draw_text("GAME OVER", 360, 250, 24, Color::RED);
        }
    }

    fn spawn_wave_enemies(count: i32, bounds: Rectangle) -> Vec<Enemy> {
        let mut enemies = Vec::new();
        let center = Vector2::new(bounds.x + bounds.width * 0.5, bounds.y + bounds.height * 0.5);
        let radius = 180.0;
        let total = count.max(1) as usize;
        for i in 0..total {
            let angle = (i as f32) / (total as f32) * std::f32::consts::PI * 2.0;
            let offset = Vector2::new(angle.cos() * radius, angle.sin() * radius);
            let pos = center + offset;
            enemies.push(Enemy::new(EnemyKind::Skeleton, pos));
        }
        enemies
    }
}
