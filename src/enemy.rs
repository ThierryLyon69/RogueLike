use raylib::prelude::*;

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
}

impl Enemy {
    pub fn new(kind: EnemyKind, pos: Vector2) -> Self {
        let max_hp = match kind {
            EnemyKind::Chaser => 3,
            EnemyKind::Shooter => 2,
            EnemyKind::Skeleton => 4,
        };
        Self {
            pos,
            hp: max_hp,
            max_hp,
            kind,
            attack_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, player_pos: Vector2, dt: f32) {
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown = (self.attack_cooldown - dt).max(0.0);
        }

        match self.kind {
            EnemyKind::Skeleton => {
                let detection_radius = 220.0;
                let attack_radius = 22.0;
                let speed = 80.0;
                let to_player = player_pos - self.pos;
                let dist = to_player.length();
                if dist < detection_radius && dist > attack_radius {
                    let dir = to_player.normalized();
                    self.pos += dir * speed * dt;
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
                let attack_radius = 32.0;
                let attack_interval = 0.8;
                let damage = 1;
                let dist = (player_pos - self.pos).length();
                if dist <= attack_radius && self.attack_cooldown <= 0.0 {
                    self.attack_cooldown = attack_interval;
                    return Some(damage);
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
