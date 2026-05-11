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
        }
    }

    pub fn update(&mut self, _player_pos: Vector2, _dt: f32) {}

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
