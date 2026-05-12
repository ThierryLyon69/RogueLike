use raylib::prelude::*;

#[derive(Clone, Copy)]
pub enum EnemyKind {
    Chaser,
    Shooter,
    Skeleton,
}

pub struct Enemy {
    pos: Vector2,
    hp: i32,
    max_hp: i32,
    kind: EnemyKind,
    attack_cooldown: f32,
    stats: EnemyStats,
}

struct EnemyStats {
    max_hp: i32,
    detection_radius: f32,
    attack_radius: f32,
    speed: f32,
    damage: i32,
    attack_interval: f32,
}

impl Enemy {
    pub fn new(kind: EnemyKind, pos: Vector2, wave: i32) -> Self {
        let stats = scaled_stats(kind, wave);
        let max_hp = stats.max_hp;
        Self {
            pos,
            hp: max_hp,
            max_hp,
            kind,
            attack_cooldown: 0.0,
            stats,
        }
    }

    pub fn update(&mut self, player_pos: Vector2, dt: f32) {
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown = (self.attack_cooldown - dt).max(0.0);
        }

        match self.kind {
            EnemyKind::Skeleton => {
                let to_player = player_pos - self.pos;
                let dist = to_player.length();
                if dist < self.stats.detection_radius && dist > self.stats.attack_radius {
                    let dir = to_player.normalized();
                    self.pos += dir * self.stats.speed * dt;
                }
            }
            _ => {}
        }
    }

    pub fn pos(&self) -> Vector2 {
        self.pos
    }

    pub fn try_melee_attack(&mut self, player_pos: Vector2) -> Option<i32> {
        match self.kind {
            EnemyKind::Skeleton => {
                let dist = (player_pos - self.pos).length();
                if dist <= self.stats.attack_radius && self.attack_cooldown <= 0.0 {
                    self.attack_cooldown = self.stats.attack_interval;
                    return Some(self.stats.damage);
                }
                None
            }
            _ => None,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let size = 18.0;
        let half = size * 0.5;
        let top_left = Vector2::new(self.pos.x - half, self.pos.y - half);
        let color = match self.kind {
            EnemyKind::Chaser => Color::ORANGE,
            EnemyKind::Shooter => Color::PURPLE,
            EnemyKind::Skeleton => Color::GRAY,
        };

        d.draw_rectangle_v(top_left, Vector2::new(size, size), color);

        let bar_width = size;
        let bar_height = 4.0;
        let bar_top_left = Vector2::new(self.pos.x - bar_width * 0.5, top_left.y - 8.0);
        let hp_ratio = if self.max_hp > 0 {
            (self.hp.max(0) as f32) / (self.max_hp as f32)
        } else {
            0.0
        };
        let fill_width = (bar_width * hp_ratio).max(0.0).min(bar_width);

        d.draw_rectangle_v(
            bar_top_left,
            Vector2::new(bar_width, bar_height),
            Color::DARKGRAY,
        );
        d.draw_rectangle_v(
            bar_top_left,
            Vector2::new(fill_width, bar_height),
            Color::RED,
        );
    }

    pub fn hitbox(&self) -> Rectangle {
        let size = 18.0;
        let half = size * 0.5;
        Rectangle::new(self.pos.x - half, self.pos.y - half, size, size)
    }

    pub fn damage(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}

fn scaled_stats(kind: EnemyKind, wave: i32) -> EnemyStats {
    let wave_idx = (wave - 1).max(0) as f32;
    let (base_hp, base_detection, base_attack, base_speed, base_damage, base_interval) =
        match kind {
            EnemyKind::Chaser => (3, 200.0, 20.0, 90.0, 1, 0.8),
            EnemyKind::Shooter => (2, 240.0, 18.0, 70.0, 1, 0.9),
            EnemyKind::Skeleton => (4, 220.0, 22.0, 80.0, 1, 0.8),
        };

    let max_hp = base_hp + wave_idx.round() as i32;
    let detection_radius = base_detection + wave_idx * 4.0;
    let attack_radius = base_attack + wave_idx * 0.4;
    let speed = base_speed + wave_idx * 1.2;
    let damage = base_damage + (wave_idx / 3.0).floor() as i32;
    let attack_interval = (base_interval - wave_idx * 0.01).max(0.5);

    EnemyStats {
        max_hp,
        detection_radius,
        attack_radius,
        speed,
        damage,
        attack_interval,
    }
}
