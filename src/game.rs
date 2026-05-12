use raylib::prelude::*;

use crate::{
    bullet::Bullet, collision::aabb_intersects, enemy::Enemy, enemy::EnemyKind,
    input::InputState, player::Player, room::Room, shop::roll_offers, shop::ShopItem, ui::Ui,
};

use crate::renderer::Renderer;

pub struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    items: Vec<Vector2>,
    room: Room,
    ui: Ui,
    input: InputState,
    game_over: bool,
    renderer: Renderer,
    wave: i32,
    enemies_per_wave: i32,
    shop_open: bool,
    coins: i32,
    shop_offers: Vec<Option<ShopItem>>,
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
            items: Vec::new(),
            room,
            ui: Ui::new(),
            input: InputState::default(),
            game_over: false,
            renderer,
            wave,
            enemies_per_wave,
            shop_open: false,
            coins: 0,
            shop_offers: Vec::new(),
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
            if self.input.shop_buy_1 {
                self.try_buy_offer(0);
            }
            if self.input.shop_buy_2 {
                self.try_buy_offer(1);
            }
            if self.input.shop_buy_3 {
                self.try_buy_offer(2);
            }
            if self.input.shop_confirm {
                self.shop_open = false;
                self.wave += 1;
                self.enemies_per_wave += 1;
                self.enemies = Self::spawn_wave_enemies(self.enemies_per_wave, self.room.bounds);
            }
            return;
        }

        let player_pos = self.player.pos();
        let pickup_radius = 14.0;
        let pickup_radius_sq = pickup_radius * pickup_radius;
        let mut picked_up = 0;
        self.items.retain(|item_pos| {
            let dist_sq = (player_pos - *item_pos).length_sqr();
            if dist_sq <= pickup_radius_sq {
                picked_up += 1;
                false
            } else {
                true
            }
        });
        if picked_up > 0 {
            self.player.heal(picked_up);
        }

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
        let before = self.enemies.len();
        // Collect positions of dead enemies for item drop
        let mut dead_positions = vec![];
        self.enemies.retain(|enemy| {
            if enemy.is_dead() {
                dead_positions.push(enemy.pos());
                false
            } else {
                true
            }
        });
        let killed = before.saturating_sub(self.enemies.len());
        if killed > 0 {
            self.coins += killed as i32;
            use rand::Rng;
            let mut rng = rand::thread_rng();
            for pos in dead_positions {
                // 30% de chance de drop
                if rng.gen_bool(0.3) {
                    self.items.push(pos);
                }
            }
        }

        if self.enemies.is_empty() && !self.shop_open {
            self.shop_open = true;
            self.bullets.clear();
            self.shop_offers = roll_offers(3).into_iter().map(Some).collect();
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

        for item_pos in &self.items {
            d.draw_circle_v(*item_pos, 6.0, Color::GREEN);
        }

        self.ui.draw(
            d,
            fps,
            self.player.hp,
            self.enemies.len() as i32,
            self.player.upgrade_label(),
            self.wave,
            self.shop_open,
            self.coins,
            &self.shop_offers,
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

    fn try_buy_offer(&mut self, index: usize) {
        let offer = match self.shop_offers.get(index) {
            Some(Some(item)) => *item,
            _ => return,
        };
        if self.coins < offer.cost {
            return;
        }
        self.coins -= offer.cost;
        offer.apply(&mut self.player);
        if let Some(slot) = self.shop_offers.get_mut(index) {
            *slot = None;
        }
    }
}
